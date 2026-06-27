mod common;

use chrono::{Duration, Local};
use common::{cleanup_after_test, get_test_client};

#[tokio::test]
#[ignore = "read-only live smoke; requires IIKO_* credentials for a safe test integration"]
async fn qitech_internal_transfer_list_read_only() {
    let client = get_test_client().await;

    let today = Local::now().date_naive();
    let date_from = (today - Duration::days(30)).format("%Y-%m-%d").to_string();
    let date_to = today.format("%Y-%m-%d").to_string();

    let result = client
        .documents()
        .list_internal_transfers(date_from, date_to, None, Some(-1))
        .await
        .expect("Failed to list internal transfers");

    println!(
        "internal transfers: result={}, count={}, revision={:?}",
        result.result,
        result.response.len(),
        result.revision
    );

    cleanup_after_test(&client).await;
}
