mod common;
use common::{cleanup_after_test, get_test_client};

#[tokio::test]
async fn test_typed_departments() {
    let client = get_test_client().await;

    let departments = client
        .corporation()
        .get_departments(None)
        .await
        .expect("Failed to fetch departments");

    assert!(!departments.is_empty());
    println!("Found {} departments", departments.len());

    for dept in departments.iter() {
        println!("{:#?}", dept);
        // println!(
        //     "Department: {} (type: {})",
        //     dept.name.as_deref().unwrap_or("N/A"),
        //     dept.r#type
        // );
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_typed_stores() {
    let client = get_test_client().await;

    let stores = client
        .corporation()
        .get_stores(None)
        .await
        .expect("Failed to fetch stores");

    assert!(!stores.is_empty());
    println!("Found {} stores", stores.len());

    for store in stores.iter() {
        println!("{:#?}", store);
        // println!(
        //     "Store: {} (type: {})",
        //     store.name.as_deref().unwrap_or("N/A"),
        //     store.r#type
        // );
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_typed_groups() {
    let client = get_test_client().await;

    let groups = client
        .corporation()
        .get_groups(None)
        .await
        .expect("Failed to fetch groups");

    assert!(!groups.is_empty());
    println!("Found {} groups", groups.len());

    for group in groups.iter().take(2) {
        println!(
            "Group: {} (service mode: {})",
            group.name, group.group_service_mode
        );
        println!(
            "  Points of sale: {}",
            group.point_of_sale_dtoes.items.len()
        );
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_typed_terminals() {
    let client = get_test_client().await;

    let terminals = client
        .corporation()
        .get_terminals(None)
        .await
        .expect("Failed to fetch terminals");

    assert!(!terminals.is_empty());
    println!("Found {} terminals", terminals.len());

    for terminal in terminals.iter().take(3) {
        println!(
            "Terminal: {} (anonymous: {})",
            terminal.name, terminal.anonymous
        );
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_typed_search_terminals() {
    let client = get_test_client().await;

    // Search for front terminals (anonymous=false)
    let front_terminals = client
        .corporation()
        .search_terminals(None, None, Some(false))
        .await
        .expect("Failed to search terminals");

    println!("Found {} front terminals", front_terminals.len());

    for terminal in front_terminals.iter().take(3) {
        println!(
            "Front terminal: {} (computer: {:?})",
            terminal.name, terminal.computer_name
        );
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_typed_settings() {
    let client = get_test_client().await;

    let settings = client
        .corporation()
        .get_settings()
        .await
        .expect("Failed to fetch corporation settings");

    println!("VAT accounting: {}", settings.vat_accounting);
    assert!(!settings.vat_accounting.is_empty());

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}
