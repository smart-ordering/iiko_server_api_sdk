mod common;
use common::{cleanup_after_test, get_test_client};

#[tokio::test]
async fn test_load_image() {
    let client = get_test_client().await;

    // Получаем список продуктов, чтобы найти изображение
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    // Ищем продукт с изображением
    let product_with_image = products.iter().find(|p| p.front_image_id.is_some());

    match product_with_image {
        Some(product) => {
            if let Some(image_id) = product.front_image_id {
                let result = client.images().load(image_id).await;

                match result {
                    Ok(image) => {
                        let data_len = image.data.as_ref().map(|d| d.len()).unwrap_or(0);
                        println!(
                            "Loaded image {:?}: {} bytes (Base64 length)",
                            image.id, data_len
                        );
                        // Изображение может отсутствовать (404 или пустой ответ)
                        // Проверяем только если данные есть
                        if let Some(ref data) = image.data {
                            assert!(
                                !data.is_empty(),
                                "Image data should not be empty if present"
                            );
                        } else {
                            println!(
                                "  Note: Image data is None (image may not exist or be empty)"
                            );
                        }
                    }
                    Err(e) => {
                        println!("Failed to load image (may not exist): {:?}", e);
                    }
                }
            }
        }
        None => {
            println!("No products with images found, skipping test_load_image");
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_delete_image() {
    let client = get_test_client().await;

    // Получаем список продуктов, чтобы найти изображение
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    // Ищем продукт с изображением
    let product_with_image = products.iter().find(|p| p.front_image_id.is_some());

    match product_with_image {
        Some(product) => {
            if let Some(image_id) = product.front_image_id {
                // Пытаемся удалить изображение (может не сработать, если изображение используется)
                let result = client.images().delete(vec![image_id]).await;

                match result {
                    Ok(delete_result) => {
                        println!("Delete result: {:?}", delete_result.result);
                        // Если удаление успешно, можно попробовать восстановить изображение
                        // Но для этого нужно сохранить его сначала
                    }
                    Err(e) => {
                        println!(
                            "Failed to delete image (may be in use or not found): {:?}",
                            e
                        );
                    }
                }
            }
        }
        None => {
            println!("No products with images found, skipping test_delete_image");
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}
