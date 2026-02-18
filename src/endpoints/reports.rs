use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::response::reports::{
    BudgetPlanItemDto, DayDishValue, DeliveryConsolidatedReport, DeliveryCouriersReport,
    DeliveryHalfHourDetailedReport, DeliveryLoyaltyReport, DeliveryOrderCycleReport,
    DeliveryRegionsReport, IngredientEntryDto, StoreReportItemDto, StoreReportPreset,
};
use crate::xml::response::{
    BalanceCounteragent, BalanceStore, EgaisMarksList, OlapColumns, OlapReportRequest,
    OlapReportResponse, OlapReportType, OlapReportTypeV1,
};
use quick_xml::de::from_str;
use serde_json;

pub struct ReportsEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> ReportsEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Балансы по счетам, контрагентам и подразделениям
    ///
    /// Версия iiko: 5.2
    /// Endpoint: GET `/v2/reports/balance/counteragents`
    ///
    /// # Параметры
    /// - `timestamp`: учетная дата-время отчета в формате yyyy-MM-dd'T'HH:mm:ss (обязательный)
    /// - `account`: ID счета для фильтрации (необязательный, можно указать несколько)
    /// - `counteragent`: ID контрагента для фильтрации (необязательный, можно указать несколько)
    /// - `department`: ID подразделения для фильтрации (необязательный, можно указать несколько)
    ///
    /// # Что в ответе
    /// Возвращает денежные балансы по указанным счетам, контрагентам и подразделениям на заданную учетную дату-время.
    pub async fn get_balance_counteragents(
        &self,
        timestamp: &str,
        accounts: Option<&[&str]>,
        counteragents: Option<&[&str]>,
        departments: Option<&[&str]>,
    ) -> Result<Vec<BalanceCounteragent>> {
        let mut params = vec![("timestamp", timestamp)];

        if let Some(accs) = accounts {
            for account in accs {
                params.push(("account", account));
            }
        }

        if let Some(ctrs) = counteragents {
            for counteragent in ctrs {
                params.push(("counteragent", counteragent));
            }
        }

        if let Some(depts) = departments {
            for department in depts {
                params.push(("department", department));
            }
        }

        let response_json = self
            .client
            .get_with_params("v2/reports/balance/counteragents", &params)
            .await?;

        let balances: Vec<BalanceCounteragent> = serde_json::from_str(&response_json)?;
        Ok(balances)
    }

    /// Остатки на складах
    ///
    /// Версия iiko: 5.2
    /// Endpoint: GET `/v2/reports/balance/stores`
    ///
    /// # Параметры
    /// - `timestamp`: учетная дата-время отчета в формате yyyy-MM-dd'T'HH:mm:ss (обязательный)
    /// - `department`: ID подразделения для фильтрации (необязательный, можно указать несколько)
    /// - `store`: ID склада для фильтрации (необязательный, можно указать несколько)
    /// - `product`: ID элемента номенклатуры для фильтрации (необязательный, можно указать несколько)
    ///
    /// # Что в ответе
    /// Возвращает количественные (amount) и денежные (sum) остатки товаров (product) на складах (store) на заданную учетную дату-время.
    pub async fn get_balance_stores(
        &self,
        timestamp: &str,
        departments: Option<&[&str]>,
        stores: Option<&[&str]>,
        products: Option<&[&str]>,
    ) -> Result<Vec<BalanceStore>> {
        let mut params = vec![("timestamp", timestamp)];

        if let Some(depts) = departments {
            for department in depts {
                params.push(("department", department));
            }
        }

        if let Some(strs) = stores {
            for store in strs {
                params.push(("store", store));
            }
        }

        if let Some(prods) = products {
            for product in prods {
                params.push(("product", product));
            }
        }

        let response_json = self
            .client
            .get_with_params("v2/reports/balance/stores", &params)
            .await?;

        let balances: Vec<BalanceStore> = serde_json::from_str(&response_json)?;
        Ok(balances)
    }

    /// Отчет по балансу на 3 регистре ЕГАИС (акцизные марки)
    ///
    /// Получение обновлений состояния на 3 регистре
    ///
    /// Версия iiko: 7.4
    /// Endpoint: GET `/v2/reports/egais/marks/list`
    ///
    /// # Параметры
    /// - `fs_rar_id`: Список РАР-идентификаторов организаций, баланс которых запрашивается (необязательный, по умолчанию возвращаются данные для всех организаций)
    /// - `revision_from`: Номер ревизии, начиная с которой необходимо отфильтровать сущности (необязательный, по умолчанию -1)
    ///
    /// # Что в ответе
    /// Возвращает данные по балансу акцизных марок на 3 регистре ЕГАИС.
    pub async fn get_egais_marks_list(
        &self,
        fs_rar_ids: Option<&[&str]>,
        revision_from: Option<i64>,
    ) -> Result<EgaisMarksList> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let revision_str;

        if let Some(ids) = fs_rar_ids {
            for id in ids {
                params.push(("fsRarId", *id));
            }
        }

        if let Some(rev) = revision_from {
            revision_str = rev.to_string();
            params.push(("revisionFrom", revision_str.as_str()));
        }

        let response_json = self
            .client
            .get_with_params("v2/reports/egais/marks/list", &params)
            .await?;

        let marks_list: EgaisMarksList = serde_json::from_str(&response_json)?;
        Ok(marks_list)
    }

    /// Получить список полей OLAP-отчета
    ///
    /// Версия iiko: 4.1
    /// Endpoint: GET `/v2/reports/olap/columns`
    ///
    /// # Параметры
    /// - `report_type`: Тип отчета (SALES, TRANSACTIONS, DELIVERIES)
    ///
    /// # Что в ответе
    /// Возвращает JSON структуру списка полей с информацией по возможностям фильтрации, агрегации и группировки.
    /// Устаревшие поля (deprecated) не выводятся.
    pub async fn get_olap_columns(
        &self,
        report_type: OlapReportType,
    ) -> Result<OlapColumns> {
        let report_type_str = match report_type {
            OlapReportType::Sales => "SALES",
            OlapReportType::Transactions => "TRANSACTIONS",
            OlapReportType::Deliveries => "DELIVERIES",
        };

        let response_json = self
            .client
            .get_with_params("v2/reports/olap/columns", &[("reportType", report_type_str)])
            .await?;

        let columns: OlapColumns = serde_json::from_str(&response_json)?;
        Ok(columns)
    }

    /// Получить данные OLAP-отчета
    ///
    /// Версия iiko: 4.1
    /// Endpoint: POST `/v2/reports/olap`
    ///
    /// # Параметры
    /// - `request`: Запрос на получение OLAP-отчета
    ///
    /// # Что в ответе
    /// Возвращает данные отчета с промежуточными и общими итогами (если buildSummary = true).
    ///
    /// # Важно
    /// - Рекомендуется использовать не более 7 полей
    /// - Используйте `build_summary=false` если не нужны общие результаты
    /// - Начиная с версии 5.5, каждый OLAP-запрос должен содержать фильтр по дате
    pub async fn get_olap_report(
        &self,
        request: OlapReportRequest,
    ) -> Result<OlapReportResponse> {
        let json_body = serde_json::to_string(&request)?;

        let response_json = self
            .client
            .post_json("v2/reports/olap", &json_body, &[])
            .await?;

        let report: OlapReportResponse = serde_json::from_str(&response_json)?;
        Ok(report)
    }

    /// Получить данные OLAP-отчета (старый API)
    ///
    /// Версия iiko: 3.9
    /// Endpoint: GET `/reports/olap`
    ///
    /// # Параметры
    /// - `report`: Тип отчета (SALES, TRANSACTIONS, DELIVERIES, STOCK)
    /// - `from`: Начальная дата в формате DD.MM.YYYY (например, "01.12.2014")
    /// - `to`: Конечная дата в формате DD.MM.YYYY (например, "18.12.2014")
    /// - `summary`: Вычислять ли итоговые значения (по умолчанию true, с версии 9.1.2 по умолчанию false)
    /// - `group_row`: Поля для группировки по строкам (можно указать несколько)
    /// - `group_col`: Поля для группировки по столбцам (можно указать несколько, необязательно)
    /// - `agr`: Поля для агрегации (можно указать несколько)
    ///
    /// # Что в ответе
    /// Возвращает данные отчета с промежуточными и общими итогами (если summary = true).
    ///
    /// # Важно
    /// - Этот метод использует старый API (версия 3.9)
    /// - Для новых проектов рекомендуется использовать `get_olap_report` (версия 4.1+)
    /// - Формат дат: DD.MM.YYYY (не ISO)
    /// - Рекомендуется использовать не более 7 полей
    ///
    /// # Пример
    /// ```no_run
    /// use iiko_server_api_sdk::{IikoClient, OlapReportTypeV1};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = IikoClient::new(...)?;
    /// let reports = client.reports();
    ///
    /// let result = reports.get_olap_report_v1(
    ///     OlapReportTypeV1::Sales,
    ///     "01.12.2014",
    ///     "18.12.2014",
    ///     Some(true),
    ///     Some(&["WaiterName", "OpenTime"]),
    ///     None,
    ///     Some(&["fullSum", "OrderNum"]),
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_olap_report_v1(
        &self,
        report: OlapReportTypeV1,
        from: &str,
        to: &str,
        summary: Option<bool>,
        group_row: Option<&[&str]>,
        group_col: Option<&[&str]>,
        agr: Option<&[&str]>,
    ) -> Result<OlapReportResponse> {
        let mut params = vec![
            ("report", report.as_str()),
            ("from", from),
            ("to", to),
        ];

        if let Some(sum) = summary {
            params.push(("summary", if sum { "true" } else { "false" }));
        }

        if let Some(rows) = group_row {
            for row in rows {
                params.push(("groupRow", row));
            }
        }

        if let Some(cols) = group_col {
            for col in cols {
                params.push(("groupCol", col));
            }
        }

        if let Some(agr_fields) = agr {
            for field in agr_fields {
                params.push(("agr", field));
            }
        }

        let response_json = self
            .client
            .get_with_params("reports/olap", &params)
            .await?;

        let report: OlapReportResponse = serde_json::from_str(&response_json)?;
        Ok(report)
    }

    // ============================================================================
    // Отчеты по доставке
    // ============================================================================

    /// Сводный отчет по доставке
    ///
    /// Версия iiko: неизвестна
    /// Endpoint: GET `/reports/delivery/consolidated`
    ///
    /// # Параметры
    /// - `department`: Подразделения (код или ID) в формате `{code="005"}` или `{id="guid"}` (можно указать несколько)
    /// - `date_from`: Дата начала отчета в формате DD.MM.YYYY или YYYY-MM-DD
    /// - `date_to`: Дата окончания отчета в формате DD.MM.YYYY или YYYY-MM-DD
    /// - `writeoff_accounts`: Список счетов списания (код или ID) в формате `{code="5.14"}` (можно указать несколько)
    ///
    /// # Что в ответе
    /// Возвращает сводный отчет по доставке с метриками по датам.
    pub async fn get_delivery_consolidated(
        &self,
        date_from: &str,
        date_to: &str,
        departments: Option<&[&str]>,
        writeoff_accounts: Option<&[&str]>,
    ) -> Result<DeliveryConsolidatedReport> {
        let mut params = vec![("dateFrom", date_from), ("dateTo", date_to)];

        if let Some(depts) = departments {
            for dept in depts {
                params.push(("department", dept));
            }
        }

        if let Some(accounts) = writeoff_accounts {
            for account in accounts {
                params.push(("writeoffAccounts", account));
            }
        }

        let response_xml = self
            .client
            .get_with_params("reports/delivery/consolidated", &params)
            .await?;

        let report: DeliveryConsolidatedReport = from_str(&response_xml)?;
        Ok(report)
    }

    /// Отчет по курьерам
    ///
    /// Версия iiko: неизвестна
    /// Endpoint: GET `/reports/delivery/couriers`
    ///
    /// # Параметры
    /// - `department`: Подразделения (код или ID) в формате `{code="005"}` или `{id="guid"}` (можно указать несколько)
    /// - `date_from`: Дата начала отчета в формате DD.MM.YYYY или YYYY-MM-DD
    /// - `date_to`: Дата окончания отчета в формате DD.MM.YYYY или YYYY-MM-DD
    /// - `target_common_time`: Целевое значение общего времени в минутах (по умолчанию 30)
    /// - `target_on_the_way_time`: Целевое значение времени в пути в минутах (по умолчанию 0)
    /// - `target_doubled_orders`: Целевое количество сдвоенных заказов за день (по умолчанию 0)
    /// - `target_tripled_orders`: Целевое количество строенных заказов за день (по умолчанию 0)
    /// - `target_total_orders`: Целевое количество заказов за день (по умолчанию 0)
    ///
    /// # Что в ответе
    /// Возвращает отчет по курьерам с метриками производительности.
    pub async fn get_delivery_couriers(
        &self,
        date_from: &str,
        date_to: &str,
        departments: Option<&[&str]>,
        target_common_time: Option<i32>,
        target_on_the_way_time: Option<i32>,
        target_doubled_orders: Option<i32>,
        target_tripled_orders: Option<i32>,
        target_total_orders: Option<i32>,
    ) -> Result<DeliveryCouriersReport> {
        let mut params = vec![("dateFrom", date_from), ("dateTo", date_to)];

        if let Some(depts) = departments {
            for dept in depts {
                params.push(("department", dept));
            }
        }

        let target_common_time_str;
        let target_on_the_way_time_str;
        let target_doubled_orders_str;
        let target_tripled_orders_str;
        let target_total_orders_str;

        if let Some(time) = target_common_time {
            target_common_time_str = time.to_string();
            params.push(("targetCommonTime", &target_common_time_str));
        }

        if let Some(time) = target_on_the_way_time {
            target_on_the_way_time_str = time.to_string();
            params.push(("targetOnTheWayTime", &target_on_the_way_time_str));
        }

        if let Some(orders) = target_doubled_orders {
            target_doubled_orders_str = orders.to_string();
            params.push(("targetDoubledOrders", &target_doubled_orders_str));
        }

        if let Some(orders) = target_tripled_orders {
            target_tripled_orders_str = orders.to_string();
            params.push(("targetTripledOrders", &target_tripled_orders_str));
        }

        if let Some(orders) = target_total_orders {
            target_total_orders_str = orders.to_string();
            params.push(("targetTotalOrders", &target_total_orders_str));
        }

        let response_xml = self
            .client
            .get_with_params("reports/delivery/couriers", &params)
            .await?;

        let report: DeliveryCouriersReport = from_str(&response_xml)?;
        Ok(report)
    }

    /// Отчет по циклу заказа
    ///
    /// Версия iiko: неизвестна
    /// Endpoint: GET `/reports/delivery/orderCycle`
    ///
    /// # Параметры
    /// - `department`: Подразделения (код или ID) в формате `{code="005"}` или `{id="guid"}` (можно указать несколько)
    /// - `date_from`: Дата начала отчета в формате DD.MM.YYYY или YYYY-MM-DD
    /// - `date_to`: Дата окончания отчета в формате DD.MM.YYYY или YYYY-MM-DD
    /// - `target_pizza_time`: Целевое значение времени на столе Пицца в минутах (по умолчанию 0)
    /// - `target_cutting_time`: Целевое значение времени на столе нарезки в минутах (по умолчанию 0)
    /// - `target_on_shelf_time`: Целевое значение времени на стеллаже оперативности в минутах (по умолчанию 0)
    /// - `target_in_restaurant_time`: Целевое значение времени в ресторане в минутах (по умолчанию 0)
    /// - `target_on_the_way_time`: Целевое значение времени в пути в минутах (по умолчанию 0)
    /// - `target_total_time`: Целевое значение общего времени доставки в минутах (по умолчанию 0)
    ///
    /// # Что в ответе
    /// Возвращает отчет по циклу заказа с временными метриками.
    pub async fn get_delivery_order_cycle(
        &self,
        date_from: &str,
        date_to: &str,
        departments: Option<&[&str]>,
        target_pizza_time: Option<i32>,
        target_cutting_time: Option<i32>,
        target_on_shelf_time: Option<i32>,
        target_in_restaurant_time: Option<i32>,
        target_on_the_way_time: Option<i32>,
        target_total_time: Option<i32>,
    ) -> Result<DeliveryOrderCycleReport> {
        let mut params = vec![("dateFrom", date_from), ("dateTo", date_to)];

        if let Some(depts) = departments {
            for dept in depts {
                params.push(("department", dept));
            }
        }

        let target_pizza_time_str;
        let target_cutting_time_str;
        let target_on_shelf_time_str;
        let target_in_restaurant_time_str;
        let target_on_the_way_time_str;
        let target_total_time_str;

        if let Some(time) = target_pizza_time {
            target_pizza_time_str = time.to_string();
            params.push(("targetPizzaTime", &target_pizza_time_str));
        }

        if let Some(time) = target_cutting_time {
            target_cutting_time_str = time.to_string();
            params.push(("targetCuttingTime", &target_cutting_time_str));
        }

        if let Some(time) = target_on_shelf_time {
            target_on_shelf_time_str = time.to_string();
            params.push(("targetOnShelfTime", &target_on_shelf_time_str));
        }

        if let Some(time) = target_in_restaurant_time {
            target_in_restaurant_time_str = time.to_string();
            params.push(("targetInRestaurantTime", &target_in_restaurant_time_str));
        }

        if let Some(time) = target_on_the_way_time {
            target_on_the_way_time_str = time.to_string();
            params.push(("targetOnTheWayTime", &target_on_the_way_time_str));
        }

        if let Some(time) = target_total_time {
            target_total_time_str = time.to_string();
            params.push(("targetTotalTime", &target_total_time_str));
        }

        let response_xml = self
            .client
            .get_with_params("reports/delivery/orderCycle", &params)
            .await?;

        let report: DeliveryOrderCycleReport = from_str(&response_xml)?;
        Ok(report)
    }

    /// Получасовой детальный отчет по доставке
    ///
    /// Версия iiko: неизвестна
    /// Endpoint: GET `/reports/delivery/halfHourDetailed`
    ///
    /// # Параметры
    /// - `department`: Подразделения (код или ID) в формате `{code="005"}` или `{id="guid"}` (можно указать несколько)
    /// - `date_from`: Дата начала отчета в формате DD.MM.YYYY или YYYY-MM-DD
    /// - `date_to`: Дата окончания отчета в формате DD.MM.YYYY или YYYY-MM-DD
    ///
    /// # Что в ответе
    /// Возвращает получасовой детальный отчет по доставке с разбивкой по типам доставки.
    pub async fn get_delivery_half_hour_detailed(
        &self,
        date_from: &str,
        date_to: &str,
        departments: Option<&[&str]>,
    ) -> Result<DeliveryHalfHourDetailedReport> {
        let mut params = vec![("dateFrom", date_from), ("dateTo", date_to)];

        if let Some(depts) = departments {
            for dept in depts {
                params.push(("department", dept));
            }
        }

        let response_xml = self
            .client
            .get_with_params("reports/delivery/halfHourDetailed", &params)
            .await?;

        let report: DeliveryHalfHourDetailedReport = from_str(&response_xml)?;
        Ok(report)
    }

    /// Отчет по регионам доставки
    ///
    /// Версия iiko: неизвестна
    /// Endpoint: GET `/reports/delivery/regions`
    ///
    /// # Параметры
    /// - `department`: Подразделения (код или ID) в формате `{code="005"}` или `{id="guid"}` (можно указать несколько)
    /// - `date_from`: Дата начала отчета в формате DD.MM.YYYY или YYYY-MM-DD
    /// - `date_to`: Дата окончания отчета в формате DD.MM.YYYY или YYYY-MM-DD
    ///
    /// # Что в ответе
    /// Возвращает отчет по регионам доставки с метриками по каждому региону.
    pub async fn get_delivery_regions(
        &self,
        date_from: &str,
        date_to: &str,
        departments: Option<&[&str]>,
    ) -> Result<DeliveryRegionsReport> {
        let mut params = vec![("dateFrom", date_from), ("dateTo", date_to)];

        if let Some(depts) = departments {
            for dept in depts {
                params.push(("department", dept));
            }
        }

        let response_xml = self
            .client
            .get_with_params("reports/delivery/regions", &params)
            .await?;

        let report: DeliveryRegionsReport = from_str(&response_xml)?;
        Ok(report)
    }

    /// Отчет по лояльности доставки
    ///
    /// Версия iiko: неизвестна
    /// Endpoint: GET `/reports/delivery/loyalty`
    ///
    /// # Параметры
    /// - `department`: Подразделения (код или ID) в формате `{code="005"}` или `{id="guid"}` (можно указать несколько)
    /// - `date_from`: Дата начала отчета в формате DD.MM.YYYY или YYYY-MM-DD
    /// - `date_to`: Дата окончания отчета в формате DD.MM.YYYY или YYYY-MM-DD
    /// - `metric_type`: Тип метрики (AVERAGE, MINIMUM, MAXIMUM)
    ///
    /// # Что в ответе
    /// Возвращает отчет по лояльности доставки с метриками по датам и регионам.
    pub async fn get_delivery_loyalty(
        &self,
        date_from: &str,
        date_to: &str,
        departments: Option<&[&str]>,
        metric_type: Option<&str>,
    ) -> Result<DeliveryLoyaltyReport> {
        let mut params = vec![("dateFrom", date_from), ("dateTo", date_to)];

        if let Some(depts) = departments {
            for dept in depts {
                params.push(("department", dept));
            }
        }

        if let Some(mt) = metric_type {
            params.push(("metricType", mt));
        }

        let response_xml = self
            .client
            .get_with_params("reports/delivery/loyalty", &params)
            .await?;

        let report: DeliveryLoyaltyReport = from_str(&response_xml)?;
        Ok(report)
    }

    // ============================================================================
    // Отчеты по складским операциям и другие отчеты
    // ============================================================================

    /// Отчет по складским операциям
    ///
    /// Версия iiko: 3.9
    /// Endpoint: GET `/reports/storeOperations`
    ///
    /// # Параметры
    /// - `date_from`: Начальная дата в формате DD.MM.YYYY
    /// - `date_to`: Конечная дата в формате DD.MM.YYYY
    /// - `stores`: Список складов (GUID) (можно указать несколько)
    /// - `document_types`: Типы документов (можно указать несколько)
    /// - `product_detalization`: Если true, отчет включает информацию по товарам, но не включает дату. Если false - отчет включает каждый документ одной строкой
    /// - `show_cost_corrections`: Включать ли коррекции себестоимости
    /// - `preset_id`: ID преднастроенного отчета (GUID). Если указан, все настройки, кроме дат, игнорируются
    ///
    /// # Что в ответе
    /// Возвращает список элементов отчета по складским операциям.
    pub async fn get_store_operations(
        &self,
        date_from: &str,
        date_to: &str,
        stores: Option<&[&str]>,
        document_types: Option<&[&str]>,
        product_detalization: Option<bool>,
        show_cost_corrections: Option<bool>,
        preset_id: Option<&str>,
    ) -> Result<Vec<StoreReportItemDto>> {
        let mut params = vec![("dateFrom", date_from), ("dateTo", date_to)];

        if let Some(strs) = stores {
            for store in strs {
                params.push(("stores", store));
            }
        }

        if let Some(doc_types) = document_types {
            for doc_type in doc_types {
                params.push(("documentTypes", doc_type));
            }
        }

        if let Some(det) = product_detalization {
            params.push(("productDetalization", if det { "true" } else { "false" }));
        }

        if let Some(corr) = show_cost_corrections {
            params.push(("showCostCorrections", if corr { "true" } else { "false" }));
        }

        if let Some(preset) = preset_id {
            params.push(("presetId", preset));
        }

        let response_xml = self
            .client
            .get_with_params("reports/storeOperations", &params)
            .await?;

        // XML может быть списком элементов или одним элементом
        let items: Vec<StoreReportItemDto> = match from_str::<Vec<StoreReportItemDto>>(&response_xml) {
            Ok(list) => list,
            Err(_) => {
                // Пробуем как один элемент
                let item: StoreReportItemDto = from_str(&response_xml)?;
                vec![item]
            }
        };
        Ok(items)
    }

    /// Пресеты отчетов по складским операциям
    ///
    /// Версия iiko: 3.9
    /// Endpoint: GET `/reports/storeReportPresets`
    ///
    /// # Что в ответе
    /// Возвращает список пресетов отчетов по складским операциям.
    pub async fn get_store_report_presets(&self) -> Result<Vec<StoreReportPreset>> {
        let response_xml = self
            .client
            .get("reports/storeReportPresets")
            .await?;

        // XML может быть списком элементов или одним элементом
        let presets: Vec<StoreReportPreset> = match from_str::<Vec<StoreReportPreset>>(&response_xml) {
            Ok(list) => list,
            Err(_) => {
                // Пробуем как один элемент
                let preset: StoreReportPreset = from_str(&response_xml)?;
                vec![preset]
            }
        };
        Ok(presets)
    }

    /// Расход продуктов по продажам
    ///
    /// Версия iiko: 3.9
    /// Endpoint: GET `/reports/productExpense`
    ///
    /// # Параметры запроса
    /// - `department`: Подразделение (GUID)
    /// - `date_from`: Начальная дата в формате DD.MM.YYYY
    /// - `date_to`: Конечная дата в формате DD.MM.YYYY
    /// - `hour_from`: Час начала интервала выборки в сутках (по умолчанию -1, все время)
    /// - `hour_to`: Час окончания интервала выборки в сутках (по умолчанию -1, все время)
    ///
    /// # Что в ответе
    /// Структура dayDishValue (см. XSD Расход продуктов по продажам)
    /// - `date`: Дата
    /// - `productId`: ID продукта
    /// - `productName`: Название продукта
    /// - `value`: Значение (количество) в формате decimal
    pub async fn get_product_expense(
        &self,
        department: &str,
        date_from: &str,
        date_to: &str,
        hour_from: Option<i32>,
        hour_to: Option<i32>,
    ) -> Result<Vec<DayDishValue>> {
        let mut params = vec![
            ("department", department),
            ("dateFrom", date_from),
            ("dateTo", date_to),
        ];

        let hour_from_str;
        let hour_to_str;

        if let Some(hf) = hour_from {
            hour_from_str = hf.to_string();
            params.push(("hourFrom", &hour_from_str));
        }

        if let Some(ht) = hour_to {
            hour_to_str = ht.to_string();
            params.push(("hourTo", &hour_to_str));
        }

        let response_xml = self
            .client
            .get_with_params("reports/productExpense", &params)
            .await?;

        // XML может быть списком элементов или одним элементом
        let items: Vec<DayDishValue> = match from_str::<Vec<DayDishValue>>(&response_xml) {
            Ok(list) => list,
            Err(_) => {
                // Пробуем как один элемент
                let item: DayDishValue = from_str(&response_xml)?;
                vec![item]
            }
        };
        Ok(items)
    }

    /// Отчет по выручке
    ///
    /// Версия iiko: 3.9
    /// Endpoint: GET `/reports/sales`
    ///
    /// # Параметры
    /// - `department`: Подразделение (GUID)
    /// - `date_from`: Начальная дата в формате DD.MM.YYYY
    /// - `date_to`: Конечная дата в формате DD.MM.YYYY
    /// - `hour_from`: Час начала интервала выборки в сутках (по умолчанию -1, все время)
    /// - `hour_to`: Час окончания интервала выборки в сутках (по умолчанию -1, все время)
    /// - `dish_details`: Включать ли разбивку по блюдам (по умолчанию false)
    /// - `all_revenue`: Фильтрация по типам оплат (true - все типы, false - только выручка, по умолчанию true)
    ///
    /// # Что в ответе
    /// Возвращает список элементов отчета по выручке.
    pub async fn get_sales(
        &self,
        department: &str,
        date_from: &str,
        date_to: &str,
        hour_from: Option<i32>,
        hour_to: Option<i32>,
        dish_details: Option<bool>,
        all_revenue: Option<bool>,
    ) -> Result<Vec<DayDishValue>> {
        let mut params = vec![
            ("department", department),
            ("dateFrom", date_from),
            ("dateTo", date_to),
        ];

        let hour_from_str;
        let hour_to_str;

        if let Some(hf) = hour_from {
            hour_from_str = hf.to_string();
            params.push(("hourFrom", &hour_from_str));
        }

        if let Some(ht) = hour_to {
            hour_to_str = ht.to_string();
            params.push(("hourTo", &hour_to_str));
        }

        if let Some(dd) = dish_details {
            params.push(("dishDetails", if dd { "true" } else { "false" }));
        }

        if let Some(ar) = all_revenue {
            params.push(("allRevenue", if ar { "true" } else { "false" }));
        }

        let response_xml = self
            .client
            .get_with_params("reports/sales", &params)
            .await?;

        // XML может быть списком элементов или одним элементом
        let items: Vec<DayDishValue> = match from_str::<Vec<DayDishValue>>(&response_xml) {
            Ok(list) => list,
            Err(_) => {
                // Пробуем как один элемент
                let item: DayDishValue = from_str(&response_xml)?;
                vec![item]
            }
        };
        Ok(items)
    }

    /// План по выручке за день
    ///
    /// Версия iiko: 3.9
    /// Endpoint: GET `/reports/monthlyIncomePlan`
    ///
    /// # Параметры
    /// - `department`: Подразделение (GUID)
    /// - `date_from`: Начальная дата в формате DD.MM.YYYY
    /// - `date_to`: Конечная дата в формате DD.MM.YYYY
    ///
    /// # Что в ответе
    /// Возвращает список элементов плана по выручке за день.
    pub async fn get_monthly_income_plan(
        &self,
        department: &str,
        date_from: &str,
        date_to: &str,
    ) -> Result<Vec<BudgetPlanItemDto>> {
        let params = vec![
            ("department", department),
            ("dateFrom", date_from),
            ("dateTo", date_to),
        ];

        let response_xml = self
            .client
            .get_with_params("reports/monthlyIncomePlan", &params)
            .await?;

        // XML может быть списком элементов или одним элементом
        let items: Vec<BudgetPlanItemDto> = match from_str::<Vec<BudgetPlanItemDto>>(&response_xml) {
            Ok(list) => list,
            Err(_) => {
                // Пробуем как один элемент
                let item: BudgetPlanItemDto = from_str(&response_xml)?;
                vec![item]
            }
        };
        Ok(items)
    }

    /// Отчет о вхождении товара в блюдо
    ///
    /// Версия iiko: 3.9
    /// Endpoint: GET `/reports/ingredientEntry`
    ///
    /// # Параметры
    /// - `department`: Подразделение (GUID)
    /// - `date`: На какую дату в формате DD.MM.YYYY
    /// - `product`: ID продукта (GUID) (приоритет поиска: productArticle, product)
    /// - `product_article`: Артикул продукта (приоритет поиска: productArticle, product)
    /// - `include_subtree`: Включать ли в отчет строки поддеревьев (по умолчанию false)
    ///
    /// # Что в ответе
    /// Возвращает список элементов отчета о вхождении товара в блюдо.
    pub async fn get_ingredient_entry(
        &self,
        department: &str,
        date: &str,
        product: Option<&str>,
        product_article: Option<&str>,
        include_subtree: Option<bool>,
    ) -> Result<Vec<IngredientEntryDto>> {
        let mut params = vec![("department", department), ("date", date)];

        if let Some(prod) = product {
            params.push(("product", prod));
        }

        if let Some(art) = product_article {
            params.push(("productArticle", art));
        }

        if let Some(sub) = include_subtree {
            params.push(("includeSubtree", if sub { "true" } else { "false" }));
        }

        let response_xml = self
            .client
            .get_with_params("reports/ingredientEntry", &params)
            .await?;

        // XML может быть списком элементов или одним элементом
        let items: Vec<IngredientEntryDto> = match from_str::<Vec<IngredientEntryDto>>(&response_xml) {
            Ok(list) => list,
            Err(_) => {
                // Пробуем как один элемент
                let item: IngredientEntryDto = from_str(&response_xml)?;
                vec![item]
            }
        };
        Ok(items)
    }
}

