//! Offline JSON fixtures for the actual catalog, report and document DTOs.

use iiko_server_api_sdk::{
    BalanceCounteragent, BalanceStore, InternalTransferDto, OlapFieldValue, OlapFilter,
    OlapReportRequest, OlapReportResponse, ProductDto,
};
use serde_json::{Value, from_value, json, to_value};

const PRODUCT: &str = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
const STORE: &str = "11111111-2222-3333-4444-555555555555";

#[test]
fn catalog_product_keeps_nested_arrays_nulls_unicode_and_ntin() {
    let fixture = json!({
        "id": PRODUCT, "name": "  Сыр & сливки  ", "type": "GOODS",
        "mainUnit": STORE, "defaultSalePrice": 1200.25,
        "ntin": "0001234567890", "excludedSections": null,
        "containers": [
            {"name": "box", "num": "0007", "count": 12.5, "containerWeight": 0.2,
             "fullContainerWeight": 12.7, "useInFront": true},
            {"name": "single", "num": "0008", "count": 1.0}
        ],
        "barcodes": [{"barcode": "0000123456789", "containerId": null},
                     {"barcode": "0000987654321", "containerId": STORE}],
        "modifiers": [{"modifier": STORE, "minimumAmount": 1, "maximumAmount": 2,
                       "required": true, "childModifiers": [{"modifier": PRODUCT}]}],
        "allergenGroups": ["milk", "soy"], "futureServerField": {"value": 1}
    });
    let product: ProductDto = from_value(fixture).unwrap();
    assert_eq!(product.name.as_deref(), Some("  Сыр & сливки  "));
    assert_eq!(product.ntin.as_deref(), Some("0001234567890"));
    assert!(product.excluded_sections.is_none());
    let containers = product.containers.as_ref().unwrap();
    assert_eq!(containers.len(), 2);
    assert_eq!(containers[0].num.as_deref(), Some("0007"));
    assert_eq!(containers[0].count, Some(12.5));
    assert!(containers[0].use_in_front);
    assert_eq!(containers[1].num.as_deref(), Some("0008"));
    assert_eq!(
        product.barcodes.as_ref().unwrap()[0].barcode,
        "0000123456789"
    );
    assert_eq!(
        product.modifiers[0].child_modifiers.as_ref().unwrap()[0]
            .modifier
            .to_string(),
        PRODUCT
    );
    let encoded = to_value(&product).unwrap();
    assert_eq!(encoded["name"], "  Сыр & сливки  ");
    assert_eq!(encoded["ntin"], "0001234567890");
    assert_eq!(encoded["mainUnit"], STORE);
    assert_eq!(encoded["defaultSalePrice"], 1200.25);
    assert!(encoded["excludedSections"].is_null());
    assert!(encoded.get("futureServerField").is_none());
    let reparsed: ProductDto = from_value(encoded.clone()).unwrap();
    assert_eq!(to_value(reparsed).unwrap(), encoded);
}

#[test]
fn catalog_product_defaults_and_optional_ntin_remain_distinct_from_empty_strings() {
    for fixture in [json!({}), json!({"ntin": null})] {
        let product: ProductDto = from_value(fixture).unwrap();
        assert!(!product.deleted);
        assert!(product.modifiers.is_empty());
        assert!(product.name.is_none());
        assert!(product.containers.is_none());
        assert!(to_value(product).unwrap().get("ntin").is_none());
    }
    let product: ProductDto =
        from_value(json!({"ntin": "", "name": "", "containers": []})).unwrap();
    let encoded = to_value(product).unwrap();
    assert_eq!(encoded["ntin"], "");
    assert_eq!(encoded["name"], "");
    assert_eq!(encoded["containers"], json!([]));
}

#[test]
fn olap_report_keeps_numeric_strings_integer_float_null_and_summary_shape() {
    let fixture = json!({
        "data": [{"Product.Num": "0007", "GuestNum": 2, "DishSumInt": 1200.25,
                  "Optional": null, "OpenDate": "2026-10-04"},
                 {"Product.Num": "0008", "GuestNum": 0, "DishSumInt": -12.5}],
        "summary": [[{"GuestNum": 2, "DishSumInt": 1187.75}], []]
    });
    let report: OlapReportResponse = from_value(fixture.clone()).unwrap();
    assert_eq!(report.data.len(), 2);
    assert!(matches!(
        report.data[0]["Product.Num"],
        OlapFieldValue::String(_)
    ));
    assert_eq!(report.data[0]["Product.Num"].as_string(), Some("0007"));
    assert!(matches!(
        report.data[0]["GuestNum"],
        OlapFieldValue::Integer(2)
    ));
    assert!(matches!(
        report.data[0]["DishSumInt"],
        OlapFieldValue::Float(_)
    ));
    assert_eq!(report.data[0]["DishSumInt"].as_float(), Some(1200.25));
    assert!(report.data[0]["Optional"].is_null());
    assert_eq!(report.summary.len(), 2);
    assert!(report.summary[1].is_empty());
    assert_eq!(to_value(report).unwrap(), fixture);
}

#[test]
fn olap_request_serializes_filter_variants_defaults_and_omitted_optional_fields() {
    let fixture = json!({
        "reportType": "SALES", "groupByRowFields": ["OpenDate.Typed", "DishName"],
        "aggregateFields": ["DishSumInt"], "filters": {
            "DishName": {"filterType": "IncludeValues", "values": ["Tea", "Сыр"]},
            "DishSumInt": {"filterType": "Range", "from": 0.0, "to": 1000.0},
            "OpenDate.Typed": {"filterType": "DateRange", "periodType": "CUSTOM",
                               "from": "2026-10-01T00:00:00", "to": "2026-10-04T00:00:00"}
        }
    });
    let request: OlapReportRequest = from_value(fixture).unwrap();
    let filters = request.filters.as_ref().unwrap();
    assert!(matches!(filters["DishName"], OlapFilter::Value(_)));
    assert!(matches!(filters["DishSumInt"], OlapFilter::Range(_)));
    assert!(matches!(
        filters["OpenDate.Typed"],
        OlapFilter::DateRange(_)
    ));
    let encoded = to_value(&request).unwrap();
    assert!(encoded.get("buildSummary").is_none());
    assert!(encoded.get("groupByColFields").is_none());
    assert_eq!(encoded["filters"]["DishSumInt"]["includeLow"], true);
    assert_eq!(encoded["filters"]["DishSumInt"]["includeHigh"], false);
    assert_eq!(encoded["filters"]["OpenDate.Typed"]["includeLow"], true);
    assert_eq!(encoded["filters"]["OpenDate.Typed"]["includeHigh"], false);
    let reparsed: OlapReportRequest = from_value(encoded.clone()).unwrap();
    assert_eq!(to_value(reparsed).unwrap(), encoded);
}

#[test]
fn internal_transfer_omits_readonly_item_fields_and_unset_accounting_number() {
    let transfer: InternalTransferDto = from_value(json!({
        "dateIncoming": "2026-10-04T09:30:00", "status": "NEW",
        "storeFromId": STORE, "storeToId": PRODUCT,
        "items": [{"num": 7, "productId": PRODUCT, "amount": 2.5,
                   "measureUnitId": STORE, "cost": 300.25, "containerId": null}]
    }))
    .unwrap();
    assert_eq!(transfer.items[0].num, Some(7));
    assert_eq!(transfer.items[0].cost, Some(300.25));
    assert_eq!(
        transfer.items[0].measure_unit_id.unwrap().to_string(),
        STORE
    );
    assert_eq!(
        to_value(transfer).unwrap(),
        json!({
            "dateIncoming": "2026-10-04T09:30:00", "status": "NEW",
            "storeFromId": STORE, "storeToId": PRODUCT,
            "items": [{"productId": PRODUCT, "amount": 2.5}]
        })
    );
}

#[test]
fn balance_reports_keep_negative_fractional_amounts_and_nullable_counteragent() {
    let stores: Vec<BalanceStore> = from_value(json!([
        {"store": STORE, "product": PRODUCT, "amount": -0.125, "sum": -100.25},
        {"store": STORE, "product": PRODUCT, "amount": 0.0, "sum": 0.0}
    ]))
    .unwrap();
    assert_eq!((stores[0].amount, stores[0].sum), (-0.125, -100.25));
    assert_eq!((stores[1].amount, stores[1].sum), (0.0, 0.0));
    let balance: BalanceCounteragent = from_value(json!({
        "account": "5.01", "department": STORE, "sum": -100.25
    }))
    .unwrap();
    assert!(balance.counteragent.is_none());
    assert_eq!(to_value(balance).unwrap()["counteragent"], Value::Null);
}

#[test]
fn typed_json_rejects_invalid_uuid_numeric_fields_null_arrays_and_olap_booleans() {
    for fixture in [
        json!({"id": "bad-uuid"}),
        json!({"defaultSalePrice": "12.5"}),
        json!({"modifiers": null}),
        json!({"barcodes": [{"barcode": 12345}]}),
    ] {
        assert!(
            from_value::<ProductDto>(fixture.clone()).is_err(),
            "accepted {fixture}"
        );
    }
    assert!(
        from_value::<OlapReportResponse>(json!({"data": [{"GuestNum": true}], "summary": []}))
            .is_err()
    );
    assert!(
        from_value::<InternalTransferDto>(json!({
            "dateIncoming": "2026-10-04", "status": "FUTURE_STATUS",
            "storeFromId": STORE, "storeToId": PRODUCT, "items": []
        }))
        .is_err()
    );
}
