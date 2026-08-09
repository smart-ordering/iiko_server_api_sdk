use chrono::NaiveDate;
use iiko_server_api_sdk::{
    AnalyticalEntityKind, IikoClient, IikoConfig, InternalDocumentKind, InternalReadResult,
    InternalXmlNode,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = required("IIKO_SMOKE_BASE_URL")?;
    let expected_base_url = required("IIKO_SMOKE_EXPECTED_BASE_URL")?;
    if base_url != expected_base_url {
        return Err("smoke scope guard rejected the iiko base URL".into());
    }

    let config = IikoConfig::new(
        base_url,
        required("IIKO_SMOKE_LOGIN")?,
        required("IIKO_SMOKE_HASHED_PASSWORD")?,
    )
    .with_timeout(45);
    let client = IikoClient::new(config)?;
    let result = run_smoke(&client).await;
    let logout = client.logout_if_authenticated().await;

    if logout.is_ok() {
        println!("logout=ok");
    } else {
        println!("logout=failed");
    }
    result?;
    logout?;
    Ok(())
}

async fn run_smoke(client: &IikoClient) -> Result<(), Box<dyn std::error::Error>> {
    let document_id = required("IIKO_SMOKE_DOCUMENT_ID")?.parse::<Uuid>()?;
    let order_id = required("IIKO_SMOKE_ORDER_ID")?.parse::<Uuid>()?;
    let product_id = required("IIKO_SMOKE_PRODUCT_ID")?.parse::<Uuid>()?;
    let store_id = required("IIKO_SMOKE_STORE_ID")?.parse::<Uuid>()?;
    let department_id = required("IIKO_SMOKE_DEPARTMENT_ID")?.parse::<Uuid>()?;
    let supplier_code = required("IIKO_SMOKE_SUPPLIER_CODE")?;
    let date = NaiveDate::parse_from_str(&required("IIKO_SMOKE_DATE")?, "%Y-%m-%d")?;
    let at = date.and_hms_opt(0, 0, 0).ok_or("invalid smoke time")?;

    print_shape(
        "document",
        &client
            .internal_documents()
            .get_abstract_document(document_id)
            .await?,
    );
    print_shape(
        "document_batch",
        &client
            .internal_documents()
            .get_abstract_documents(&[document_id])
            .await?,
    );
    print_shape(
        "document_costs",
        &client
            .internal_documents()
            .get_document_item_costs(document_id)
            .await?,
    );
    print_shape(
        "document_transactions",
        &client
            .internal_documents()
            .get_document_transactions(document_id)
            .await?,
    );
    print_shape(
        "document_index",
        &client
            .internal_document_index()
            .get_documents_by_ids(InternalDocumentKind::IncomingInvoice, &[document_id])
            .await?,
    );
    print_shape(
        "incoming_document_records",
        &client
            .internal_document_index()
            .get_incoming_records_by_ids(InternalDocumentKind::IncomingInvoice, &[document_id])
            .await?,
    );
    print_shape(
        "historical_stock",
        &client
            .internal_historical_stock()
            .get_product_balances_for_date(date)
            .await?,
    );
    print_shape(
        "stock_product_usage",
        &client
            .internal_stock_movements()
            .get_product_usage(date, product_id)
            .await?,
    );
    print_shape(
        "last_store_costs",
        &client
            .internal_cost_history()
            .get_last_costs_by_stores(&[store_id], at)
            .await?,
    );
    print_shape(
        "recipe_modifier_edges",
        &client
            .internal_recipe_graph()
            .get_modifiers_containing_product(&[product_id], product_id)
            .await?,
    );
    print_shape(
        "department_price_snapshot",
        &client
            .internal_price_history()
            .get_department_snapshot(department_id, at)
            .await?,
    );

    let supplier_prices = client
        .internal_supplier_history()
        .get_pricelist_on_date(&supplier_code, date)
        .await?;
    println!("supplier_pricelist=ok item_count={}", supplier_prices.len());

    let entity_changes = client
        .internal_entity_changes()
        .get_changes(
            &[AnalyticalEntityKind::ProductCategory],
            i64::from(i32::MAX),
        )
        .await?;
    println!("entity_changes=ok item_count={}", entity_changes.len());

    let definition_ids = client
        .internal_production_trace()
        .get_order_definition_ids(None)
        .await?;
    println!(
        "production_order_definition_ids=ok item_count={}",
        definition_ids.len()
    );
    if let Some(definition_id) = definition_ids.first().copied() {
        let definitions = client
            .internal_production_trace()
            .get_order_definitions(&[definition_id])
            .await?;
        println!(
            "production_order_definitions=ok item_count={}",
            definitions.len()
        );
    }

    let past_order = client
        .internal_line_sales()
        .get_past_order(order_id)
        .await?;
    print_shape("past_order", &past_order);
    if let Some(session_id) = find_uuid(&past_order, "cafeSessionId") {
        print_shape(
            "cash_session_transactions",
            &client
                .internal_cash_sessions()
                .get_transactions(session_id)
                .await?,
        );
    } else {
        println!("cash_session_transactions=skipped reason=no_session_id");
    }
    if let Some(event_id) =
        find_uuid(&past_order, "sourceEventId").or_else(|| find_order_item_id(&past_order))
    {
        print_shape(
            "item_sale_event",
            &client
                .internal_sales_events()
                .get_item_sale_event(event_id)
                .await?,
        );
    } else {
        println!("item_sale_event=skipped reason=no_event_id");
    }
    Ok(())
}

fn find_order_item_id(result: &InternalReadResult) -> Option<Uuid> {
    fn find_items(node: &InternalXmlNode) -> Option<Uuid> {
        if node.name == "items" {
            return node
                .children_named("i")
                .filter_map(|item| item.child("id"))
                .filter_map(|id| id.text.as_deref())
                .find_map(|id| id.parse().ok());
        }
        node.children.iter().find_map(find_items)
    }

    result.value.as_ref().and_then(find_items)
}

fn find_uuid(result: &InternalReadResult, name: &str) -> Option<Uuid> {
    fn find_text<'a>(node: &'a InternalXmlNode, name: &str) -> Option<&'a str> {
        if node.name == name {
            return node.text.as_deref();
        }
        node.children
            .iter()
            .find_map(|child| find_text(child, name))
    }

    result
        .value
        .as_ref()
        .and_then(|value| find_text(value, name))
        .and_then(|value| value.parse().ok())
}

fn print_shape(label: &str, result: &InternalReadResult) {
    let class_name = result
        .value
        .as_ref()
        .and_then(|value| value.class_name())
        .unwrap_or("none");
    let child_count = result
        .value
        .as_ref()
        .map(|value| value.children.len())
        .unwrap_or_default();
    println!("{label}=ok class={class_name} child_count={child_count}");
}

fn required(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    std::env::var(name).map_err(|_| format!("{name} is required").into())
}
