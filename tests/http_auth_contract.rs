mod http_contract_support;

use http_contract_support::{MockServer, Reply, pairs};
use iiko_server_api_sdk::{IikoClient, IikoConfig, IikoError};
use sha1::{Digest, Sha1};

#[tokio::test]
async fn auth_hash_form_query_and_raw_mutation_bodies_keep_the_wire_contract() {
    let token = "session+/ &=Ю";
    let server = MockServer::start(vec![
        Reply::ok(format!(" \n{token}\r\n")),
        Reply::ok("get result"),
        Reply::ok("post result"),
        Reply::new("201 Created", "put result"),
        Reply::ok("form result"),
        Reply::new("201 Created", "json result"),
        Reply::ok("delete result"),
        Reply::ok(" released \n"),
    ])
    .await;
    let hash = Sha1::digest(b"abc")
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    assert_eq!(hash, "a9993e364706816aba3e25717850c26c9cd0d89d");
    let client = IikoClient::new(IikoConfig::new(&server.base_url, "user +&Ю", &hash)).unwrap();
    assert_eq!(client.authenticate().await.unwrap(), token);
    assert_eq!(client.clone().authenticate().await.unwrap(), token);
    assert_eq!(
        client
            .get_with_params("resource", &[("item", "a+b &"), ("item", "Ю")])
            .await
            .unwrap(),
        "get result"
    );
    let xml = "\u{feff}<?xml version=\"1.0\"?><request>Ю &amp; X\n</request>";
    assert_eq!(client.post_xml("post", xml).await.unwrap(), "post result");
    assert_eq!(client.put_xml("put", xml).await.unwrap(), "put result");
    client
        .post_form("form", &[("name", "Ю +&="), ("id", "1"), ("id", "2")])
        .await
        .unwrap();
    let json = "{\n  \"name\": \"Ю +&=\"\n}";
    client
        .post_json("json", json, &[("id", "1"), ("id", "2")])
        .await
        .unwrap();
    client.delete("delete").await.unwrap();
    assert_eq!(
        client.logout_if_authenticated().await.unwrap(),
        Some("released".into())
    );
    assert_eq!(client.logout_if_authenticated().await.unwrap(), None);

    let requests = server.finish().await;
    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[0].target, "/resto/api/auth");
    assert_eq!(
        requests[0].header("content-type"),
        Some("application/x-www-form-urlencoded")
    );
    assert_eq!(
        requests[0].form(),
        pairs(&[("login", "user +&Ю"), ("pass", &hash)])
    );
    assert_eq!(requests[1].method, "GET");
    assert_eq!(
        requests[1].query(),
        pairs(&[("key", token), ("item", "a+b &"), ("item", "Ю")])
    );
    assert!(requests[1].body.is_empty());
    for (index, method) in [(2, "POST"), (3, "PUT")] {
        assert_eq!(requests[index].method, method);
        assert_eq!(requests[index].query(), pairs(&[("key", token)]));
        assert_eq!(
            requests[index].header("content-type"),
            Some("application/xml")
        );
        assert_eq!(requests[index].body, xml.as_bytes());
    }
    assert_eq!(
        requests[4].header("content-type"),
        Some("application/x-www-form-urlencoded")
    );
    assert_eq!(
        requests[4].form(),
        pairs(&[("name", "Ю +&="), ("id", "1"), ("id", "2")])
    );
    assert_eq!(requests[5].header("content-type"), Some("application/json"));
    assert_eq!(requests[5].body, json.as_bytes());
    assert_eq!(
        requests[5].query(),
        pairs(&[("key", token), ("id", "1"), ("id", "2")])
    );
    assert_eq!(requests[6].method, "DELETE");
    assert_eq!(requests[6].query(), pairs(&[("key", token)]));
    assert_eq!(requests[7].target, "/resto/api/logout");
    assert_eq!(requests[7].form(), pairs(&[("key", token)]));
}

#[tokio::test]
async fn authentication_failure_and_empty_token_are_not_cached() {
    let server = MockServer::start(vec![
        Reply::new("403 Forbidden", "license denied"),
        Reply::ok(" \r\n "),
        Reply::ok("valid"),
    ])
    .await;
    let client = server.client(2);
    assert!(
        matches!(client.authenticate().await.unwrap_err(), IikoError::Authentication(message) if message == "Authentication failed with status: 403 Forbidden - license denied")
    );
    assert!(
        matches!(client.authenticate().await.unwrap_err(), IikoError::Authentication(message) if message == "Empty token in response")
    );
    assert_eq!(client.authenticate().await.unwrap(), "valid");
    assert_eq!(client.authenticate().await.unwrap(), "valid");
    assert_eq!(server.finish().await.len(), 3);
}

#[tokio::test]
async fn get_retries_once_with_a_new_key_but_mutation_is_never_replayed() {
    let server = MockServer::start(vec![
        Reply::ok("old"),
        Reply::new("401 Unauthorized", "expired"),
        Reply::ok("new"),
        Reply::ok("read"),
        Reply::new("401 Unauthorized", "write denied"),
        Reply::ok("later read"),
    ])
    .await;
    let client = server.client(2);
    assert_eq!(
        client
            .get_with_params("read", &[("cursor", "42")])
            .await
            .unwrap(),
        "read"
    );
    assert!(
        matches!(client.post_xml("write", "<request/>").await.unwrap_err(), IikoError::Unauthorized(message) if message == "write denied")
    );
    assert_eq!(client.get("after").await.unwrap(), "later read");
    let requests = server.finish().await;
    assert_eq!(
        requests[1].query(),
        pairs(&[("key", "old"), ("cursor", "42")])
    );
    assert_eq!(
        requests[3].query(),
        pairs(&[("key", "new"), ("cursor", "42")])
    );
    assert_eq!(requests[4].method, "POST");
    assert_eq!(requests[5].method, "GET");
    assert_eq!(requests[5].query(), pairs(&[("key", "new")]));
}

#[tokio::test]
async fn http_error_statuses_preserve_variants_and_response_messages() {
    let statuses = [
        "400 Bad Request",
        "403 Forbidden",
        "404 Not Found",
        "409 Conflict",
        "500 Internal Server Error",
        "503 Service Unavailable",
    ];
    let mut replies = vec![Reply::ok("session")];
    replies.extend(
        statuses
            .iter()
            .map(|status| Reply::new(status, "iiko error Ю")),
    );
    let server = MockServer::start(replies).await;
    let client = server.client(2);
    for (index, _) in statuses.iter().enumerate() {
        let error = client.get("resource").await.unwrap_err();
        match (index, error) {
            (0, IikoError::BadRequest(message))
            | (1, IikoError::Forbidden(message))
            | (2, IikoError::NotFound(message))
            | (3, IikoError::BusinessLogic(message))
            | (4, IikoError::InternalServerError(message)) => assert_eq!(message, "iiko error Ю"),
            (5, IikoError::Api(message)) => assert_eq!(
                message,
                "Request failed with status: 503 Service Unavailable - iiko error Ю"
            ),
            (_, error) => panic!("wrong error variant: {error:?}"),
        }
    }
    assert_eq!(server.finish().await.len(), 7);
}

#[tokio::test]
async fn redirect_302_discards_post_body_while_307_preserves_xml_body() {
    let server = MockServer::start(vec![
        Reply::new("302 Found", "").header("Location", "/resto/api/auth-final"),
        Reply::ok("session"),
        Reply::new("307 Temporary Redirect", "").header("Location", "/resto/api/xml-final"),
        Reply::ok("accepted"),
    ])
    .await;
    let client = server.client(2);
    assert_eq!(
        client
            .post_xml("xml-start", "<request>Ю</request>")
            .await
            .unwrap(),
        "accepted"
    );
    let requests = server.finish().await;
    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[1].method, "GET");
    assert!(requests[1].body.is_empty());
    assert_eq!(requests[1].header("content-type"), None);
    assert_eq!(requests[2].method, "POST");
    assert_eq!(requests[3].method, "POST");
    assert_eq!(requests[3].target, "/resto/api/xml-final");
    assert_eq!(requests[3].body, requests[2].body);
    assert_eq!(requests[3].header("content-type"), Some("application/xml"));
}

#[tokio::test]
async fn redirect_loop_stops_at_default_limit_and_redacts_the_session_key() {
    let secret = "session-must-not-leak";
    let mut replies = vec![Reply::ok(secret)];
    replies.extend((0..11).map(|_| {
        Reply::new("302 Found", "").header("Location", format!("/resto/api/loop?key={secret}"))
    }));
    let server = MockServer::start(replies).await;
    let error = server.client(2).get("loop").await.unwrap_err();
    assert!(matches!(error, IikoError::Http(_)));
    let message = error.to_string();
    assert!(!message.contains(secret));
    assert!(!message.contains("key="));
    assert!(!message.contains(&server.base_url));
    assert_eq!(server.finish().await.len(), 12);
}
