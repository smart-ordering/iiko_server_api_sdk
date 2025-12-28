mod common;
use common::{cleanup_after_test, get_test_client};

#[tokio::test]
async fn test_entities_list_discount_and_payment_types() {
    let client = get_test_client().await;
    let entities = client.entities();

    let result = entities
        .list(
            &[
                iiko_server_api_sdk::EntityType::DiscountType,
                iiko_server_api_sdk::EntityType::PaymentType,
            ],
            Some(false),
            None,
        )
        .await;

    match result {
        Ok(entities_list) => {
            println!("Получено сущностей: {}", entities_list.len());
            for entity in entities_list.iter().take(10) {
                println!(
                    "ID: {}, RootType: {}, Deleted: {}, Code: {:?}, Name: {}",
                    entity.id, entity.root_type, entity.deleted, entity.code, entity.name
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения сущностей: {:?}", e);
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_entities_list_tax_category() {
    let client = get_test_client().await;
    let entities = client.entities();

    let result = entities
        .list(
            &[iiko_server_api_sdk::EntityType::TaxCategory],
            None,
            None,
        )
        .await;

    match result {
        Ok(entities_list) => {
            println!("Получено налоговых категорий: {}", entities_list.len());
            for entity in entities_list.iter().take(5) {
                println!(
                    "ID: {}, Name: {}, Code: {:?}",
                    entity.id, entity.name, entity.code
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения налоговых категорий: {:?}", e);
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_entities_list_with_extended_fields() {
    let client = get_test_client().await;
    let entities = client.entities();

    let result = entities
        .list_with_extended_fields(
            &[
                iiko_server_api_sdk::EntityType::OrderType,
                iiko_server_api_sdk::EntityType::TaxCategory,
            ],
            None,
            None,
        )
        .await;

    match result {
        Ok(entities_list) => {
            println!("Получено сущностей с расширенными полями: {}", entities_list.len());
            for entity in entities_list.iter().take(5) {
                let base = entity.base();
                println!(
                    "ID: {}, RootType: {}, Name: {}",
                    base.id, base.root_type, base.name
                );
                match entity {
                    iiko_server_api_sdk::ReferenceEntity::TaxCategory(tax) => {
                        if let Some(vat) = tax.vat_percent {
                            println!("  VAT: {}%", vat);
                        }
                    }
                    iiko_server_api_sdk::ReferenceEntity::OrderType(order) => {
                        if let Some(service_type) = &order.order_service_type {
                            println!("  ServiceType: {:?}", service_type);
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            println!("Ошибка получения сущностей с расширенными полями: {:?}", e);
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_entities_get_ids_account() {
    let client = get_test_client().await;
    let entities = client.entities();

    let result = entities
        .get_ids(
            iiko_server_api_sdk::EntityType::Account,
            Some(false),
            None,
        )
        .await;

    match result {
        Ok(ids) => {
            println!("Получено ID счетов: {}", ids.len());
            for id in ids.iter().take(10) {
                println!("Account ID: {}", id);
            }
        }
        Err(e) => {
            println!("Ошибка получения ID счетов: {:?}", e);
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_entities_get_ids_payment_type() {
    let client = get_test_client().await;
    let entities = client.entities();

    let result = entities
        .get_ids(
            iiko_server_api_sdk::EntityType::PaymentType,
            None,
            None,
        )
        .await;

    match result {
        Ok(ids) => {
            println!("Получено ID типов оплат: {}", ids.len());
            for id in ids.iter().take(10) {
                println!("PaymentType ID: {}", id);
            }
        }
        Err(e) => {
            println!("Ошибка получения ID типов оплат: {:?}", e);
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_entities_list_with_revision() {
    let client = get_test_client().await;
    let entities = client.entities();

    let result = entities
        .list(
            &[iiko_server_api_sdk::EntityType::DiscountType],
            None,
            Some(-1),
        )
        .await;

    match result {
        Ok(entities_list) => {
            println!("Получено сущностей с ревизией: {}", entities_list.len());
        }
        Err(e) => {
            println!("Ошибка получения сущностей с ревизией: {:?}", e);
        }
    }

    cleanup_after_test(&client).await;
}

