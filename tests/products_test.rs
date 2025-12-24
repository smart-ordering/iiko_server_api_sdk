mod common;
use common::{cleanup_after_test, get_test_client};

#[tokio::test]
async fn test_list_products() {
    let client = get_test_client().await;

    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    assert!(!products.is_empty());
    println!("Found {} products", products.len());

    for product in products.iter().take(3) {
        println!(
            "Product: {} (type: {:?}, num: {:?})",
            product.name.as_deref().unwrap_or("N/A"),
            product.r#type,
            product.num
        );
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_list_products_post() {
    let client = get_test_client().await;

    let products = client
        .products()
        .list_post(Some(false), None, None, None, None, None, None, None)
        .await
        .expect("Failed to get products via POST");

    assert!(!products.is_empty());
    println!("Found {} products via POST", products.len());

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_list_product_groups() {
    let client = get_test_client().await;

    let groups = client
        .products()
        .list_groups(Some(false))
        .await
        .expect("Failed to get product groups");

    assert!(!groups.is_empty());
    println!("Found {} product groups", groups.len());

    for group in groups.iter().take(3) {
        println!(
            "Group: {} (num: {:?})",
            group.name.as_deref().unwrap_or("N/A"),
            group.num
        );
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}
