use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Шкала размеров продукта (ProductScaleDto)
///
/// # Согласно документации iiko API:
/// Используется для работы со шкалами размеров продуктов
///
/// # Структура:
/// - `id`: UUID шкалы
/// - `deleted`: Удалена или нет
/// - `name`: Название шкалы
/// - `productSizes`: Список размеров
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductScaleDto {
    /// UUID шкалы
    #[serde(rename = "id")]
    pub id: Uuid,
    /// Удалена или нет
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    /// Название шкалы
    #[serde(rename = "name")]
    pub name: String,
    /// Список размеров
    #[serde(rename = "productSizes")]
    pub product_sizes: Vec<ProductSizeDto>,
}

/// Размер в шкале (ProductSizeDto)
///
/// # Структура:
/// - `id`: UUID размера (может отсутствовать при создании)
/// - `deleted`: Удалён или нет
/// - `name`: Название размера
/// - `shortName`: Короткое название
/// - `priority`: Местоположение размера в списке размеров шкалы
/// - `default`: Является ли размером по умолчанию для данной шкалы
/// - `disabled`: Доступен ли данный размер в текущем продукте (опционально, для продукт-специфичных шкал)
/// - `factors`: Коэффициенты для данного размера (опционально, для продукт-специфичных шкал)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSizeDto {
    /// UUID размера (может отсутствовать при создании)
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Удалён или нет
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    /// Название размера
    #[serde(rename = "name")]
    pub name: String,
    /// Короткое название
    #[serde(rename = "shortName")]
    pub short_name: String,
    /// Местоположение размера в списке размеров шкалы
    #[serde(rename = "priority")]
    pub priority: i32,
    /// Является ли размером по умолчанию для данной шкалы
    /// У шкалы может быть не больше одного размера по умолчанию
    #[serde(rename = "default", default)]
    pub default: bool,
    /// Доступен ли данный размер в текущем продукте
    /// Используется только для продукт-специфичных шкал
    #[serde(rename = "disabled", default)]
    pub disabled: Option<bool>,
    /// Коэффициенты для данного размера
    /// Используется только для продукт-специфичных шкал
    #[serde(rename = "factors", default)]
    pub factors: Option<Vec<ProductSizeFactorDto>>,
}

/// Коэффициент для размера (ProductSizeFactorDto)
///
/// # Структура:
/// - `startNumber`: Количество от
/// - `factor`: Коэффициент
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSizeFactorDto {
    /// Количество от
    #[serde(rename = "startNumber")]
    pub start_number: i32,
    /// Коэффициент
    #[serde(rename = "factor")]
    pub factor: f64,
}

/// Запрос на создание шкалы размеров
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductScaleSaveRequest {
    /// Название шкалы
    #[serde(rename = "name")]
    pub name: String,
    /// Список размеров
    #[serde(rename = "productSizes")]
    pub product_sizes: Vec<ProductSizeSaveDto>,
}

/// Размер для создания/обновления шкалы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSizeSaveDto {
    /// UUID размера (необязательно при создании, обязательно при обновлении)
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Удалён или нет (для обновления)
    #[serde(rename = "deleted", default)]
    pub deleted: Option<bool>,
    /// Название размера
    #[serde(rename = "name")]
    pub name: String,
    /// Короткое название
    #[serde(rename = "shortName")]
    pub short_name: String,
    /// Место в списке
    #[serde(rename = "priority")]
    pub priority: i32,
    /// Является ли размером по умолчанию
    #[serde(rename = "default", default)]
    pub default: bool,
}

/// Запрос на обновление шкалы размеров
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductScaleUpdateRequest {
    /// UUID шкалы
    #[serde(rename = "id")]
    pub id: Uuid,
    /// Название шкалы
    #[serde(rename = "name")]
    pub name: String,
    /// Список размеров
    #[serde(rename = "productSizes")]
    pub product_sizes: Vec<ProductSizeSaveDto>,
}

/// Запрос на задание/редактирование шкалы для продукта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductProductScaleRequest {
    /// UUID шкалы
    #[serde(rename = "id")]
    pub id: Uuid,
    /// Список коэффициентов и доступности для размеров
    #[serde(rename = "productSizes")]
    pub product_sizes: Vec<ProductSizeProductRequest>,
}

/// Размер для задания/редактирования шкалы продукта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSizeProductRequest {
    /// UUID размера
    #[serde(rename = "id")]
    pub id: Uuid,
    /// Доступен ли данный размер в текущем продукте
    #[serde(rename = "disabled", default)]
    pub disabled: bool,
    /// Коэффициенты для данного размера
    #[serde(rename = "factors")]
    pub factors: Vec<ProductSizeFactorDto>,
}

/// Результат операции с шкалой размеров для продукта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductScaleOperationResult {
    /// Результат операции: "SUCCESS" или "ERROR"
    #[serde(rename = "result")]
    pub result: String,
    /// Список ошибок валидации (null при успешной операции)
    #[serde(rename = "errors", default)]
    pub errors: Option<Vec<crate::xml::response::products::ErrorDto>>,
    /// Результат операции:
    /// - Шкала с коэффициентами и доступностью размеров (ProductScaleDto)
    /// - Или null, если продукт не имеет шкалы
    #[serde(rename = "response")]
    pub response: Option<ProductScaleDto>,
}

