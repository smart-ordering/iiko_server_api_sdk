//! Candidate-only acceptance limits introduced by quick-xml >= 0.41.
//! Do not include this target in the pre-upgrade portable wire-contract run.

use iiko_server_api_sdk::{DocumentStatus, IncomingInvoiceDto};
use quick_xml::de::from_str;

fn declarations(start: usize, count: usize) -> String {
    (start..start + count)
        .map(|index| format!(" xmlns:n{index}='urn:synthetic:{index}'"))
        .collect()
}

#[test]
fn invoice_namespace_budget_accepts_128_and_rejects_129_root_bindings() {
    let invoice: IncomingInvoiceDto = from_str(&format!(
        "<document{}><status>NEW</status></document>",
        declarations(0, 128)
    ))
    .unwrap();
    assert_eq!(invoice.status, Some(DocumentStatus::New));
    let excessive = format!(
        "<document{}><status>NEW</status></document>",
        declarations(0, 129)
    );
    assert!(from_str::<IncomingInvoiceDto>(&excessive).is_err());
}

#[test]
fn invoice_namespace_budget_applies_across_active_ancestor_scopes() {
    let at_limit = format!(
        "<document{}><comment{}>keep</comment></document>",
        declarations(0, 64),
        declarations(64, 64)
    );
    let invoice: IncomingInvoiceDto = from_str(&at_limit).unwrap();
    assert_eq!(invoice.comment.as_deref(), Some("keep"));
    let over_limit = format!(
        "<document{}><comment{}>reject</comment></document>",
        declarations(0, 64),
        declarations(64, 65)
    );
    assert!(from_str::<IncomingInvoiceDto>(&over_limit).is_err());
}

#[test]
fn closing_child_scope_releases_namespace_budget_for_the_next_field() {
    let invoice: IncomingInvoiceDto = from_str(&format!(
        "<document{}><comment{}>first</comment>\
         <incomingDocumentNumber{}>00042</incomingDocumentNumber></document>",
        declarations(0, 64),
        declarations(64, 64),
        declarations(128, 64)
    ))
    .unwrap();
    assert_eq!(invoice.comment.as_deref(), Some("first"));
    assert_eq!(invoice.incoming_document_number.as_deref(), Some("00042"));
}
