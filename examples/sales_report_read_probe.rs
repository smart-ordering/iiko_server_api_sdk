//! Explicit, bounded read-only sales inspection. Never loads .env or prints credentials.
use chrono::{Duration, NaiveDate, Utc};
use iiko_server_api_sdk::{IikoClient, IikoConfig, OlapReportType};
use serde_json::json;
use sha1::{Digest, Sha1};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    if run().await.is_err() {
        eprintln!("Read probe failed; provider response suppressed to protect credentials");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let base = std::env::var("IIKO_READ_BASE_URL")?;
    if base != "https://fskz-co.iiko.it/resto/api" {
        return Err("unexpected host".into());
    }
    let day = NaiveDate::parse_from_str(&std::env::var("IIKO_READ_DATE")?, "%Y-%m-%d")?;
    let today = (Utc::now() + Duration::hours(5)).date_naive();
    if day < today - Duration::days(7) || day >= today {
        return Err("only the last seven completed days are permitted".into());
    }
    let department: Uuid = std::env::var("IIKO_READ_DEPARTMENT")?.parse()?;
    let password_hash = format!("{:x}", Sha1::digest(std::env::var("IM_PASS")?.as_bytes()));
    let client = IikoClient::new(IikoConfig::new(base, "Ailillu", password_hash).with_timeout(45))?;
    let result = async {
        let columns = match client.reports().get_olap_columns(OlapReportType::Sales).await {
            Ok(v) => v,
            Err(e) => {
                let message = e.to_string();
                let category = if message.contains("401") { "authentication_401" } else if message.contains("403") { "forbidden_403" } else if message.contains("license") || message.contains("License") { "license" } else if message.contains("error decoding") || message.contains("JSON") { "schema" } else { "request_failed" };
                println!("{}", json!({"metadata_error":category}));
                return Err("metadata unavailable".into());
            }
        };
        let selected: std::collections::BTreeMap<_, _> = columns.iter().filter(|(k, v)| {
            let key = k.to_lowercase();
            key.contains("vat") || key.contains("orderid") || key.contains("close") || key.contains("cost")
                || key.contains("hour") || key.contains("orderType".to_lowercase().as_str()) || key.contains("dishid")
                || v.name.contains("НДС") || key.contains("combo")
        }).map(|(k,v)|(k,json!({"name":v.name,"aggregate":v.aggregation_allowed,"group":v.grouping_allowed}))).collect();
        println!("{}", json!({"sales_columns":selected}));
        let snapshot = client.internal_price_history().get_department_snapshot(department, day.and_hms_opt(0,0,0).unwrap()).await;
        match snapshot {
            Ok(value) => println!("{}", json!({"internal_price_snapshot":"ok","root":value.value.as_ref().map(|v| &v.name),"children":value.value.as_ref().map(|v|v.children.len())})),
            Err(_) => println!("{}", json!({"internal_price_snapshot":"unavailable","detail":"provider response suppressed"})),
        }
        Ok::<(), Box<dyn std::error::Error>>(())
    }.await;
    let logout = client.logout().await;
    println!("{}", json!({"logout_ok":logout.is_ok()}));
    result
}
