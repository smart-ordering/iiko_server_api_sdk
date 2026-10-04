//! Synthetic, offline fixtures for the XML DTOs used by document and report endpoints.
//! These tests deliberately do not import the live-server `tests/common.rs` helper.

use iiko_server_api_sdk::xml::request::events::{EventsFilter, EventsRequestData, OrderNumsFilter};
use iiko_server_api_sdk::xml::response::reports::DayDishValues;
use iiko_server_api_sdk::{
    Document, DocumentStatus, DocumentValidationResult, IncomingInvoiceDto, StoreDocumentType,
    StoreReportItemDto, StoreTransactionType,
};
use quick_xml::{de::from_str, se::to_string};

const SUPPLIER: &str = "11111111-2222-3333-4444-555555555555";
const PRODUCT: &str = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";

#[test]
fn incoming_invoice_keeps_ordered_items_and_external_number_without_accounting_number() {
    let xml = format!(
        "<document><supplier>{SUPPLIER}</supplier><incomingDocumentNumber>SUP-0042</incomingDocumentNumber>\
         <status>NEW</status><items>\
         <item><num>1</num><product>{PRODUCT}</product><amount>2.5</amount><price>120</price><sum>300</sum></item>\
         <item><num>2</num><productArticle>0007</productArticle><amount>0</amount><sum>0</sum></item>\
         </items></document>"
    );
    let invoice: IncomingInvoiceDto = from_str(&xml).unwrap();
    assert_eq!(invoice.supplier.unwrap().to_string(), SUPPLIER);
    assert_eq!(invoice.status, Some(DocumentStatus::New));
    assert_eq!(invoice.document_number, None);
    assert_eq!(
        invoice.incoming_document_number.as_deref(),
        Some("SUP-0042")
    );
    let items = &invoice.items.as_ref().unwrap().items;
    assert_eq!(items.len(), 2);
    assert_eq!(
        (items[0].num, items[0].amount, items[0].sum),
        (1, Some(2.5), 300.0)
    );
    assert_eq!(items[0].product.unwrap().to_string(), PRODUCT);
    assert_eq!(items[0].price, Some(120.0));
    assert_eq!(
        (items[1].num, items[1].amount, items[1].sum),
        (2, Some(0.0), 0.0)
    );
    assert_eq!(items[1].product_article.as_deref(), Some("0007"));

    let outgoing = to_string(&invoice).unwrap();
    assert!(outgoing.starts_with("<document>"), "{outgoing}");
    assert!(!outgoing.contains("<documentNumber"), "{outgoing}");
    assert!(outgoing.contains("<incomingDocumentNumber>SUP-0042</incomingDocumentNumber>"));
    assert_eq!(outgoing.matches("<item>").count(), 2);
    assert!(outgoing.find("<num>1</num>").unwrap() < outgoing.find("<num>2</num>").unwrap());
    assert!(outgoing.contains("<amount>2.5</amount>"));
    assert!(outgoing.contains("<sum>300</sum>"));
}

#[test]
fn incoming_invoice_distinguishes_omitted_and_empty_optional_strings_and_items() {
    let omitted: IncomingInvoiceDto = from_str("<document/>").unwrap();
    assert!(omitted.comment.is_none());
    assert!(omitted.items.is_none());
    assert!(!omitted.use_default_document_time);
    let empty: IncomingInvoiceDto = from_str(
        "<document><comment/><conceptionCode></conceptionCode><incomingDocumentNumber/>\
         <documentNumber/><items/></document>",
    )
    .unwrap();
    assert_eq!(empty.comment.as_deref(), Some(""));
    assert_eq!(empty.conception_code.as_deref(), Some(""));
    assert_eq!(empty.incoming_document_number.as_deref(), Some(""));
    assert_eq!(empty.document_number.as_deref(), Some(""));
    assert!(empty.items.unwrap().items.is_empty());
}

#[test]
fn invoice_validation_text_preserves_entities_cdata_and_interior_whitespace() {
    let result: DocumentValidationResult = from_str(
        "<documentValidationResult><valid>false</valid><warning>true</warning>\
         <errorMessage>  dock &amp; dry <![CDATA[<keep>]]> &#x1F34E;  </errorMessage>\
         <additionalInfo><![CDATA[milk  cream & <raw>]]></additionalInfo>\
         <otherSuggestedNumber>  0042  </otherSuggestedNumber></documentValidationResult>",
    )
    .unwrap();
    assert!(!result.valid);
    assert!(result.warning);
    assert_eq!(
        result.error_message.as_deref(),
        Some("  dock & dry <keep> 🍎  ")
    );
    assert_eq!(
        result.additional_info.as_deref(),
        Some("milk  cream & <raw>")
    );
    assert_eq!(result.other_suggested_number.as_deref(), Some("  0042  "));
    let encoded = to_string(&result).unwrap();
    assert!(encoded.contains("&amp;"));
    assert!(encoded.contains("&lt;raw&gt;"));
    let reparsed: DocumentValidationResult = from_str(&encoded).unwrap();
    assert_eq!(reparsed.error_message, result.error_message);
    assert_eq!(reparsed.additional_info, result.additional_info);
    assert_eq!(reparsed.valid, result.valid);
}

#[test]
fn document_list_preserves_default_and_prefixed_namespaces_and_order() {
    for xml in [
        format!(
            "<document xmlns='urn:synthetic:iiko'><id>{SUPPLIER}</id><number>0042</number>\
             <date>2026-10-04</date><type>INCOMING_INVOICE</type></document>\
             <document xmlns='urn:synthetic:iiko'><id>{PRODUCT}</id><number>0043</number><date>2026-10-05</date>\
             <type>INTERNAL_TRANSFER</type></document>"
        ),
        format!(
            "<i:document xmlns:i='urn:synthetic:iiko'><i:id>{SUPPLIER}</i:id>\
             <i:number>0042</i:number><i:date>2026-10-04</i:date><i:type>INCOMING_INVOICE</i:type>\
             </i:document><i:document xmlns:i='urn:synthetic:iiko'><i:id>{PRODUCT}</i:id><i:number>0043</i:number>\
             <i:date>2026-10-05</i:date><i:type>INTERNAL_TRANSFER</i:type></i:document>"
        ),
    ] {
        let documents: Vec<Document> = from_str(&xml).unwrap();
        assert_eq!(documents.len(), 2);
        assert_eq!(documents[0].id.to_string(), SUPPLIER);
        assert_eq!(documents[0].number, "0042");
        assert_eq!(documents[1].id.to_string(), PRODUCT);
        assert_eq!(documents[1].doc_type, "INTERNAL_TRANSFER");
    }
}

#[test]
fn bounded_namespace_declarations_do_not_change_invoice_fields() {
    // Deliberately below the candidate's namespace budget: ordinary wire parity,
    // rather than demanding continued acceptance of the unbounded old behavior.
    let declarations = (0..64)
        .map(|index| format!(" xmlns:n{index}='urn:synthetic:{index}'"))
        .collect::<String>();
    let invoice: IncomingInvoiceDto = from_str(&format!(
        "<document{declarations}><incomingDocumentNumber>00042</incomingDocumentNumber>\
         <status>PROCESSED</status></document>"
    ))
    .unwrap();
    assert_eq!(invoice.incoming_document_number.as_deref(), Some("00042"));
    assert_eq!(invoice.status, Some(DocumentStatus::Processed));
}

#[test]
fn product_expense_report_preserves_repeated_rows_decimal_values_and_unicode() {
    let report: DayDishValues = from_str(
        "<dayDishValues xmlns='urn:synthetic:iiko'><dayDishValue><date>2026-10-04</date>\
         <productId>0007</productId><productName>  Сыр &amp; сливки  </productName><value>-2.75</value>\
         </dayDishValue><dayDishValue><productId>0008</productId><productName><![CDATA[Tea <green>]]>\
         </productName><value>0</value></dayDishValue></dayDishValues>",
    )
    .unwrap();
    assert_eq!(report.items.len(), 2);
    assert_eq!(report.items[0].product_id.as_deref(), Some("0007"));
    assert_eq!(
        report.items[0].product_name.as_deref(),
        Some("  Сыр & сливки  ")
    );
    assert_eq!(report.items[0].value, Some(-2.75));
    assert_eq!(report.items[1].product_name.as_deref(), Some("Tea <green>"));
    assert_eq!(report.items[1].value, Some(0.0));
    assert!(report.items[1].date.is_none());
    let reparsed: DayDishValues = from_str(&to_string(&report).unwrap()).unwrap();
    assert_eq!(reparsed.items[0].value, report.items[0].value);
    assert_eq!(reparsed.items[1].product_name, report.items[1].product_name);
    assert!(
        from_str::<DayDishValues>("<dayDishValues/>")
            .unwrap()
            .items
            .is_empty()
    );
}

#[test]
fn store_report_retains_known_and_future_enum_values_and_missing_defaults() {
    let known: StoreReportItemDto = from_str(
        "<item><product>0007</product><documentType>INCOMING_INVOICE</documentType>\
         <type>INVOICE</type><incoming>true</incoming><amount>2.5</amount><sum>1200.25</sum>\
         <documentComment>dry &amp; cold</documentComment></item>",
    )
    .unwrap();
    assert_eq!(
        known.document_type,
        Some(StoreDocumentType::IncomingInvoice)
    );
    assert_eq!(known.r#type, Some(StoreTransactionType::Invoice));
    assert!(known.incoming);
    assert_eq!(known.amount, Some(2.5));
    assert_eq!(known.sum, Some(1200.25));
    assert_eq!(known.document_comment.as_deref(), Some("dry & cold"));
    let future: StoreReportItemDto = from_str(
        "<item><documentType>FUTURE_DOCUMENT</documentType><type>FUTURE_TRANSACTION</type></item>",
    )
    .unwrap();
    assert_eq!(future.document_type, Some(StoreDocumentType::Unknown));
    assert_eq!(future.r#type, Some(StoreTransactionType::Unknown));
    assert!(!future.incoming);
    assert!(future.amount.is_none());
}

#[test]
fn events_filter_request_keeps_exact_wire_names_order_and_escaping() {
    // The actual serializer boundary in EventsEndpoint::get_events_by_filter.
    let request = EventsRequestData {
        events: EventsFilter {
            items: vec!["ORDER<&>".into(), "CHECK".into()],
        },
        order_nums: Some(OrderNumsFilter {
            items: vec!["0007".into(), "0008".into()],
        }),
    };
    assert_eq!(
        to_string(&request).unwrap(),
        "<eventsRequestData><events><event>ORDER&lt;&amp;&gt;</event><event>CHECK</event>\
         </events><orderNums><orderNum>0007</orderNum><orderNum>0008</orderNum>\
         </orderNums></eventsRequestData>"
    );
}

#[test]
fn invoice_rejects_invalid_scalar_values_duplicate_fields_and_malformed_xml() {
    for xml in [
        "<document><supplier>not-a-uuid</supplier></document>",
        "<document><supplier/></document>",
        "<document><status>FUTURE_STATUS</status></document>",
        "<document><status>NEW</status><status>DELETED</status></document>",
        "<document><useDefaultDocumentTime>sometimes</useDefaultDocumentTime></document>",
        "<document><items><item><num>1</num><sum>oops</sum></item></items></document>",
        "<document><items><item><num>1</num></item></items></document>",
        "<document><comment>&undefined;</comment></document>",
        "<document><comment>unclosed</document>",
        "<document xmlns:xml='urn:invalid-reserved-namespace'/>",
    ] {
        assert!(
            from_str::<IncomingInvoiceDto>(xml).is_err(),
            "accepted {xml}"
        );
    }
}
