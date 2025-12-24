use iiko_server_api_sdk::{IikoClient, IikoConfig};

#[tokio::test]
async fn test_auth() {
    dotenvy::dotenv().ok();

    let config = IikoConfig::from_env().expect("Failed to load config from env");
    let client = IikoClient::new(config).expect("Failed to create client");

    let token = client.auth().login().await.expect("Failed to authenticate");
    assert!(!token.is_empty(), "Token should not be empty");
    println!("Auth token: {}", token);
}

#[tokio::test]
async fn test_departments() {
    dotenvy::dotenv().ok();

    let config = IikoConfig::from_env().expect("Failed to load config from env");
    let client = IikoClient::new(config).expect("Failed to create client");

    let response = client
        .get_with_params("corporation/departments/", &[("revisionFrom", "-1")])
        .await
        .expect("Failed to fetch departments");

    println!(
        "Departments response (first 500 chars): {}",
        &response[..response.len().min(500)]
    );
    assert!(!response.is_empty());
}

#[tokio::test]
async fn test_stores() {
    dotenvy::dotenv().ok();

    let config = IikoConfig::from_env().expect("Failed to load config from env");
    let client = IikoClient::new(config).expect("Failed to create client");

    let response = client
        .get_with_params("corporation/stores/", &[("revisionFrom", "-1")])
        .await
        .expect("Failed to fetch stores");

    println!(
        "Stores response (first 500 chars): {}",
        &response[..response.len().min(500)]
    );
    assert!(!response.is_empty());
}

#[tokio::test]
async fn test_groups() {
    dotenvy::dotenv().ok();

    let config = IikoConfig::from_env().expect("Failed to load config from env");
    let client = IikoClient::new(config).expect("Failed to create client");

    let response = client
        .get_with_params("corporation/groups/", &[("revisionFrom", "-1")])
        .await
        .expect("Failed to fetch groups");

    println!(
        "Groups response (first 500 chars): {}",
        &response[..response.len().min(500)]
    );
    assert!(!response.is_empty());
}

#[tokio::test]
async fn test_terminals() {
    dotenvy::dotenv().ok();

    let config = IikoConfig::from_env().expect("Failed to load config from env");
    let client = IikoClient::new(config).expect("Failed to create client");

    let response = client
        .get_with_params("corporation/terminals/", &[("revisionFrom", "-1")])
        .await
        .expect("Failed to fetch terminals");

    println!(
        "Terminals response (first 500 chars): {}",
        &response[..response.len().min(500)]
    );
    assert!(!response.is_empty());
}

#[tokio::test]
async fn test_products_list() {
    dotenvy::dotenv().ok();

    let config = IikoConfig::from_env().expect("Failed to load config from env");
    let client = IikoClient::new(config).expect("Failed to create client");

    let response = client
        .get_with_params("v2/entities/products/list", &[("includeDeleted", "false")])
        .await
        .expect("Failed to fetch products");

    println!(
        "Products response (first 500 chars): {}",
        &response[..response.len().min(500)]
    );
    assert!(!response.is_empty());
}

#[tokio::test]
async fn test_product_groups() {
    dotenvy::dotenv().ok();

    let config = IikoConfig::from_env().expect("Failed to load config from env");
    let client = IikoClient::new(config).expect("Failed to create client");

    let response = client
        .get_with_params(
            "v2/entities/products/group/list",
            &[("includeDeleted", "false")],
        )
        .await
        .expect("Failed to fetch product groups");

    println!(
        "Product groups response (first 500 chars): {}",
        &response[..response.len().min(500)]
    );
    assert!(!response.is_empty());
}

#[tokio::test]
async fn test_product_scales() {
    dotenvy::dotenv().ok();

    let config = IikoConfig::from_env().expect("Failed to load config from env");
    let client = IikoClient::new(config).expect("Failed to create client");

    let response = client
        .post_form("v2/entities/productScales", &[("includeDeleted", "false")])
        .await
        .expect("Failed to fetch product scales");

    let preview: String = response.chars().take(500).collect();
    println!("Product scales response (first 500 chars): {}", preview);
    assert!(!response.is_empty());
}

#[tokio::test]
async fn test_corporation_settings() {
    dotenvy::dotenv().ok();

    let config = IikoConfig::from_env().expect("Failed to load config from env");
    let client = IikoClient::new(config).expect("Failed to create client");

    let response = client
        .get("v2/corporation/settings")
        .await
        .expect("Failed to fetch corporation settings");

    println!(
        "Corporation settings response (first 500 chars): {}",
        &response[..response.len().min(500)]
    );
    assert!(!response.is_empty());
}

#[tokio::test]
async fn test_search_department() {
    dotenvy::dotenv().ok();

    let config = IikoConfig::from_env().expect("Failed to load config from env");
    let client = IikoClient::new(config).expect("Failed to create client");

    // Try to search (may fail if no code provided, but tests the endpoint)
    let response = client
        .get_with_params("corporation/departments/search", &[])
        .await;

    match response {
        Ok(resp) => println!(
            "Search department response (first 500 chars): {}",
            &resp[..resp.len().min(500)]
        ),
        Err(e) => println!("Search department error (expected if no code): {:?}", e),
    }
}

#[tokio::test]
async fn test_session_caching() {
    dotenvy::dotenv().ok();

    let config = IikoConfig::from_env().expect("Failed to load config from env");
    let client = IikoClient::new(config).expect("Failed to create client");

    // First auth
    let token1 = client.auth().login().await.expect("Failed to authenticate");

    // Second auth should use cached session
    let token2 = client.auth().login().await.expect("Failed to authenticate");

    assert_eq!(token1, token2, "Session should be cached");

    // Logout and re-auth
    let _logout_result = client.auth().logout().await.expect("Failed to logout");
    let token3 = client
        .auth()
        .login()
        .await
        .expect("Failed to authenticate after logout");

    // Token might be the same or different, but should be valid
    assert!(!token3.is_empty());
}
