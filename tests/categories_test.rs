mod common;
use common::{cleanup_after_test, get_test_client};

#[tokio::test]
async fn test_list_categories() {
    let client = get_test_client().await;

    let categories = client
        .products()
        .list_categories(Some(false), None, None)
        .await
        .expect("Failed to get categories");

    println!("Found {} categories", categories.len());
    for category in categories.iter().take(3) {
        println!(
            "Category: {} (id: {}, deleted: {})",
            category.name.as_deref().unwrap_or("N/A"),
            category.id,
            category.deleted
        );
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_list_categories_post() {
    let client = get_test_client().await;

    let categories = client
        .products()
        .list_categories_post(Some(false), None, None)
        .await
        .expect("Failed to get categories via POST");

    println!("Found {} categories via POST", categories.len());

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_save_category() {
    let client = get_test_client().await;

    // Создаем новую категорию
    let result = client
        .products()
        .save_category("Test Category".to_string())
        .await;

    match result {
        Ok(category_result) => {
            println!("Category saved: {:?}", category_result.response);
            assert_eq!(category_result.result, "SUCCESS");

            // Удаляем созданную категорию для очистки
            let _ = client
                .products()
                .delete_category(category_result.response.id.clone())
                .await;
        }
        Err(e) => {
            println!("Failed to save category (may already exist): {:?}", e);
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}
