use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Объединенный тип фильтра для OLAP-отчета
///
/// Версия iiko: 4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OlapFilter {
    /// Фильтр по значению
    Value(ValueFilter),
    /// Фильтр по диапазону
    Range(RangeFilter),
    /// Фильтр по дате
    DateRange(DateRangeFilter),
}

/// Баланс по счету, контрагенту и подразделению
///
/// Версия iiko: 5.2
/// Endpoint: GET `/v2/reports/balance/counteragents`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceCounteragent {
    /// ID счета
    pub account: String,
    /// ID контрагента (может быть null)
    #[serde(default)]
    pub counteragent: Option<String>,
    /// ID подразделения
    pub department: String,
    /// Сумма баланса
    pub sum: f64,
}

/// Остаток товара на складе
///
/// Версия iiko: 5.2
/// Endpoint: GET `/v2/reports/balance/stores`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceStore {
    /// ID склада
    pub store: String,
    /// ID элемента номенклатуры
    pub product: String,
    /// Количество товара
    pub amount: f64,
    /// Сумма остатка
    pub sum: f64,
}

/// Состояние акцизной марки
///
/// Версия iiko: 7.4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EgaisMarkStateDto {
    /// Дата-время актуальности состояния
    ///
    /// - MAX_DATE, если марка еще не списана
    /// - Дата-время списания + MAX_MARK_KEEP_DAYS дней, если списана документом, находящимся в нередактируемом статусе
    /// - Дата-время удаления последнего известного EgaisMarkTableItem (для отсутствующих марок)
    #[serde(rename = "dateTo")]
    pub date_to: String,
}

/// Данные по Справке Б (Справке 2)
///
/// Версия iiko: 7.4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EgaisBRegDto {
    /// РАР-идентификатор организации-источника
    #[serde(rename = "sourceRarId")]
    pub source_rar_id: String,
    /// Алкогольный код
    #[serde(rename = "alcCode")]
    pub alc_code: String,
    /// Множество акцизных марок, находящихся на балансе организации
    ///
    /// Ключ - полный текст акцизной марки
    #[serde(rename = "marksOnBalance")]
    pub marks_on_balance: HashMap<String, EgaisMarkStateDto>,
    /// Множество акцизных марок, списанных с баланса организации
    ///
    /// Ключ - полный текст акцизной марки
    #[serde(rename = "marksWrittenOff")]
    pub marks_written_off: HashMap<String, EgaisMarkStateDto>,
}

/// Отчет по балансу на 3 регистре ЕГАИС (акцизные марки)
///
/// Версия iiko: 7.4
/// Endpoint: GET `/v2/reports/egais/marks/list`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EgaisMarksList {
    /// Ревизия, по которую (включительно) выданы данные
    pub revision: i64,
    /// true - пакет является "полным обновлением", клиент должен удалить все имеющие данные, не перечисленные явно
    /// false - пакет является "частичным обновлением", клиент должен заменить закешированные записи с теми же ключами
    #[serde(rename = "fullUpdate")]
    pub full_update: bool,
    /// Данные по Справкам Б (Справкам 2)
    ///
    /// Ключ - BRegId (Идентификатор Справки Б)
    #[serde(rename = "marksByBRegId")]
    pub marks_by_b_reg_id: HashMap<String, EgaisBRegDto>,
}

/// Тип отчета OLAP
///
/// Версия iiko: 4.1
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OlapReportType {
    /// По продажам
    Sales,
    /// По транзакциям
    Transactions,
    /// По доставкам
    Deliveries,
}

/// Тип отчета OLAP для старого API (версия 3.9)
///
/// Версия iiko: 3.9
/// Endpoint: GET `/reports/olap`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OlapReportTypeV1 {
    /// По продажам
    Sales,
    /// По транзакциям
    Transactions,
    /// По доставкам
    Deliveries,
    /// Контроль хранения
    Stock,
}

impl OlapReportTypeV1 {
    /// Преобразовать в строку для использования в API
    pub fn as_str(&self) -> &'static str {
        match self {
            OlapReportTypeV1::Sales => "SALES",
            OlapReportTypeV1::Transactions => "TRANSACTIONS",
            OlapReportTypeV1::Deliveries => "DELIVERIES",
            OlapReportTypeV1::Stock => "STOCK",
        }
    }
}

/// Тип фильтра по значению
///
/// Версия iiko: 4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FilterType {
    /// В фильтрации участвуют только перечисленные значения поля
    IncludeValues,
    /// В фильтрации участвуют значения поля, за исключением перечисленных
    ExcludeValues,
    /// Фильтр по диапазону значений
    Range,
    /// Фильтр по диапазону дат
    DateRange,
}

/// Тип периода для фильтра по дате
///
/// Версия iiko: 4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeriodType {
    /// Вручную
    Custom,
    /// Текущий открытый период
    OpenPeriod,
    /// Сегодня
    Today,
    /// Вчера
    Yesterday,
    /// Текущая неделя
    CurrentWeek,
    /// Текущий месяц
    CurrentMonth,
    /// Текущий год
    CurrentYear,
    /// Прошлая неделя
    LastWeek,
    /// Прошлый месяц
    LastMonth,
    /// Прошлый год
    LastYear,
}

/// Фильтр по значению
///
/// Версия iiko: 4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueFilter {
    #[serde(rename = "filterType")]
    pub filter_type: FilterType,
    pub values: Vec<String>,
}

/// Фильтр по диапазону значений
///
/// Работает для полей с типами: INTEGER, PERCENT, AMOUNT, MONEY
///
/// Версия iiko: 4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeFilter {
    #[serde(rename = "filterType")]
    pub filter_type: FilterType,
    /// Нижняя граница диапазона (числовое значение)
    pub from: f64,
    /// Верхняя граница диапазона (числовое значение)
    pub to: f64,
    #[serde(rename = "includeLow", default = "default_true")]
    pub include_low: bool,
    #[serde(rename = "includeHigh", default = "default_false")]
    pub include_high: bool,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

/// Фильтр по дате
///
/// Версия iiko: 4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeFilter {
    #[serde(rename = "filterType")]
    pub filter_type: FilterType,
    #[serde(rename = "periodType")]
    pub period_type: PeriodType,
    pub from: String,
    pub to: String,
    #[serde(rename = "includeLow", default = "default_true")]
    pub include_low: bool,
    #[serde(rename = "includeHigh", default = "default_false")]
    pub include_high: bool,
}

/// Информация о поле OLAP-отчета
///
/// Версия iiko: 4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlapColumnInfo {
    /// Название колонки отчета в iikoOffice
    pub name: String,
    /// Тип поля
    pub r#type: String,
    /// Если true, то по данной колонке можно агрегировать данные
    #[serde(rename = "aggregationAllowed")]
    pub aggregation_allowed: bool,
    /// Если true, то по данной колонке можно группировать данные
    #[serde(rename = "groupingAllowed")]
    pub grouping_allowed: bool,
    /// Если true, то по данной колонке можно фильтровать данные
    #[serde(rename = "filteringAllowed")]
    pub filtering_allowed: bool,
    /// Список категорий отчета, к которому относится данное поле
    pub tags: Vec<String>,
}

/// Список полей OLAP-отчета
///
/// Версия iiko: 4.1
/// Endpoint: GET `/v2/reports/olap/columns`
pub type OlapColumns = HashMap<String, OlapColumnInfo>;

/// Запрос на получение OLAP-отчета
///
/// Версия iiko: 4.1
/// Endpoint: POST `/v2/reports/olap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlapReportRequest {
    /// Тип отчета
    #[serde(rename = "reportType")]
    pub report_type: OlapReportType,
    /// Считать ли итоговые значения (появился в 5.3.4, с версии 9.1.2 по умолчанию false)
    #[serde(rename = "buildSummary", skip_serializing_if = "Option::is_none")]
    pub build_summary: Option<bool>,
    /// Поля для группировки по строкам
    #[serde(rename = "groupByRowFields")]
    pub group_by_row_fields: Vec<String>,
    /// Поля для группировки по столбцам (необязательный)
    #[serde(rename = "groupByColFields", skip_serializing_if = "Option::is_none")]
    pub group_by_col_fields: Option<Vec<String>>,
    /// Поля для агрегации
    #[serde(rename = "aggregateFields")]
    pub aggregate_fields: Vec<String>,
    /// Фильтры
    ///
    /// Ключ - имя поля для фильтрации (FieldName из OlapColumns)
    /// Значение - фильтр (ValueFilter, RangeFilter или DateRangeFilter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, OlapFilter>>,
}

/// Значение поля в OLAP-отчете
///
/// Может быть строкой, числом (целым или с плавающей точкой), датой или null
///
/// Версия iiko: 4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OlapFieldValue {
    /// Строковое значение (для ENUM, STRING, ID, DATETIME)
    String(String),
    /// Целое число (для INTEGER, DURATION_IN_SECONDS)
    Integer(i64),
    /// Число с плавающей точкой (для PERCENT, AMOUNT, MONEY)
    Float(f64),
    /// Null значение
    Null,
}

impl OlapFieldValue {
    /// Получить значение как строку, если это строка
    pub fn as_string(&self) -> Option<&str> {
        match self {
            OlapFieldValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Получить значение как целое число, если это целое число
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            OlapFieldValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Получить значение как число с плавающей точкой, если это число
    pub fn as_float(&self) -> Option<f64> {
        match self {
            OlapFieldValue::Float(f) => Some(*f),
            OlapFieldValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Проверить, является ли значение null
    pub fn is_null(&self) -> bool {
        matches!(self, OlapFieldValue::Null)
    }
}

/// Данные OLAP-отчета
///
/// Версия iiko: 4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlapReportResponse {
    /// Линейные данные отчета (построчно)
    pub data: Vec<HashMap<String, OlapFieldValue>>,
    /// Промежуточные и общие итоги по отчету
    pub summary: Vec<Vec<HashMap<String, OlapFieldValue>>>,
}

// ============================================================================
// Отчеты по доставке
// ============================================================================

/// Тип метрики для отчетов по доставке
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliveryMetricType {
    /// Среднее значение
    Average,
    /// Минимальное значение
    Minimum,
    /// Максимальное значение
    Maximum,
    /// Отношение к целевым показателям
    Target,
}

/// Тип доставки
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliveryType {
    /// Курьер
    Courier,
    /// Самовывоз
    Pickup,
}

/// Сводный отчет по доставке
///
/// Версия iiko: неизвестна
/// Endpoint: GET `/reports/delivery/consolidated`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "report")]
pub struct DeliveryConsolidatedReport {
    #[serde(rename = "rows")]
    pub rows: DeliveryConsolidatedRows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryConsolidatedRows {
    #[serde(rename = "row", default)]
    pub rows: Vec<DeliveryConsolidatedRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryConsolidatedRow {
    /// Средний чек
    #[serde(rename = "avgReceipt")]
    pub avg_receipt: f64,
    /// Дата
    pub date: String,
    /// Количество блюд
    #[serde(rename = "dishAmount")]
    pub dish_amount: f64,
    /// Количество блюд в чеке
    #[serde(rename = "dishAmountPerOrder")]
    pub dish_amount_per_order: f64,
    /// Количество заказов
    #[serde(rename = "orderCount")]
    pub order_count: f64,
    /// Заказов "курьер"
    #[serde(rename = "orderCountCourier")]
    pub order_count_courier: f64,
    /// Заказов "с собой"
    #[serde(rename = "orderCountPickup")]
    pub order_count_pickup: f64,
    /// % выполнения бюджета
    #[serde(rename = "planExecutionPercent")]
    pub plan_execution_percent: f64,
    /// % списания
    #[serde(rename = "ratioCostWriteoff")]
    pub ratio_cost_writeoff: f64,
    /// Выручка
    pub revenue: f64,
}

/// Отчет по курьерам
///
/// Версия iiko: неизвестна
/// Endpoint: GET `/reports/delivery/couriers`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "report")]
pub struct DeliveryCouriersReport {
    #[serde(rename = "rows")]
    pub rows: DeliveryCouriersRows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryCouriersRows {
    #[serde(rename = "row", default)]
    pub rows: Vec<DeliveryCouriersRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryCouriersRow {
    /// Курьер
    pub courier: String,
    #[serde(rename = "metrics")]
    pub metrics: DeliveryCourierMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryCourierMetrics {
    #[serde(rename = "metric", default)]
    pub metrics: Vec<DeliveryCourierMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryCourierMetric {
    /// Сдвоенные заказы
    #[serde(rename = "doubledOrders")]
    pub doubled_orders: f64,
    /// Тип метрики
    #[serde(rename = "metricType")]
    pub metric_type: DeliveryMetricType,
    /// Время в пути
    #[serde(rename = "onTheWayTime")]
    pub on_the_way_time: f64,
    /// Количество заказов
    #[serde(rename = "orderCount")]
    pub order_count: f64,
    /// Общее время
    #[serde(rename = "totalTime")]
    pub total_time: f64,
    /// Строенные заказы
    #[serde(rename = "tripledOrders")]
    pub tripled_orders: f64,
}

/// Отчет по циклу заказа
///
/// Версия iiko: неизвестна
/// Endpoint: GET `/reports/delivery/orderCycle`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "report")]
pub struct DeliveryOrderCycleReport {
    #[serde(rename = "rows")]
    pub rows: DeliveryOrderCycleRows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryOrderCycleRows {
    #[serde(rename = "row", default)]
    pub rows: Vec<DeliveryOrderCycleRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryOrderCycleRow {
    /// Время на столе нарезки
    #[serde(rename = "cuttingTime")]
    pub cutting_time: f64,
    /// Время в ресторане
    #[serde(rename = "inRestaurantTime")]
    pub in_restaurant_time: f64,
    /// Время на стеллаже оперативности
    #[serde(rename = "onShelfTime")]
    pub on_shelf_time: f64,
    /// Время в пути
    #[serde(rename = "onTheWayTime")]
    pub on_the_way_time: f64,
    /// Время на столе Пицца
    #[serde(rename = "pizzaTime")]
    pub pizza_time: f64,
    /// Общее время
    #[serde(rename = "totalTime")]
    pub total_time: f64,
    /// Тип метрики
    #[serde(rename = "metricType")]
    pub metric_type: DeliveryMetricType,
}

/// Получасовой детальный отчет по доставке
///
/// Версия iiko: неизвестна
/// Endpoint: GET `/reports/delivery/halfHourDetailed`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "report")]
pub struct DeliveryHalfHourDetailedReport {
    #[serde(rename = "rows")]
    pub rows: DeliveryHalfHourDetailedRows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryHalfHourDetailedRows {
    #[serde(rename = "row", default)]
    pub rows: Vec<DeliveryHalfHourDetailedRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryHalfHourDetailedRow {
    /// Время (каждые полчаса)
    #[serde(rename = "halfHourDate")]
    pub half_hour_date: String,
    #[serde(rename = "metrics")]
    pub metrics: DeliveryHalfHourMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryHalfHourMetrics {
    #[serde(rename = "metric", default)]
    pub metrics: Vec<DeliveryHalfHourMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryHalfHourMetric {
    /// Среднее количество блюд на чек
    #[serde(rename = "avgDishAmountPerReceipt")]
    pub avg_dish_amount_per_receipt: f64,
    /// Средний чек
    #[serde(rename = "avgReceipt")]
    pub avg_receipt: f64,
    /// Тип доставки
    #[serde(rename = "deliveryType")]
    pub delivery_type: DeliveryType,
    /// Количество блюд
    #[serde(rename = "dishAmount")]
    pub dish_amount: f64,
    /// Количество заказов
    #[serde(rename = "orderCount")]
    pub order_count: f64,
}

/// Отчет по регионам доставки
///
/// Версия iiko: неизвестна
/// Endpoint: GET `/reports/delivery/regions`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "report")]
pub struct DeliveryRegionsReport {
    #[serde(rename = "rows")]
    pub rows: DeliveryRegionsRows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRegionsRows {
    #[serde(rename = "row", default)]
    pub rows: Vec<DeliveryRegionsRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRegionsRow {
    /// Среднее время доставки
    #[serde(rename = "averageDeliveryTime")]
    pub average_delivery_time: f64,
    /// Процент доставленных заказов
    #[serde(rename = "deliveredOrdersPercent")]
    pub delivered_orders_percent: f64,
    /// Максимальное количество заказов в день
    #[serde(rename = "maxOrderCountDay")]
    pub max_order_count_day: f64,
    /// Общее количество заказов
    #[serde(rename = "orderCount")]
    pub order_count: f64,
    /// Регион
    pub region: String,
}

/// Отчет по лояльности доставки
///
/// Версия iiko: неизвестна
/// Endpoint: GET `/reports/delivery/loyalty`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "report")]
pub struct DeliveryLoyaltyReport {
    #[serde(rename = "rows")]
    pub rows: DeliveryLoyaltyRows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryLoyaltyRows {
    #[serde(rename = "row", default)]
    pub rows: Vec<DeliveryLoyaltyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryLoyaltyRow {
    /// Дата
    pub date: String,
    /// Тип метрики
    #[serde(rename = "metricType")]
    pub metric_type: DeliveryMetricType,
    /// Количество новых гостей
    #[serde(rename = "newGuestCount")]
    pub new_guest_count: f64,
    /// Среднее количество заказов на гостя
    #[serde(rename = "orderCountPerGuest")]
    pub order_count_per_guest: f64,
    #[serde(rename = "regions")]
    pub regions: DeliveryLoyaltyRegions,
    /// Общее количество заказов
    #[serde(rename = "totalOrderCount")]
    pub total_order_count: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryLoyaltyRegions {
    #[serde(rename = "region", default)]
    pub regions: Vec<DeliveryLoyaltyRegion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryLoyaltyRegion {
    /// Количество заказов
    #[serde(rename = "orderCount")]
    pub order_count: f64,
    /// Регион
    pub region: String,
}

// ============================================================================
// Отчеты по складским операциям и другие отчеты
// ============================================================================

/// Тип транзакции для отчетов по складским операциям
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StoreTransactionType {
    OpeningBalance,
    Custom,
    Cash,
    PrepayClosed,
    Prepay,
    PrepayReturn,
    PrepayClosedReturn,
    Discount,
    Card,
    Credit,
    Payin,
    Payout,
    PayCollection,
    CashCorrection,
    InventoryCorrection,
    StoreCostCorrection,
    CashSurplus,
    CashShortage,
    Penalty,
    Bonus,
    Invoice,
    NdsIncoming,
    NdsSales,
    SalesRevenue,
    OutgoingInvoice,
    OutgoingInvoiceRevenue,
    ReturnedInvoice,
    ReturnedInvoiceRevenue,
    Writeoff,
    SessionWriteoff,
    Transfer,
    Transformation,
    TariffHour,
    OnTheHouse,
    Advance,
    IncomingService,
    OutgoingService,
    IncomingServicePayment,
    OutgoingServicePayment,
    CloseAtEmployeeExpense,
    IncentivePayment,
    TariffPercent,
    SessionAcceptance,
    EmployeeCashPayment,
    EmployeePayment,
    InvoicePayment,
    OutgoingDocumentPayment,
    OutgoingSalesDocumentPayment,
    Production,
    SalesReturnPayment,
    SalesReturnWriteoff,
    Disassemble,
    ImportedBankStatement,
}

/// Тип документа для отчетов по складским операциям
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StoreDocumentType {
    IncomingInvoice,
    IncomingInventory,
    IncomingService,
    OutgoingService,
    WriteoffDocument,
    SalesDocument,
    SessionAcceptance,
    InternalTransfer,
    OutgoingInvoice,
    ReturnedInvoice,
    ProductionDocument,
    TransformationDocument,
    ProductionOrder,
    ConsolidatedOrder,
    PreparedRegister,
    MenuChange,
    ProductReplacement,
    SalesReturnDocument,
    DisassembleDocument,
    FuelAcceptance,
    FuelGagingDocument,
    Payroll,
    OutgoingCashOrder,
    IncomingCashOrder,
}

/// Направление данных для отчетов по складским операциям
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StoreDataDirection {
    /// Входящие
    In,
    /// Входящие и исходящие
    Inout,
    /// Исходящие
    Out,
}

/// Детализация по дате
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DateDetalization {
    /// По дням
    Day,
    /// По годам
    Year,
    /// По месяцам
    Month,
    /// По неделям
    Week,
    /// По полумесяцам
    HalfMonth,
    /// Только итоги
    TotalOnly,
    /// По кварталам
    Quarter,
}

/// Тип значения плана по выручке
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BudgetPlanItemValueType {
    /// Абсолютное значение
    Absolute,
    /// Процент
    Percent,
    /// Автоматическое
    Automatic,
}

/// Отчет по складским операциям
///
/// Версия iiko: 3.9
/// Endpoint: GET `/reports/storeOperations`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "item")]
pub struct StoreReportItemDto {
    /// Процент НДС
    #[serde(rename = "ndsPercent", default)]
    pub nds_percent: Option<f64>,
    /// Категория продукта
    #[serde(rename = "productCategory", default)]
    pub product_category: Option<String>,
    /// Группа продукта
    #[serde(rename = "productGroup", default)]
    pub product_group: Option<String>,
    /// Продукт
    #[serde(rename = "product", default)]
    pub product: Option<String>,
    /// Корреспондирующий счет
    #[serde(rename = "secondaryAccount", default)]
    pub secondary_account: Option<String>,
    /// Основной склад
    #[serde(rename = "primaryStore", default)]
    pub primary_store: Option<String>,
    /// Номер документа
    #[serde(rename = "documentNum", default)]
    pub document_num: Option<String>,
    /// Счет расходов
    #[serde(rename = "expenseAccount", default)]
    pub expense_account: Option<String>,
    /// Счет выручки
    #[serde(rename = "revenueAccount", default)]
    pub revenue_account: Option<String>,
    /// Комментарий к документу
    #[serde(rename = "documentComment", default)]
    pub document_comment: Option<String>,
    /// ID документа
    #[serde(rename = "documentId", default)]
    pub document_id: Option<String>,
    /// Тип документа
    #[serde(rename = "documentType", default)]
    pub document_type: Option<StoreDocumentType>,
    /// Приход (true) или расход (false)
    #[serde(rename = "incoming", default)]
    pub incoming: bool,
    /// Тип транзакции
    #[serde(rename = "type", default)]
    pub r#type: Option<StoreTransactionType>,
    /// Дата
    #[serde(rename = "date", default)]
    pub date: Option<String>,
    /// Операционная дата
    #[serde(rename = "operationalDate", default)]
    pub operational_date: Option<String>,
    /// Себестоимость
    #[serde(rename = "cost", default)]
    pub cost: Option<f64>,
    /// Вторая расчетная цена покупки
    #[serde(rename = "secondEstimatedPurchasePrice", default)]
    pub second_estimated_purchase_price: Option<f64>,
    /// Первая расчетная цена покупки
    #[serde(rename = "firstEstimatedPurchasePrice", default)]
    pub first_estimated_purchase_price: Option<f64>,
    /// Сумма документа
    #[serde(rename = "documentSum", default)]
    pub document_sum: Option<f64>,
    /// Количество по корреспондирующему счету
    #[serde(rename = "secondaryAmount", default)]
    pub secondary_amount: Option<f64>,
    /// Количество
    #[serde(rename = "amount", default)]
    pub amount: Option<f64>,
    /// Сумма без НДС
    #[serde(rename = "sumWithoutNds", default)]
    pub sum_without_nds: Option<f64>,
    /// Сумма НДС
    #[serde(rename = "sumNds", default)]
    pub sum_nds: Option<f64>,
    /// Сумма
    #[serde(rename = "sum", default)]
    pub sum: Option<f64>,
}

/// Пресет отчета по складским операциям
///
/// Версия iiko: 3.9
/// Endpoint: GET `/reports/storeReportPresets`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "storeReportPreset")]
pub struct StoreReportPreset {
    /// ID пресета
    #[serde(rename = "id", default)]
    pub id: Option<String>,
    /// Является ли пресет отчетом по умолчанию
    #[serde(rename = "defaultReport", default)]
    pub default_report: bool,
    /// Название пресета
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// Комментарий
    #[serde(rename = "comment", default)]
    pub comment: Option<String>,
    /// Группировка
    #[serde(rename = "grouping", default)]
    pub grouping: Option<StoreOperationsReportGrouping>,
    /// Фильтр
    #[serde(rename = "filter", default)]
    pub filter: Option<StoreReportFilter>,
    /// Заголовки колонок
    #[serde(rename = "columnCaptions", default)]
    pub column_captions: Option<ColumnCaptions>,
}

/// Группировка отчета по складским операциям
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreOperationsReportGrouping {
    /// Детализация по дате
    #[serde(rename = "dateDetalization", default)]
    pub date_detalization: Option<DateDetalization>,
}

/// Фильтр отчета по складским операциям
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreReportFilter {
    /// Основные склады
    #[serde(rename = "primaryStores", default)]
    pub primary_stores: Option<StoreFilterList>,
    /// Корреспондирующие счета
    #[serde(rename = "secondaryAccounts", default)]
    pub secondary_accounts: Option<StoreFilterList>,
    /// Контрагенты
    #[serde(rename = "counteragents", default)]
    pub counteragents: Option<StoreFilterList>,
    /// Продукты
    #[serde(rename = "products", default)]
    pub products: Option<StoreFilterList>,
    /// Корреспондирующие продукты
    #[serde(rename = "secondaryProducts", default)]
    pub secondary_products: Option<StoreFilterList>,
    /// Типы транзакций
    #[serde(rename = "transactionTypes", default)]
    pub transaction_types: Option<TransactionTypeList>,
    /// Типы документов
    #[serde(rename = "documentTypes", default)]
    pub document_types: Option<DocumentTypeList>,
    /// Направление данных
    #[serde(rename = "dataDirection", default)]
    pub data_direction: Option<StoreDataDirection>,
    /// Включать ли нулевые количества и суммы
    #[serde(rename = "includeZeroAmountAndSum", default)]
    pub include_zero_amount_and_sum: bool,
}

/// Список фильтров (общий тип для списков строк)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreFilterList {
    #[serde(rename = "i", default)]
    pub items: Vec<String>,
}

/// Список типов транзакций
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionTypeList {
    #[serde(rename = "i", default)]
    pub items: Vec<StoreTransactionType>,
}

/// Список типов документов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTypeList {
    #[serde(rename = "i", default)]
    pub items: Vec<StoreDocumentType>,
}

/// Заголовки колонок
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnCaptions {
    #[serde(rename = "i", default)]
    pub items: Vec<KeyValue>,
}

/// Ключ-значение
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue {
    /// Ключ
    #[serde(rename = "k", default)]
    pub key: Option<String>,
    /// Значение
    #[serde(rename = "v", default)]
    pub value: Option<String>,
}

/// Расход продуктов по продажам
///
/// Версия iiko: 3.9
/// Endpoint: GET `/reports/productExpense`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "dayDishValue")]
pub struct DayDishValue {
    /// Дата
    #[serde(rename = "date", default)]
    pub date: Option<String>,
    /// ID продукта
    #[serde(rename = "productId", default)]
    pub product_id: Option<String>,
    /// Название продукта
    #[serde(rename = "productName", default)]
    pub product_name: Option<String>,
    /// Значение (количество)
    #[serde(rename = "value", default)]
    pub value: Option<f64>,
}

/// План по выручке за день
///
/// Версия iiko: 3.9
/// Endpoint: GET `/reports/monthlyIncomePlan`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "budgetPlanItemDto")]
pub struct BudgetPlanItemDto {
    /// Дата
    #[serde(rename = "date", default)]
    pub date: Option<String>,
    /// Плановое значение
    #[serde(rename = "planValue", default)]
    pub plan_value: Option<f64>,
    /// Тип значения
    #[serde(rename = "valueType", default)]
    pub value_type: Option<BudgetPlanItemValueType>,
}

/// Отчет о вхождении товара в блюдо
///
/// Версия iiko: 3.9
/// Endpoint: GET `/reports/ingredientEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "item")]
pub struct IngredientEntryDto {
    /// Брутто в основной единице измерения (кг)
    #[serde(rename = "amountInMainUnit", default)]
    pub amount_in_main_unit: Option<f64>,
    /// Брутто в единицах измерения продукта
    #[serde(rename = "amountInMeasureUnit", default)]
    pub amount_in_measure_unit: Option<f64>,
    /// Нетто в основной единице измерения (кг)
    #[serde(rename = "amountMiddleMainUnit", default)]
    pub amount_middle_main_unit: Option<f64>,
    /// Нетто в единицах измерения продукта
    #[serde(rename = "amountMiddleMeasureUnit", default)]
    pub amount_middle_measure_unit: Option<f64>,
    /// Выход в основной единице измерения (кг)
    #[serde(rename = "amountOutMainUnit", default)]
    pub amount_out_main_unit: Option<f64>,
    /// Выход в единицах измерения продукта
    #[serde(rename = "amountOutMeasureUnit", default)]
    pub amount_out_measure_unit: Option<f64>,
    /// Потери при холодной обработке (%)
    #[serde(rename = "coldLoss", default)]
    pub cold_loss: Option<f64>,
    /// Процент себестоимости
    #[serde(rename = "costNorm", default)]
    pub cost_norm: Option<f64>,
    /// Себестоимость блюда
    #[serde(rename = "dishCostNorm", default)]
    pub dish_cost_norm: Option<f64>,
    /// Цена продажи блюда
    #[serde(rename = "dishSalePrice", default)]
    pub dish_sale_price: Option<f64>,
    /// Потери при горячей обработке (%)
    #[serde(rename = "hotLoss", default)]
    pub hot_loss: Option<f64>,
    /// ID строки отчета для представления в виде дерева
    #[serde(rename = "itemId", default)]
    pub item_id: Option<String>,
    /// ID родительской строки отчета для представления в виде дерева
    #[serde(rename = "itemParentId", default)]
    pub item_parent_id: Option<String>,
    /// Наименование продукта
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// Артикул продукта
    #[serde(rename = "num", default)]
    pub num: Option<String>,
    /// ID продукта (guid)
    #[serde(rename = "product", default)]
    pub product: Option<String>,
    /// Стоимость товара в блюде
    #[serde(rename = "productInDishCost", default)]
    pub product_in_dish_cost: Option<f64>,
    /// Себестоимость продукта, по которому строится отчет
    #[serde(rename = "sourceProductCostNorm", default)]
    pub source_product_cost_norm: Option<f64>,
    /// Уровень строки в дереве
    #[serde(rename = "treeLevel", default)]
    pub tree_level: i32,
    /// Наименование единицы измерения
    #[serde(rename = "unit", default)]
    pub unit: Option<String>,
}

