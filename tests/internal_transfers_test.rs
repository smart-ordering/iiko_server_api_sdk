mod common;

use chrono::{Duration, Local};
use common::{cleanup_after_test, get_test_client};
use iiko_server_api_sdk::{DocumentStatus, InternalTransferDto, InternalTransferItemDto};
use std::env;
use uuid::Uuid;

#[tokio::test]
#[ignore = "read-only live smoke; requires IIKO_* credentials for a safe test integration"]
async fn qitech_internal_transfer_list_read_only() {
    let client = get_test_client().await;

    let today = Local::now().date_naive();
    let date_from = (today - Duration::days(30)).format("%Y-%m-%d").to_string();
    let date_to = today.format("%Y-%m-%d").to_string();

    let result = client
        .documents()
        .list_internal_transfers(date_from, date_to, None, Some(-1))
        .await
        .expect("Failed to list internal transfers");

    println!(
        "internal transfers: result={}, count={}, revision={:?}",
        result.result,
        result.response.len(),
        result.revision
    );

    cleanup_after_test(&client).await;
}

#[tokio::test]
#[ignore = "live write smoke; creates a real internalTransfer in the configured safe iiko integration"]
async fn qitech_internal_transfer_create_live() {
    if env::var("IIKO_ALLOW_INTERNAL_TRANSFER_WRITE").ok().as_deref() != Some("1") {
        panic!("Set IIKO_ALLOW_INTERNAL_TRANSFER_WRITE=1 to create a real iiko internalTransfer");
    }

    let client = get_test_client().await;
    let stores = client
        .corporation()
        .get_stores(None)
        .await
        .expect("Failed to fetch stores");
    assert!(stores.len() >= 2, "Need at least two stores for internalTransfer");

    let store_from_id = env::var("IIKO_TEST_INTERNAL_TRANSFER_STORE_FROM_ID")
        .ok()
        .and_then(|value| Uuid::parse_str(&value).ok())
        .unwrap_or(stores[0].id);
    let store_to_id = env::var("IIKO_TEST_INTERNAL_TRANSFER_STORE_TO_ID")
        .ok()
        .and_then(|value| Uuid::parse_str(&value).ok())
        .unwrap_or(stores[1].id);
    assert_ne!(store_from_id, store_to_id, "source and destination stores must differ");

    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to fetch products");
    let product_id = env::var("IIKO_TEST_INTERNAL_TRANSFER_PRODUCT_ID")
        .ok()
        .and_then(|value| Uuid::parse_str(&value).ok())
        .or_else(|| products.iter().find_map(|product| product.id))
        .expect("Need at least one active product for internalTransfer");
    let amount = env::var("IIKO_TEST_INTERNAL_TRANSFER_AMOUNT")
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(0.001);
    assert!(amount > 0.0, "amount must be positive");

    let status = match env::var("IIKO_TEST_INTERNAL_TRANSFER_STATUS")
        .unwrap_or_else(|_| "NEW".to_string())
        .to_uppercase()
        .as_str()
    {
        "PROCESSED" => DocumentStatus::Processed,
        "NEW" => DocumentStatus::New,
        other => panic!("Unsupported IIKO_TEST_INTERNAL_TRANSFER_STATUS={other}; use NEW or PROCESSED"),
    };

    let document_number = format!("NAKLAD-QITECH-IT-{}", Local::now().timestamp_millis());
    let transfer = InternalTransferDto {
        id: None,
        date_incoming: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
        document_number: Some(document_number.clone()),
        status,
        conception_id: None,
        comment: Some("NAKLAD live qitech internalTransfer smoke".to_string()),
        store_from_id,
        store_to_id,
        items: vec![InternalTransferItemDto {
            num: None,
            product_id,
            amount,
            measure_unit_id: None,
            container_id: None,
            cost: None,
        }],
    };

    let result = client
        .documents()
        .upsert_internal_transfer(transfer)
        .await
        .expect("Failed to create internalTransfer");

    println!(
        "create internalTransfer result={}, id={:?}, number={:?}, errors={:?}",
        result.result, result.response.id, result.response.document_number, result.errors
    );
    assert_eq!(result.result.to_uppercase(), "SUCCESS");
    assert_eq!(result.response.document_number.as_deref(), Some(document_number.as_str()));

    if let Some(id) = result.response.id {
        let by_id = client
            .documents()
            .get_internal_transfer_by_id(id)
            .await
            .expect("Failed to load internalTransfer by id");
        assert_eq!(by_id.id, Some(id));
        assert_eq!(by_id.document_number.as_deref(), Some(document_number.as_str()));
    }

    let by_number = client
        .documents()
        .get_internal_transfers_by_number(&document_number)
        .await
        .expect("Failed to load internalTransfer by number");
    if !by_number
        .iter()
        .any(|transfer| transfer.document_number.as_deref() == Some(document_number.as_str()))
    {
        println!(
            "created internalTransfer was not returned by number immediately; byNumber count={}",
            by_number.len()
        );
    }

    cleanup_after_test(&client).await;
}
