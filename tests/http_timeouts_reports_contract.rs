mod http_contract_support;

use http_contract_support::{MockServer, Reply, pairs};
use iiko_server_api_sdk::IikoError;
use std::time::Duration;
use uuid::Uuid;

fn assert_timeout(error: IikoError, secret: &str, base_url: &str) {
    assert!(matches!(&error, IikoError::Http(message) if message == "request timed out"));
    let message = error.to_string();
    assert!(!message.contains(secret));
    assert!(!message.contains("key="));
    assert!(!message.contains(base_url));
}

#[tokio::test]
async fn sdk_total_timeout_covers_both_response_headers_and_body_reads() {
    for body_delay in [false, true] {
        let mut reply = Reply::ok("payload");
        let delay = Duration::from_millis(1400);
        reply = if body_delay {
            reply.body_chunks(vec![
                (Duration::ZERO, b"pay".to_vec()),
                (delay, b"load".to_vec()),
            ])
        } else {
            reply.delay_headers(delay)
        };
        let server = MockServer::start(vec![Reply::ok("secret-token"), reply]).await;
        let error = tokio::time::timeout(Duration::from_secs(3), server.client(1).get("read"))
            .await
            .expect("SDK total timeout was not enforced")
            .unwrap_err();
        assert_timeout(error, "secret-token", &server.base_url);
        assert_eq!(server.finish().await.len(), 2);
    }
}

#[tokio::test]
async fn zero_sdk_timeout_leaves_the_caller_in_control() {
    let server = MockServer::start(vec![
        Reply::ok("session"),
        Reply::ok("payload").delay_headers(Duration::from_millis(1100)),
    ])
    .await;
    let result = tokio::time::timeout(Duration::from_secs(3), server.client(0).get("read"))
        .await
        .expect("test watchdog expired")
        .unwrap();
    assert_eq!(result, "payload");
    assert_eq!(server.finish().await.len(), 2);
}

#[tokio::test]
async fn report_request_timeout_override_covers_body_and_leaves_other_requests_unchanged() {
    let xml = "<dayDishValues><dayDishValue><date>04.10.2026</date><productId>p1</productId><productName>Ю &amp; X</productName><value>1.5</value></dayDishValue></dayDishValues>";
    let server = MockServer::start(vec![
        Reply::ok("session"),
        Reply::ok(xml).body_chunks(vec![(Duration::from_millis(1100), xml.as_bytes().to_vec())]),
        Reply::ok("ordinary").delay_headers(Duration::from_millis(1400)),
    ])
    .await;
    let client = server.client(1);
    let rows = tokio::time::timeout(
        Duration::from_secs(3),
        client.reports().get_product_expense(
            "department",
            "04.10.2026",
            "04.10.2026",
            Some(0),
            Some(23),
        ),
    )
    .await
    .expect("report override did not finish")
    .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].product_name.as_deref(), Some("Ю & X"));
    assert_eq!(rows[0].value, Some(1.5));
    assert_timeout(
        client.get("ordinary").await.unwrap_err(),
        "session",
        &server.base_url,
    );
    let requests = server.finish().await;
    assert_eq!(
        requests[1].query(),
        pairs(&[
            ("key", "session"),
            ("department", "department"),
            ("dateFrom", "04.10.2026"),
            ("dateTo", "04.10.2026"),
            ("hourFrom", "0"),
            ("hourTo", "23")
        ])
    );
    assert_eq!(requests[2].query(), pairs(&[("key", "session")]));
}

// The SDK exposes a total deadline only. This checks the reqwest primitive used by
// callers that need separate per-read and total deadlines without adding SDK API.
#[tokio::test]
async fn reqwest_read_timeout_resets_per_chunk_while_total_timeout_does_not() {
    let chunks = || {
        vec![
            (Duration::from_secs(1), b"one".to_vec()),
            (Duration::from_secs(1), b"two".to_vec()),
            (Duration::from_secs(1), b"three".to_vec()),
        ]
    };
    let server = MockServer::start(vec![
        Reply::ok("").body_chunks(chunks()).chunked(),
        Reply::ok("").body_chunks(chunks()).chunked(),
        Reply::ok("stalled").body_chunks(vec![(Duration::from_secs(3), b"stalled".to_vec())]),
    ])
    .await;
    let client = reqwest::Client::builder()
        .read_timeout(Duration::from_secs(2))
        .timeout(Duration::from_secs(8))
        .build()
        .unwrap();
    assert_eq!(
        client
            .get(&server.base_url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
        "onetwothree"
    );
    let error = client
        .get(&server.base_url)
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap_err();
    assert!(error.is_timeout());
    let error = client
        .get(&server.base_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap_err();
    assert!(error.is_timeout());
    assert_eq!(server.finish().await.len(), 3);
}

#[tokio::test]
async fn bounded_read_rejects_declared_and_chunked_oversize_without_poisoning_session() {
    const LIMIT: usize = 4 * 1024 * 1024;
    let server = MockServer::start(vec![
        Reply::ok("session"),
        Reply::ok("").declared_length(LIMIT + 1),
        Reply::ok("")
            .body_chunks(vec![
                (Duration::ZERO, vec![b' '; LIMIT]),
                (Duration::ZERO, vec![b' ']),
            ])
            .chunked(),
        Reply::ok("[]"),
    ])
    .await;
    let client = server.client(2);
    for _ in 0..2 {
        let error = tokio::time::timeout(
            Duration::from_secs(3),
            client
                .internal_production_trace()
                .get_order_definition_ids(Some(42)),
        )
        .await
        .expect("bounded read hung")
        .unwrap_err();
        assert!(
            matches!(error, IikoError::Api(message) if message == "read-only response exceeds 4194304 bytes")
        );
    }
    assert!(
        client
            .internal_production_trace()
            .get_order_definition_ids(None)
            .await
            .unwrap()
            .is_empty()
    );
    let requests = server.finish().await;
    assert_eq!(requests.len(), 4);
    assert_eq!(
        requests[1].query(),
        pairs(&[
            ("key", "session"),
            ("includeDeleted", "false"),
            ("revisionFrom", "42")
        ])
    );
    assert_eq!(requests[1].query(), requests[2].query());
    assert_eq!(
        requests[3].query(),
        pairs(&[("key", "session"), ("includeDeleted", "false")])
    );
}

#[tokio::test]
async fn bounded_reads_enforce_utf8_and_bound_error_responses_before_status_mapping() {
    let server = MockServer::start(vec![
        Reply::ok("session"),
        Reply::ok([0xff, 0xfe]).header("Content-Type", "application/json; charset=iso-8859-1"),
        Reply::new("500 Internal Server Error", "").declared_length(4 * 1024 * 1024 + 1),
        Reply::ok([b'C', b'a', b'f', 0xe9])
            .header("Content-Type", "text/plain; charset=iso-8859-1"),
    ])
    .await;
    let client = server.client(2);
    let error = client
        .internal_production_trace()
        .get_order_definition_ids(None)
        .await
        .unwrap_err();
    assert!(
        matches!(error, IikoError::Api(message) if message == "read-only response is not valid UTF-8")
    );
    let error = client
        .internal_production_trace()
        .get_order_definition_ids(None)
        .await
        .unwrap_err();
    assert!(
        matches!(error, IikoError::Api(message) if message == "read-only response exceeds 4194304 bytes")
    );
    // Ordinary response.text() retains reqwest's existing charset decoding contract.
    assert_eq!(client.get("ordinary").await.unwrap(), "Café");
    assert_eq!(server.finish().await.len(), 4);
}

#[tokio::test]
async fn internal_xml_read_preserves_tree_and_uses_a_single_authenticated_retry() {
    let xml = "<result><status>SUCCESS</status><correlationId>correlation</correlationId><resultValue><r cls=\"PastOrder\" note=\"a\r\nb\rc\td\ne&#13;f&amp;g\"><name>Ю &amp; X</name><whitespace>a\r\nb\rc\u{85}d\u{2028}e</whitespace><raw><![CDATA[a\r\nb\rc\u{85}d\u{2028}e]]></raw><items><i id=\"1\">one</i><i id=\"2\"><![CDATA[two < three]]></i></items><optional null=\"1\"/></r></resultValue></result>";
    let server = MockServer::start(vec![
        Reply::ok("expired"),
        Reply::new("401 Unauthorized", "expired"),
        Reply::ok("fresh"),
        Reply::ok(xml),
        Reply::ok("<result><status>ERROR</status><errorsContainer><rootError>  upstream unavailable  </rootError></errorsContainer></result>"),
    ])
    .await;
    let client = server.client(2);
    let id = Uuid::from_u128(1);
    let result = client
        .internal_line_sales()
        .get_past_order(id)
        .await
        .unwrap();
    assert_eq!(result.correlation_id.as_deref(), Some("correlation"));
    let value = result.value.unwrap();
    assert_eq!(value.class_name(), Some("PastOrder"));
    assert_eq!(
        value.attributes.get("note").map(String::as_str),
        Some("a\r\nb\rc\td\ne\rf&g")
    );
    assert_eq!(value.child("name").unwrap().text.as_deref(), Some("Ю & X"));
    assert_eq!(
        value.child("whitespace").unwrap().text.as_deref(),
        Some("a\nb\nc\nd\ne")
    );
    assert_eq!(
        value.child("raw").unwrap().text.as_deref(),
        Some("a\r\nb\rc\u{85}d\u{2028}e")
    );
    let items: Vec<_> = value.child("items").unwrap().children_named("i").collect();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].attributes.get("id").map(String::as_str), Some("1"));
    assert_eq!(items[1].text.as_deref(), Some("two < three"));
    assert!(value.child("optional").unwrap().is_null());
    assert!(
        matches!(client.internal_line_sales().get_past_order(id).await.unwrap_err(), IikoError::Api(message) if message == "upstream unavailable")
    );
    let requests = server.finish().await;
    assert_eq!(
        requests[1].body,
        format!("<request><orderId>{id}</orderId></request>").as_bytes()
    );
    assert_eq!(requests[1].body, requests[3].body);
    assert_eq!(requests[1].header("content-type"), Some("application/xml"));
    assert_eq!(requests[1].query(), pairs(&[("key", "expired")]));
    assert_eq!(requests[3].query(), pairs(&[("key", "fresh")]));
}

#[tokio::test]
async fn product_expense_rejects_oversize_before_reading_or_parsing_the_body() {
    let server = MockServer::start(vec![
        Reply::ok("session"),
        Reply::ok("").declared_length(64 * 1024 * 1024 + 1),
    ])
    .await;
    let error = server
        .client(2)
        .reports()
        .get_product_expense("department", "04.10.2026", "04.10.2026", None, None)
        .await
        .unwrap_err();
    assert!(
        matches!(error, IikoError::Api(message) if message == "read-only response exceeds 67108864 bytes")
    );
    assert_eq!(server.finish().await.len(), 2);
}
