use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Тип сущности справочника
///
/// Версия iiko: 5.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EntityType {
    /// Счет (в том числе склады)
    Account,
    /// Бухгалтерская категория номенклатуры
    AccountingCategory,
    /// Класс алкогольной продукции
    AlcoholClass,
    /// Группа аллергенов
    AllergenGroup,
    /// Тип явки сотрудника
    AttendanceType,
    /// Концепция
    Conception,
    /// Тип места приготовления
    CookingPlaceType,
    /// Тип скидки
    DiscountType,
    /// Единица измерения
    MeasureUnit,
    /// Тип заказа
    OrderType,
    /// Тип оплаты
    PaymentType,
    /// Пользовательская категория номенклатуры
    ProductCategory,
    /// Шкала размеров
    ProductScale,
    /// Размер продукта
    ProductSize,
    /// Тип смены
    ScheduleType,
    /// Налоговая категория
    TaxCategory,
}

impl EntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::Account => "Account",
            EntityType::AccountingCategory => "AccountingCategory",
            EntityType::AlcoholClass => "AlcoholClass",
            EntityType::AllergenGroup => "AllergenGroup",
            EntityType::AttendanceType => "AttendanceType",
            EntityType::Conception => "Conception",
            EntityType::CookingPlaceType => "CookingPlaceType",
            EntityType::DiscountType => "DiscountType",
            EntityType::MeasureUnit => "MeasureUnit",
            EntityType::OrderType => "OrderType",
            EntityType::PaymentType => "PaymentType",
            EntityType::ProductCategory => "ProductCategory",
            EntityType::ProductScale => "ProductScale",
            EntityType::ProductSize => "ProductSize",
            EntityType::ScheduleType => "ScheduleType",
            EntityType::TaxCategory => "TaxCategory",
        }
    }
}

/// Базовая информация о сущности справочника
///
/// Версия iiko: 5.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceEntityDto {
    /// UUID объекта
    pub id: Uuid,
    /// "Основной" тип объекта (тот, что передан как аргумент метода list)
    #[serde(rename = "rootType")]
    pub root_type: String,
    /// false — объект действующий, true — объект помечен как удаленный
    pub deleted: bool,
    /// Код объекта (в том числе артикул, табельный номер, и т.п.). Является строкой: "1234", "3.04". Может быть null.
    pub code: Option<String>,
    /// Название объекта.
    /// Для локализуемых предустановленных (например, стандартных счетов Account) — название на языке запроса.
    pub name: String,
}

/// Информация о типе заказа (дополнительные поля для OrderType)
///
/// Версия iiko: 5.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderTypeEntityDto {
    #[serde(flatten)]
    pub base: ReferenceEntityDto,
    /// Режим обслуживания
    #[serde(rename = "orderServiceType", skip_serializing_if = "Option::is_none")]
    pub order_service_type: Option<OrderServiceType>,
    /// У каждого из режимов обслуживания может быть выбран один тип заказа по умолчанию.
    #[serde(rename = "defaultForServiceType", skip_serializing_if = "Option::is_none")]
    pub default_for_service_type: Option<bool>,
}

/// Режим обслуживания для типа заказа
///
/// Версия iiko: 5.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderServiceType {
    /// Обычный заказ
    Common,
    /// Доставка курьером
    DeliveryByCourier,
    /// Доставка самовывоз
    DeliveryPickup,
}

/// Информация о размере продукта (дополнительные поля для ProductSize)
///
/// Версия iiko: 5.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSizeEntityDto {
    #[serde(flatten)]
    pub base: ReferenceEntityDto,
    /// Короткое название для размера
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
}

/// Информация о налоговой категории (дополнительные поля для TaxCategory)
///
/// Версия iiko: 5.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxCategoryEntityDto {
    #[serde(flatten)]
    pub base: ReferenceEntityDto,
    /// Значение ставки НДС
    #[serde(rename = "vatPercent", skip_serializing_if = "Option::is_none")]
    pub vat_percent: Option<f64>,
}

/// Объединенный тип сущности справочника с дополнительными полями
///
/// Версия iiko: 5.0
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReferenceEntity {
    /// Базовая сущность
    Base(ReferenceEntityDto),
    /// Тип заказа с дополнительными полями
    OrderType(OrderTypeEntityDto),
    /// Размер продукта с дополнительными полями
    ProductSize(ProductSizeEntityDto),
    /// Налоговая категория с дополнительными полями
    TaxCategory(TaxCategoryEntityDto),
}

impl ReferenceEntity {
    /// Получить базовую информацию о сущности
    pub fn base(&self) -> &ReferenceEntityDto {
        match self {
            ReferenceEntity::Base(base) => base,
            ReferenceEntity::OrderType(dto) => &dto.base,
            ReferenceEntity::ProductSize(dto) => &dto.base,
            ReferenceEntity::TaxCategory(dto) => &dto.base,
        }
    }
}

