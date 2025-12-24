mod common;
use common::{cleanup_after_test, get_test_client};

#[tokio::test]
async fn test_server_type() {
    let client = get_test_client().await;

    let server_type = client
        .replication()
        .get_server_type()
        .await
        .expect("Failed to get server type");

    println!("Server type: {:?}", server_type);
    // Server type can be CHAIN, REPLICATED_RMS, or STANDALONE_RMS
    match server_type {
        iiko_server_api_sdk::ServerType::Chain => println!("This is a Chain server"),
        iiko_server_api_sdk::ServerType::ReplicatedRms => println!("This is a Replicated RMS"),
        iiko_server_api_sdk::ServerType::StandaloneRms => println!("This is a Standalone RMS"),
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_replication_statuses() {
    let client = get_test_client().await;

    // This endpoint only works on Chain servers
    // On RMS it will return an error
    let result = client.replication().get_statuses().await;

    match result {
        Ok(statuses) => {
            println!("Found {} replication statuses", statuses.len());
            for status in statuses.iter().take(3) {
                println!(
                    "Department: {} - Status: {:?} - Last replication: {:?}",
                    status.department_name.as_deref().unwrap_or("Unknown"),
                    status.status,
                    status.last_replication_date
                );
            }
        }
        Err(e) => {
            // Expected on RMS servers
            println!("Cannot get replication statuses (expected on RMS): {:?}", e);
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_replication_status_by_department() {
    let client = get_test_client().await;

    // First get a department ID
    let departments = client
        .corporation()
        .get_departments(None)
        .await
        .expect("Failed to get departments");

    if let Some(department) = departments
        .iter()
        .find(|d| d.r#type == "DEPARTMENT" || d.r#type == "JURPERSON")
    {
        let result = client
            .replication()
            .get_status_by_department(department.id)
            .await;

        match result {
            Ok(status) => {
                println!(
                    "Replication status for {}: {:?}",
                    department.name.as_deref().unwrap_or("Unknown"),
                    status.status
                );
                if let Some(error) = status.error_message {
                    println!("Error message: {}", error);
                }
            }
            Err(e) => {
                // Expected on RMS servers or if department not in Chain
                println!(
                    "Cannot get replication status (expected on RMS or if not in Chain): {:?}",
                    e
                );
            }
        }
    } else {
        println!("No suitable department found for testing");
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}
