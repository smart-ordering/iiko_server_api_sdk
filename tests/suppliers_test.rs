mod common;

use common::{cleanup_after_test, get_test_client};
use std::collections::HashSet;
use std::env;

#[tokio::test]
async fn test_list_suppliers() {
    let client = get_test_client().await;

    let suppliers = client
        .suppliers()
        .list(None)
        .await
        .expect("Failed to get suppliers");

    assert!(!suppliers.is_empty(), "Suppliers list should not be empty");
    println!("Found {} suppliers", suppliers.len());

    for supplier in suppliers.iter().take(3) {
        println!("Supplier: {} (code: {})", supplier.name, supplier.code);
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_supplier_pricelist() {
    let client = get_test_client().await;

    let suppliers = client
        .suppliers()
        .list(None)
        .await
        .expect("Failed to get suppliers for pricelist test");

    let requested_date = env::var("IIKO_TEST_SUPPLIER_PRICELIST_DATE").ok();
    let preferred_code = env::var("IIKO_TEST_SUPPLIER_CODE")
        .ok()
        .map(|code| code.trim().to_string())
        .filter(|code| !code.is_empty());

    let mut seen_codes = HashSet::new();
    let mut candidate_codes = Vec::new();

    if let Some(code) = preferred_code {
        if seen_codes.insert(code.clone()) {
            candidate_codes.push(code);
        }
    }

    for supplier in suppliers
        .iter()
        .filter(|supplier| !supplier.code.trim().is_empty())
    {
        if seen_codes.insert(supplier.code.clone()) {
            candidate_codes.push(supplier.code.clone());
        }
    }

    assert!(
        !candidate_codes.is_empty(),
        "No supplier codes available to test supplier pricelist endpoint"
    );

    let mut last_error = None;

    for code in candidate_codes {
        match client
            .suppliers()
            .get_pricelist(&code, requested_date.as_deref())
            .await
        {
            Ok(items) => {
                println!(
                    "Supplier pricelist loaded for code {} with {} items{}",
                    code,
                    items.len(),
                    requested_date
                        .as_deref()
                        .map(|date| format!(" at date {}", date))
                        .unwrap_or_default()
                );

                if let Some(item) = items.first() {
                    println!(
                        "First item: native={:?}, supplier={:?}, cost={:?}",
                        item.native_product_name, item.supplier_product_name, item.cost_price
                    );
                }

                cleanup_after_test(&client).await;
                return;
            }
            Err(error) => {
                println!(
                    "Failed to load supplier pricelist for code {}: {:?}",
                    code, error
                );
                last_error = Some(error);
            }
        }
    }

    cleanup_after_test(&client).await;
    panic!(
        "Failed to load supplier pricelist for all tested suppliers. Last error: {:?}",
        last_error
    );
}
