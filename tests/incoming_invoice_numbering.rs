use iiko_server_api_sdk::xml::response::documents::IncomingInvoiceDto;

#[test]
fn automatic_number_is_omitted_while_supplier_number_is_preserved() {
    let invoice: IncomingInvoiceDto = quick_xml::de::from_str(
        "<document><supplier>11111111-2222-3333-4444-555555555555</supplier>\
         <incomingDocumentNumber>KA-28802</incomingDocumentNumber><status>NEW</status></document>",
    )
    .unwrap();
    let xml = quick_xml::se::to_string(&invoice).unwrap();
    assert!(!xml.contains("documentNumber"), "{xml}");
    assert!(xml.contains("<incomingDocumentNumber>KA-28802</incomingDocumentNumber>"));
    assert!(xml.contains("<status>NEW</status>"));
}

#[test]
fn explicit_and_exported_accounting_numbers_still_round_trip() {
    let invoice: IncomingInvoiceDto = quick_xml::de::from_str(
        "<document><supplier>11111111-2222-3333-4444-555555555555</supplier>\
         <documentNumber>74562</documentNumber></document>",
    )
    .unwrap();
    assert_eq!(invoice.document_number.as_deref(), Some("74562"));
    let xml = quick_xml::se::to_string(&invoice).unwrap();
    assert!(xml.contains("<documentNumber>74562</documentNumber>"));
}
