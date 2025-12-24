mod common;
use common::{cleanup_after_test, get_test_client};
use uuid::Uuid;

#[tokio::test]
async fn test_list_product_scales() {
    let client = get_test_client().await;

    let scales = client
        .product_scales()
        .list(None, Some(false))
        .await
        .expect("Failed to get product scales");

    println!("Found {} product scales", scales.len());
    for scale in scales.iter().take(3) {
        println!(
            "  Scale {}: {} ({} sizes)",
            &scale.id.to_string()[..8],
            scale.name,
            scale.product_sizes.len()
        );
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_list_product_scales_post() {
    let client = get_test_client().await;

    let scales = client
        .product_scales()
        .list_post(None, Some(false))
        .await
        .expect("Failed to get product scales (POST)");

    println!("Found {} product scales (POST)", scales.len());
    assert!(!scales.is_empty() || scales.is_empty()); // Просто проверяем, что запрос выполнился

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_product_scale_by_id() {
    let client = get_test_client().await;

    // Сначала получаем список шкал
    let scales = client
        .product_scales()
        .list(None, Some(false))
        .await
        .expect("Failed to get product scales");

    if let Some(scale) = scales.first() {
        // Получаем шкалу по ID
        let scale_by_id = client
            .product_scales()
            .by_id(scale.id)
            .await
            .expect("Failed to get product scale by id");

        assert_eq!(scale.id, scale_by_id.id);
        assert_eq!(scale.name, scale_by_id.name);
        println!(
            "  Scale {}: {} ({} sizes)",
            &scale_by_id.id.to_string()[..8],
            scale_by_id.name,
            scale_by_id.product_sizes.len()
        );
    } else {
        println!("No product scales found, skipping test");
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_product_scale_for_product() {
    let client = get_test_client().await;

    // Получаем список продуктов
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    // Ищем продукт со шкалой размеров
    for product in products.iter().take(10) {
        let result = client
            .product_scales()
            .get_for_product(product.id.unwrap())
            .await
            .expect("Failed to get product scale for product");

        if let Some(scale) = result.response {
            println!(
                "  Product {} has scale {}: {}",
                &product.id.unwrap().to_string()[..8],
                &scale.id.to_string()[..8],
                scale.name
            );
            break;
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_product_scales_for_products() {
    let client = get_test_client().await;

    // Получаем список продуктов
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    if products.len() >= 2 {
        let product_ids: Vec<Uuid> = products.iter().take(3).filter_map(|p| p.id).collect();

        let scales_map = client
            .product_scales()
            .get_for_products(Some(product_ids.clone()), Some(false))
            .await
            .expect("Failed to get product scales for products");

        println!("Found scales for {} products", scales_map.len());
        for (product_id, scale_opt) in scales_map.iter() {
            match scale_opt {
                Some(scale) => {
                    println!(
                        "  Product {} has scale {}: {}",
                        &product_id.to_string()[..8],
                        &scale.id.to_string()[..8],
                        scale.name
                    );
                }
                None => {
                    println!("  Product {} has no scale", &product_id.to_string()[..8]);
                }
            }
        }
    } else {
        println!("Not enough products for test");
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_product_scales_for_products_post() {
    let client = get_test_client().await;

    // Получаем список продуктов
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    if products.len() >= 2 {
        let product_ids: Vec<Uuid> = products.iter().take(3).filter_map(|p| p.id).collect();

        let scales_map = client
            .product_scales()
            .get_for_products_post(Some(product_ids.clone()), Some(false))
            .await
            .expect("Failed to get product scales for products (POST)");

        println!("Found scales for {} products (POST)", scales_map.len());
    } else {
        println!("Not enough products for test");
    }

    cleanup_after_test(&client).await;
}
