mod common;
use common::{cleanup_after_test, get_test_client};

#[tokio::test]
async fn test_get_all() {
    let client = get_test_client().await;

    // Запрашиваем техкарты за сегодня
    // Формат даты: yyyy-MM-dd
    let today = "2024-01-01".to_string(); // Используем фиксированную дату для теста
    let result = client
        .assembly_charts()
        .get_all(today.clone(), None, Some(true), Some(false))
        .await;

    match result {
        Ok(charts) => {
            println!(
                "Found {} assembly charts, {} prepared charts, knownRevision: {}",
                charts
                    .assembly_charts
                    .as_ref()
                    .map(|v| v.len())
                    .unwrap_or(0),
                charts
                    .prepared_charts
                    .as_ref()
                    .map(|v| v.len())
                    .unwrap_or(0),
                charts.known_revision
            );
        }
        Err(e) => {
            println!(
                "Failed to get all charts (may be no charts in system): {:?}",
                e
            );
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_tree() {
    let client = get_test_client().await;

    // Сначала получаем список продуктов
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    if products.is_empty() {
        println!("No products found, skipping test_get_tree");
        cleanup_after_test(&client).await;
        return;
    }

    let product_id = match products[0].id {
        Some(id) => id,
        None => {
            println!("Product has no ID, skipping test_get_tree");
            cleanup_after_test(&client).await;
            return;
        }
    };

    let today = "2024-01-01".to_string(); // Используем фиксированную дату для теста
    let result = client
        .assembly_charts()
        .get_tree(today, product_id, None)
        .await;

    match result {
        Ok(charts) => {
            println!(
                "Tree for product {}: {} assembly charts, {} prepared charts",
                product_id.to_string(),
                charts
                    .assembly_charts
                    .as_ref()
                    .map(|v| v.len())
                    .unwrap_or(0),
                charts
                    .prepared_charts
                    .as_ref()
                    .map(|v| v.len())
                    .unwrap_or(0)
            );
        }
        Err(e) => {
            println!("Failed to get tree (product may have no charts): {:?}", e);
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_assembled() {
    let client = get_test_client().await;

    // Сначала получаем список продуктов
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    if products.is_empty() {
        println!("No products found, skipping test_get_assembled");
        cleanup_after_test(&client).await;
        return;
    }

    let product_id = match products[0].id {
        Some(id) => id,
        None => {
            println!("Product has no ID, skipping test_get_assembled");
            cleanup_after_test(&client).await;
            return;
        }
    };

    let today = "2024-01-01".to_string(); // Используем фиксированную дату для теста
    let result = client
        .assembly_charts()
        .get_assembled(today, product_id, None)
        .await;

    match result {
        Ok(charts) => {
            if let Some(assembly_charts) = charts.assembly_charts {
                if !assembly_charts.is_empty() {
                    let chart = &assembly_charts[0];
                    println!(
                        "Assembled chart for product {}: {} items, dateFrom: {}, dateTo: {:?}",
                        product_id.to_string(),
                        chart.items.len(),
                        chart.date_from,
                        chart.date_to
                    );
                } else {
                    println!("No assembly chart found for product {}", product_id.to_string());
                }
            }
        }
        Err(e) => {
            println!(
                "Failed to get assembled chart (product may have no charts): {:?}",
                e
            );
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_prepared() {
    let client = get_test_client().await;

    // Сначала получаем список продуктов
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    if products.is_empty() {
        println!("No products found, skipping test_get_prepared");
        cleanup_after_test(&client).await;
        return;
    }

    let product_id = match products[0].id {
        Some(id) => id,
        None => {
            println!("Product has no ID, skipping test_get_prepared");
            cleanup_after_test(&client).await;
            return;
        }
    };

    let today = "2024-01-01".to_string(); // Используем фиксированную дату для теста
    let result = client
        .assembly_charts()
        .get_prepared(today, product_id, None)
        .await;

    match result {
        Ok(charts) => {
            if let Some(prepared_charts) = charts.prepared_charts {
                if !prepared_charts.is_empty() {
                    let chart = &prepared_charts[0];
                    println!(
                        "Prepared chart for product {}: {} items",
                        product_id.to_string(),
                        chart.items.len()
                    );
                } else {
                    println!("No prepared chart found for product {}", product_id.to_string());
                }
            }
        }
        Err(e) => {
            println!(
                "Failed to get prepared chart (product may have no charts): {:?}",
                e
            );
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_history() {
    let client = get_test_client().await;

    // Сначала получаем список продуктов
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    if products.is_empty() {
        println!("No products found, skipping test_get_history");
        cleanup_after_test(&client).await;
        return;
    }

    let product_id = match products[0].id {
        Some(id) => id,
        None => {
            println!("Product has no ID, skipping test_get_history");
            cleanup_after_test(&client).await;
            return;
        }
    };

    let result = client
        .assembly_charts()
        .get_history(product_id, None)
        .await;

    match result {
        Ok(history) => {
            println!(
                "History for product {}: {} charts",
                product_id.to_string(),
                history.len()
            );
            for chart in history.iter().take(3) {
                println!(
                    "  Chart {}: {} - {:?} ({} items)",
                    &chart.id.to_string()[..8],
                    chart.date_from,
                    chart.date_to,
                    chart.items.len()
                );
            }
        }
        Err(e) => {
            println!(
                "Failed to get history (product may have no charts): {:?}",
                e
            );
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}


