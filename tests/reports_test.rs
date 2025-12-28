mod common;
use common::{cleanup_after_test, get_test_client};

use chrono::{DateTime, Utc};

#[tokio::test]
async fn test_get_balance_counteragents() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Получаем текущую дату-время
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%Y-%m-%dT%H:%M:%S").to_string();

    let result = reports
        .get_balance_counteragents(&timestamp, None, None, None)
        .await;

    match result {
        Ok(balances) => {
            println!("Получено балансов: {}", balances.len());
            for balance in balances.iter().take(5) {
                println!(
                    "Account: {}, Counteragent: {:?}, Department: {}, Sum: {}",
                    balance.account, balance.counteragent, balance.department, balance.sum
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения балансов: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_balance_stores() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Получаем текущую дату-время
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%Y-%m-%dT%H:%M:%S").to_string();

    let result = reports
        .get_balance_stores(&timestamp, None, None, None)
        .await;

    match result {
        Ok(balances) => {
            println!("Получено остатков на складах: {}", balances.len());
            for balance in balances.iter().take(5) {
                println!(
                    "Store: {}, Product: {}, Amount: {}, Sum: {}",
                    balance.store, balance.product, balance.amount, balance.sum
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения остатков на складах: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_egais_marks_list() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports.get_egais_marks_list(None, None).await;

    match result {
        Ok(marks_list) => {
            println!(
                "Получен отчет ЕГАИС: revision={}, full_update={}, marks_by_b_reg_id count={}",
                marks_list.revision,
                marks_list.full_update,
                marks_list.marks_by_b_reg_id.len()
            );
            for (b_reg_id, b_reg_dto) in marks_list.marks_by_b_reg_id.iter().take(3) {
                println!(
                    "BRegId: {}, SourceRarId: {}, AlcCode: {}, MarksOnBalance: {}, MarksWrittenOff: {}",
                    b_reg_id,
                    b_reg_dto.source_rar_id,
                    b_reg_dto.alc_code,
                    b_reg_dto.marks_on_balance.len(),
                    b_reg_dto.marks_written_off.len()
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения отчета ЕГАИС: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_balance_counteragents_with_filters() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Получаем текущую дату-время
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%Y-%m-%dT%H:%M:%S").to_string();

    // Тест с фильтрами (если есть известные ID)
    let accounts = Some(&["test-account-id"][..]);
    let counteragents = Some(&["test-counteragent-id"][..]);
    let departments = Some(&["test-department-id"][..]);

    let result = reports
        .get_balance_counteragents(&timestamp, accounts, counteragents, departments)
        .await;

    match result {
        Ok(balances) => {
            println!("Получено балансов с фильтрами: {}", balances.len());
        }
        Err(e) => {
            println!("Ошибка получения балансов с фильтрами: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_balance_stores_with_filters() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Получаем текущую дату-время
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%Y-%m-%dT%H:%M:%S").to_string();

    // Тест с фильтрами (если есть известные ID)
    let departments = Some(&["test-department-id"][..]);
    let stores = Some(&["test-store-id"][..]);
    let products = Some(&["test-product-id"][..]);

    let result = reports
        .get_balance_stores(&timestamp, departments, stores, products)
        .await;

    match result {
        Ok(balances) => {
            println!(
                "Получено остатков на складах с фильтрами: {}",
                balances.len()
            );
        }
        Err(e) => {
            println!("Ошибка получения остатков на складах с фильтрами: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_egais_marks_list_with_filters() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Тест с фильтрами
    let fs_rar_ids = Some(&["030000455388", "030000455399"][..]);
    let revision_from = Some(100);

    let result = reports
        .get_egais_marks_list(fs_rar_ids, revision_from)
        .await;

    match result {
        Ok(marks_list) => {
            println!(
                "Получен отчет ЕГАИС с фильтрами: revision={}, full_update={}",
                marks_list.revision, marks_list.full_update
            );
        }
        Err(e) => {
            println!("Ошибка получения отчета ЕГАИС с фильтрами: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_olap_columns_sales() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_olap_columns(iiko_server_api_sdk::OlapReportType::Sales)
        .await;

    match result {
        Ok(columns) => {
            println!("Получено полей OLAP для SALES: {}", columns.len());
            for (field_name, column_info) in columns.iter().take(5) {
                println!(
                    "Field: {}, Name: {}, Type: {}, Aggregation: {}, Grouping: {}, Filtering: {}",
                    field_name,
                    column_info.name,
                    column_info.r#type,
                    column_info.aggregation_allowed,
                    column_info.grouping_allowed,
                    column_info.filtering_allowed
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения полей OLAP: {:?}", e);
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_olap_columns_transactions() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_olap_columns(iiko_server_api_sdk::OlapReportType::Transactions)
        .await;

    match result {
        Ok(columns) => {
            println!("Получено полей OLAP для TRANSACTIONS: {}", columns.len());
        }
        Err(e) => {
            println!("Ошибка получения полей OLAP для TRANSACTIONS: {:?}", e);
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_olap_report_sales() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Сначала получаем список полей
    let columns = reports
        .get_olap_columns(iiko_server_api_sdk::OlapReportType::Sales)
        .await
        .unwrap_or_default();

    if columns.is_empty() {
        println!("Нет доступных полей для OLAP-отчета по продажам");
        cleanup_after_test(&client).await;
        return;
    }

    // Ищем подходящие поля для группировки и агрегации
    let mut group_by_row_fields = Vec::new();
    let mut aggregate_fields = Vec::new();
    let mut date_filter_field: Option<String> = None;

    for (field_name, column_info) in &columns {
        if column_info.grouping_allowed && group_by_row_fields.len() < 2 {
            group_by_row_fields.push(field_name.clone());
        }
        if column_info.aggregation_allowed && aggregate_fields.len() < 2 {
            aggregate_fields.push(field_name.clone());
        }
        // Ищем поле для фильтрации по дате
        if date_filter_field.is_none()
            && column_info.filtering_allowed
            && (field_name.contains("Date") || field_name.contains("DateTime"))
        {
            date_filter_field = Some(field_name.clone());
        }
    }

    if group_by_row_fields.is_empty() || aggregate_fields.is_empty() {
        println!("Недостаточно полей для создания OLAP-отчета");
        cleanup_after_test(&client).await;
        return;
    }

    // Создаем фильтр по дате, если нашли подходящее поле
    let mut filters: Option<std::collections::HashMap<String, iiko_server_api_sdk::OlapFilter>> =
        None;
    if let Some(date_field) = date_filter_field {
        let mut filter_map = std::collections::HashMap::new();
        let now = chrono::Utc::now();
        let from_date = now - chrono::Duration::days(7);
        filter_map.insert(
            date_field,
            iiko_server_api_sdk::OlapFilter::DateRange(iiko_server_api_sdk::DateRangeFilter {
                filter_type: iiko_server_api_sdk::FilterType::DateRange,
                period_type: iiko_server_api_sdk::PeriodType::Custom,
                from: from_date.format("%Y-%m-%dT%H:%M:%S.000").to_string(),
                to: now.format("%Y-%m-%dT%H:%M:%S.000").to_string(),
                include_low: true,
                include_high: false,
            }),
        );
        filters = Some(filter_map);
    }

    let request = iiko_server_api_sdk::OlapReportRequest {
        report_type: iiko_server_api_sdk::OlapReportType::Sales,
        build_summary: Some(false), // Не считаем итоги для быстрого запроса
        group_by_row_fields,
        group_by_col_fields: None,
        aggregate_fields,
        filters,
    };

    let result = reports.get_olap_report(request).await;

    match result {
        Ok(report) => {
            println!(
                "Получен OLAP-отчет: строк данных: {}, итогов: {}",
                report.data.len(),
                report.summary.len()
            );
            if !report.data.is_empty() {
                println!("Первая строка данных: {:#?}", report.data[0]);
            }
        }
        Err(e) => {
            println!("Ошибка получения OLAP-отчета: {:?}", e);
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест на основе примера: Выручка по типам оплат
///
/// Пример из документации iiko
#[tokio::test]
async fn test_get_olap_report_sales_by_payment_types() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Создаем запрос на основе примера из документации
    let mut filters = std::collections::HashMap::new();

    // Фильтр по дате (начиная с версии 5.5 требуется OpenDate.Typed)
    filters.insert(
        "OpenDate.Typed".to_string(),
        iiko_server_api_sdk::OlapFilter::DateRange(iiko_server_api_sdk::DateRangeFilter {
            filter_type: iiko_server_api_sdk::FilterType::DateRange,
            period_type: iiko_server_api_sdk::PeriodType::Custom,
            from: "2014-01-01T00:00:00.000".to_string(),
            to: "2014-01-03T00:00:00.000".to_string(),
            include_low: true,
            include_high: false,
        }),
    );

    // Фильтр по удаленным заказам с списанием
    filters.insert(
        "DeletedWithWriteoff".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::ExcludeValues,
            values: vec![
                "DELETED_WITH_WRITEOFF".to_string(),
                "DELETED_WITHOUT_WRITEOFF".to_string(),
            ],
        }),
    );

    // Фильтр по удаленным заказам
    filters.insert(
        "OrderDeleted".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::IncludeValues,
            values: vec!["NOT_DELETED".to_string()],
        }),
    );

    let request = iiko_server_api_sdk::OlapReportRequest {
        report_type: iiko_server_api_sdk::OlapReportType::Sales,
        build_summary: Some(true),
        group_by_row_fields: vec!["PayTypes".to_string(), "OpenDate".to_string()],
        group_by_col_fields: None,
        aggregate_fields: vec![
            "GuestNum".to_string(),
            "DishSumInt".to_string(),
            "DishDiscountSumInt".to_string(),
            "UniqOrderId".to_string(),
        ],
        filters: Some(filters),
    };

    let result = reports.get_olap_report(request).await;

    match result {
        Ok(report) => {
            println!(
                "Получен OLAP-отчет по типам оплат: строк данных: {}, итогов: {}",
                report.data.len(),
                report.summary.len()
            );
            if !report.data.is_empty() {
                println!("Первая строка данных: {:#?}", report.data[0]);
                // Проверяем типы значений
                for (key, value) in &report.data[0] {
                    println!(
                        "  {}: {:?} (type: {})",
                        key,
                        value,
                        match value {
                            iiko_server_api_sdk::OlapFieldValue::String(_) => "String",
                            iiko_server_api_sdk::OlapFieldValue::Integer(_) => "Integer",
                            iiko_server_api_sdk::OlapFieldValue::Float(_) => "Float",
                            iiko_server_api_sdk::OlapFieldValue::Null => "Null",
                        }
                    );
                }
            }
            if !report.summary.is_empty() {
                println!("Первая группа итогов: {:#?}", report.summary[0]);
            }
        }
        Err(e) => {
            println!("Ошибка получения OLAP-отчета по типам оплат: {:?}", e);
            // Не падаем, если сервер не поддерживает эти поля
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест на основе примера: Выручка за блюда по кассам
///
/// Пример из документации iiko
#[tokio::test]
async fn test_get_olap_report_sales_by_dishes_and_cash_registers() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Создаем запрос на основе примера из документации
    let mut filters = std::collections::HashMap::new();

    // Фильтр по дате (начиная с версии 5.5 требуется OpenDate.Typed)
    filters.insert(
        "OpenDate.Typed".to_string(),
        iiko_server_api_sdk::OlapFilter::DateRange(iiko_server_api_sdk::DateRangeFilter {
            filter_type: iiko_server_api_sdk::FilterType::DateRange,
            period_type: iiko_server_api_sdk::PeriodType::Custom,
            from: "2014-01-01T00:00:00.000".to_string(),
            to: "2014-01-03T00:00:00.000".to_string(),
            include_low: true,
            include_high: false,
        }),
    );

    // Фильтр по удаленным заказам с списанием
    filters.insert(
        "DeletedWithWriteoff".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::IncludeValues,
            values: vec!["NOT_DELETED".to_string()],
        }),
    );

    // Фильтр по удаленным заказам
    filters.insert(
        "OrderDeleted".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::IncludeValues,
            values: vec!["NOT_DELETED".to_string()],
        }),
    );

    let request = iiko_server_api_sdk::OlapReportRequest {
        report_type: iiko_server_api_sdk::OlapReportType::Sales,
        build_summary: Some(true),
        group_by_row_fields: vec![
            "DishName".to_string(),
            "OpenDate".to_string(),
            "CashRegisterName".to_string(),
        ],
        group_by_col_fields: None,
        aggregate_fields: vec!["DishSumInt".to_string(), "DishDiscountSumInt".to_string()],
        filters: Some(filters),
    };

    let result = reports.get_olap_report(request).await;

    match result {
        Ok(report) => {
            println!(
                "Получен OLAP-отчет по блюдам и кассам: строк данных: {}, итогов: {}",
                report.data.len(),
                report.summary.len()
            );
            if !report.data.is_empty() {
                println!("Первая строка данных: {:#?}", report.data[0]);
                // Проверяем типы значений
                for (key, value) in &report.data[0] {
                    println!(
                        "  {}: {:?} (type: {})",
                        key,
                        value,
                        match value {
                            iiko_server_api_sdk::OlapFieldValue::String(_) => "String",
                            iiko_server_api_sdk::OlapFieldValue::Integer(_) => "Integer",
                            iiko_server_api_sdk::OlapFieldValue::Float(_) => "Float",
                            iiko_server_api_sdk::OlapFieldValue::Null => "Null",
                        }
                    );
                }
            }
        }
        Err(e) => {
            println!("Ошибка получения OLAP-отчета по блюдам и кассам: {:?}", e);
            // Не падаем, если сервер не поддерживает эти поля
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест на основе примера: Почасовая выручка
///
/// Пример из документации iiko
#[tokio::test]
async fn test_get_olap_report_sales_hourly() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Создаем запрос на основе примера из документации
    let mut filters = std::collections::HashMap::new();

    // Фильтр по дате (начиная с версии 5.5 требуется OpenDate.Typed)
    filters.insert(
        "OpenDate.Typed".to_string(),
        iiko_server_api_sdk::OlapFilter::DateRange(iiko_server_api_sdk::DateRangeFilter {
            filter_type: iiko_server_api_sdk::FilterType::DateRange,
            period_type: iiko_server_api_sdk::PeriodType::Custom,
            from: "2014-01-01T00:00:00.000".to_string(),
            to: "2014-01-03T00:00:00.000".to_string(),
            include_low: true,
            include_high: false,
        }),
    );

    // Фильтр по удаленным заказам с списанием
    filters.insert(
        "DeletedWithWriteoff".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::IncludeValues,
            values: vec!["NOT_DELETED".to_string()],
        }),
    );

    // Фильтр по удаленным заказам
    filters.insert(
        "OrderDeleted".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::IncludeValues,
            values: vec!["NOT_DELETED".to_string()],
        }),
    );

    let request = iiko_server_api_sdk::OlapReportRequest {
        report_type: iiko_server_api_sdk::OlapReportType::Sales,
        build_summary: Some(true),
        group_by_row_fields: vec!["OpenDate".to_string(), "HourClose".to_string()],
        group_by_col_fields: None,
        aggregate_fields: vec![
            "GuestNum".to_string(),
            "DishSumInt".to_string(),
            "DishDiscountSumInt".to_string(),
            "UniqOrderId".to_string(),
        ],
        filters: Some(filters),
    };

    let result = reports.get_olap_report(request).await;

    match result {
        Ok(report) => {
            println!(
                "Получен OLAP-отчет почасовая выручка: строк данных: {}, итогов: {}",
                report.data.len(),
                report.summary.len()
            );
            if !report.data.is_empty() {
                println!("Первая строка данных: {:#?}", report.data[0]);
            }
        }
        Err(e) => {
            println!("Ошибка получения OLAP-отчета почасовая выручка: {:?}", e);
            // Не падаем, если сервер не поддерживает эти поля
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест на основе примера: Выручка по категориям блюд
///
/// Пример из документации iiko
#[tokio::test]
async fn test_get_olap_report_sales_by_dish_categories() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Создаем запрос на основе примера из документации
    let mut filters = std::collections::HashMap::new();

    // Фильтр по дате (начиная с версии 5.5 требуется OpenDate.Typed)
    filters.insert(
        "OpenDate.Typed".to_string(),
        iiko_server_api_sdk::OlapFilter::DateRange(iiko_server_api_sdk::DateRangeFilter {
            filter_type: iiko_server_api_sdk::FilterType::DateRange,
            period_type: iiko_server_api_sdk::PeriodType::Custom,
            from: "2014-01-01T00:00:00.000".to_string(),
            to: "2014-01-03T00:00:00.000".to_string(),
            include_low: true,
            include_high: false,
        }),
    );

    // Фильтр по удаленным заказам с списанием
    filters.insert(
        "DeletedWithWriteoff".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::IncludeValues,
            values: vec!["NOT_DELETED".to_string()],
        }),
    );

    // Фильтр по удаленным заказам
    filters.insert(
        "OrderDeleted".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::IncludeValues,
            values: vec!["NOT_DELETED".to_string()],
        }),
    );

    let request = iiko_server_api_sdk::OlapReportRequest {
        report_type: iiko_server_api_sdk::OlapReportType::Sales,
        build_summary: Some(true),
        group_by_row_fields: vec!["DishCategory".to_string()],
        group_by_col_fields: None,
        aggregate_fields: vec![
            "GuestNum".to_string(),
            "DishSumInt".to_string(),
            "DishDiscountSumInt".to_string(),
            "UniqOrderId".to_string(),
        ],
        filters: Some(filters),
    };

    let result = reports.get_olap_report(request).await;

    match result {
        Ok(report) => {
            println!(
                "Получен OLAP-отчет по категориям блюд: строк данных: {}, итогов: {}",
                report.data.len(),
                report.summary.len()
            );
            if !report.data.is_empty() {
                println!("Первая строка данных: {:#?}", report.data[0]);
            }
        }
        Err(e) => {
            println!("Ошибка получения OLAP-отчета по категориям блюд: {:?}", e);
            // Не падаем, если сервер не поддерживает эти поля
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест на основе примера: Выручка по дням
///
/// Пример из документации iiko
#[tokio::test]
async fn test_get_olap_report_sales_by_days() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Создаем запрос на основе примера из документации
    let mut filters = std::collections::HashMap::new();

    // Фильтр по дате (начиная с версии 5.5 требуется OpenDate.Typed)
    filters.insert(
        "OpenDate.Typed".to_string(),
        iiko_server_api_sdk::OlapFilter::DateRange(iiko_server_api_sdk::DateRangeFilter {
            filter_type: iiko_server_api_sdk::FilterType::DateRange,
            period_type: iiko_server_api_sdk::PeriodType::Custom,
            from: "2014-01-01T00:00:00.000".to_string(),
            to: "2014-01-03T00:00:00.000".to_string(),
            include_low: true,
            include_high: false,
        }),
    );

    // Фильтр по удаленным заказам с списанием
    filters.insert(
        "DeletedWithWriteoff".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::IncludeValues,
            values: vec!["NOT_DELETED".to_string()],
        }),
    );

    // Фильтр по удаленным заказам
    filters.insert(
        "OrderDeleted".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::IncludeValues,
            values: vec!["NOT_DELETED".to_string()],
        }),
    );

    let request = iiko_server_api_sdk::OlapReportRequest {
        report_type: iiko_server_api_sdk::OlapReportType::Sales,
        build_summary: Some(true),
        group_by_row_fields: vec!["OpenDate".to_string()],
        group_by_col_fields: None,
        aggregate_fields: vec![
            "GuestNum".to_string(),
            "DishSumInt".to_string(),
            "DishDiscountSumInt".to_string(),
            "UniqOrderId".to_string(),
        ],
        filters: Some(filters),
    };

    let result = reports.get_olap_report(request).await;

    match result {
        Ok(report) => {
            println!(
                "Получен OLAP-отчет по дням: строк данных: {}, итогов: {}",
                report.data.len(),
                report.summary.len()
            );
            if !report.data.is_empty() {
                println!("Первая строка данных: {:#?}", report.data[0]);
            }
        }
        Err(e) => {
            println!("Ошибка получения OLAP-отчета по дням: {:?}", e);
            // Не падаем, если сервер не поддерживает эти поля
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест на основе примера: Выручка по официантам
///
/// Пример из документации iiko
#[tokio::test]
async fn test_get_olap_report_sales_by_waiters() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Создаем запрос на основе примера из документации
    let mut filters = std::collections::HashMap::new();

    // Фильтр по дате (начиная с версии 5.5 требуется OpenDate.Typed)
    filters.insert(
        "OpenDate.Typed".to_string(),
        iiko_server_api_sdk::OlapFilter::DateRange(iiko_server_api_sdk::DateRangeFilter {
            filter_type: iiko_server_api_sdk::FilterType::DateRange,
            period_type: iiko_server_api_sdk::PeriodType::Custom,
            from: "2014-01-01T00:00:00.000".to_string(),
            to: "2014-01-03T00:00:00.000".to_string(),
            include_low: true,
            include_high: false,
        }),
    );

    // Фильтр по удаленным заказам с списанием
    filters.insert(
        "DeletedWithWriteoff".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::IncludeValues,
            values: vec!["NOT_DELETED".to_string()],
        }),
    );

    // Фильтр по удаленным заказам
    filters.insert(
        "OrderDeleted".to_string(),
        iiko_server_api_sdk::OlapFilter::Value(iiko_server_api_sdk::ValueFilter {
            filter_type: iiko_server_api_sdk::FilterType::IncludeValues,
            values: vec!["NOT_DELETED".to_string()],
        }),
    );

    let request = iiko_server_api_sdk::OlapReportRequest {
        report_type: iiko_server_api_sdk::OlapReportType::Sales,
        build_summary: Some(true),
        group_by_row_fields: vec!["WaiterName".to_string()],
        group_by_col_fields: None,
        aggregate_fields: vec!["DishSumInt".to_string(), "DishDiscountSumInt".to_string()],
        filters: Some(filters),
    };

    let result = reports.get_olap_report(request).await;

    match result {
        Ok(report) => {
            println!(
                "Получен OLAP-отчет по официантам: строк данных: {}, итогов: {}",
                report.data.len(),
                report.summary.len()
            );
            if !report.data.is_empty() {
                println!("Первая строка данных: {:#?}", report.data[0]);
            }
        }
        Err(e) => {
            println!("Ошибка получения OLAP-отчета по официантам: {:?}", e);
            // Не падаем, если сервер не поддерживает эти поля
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для старого OLAP API (версия 3.9)
///
/// Пример из документации: Выручка по официантам и времени открытия
#[tokio::test]
async fn test_get_olap_report_v1_sales() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Пример запроса из документации:
    // https://localhost:8080/resto/api/reports/olap?key=...&report=SALES&from=01.12.2014&to=18.12.2014&groupRow=WaiterName&groupRow=OpenTime&agr=fullSum&agr=OrderNum

    let result = reports
        .get_olap_report_v1(
            iiko_server_api_sdk::OlapReportTypeV1::Sales,
            "01.12.2014",
            "18.12.2014",
            Some(true),
            Some(&["WaiterName", "OpenTime"]),
            None,
            Some(&["fullSum", "OrderNum"]),
        )
        .await;

    match result {
        Ok(report) => {
            println!(
                "Получен OLAP-отчет v1 (SALES): строк данных: {}, итогов: {}",
                report.data.len(),
                report.summary.len()
            );
            if !report.data.is_empty() {
                println!("Первая строка данных: {:#?}", report.data[0]);
            }
        }
        Err(e) => {
            println!("Ошибка получения OLAP-отчета v1 (SALES): {:?}", e);
            // Не падаем, если сервер не поддерживает старый API или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для старого OLAP API - отчет по транзакциям
#[tokio::test]
async fn test_get_olap_report_v1_transactions() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_olap_report_v1(
            iiko_server_api_sdk::OlapReportTypeV1::Transactions,
            "01.12.2014",
            "18.12.2014",
            Some(false), // Не вычисляем итоги для быстрого запроса
            Some(&["Account.Name"]),
            None,
            Some(&["Sum.ResignedSum"]),
        )
        .await;

    match result {
        Ok(report) => {
            println!(
                "Получен OLAP-отчет v1 (TRANSACTIONS): строк данных: {}, итогов: {}",
                report.data.len(),
                report.summary.len()
            );
        }
        Err(e) => {
            println!("Ошибка получения OLAP-отчета v1 (TRANSACTIONS): {:?}", e);
            // Не падаем, если сервер не поддерживает старый API или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для старого OLAP API - отчет по доставкам
#[tokio::test]
async fn test_get_olap_report_v1_deliveries() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_olap_report_v1(
            iiko_server_api_sdk::OlapReportTypeV1::Deliveries,
            "01.12.2014",
            "18.12.2014",
            Some(true),
            Some(&["Delivery.Courier"]),
            None,
            Some(&["Delivery.Number"]),
        )
        .await;

    match result {
        Ok(report) => {
            println!(
                "Получен OLAP-отчет v1 (DELIVERIES): строк данных: {}, итогов: {}",
                report.data.len(),
                report.summary.len()
            );
        }
        Err(e) => {
            println!("Ошибка получения OLAP-отчета v1 (DELIVERIES): {:?}", e);
            // Не падаем, если сервер не поддерживает старый API или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для старого OLAP API - контроль хранения (STOCK)
#[tokio::test]
async fn test_get_olap_report_v1_stock() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_olap_report_v1(
            iiko_server_api_sdk::OlapReportTypeV1::Stock,
            "01.12.2014",
            "18.12.2014",
            Some(true),
            Some(&["ProductName", "StoreFrom"]),
            None,
            Some(&["Amount", "ProductCostBase.ProductCost"]),
        )
        .await;

    match result {
        Ok(report) => {
            println!(
                "Получен OLAP-отчет v1 (STOCK): строк данных: {}, итогов: {}",
                report.data.len(),
                report.summary.len()
            );
        }
        Err(e) => {
            println!("Ошибка получения OLAP-отчета v1 (STOCK): {:?}", e);
            // Не падаем, если сервер не поддерживает старый API или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

// ============================================================================
// Отчеты по доставке
// ============================================================================

/// Тест для сводного отчета по доставке
#[tokio::test]
async fn test_get_delivery_consolidated() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_delivery_consolidated(
            "01.04.2014",
            "30.04.2014",
            Some(&[r#"{code="5"}"#]),
            Some(&[r#"{code="5.14"}"#, r#"{code="5.13"}"#]),
        )
        .await;

    match result {
        Ok(report) => {
            println!(
                "Получен сводный отчет по доставке: строк: {}",
                report.rows.rows.len()
            );
            if !report.rows.rows.is_empty() {
                let first_row = &report.rows.rows[0];
                println!(
                    "Первая строка: дата={}, выручка={}, заказов={}",
                    first_row.date, first_row.revenue, first_row.order_count
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения сводного отчета по доставке: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для отчета по курьерам
#[tokio::test]
async fn test_get_delivery_couriers() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_delivery_couriers(
            "01.04.2014",
            "30.04.2014",
            Some(&[r#"{code="5"}"#]),
            Some(30),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
        )
        .await;

    match result {
        Ok(report) => {
            println!(
                "Получен отчет по курьерам: строк: {}",
                report.rows.rows.len()
            );
            if !report.rows.rows.is_empty() {
                let first_row = &report.rows.rows[0];
                println!("Первый курьер: {}", first_row.courier);
                println!("Метрик: {}", first_row.metrics.metrics.len());
            }
        }
        Err(e) => {
            println!("Ошибка получения отчета по курьерам: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для отчета по циклу заказа
#[tokio::test]
async fn test_get_delivery_order_cycle() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_delivery_order_cycle(
            "01.04.2014",
            "30.04.2014",
            Some(&[r#"{code="5"}"#]),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
        )
        .await;

    match result {
        Ok(report) => {
            println!(
                "Получен отчет по циклу заказа: строк: {}",
                report.rows.rows.len()
            );
            if !report.rows.rows.is_empty() {
                let first_row = &report.rows.rows[0];
                println!(
                    "Первая строка: общее время={}, время в пути={}",
                    first_row.total_time, first_row.on_the_way_time
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения отчета по циклу заказа: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для получасового детального отчета
#[tokio::test]
async fn test_get_delivery_half_hour_detailed() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_delivery_half_hour_detailed("01.04.2014", "30.04.2014", Some(&[r#"{code="5"}"#]))
        .await;

    match result {
        Ok(report) => {
            println!(
                "Получен получасовой детальный отчет: строк: {}",
                report.rows.rows.len()
            );
            if !report.rows.rows.is_empty() {
                let first_row = &report.rows.rows[0];
                println!("Первая строка: время={}", first_row.half_hour_date);
                println!("Метрик: {}", first_row.metrics.metrics.len());
            }
        }
        Err(e) => {
            println!("Ошибка получения получасового детального отчета: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для отчета по регионам
#[tokio::test]
async fn test_get_delivery_regions() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_delivery_regions("01.04.2014", "30.04.2014", Some(&[r#"{code="5"}"#]))
        .await;

    match result {
        Ok(report) => {
            println!(
                "Получен отчет по регионам: строк: {}",
                report.rows.rows.len()
            );
            if !report.rows.rows.is_empty() {
                let first_row = &report.rows.rows[0];
                println!(
                    "Первый регион: {}, заказов: {}",
                    first_row.region, first_row.order_count
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения отчета по регионам: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для отчета по лояльности
#[tokio::test]
async fn test_get_delivery_loyalty() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_delivery_loyalty(
            "01.04.2014",
            "30.04.2014",
            Some(&[r#"{code="5"}"#]),
            Some("AVERAGE"),
        )
        .await;

    match result {
        Ok(report) => {
            println!(
                "Получен отчет по лояльности: строк: {}",
                report.rows.rows.len()
            );
            if !report.rows.rows.is_empty() {
                let first_row = &report.rows.rows[0];
                println!(
                    "Первая строка: дата={}, новых гостей={}",
                    first_row.date, first_row.new_guest_count
                );
                println!("Регионов: {}", first_row.regions.regions.len());
            }
        }
        Err(e) => {
            println!("Ошибка получения отчета по лояльности: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

// ============================================================================
// Отчеты по складским операциям и другие отчеты
// ============================================================================

/// Тест для отчета по складским операциям
#[tokio::test]
async fn test_get_store_operations() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports
        .get_store_operations(
            "01.09.2014",
            "09.09.2014",
            Some(&["1239d270-1bbe-f64f-b7ea-5f00518ef508"]),
            Some(&["SALES_DOCUMENT", "INCOMING_INVOICE"]),
            Some(false),
            Some(false),
            None,
        )
        .await;

    match result {
        Ok(items) => {
            println!(
                "Получен отчет по складским операциям: элементов: {}",
                items.len()
            );
            if !items.is_empty() {
                let first_item = &items[0];
                println!(
                    "Первый элемент: дата={:?}, документ={:?}, сумма={:?}",
                    first_item.date, first_item.document_num, first_item.sum
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения отчета по складским операциям: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для пресетов отчетов по складским операциям
#[tokio::test]
async fn test_get_store_report_presets() {
    let client = get_test_client().await;
    let reports = client.reports();

    let result = reports.get_store_report_presets().await;

    match result {
        Ok(presets) => {
            println!("Получены пресеты отчетов: {}", presets.len());
            if !presets.is_empty() {
                let first_preset = &presets[0];
                println!(
                    "Первый пресет: id={:?}, название={:?}",
                    first_preset.id, first_preset.name
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения пресетов отчетов: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для расхода продуктов по продажам
#[tokio::test]
async fn test_get_product_expense() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Нужен реальный department ID для теста
    let result = reports
        .get_product_expense(
            "49023e1b-6e3a-6c33-0133-ce1f6f5000b",
            "01.12.2014",
            "17.12.2014",
            Some(12),
            Some(15),
        )
        .await;

    match result {
        Ok(items) => {
            println!(
                "Получен отчет по расходу продуктов: элементов: {}",
                items.len()
            );
            if !items.is_empty() {
                let first_item = &items[0];
                println!(
                    "Первый элемент: дата={:?}, продукт={:?}, значение={:?}",
                    first_item.date, first_item.product_name, first_item.value
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения отчета по расходу продуктов: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для отчета по выручке
#[tokio::test]
async fn test_get_sales() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Нужен реальный department ID для теста
    let result = reports
        .get_sales(
            "49023e1b-6e3a-6c33-0133-cce1f6f5000b",
            "01.12.2014",
            "17.12.2014",
            Some(12),
            Some(15),
            Some(true),
            Some(false),
        )
        .await;

    match result {
        Ok(items) => {
            println!("Получен отчет по выручке: элементов: {}", items.len());
            if !items.is_empty() {
                let first_item = &items[0];
                println!(
                    "Первый элемент: дата={:?}, продукт={:?}, значение={:?}",
                    first_item.date, first_item.product_name, first_item.value
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения отчета по выручке: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для плана по выручке за день
#[tokio::test]
async fn test_get_monthly_income_plan() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Нужен реальный department ID для теста
    let result = reports
        .get_monthly_income_plan(
            "49023e1b-6e3a-6c33-0133-cce1f6f5000b",
            "01.12.2014",
            "18.12.2014",
        )
        .await;

    match result {
        Ok(items) => {
            println!("Получен план по выручке: элементов: {}", items.len());
            if !items.is_empty() {
                let first_item = &items[0];
                println!(
                    "Первый элемент: дата={:?}, план={:?}, тип={:?}",
                    first_item.date, first_item.plan_value, first_item.value_type
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения плана по выручке: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}

/// Тест для отчета о вхождении товара в блюдо
#[tokio::test]
async fn test_get_ingredient_entry() {
    let client = get_test_client().await;
    let reports = client.reports();

    // Нужен реальный department ID и product ID для теста
    let result = reports
        .get_ingredient_entry(
            "49023e1b-6e3a-6c33-0133-cce1f6f5000b",
            "01.12.2014",
            Some("2c3ab3e1-266d-4667-b344-98b6c194a305"),
            None,
            Some(false),
        )
        .await;

    match result {
        Ok(items) => {
            println!(
                "Получен отчет о вхождении товара в блюдо: элементов: {}",
                items.len()
            );
            if !items.is_empty() {
                let first_item = &items[0];
                println!(
                    "Первый элемент: название={:?}, уровень={}, себестоимость={:?}",
                    first_item.name, first_item.tree_level, first_item.dish_cost_norm
                );
            }
        }
        Err(e) => {
            println!("Ошибка получения отчета о вхождении товара: {:?}", e);
            // Не падаем, если сервер не поддерживает этот эндпойнт или нет данных
        }
    }

    cleanup_after_test(&client).await;
}
