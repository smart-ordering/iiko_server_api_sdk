use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(rename = "id")]
    pub id: Uuid,
    #[serde(rename = "number")]
    pub number: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "type")]
    pub doc_type: String,
}

/// Статус документа
///
/// # Согласно XSD:
/// - NEW - новый документ
/// - PROCESSED - обработанный документ
/// - DELETED - удаленный документ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentStatus {
    /// Новый документ
    New,
    /// Обработанный документ
    Processed,
    /// Удаленный документ
    Deleted,
}

/// Алгоритм распределения дополнительных расходов
///
/// # Согласно XSD (с версии 6.0):
/// - DISTRIBUTION_BY_SUM - распределение по сумме
/// - DISTRIBUTION_BY_AMOUNT - распределение по количеству
/// - DISTRIBUTION_NOT_SPECIFIED - не указано
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DistributionAlgorithmType {
    /// Распределение по сумме
    DistributionBySum,
    /// Распределение по количеству
    DistributionByAmount,
    /// Не указано
    DistributionNotSpecified,
}

/// Приходная накладная (IncomingInvoiceDto)
///
/// # Согласно документации iiko API v3.9+ (редактирование с 5.2):
/// Структура соответствует XSD Приходная накладная
///
/// # Формат даты:
/// - `dateIncoming`: dd.MM.yyyy
/// - `dueDate`: dd.MM.yyyy
/// - `incomingDate`: yyyy-MM-dd (с версии 7.6.1)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "document")]
pub struct IncomingInvoiceDto {
    /// Позиции документа
    #[serde(rename = "items", default)]
    pub items: Option<IncomingInvoiceItems>,
    /// UUID документа (только чтение, с версии 5.4)
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// UUID концепции
    #[serde(rename = "conception", default)]
    pub conception: Option<Uuid>,
    /// Код концепции (с версии 7.8)
    #[serde(rename = "conceptionCode", default)]
    pub conception_code: Option<String>,
    /// Комментарий
    #[serde(rename = "comment", default)]
    pub comment: Option<String>,
    /// Учетный номер документа
    #[serde(rename = "documentNumber", default)]
    pub document_number: Option<String>,
    /// Дата документа (формат: dd.MM.yyyy)
    #[serde(
        rename = "dateIncoming",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_incoming: Option<String>,
    /// Номер счет-фактуры
    #[serde(rename = "invoice", default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    /// Склад по умолчанию (UUID)
    /// Если указан, то в каждой позиции накладной нужно указать этот же склад
    #[serde(
        rename = "defaultStore",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_store: Option<Uuid>,
    /// Поставщик (UUID)
    #[serde(rename = "supplier", default, skip_serializing_if = "Option::is_none")]
    pub supplier: Option<Uuid>,
    /// Срок оплаты (формат: dd.MM.yyyy)
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// Входящая дата внешнего документа в формате yyyy-MM-dd (с версии 7.6.1)
    /// Если при импорте не указана, то берется из dateIncoming
    #[serde(
        rename = "incomingDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub incoming_date: Option<String>,
    /// Использовать настройки проведения документов (с версии 5.2)
    /// false (по умолчанию): использовать переданные дату-время dateIncoming как есть
    /// true: использовать настройки проведения документов, заданные в подразделении
    #[serde(rename = "useDefaultDocumentTime", default)]
    pub use_default_document_time: bool,
    /// Статус документа
    #[serde(rename = "status", default)]
    pub status: Option<DocumentStatus>,
    /// Входящий номер внешнего документа
    #[serde(rename = "incomingDocumentNumber", default)]
    pub incoming_document_number: Option<String>,
    /// Сотрудник (UUID) - поле "зачесть сотруднику" на форме накладной
    #[serde(rename = "employeePassToAccount", default)]
    pub employee_pass_to_account: Option<Uuid>,
    /// Номер товарно-транспортной накладной
    #[serde(rename = "transportInvoiceNumber", default)]
    pub transport_invoice_number: Option<String>,
    /// UUID связанной расходной накладной (только чтение, с версии 5.4)
    #[serde(rename = "linkedOutgoingInvoiceId", default)]
    pub linked_outgoing_invoice_id: Option<Uuid>,
    /// Алгоритм распределения дополнительных расходов (только чтение, с версии 6.0)
    #[serde(rename = "distributionAlgorithm", default)]
    pub distribution_algorithm: Option<DistributionAlgorithmType>,
}

/// Позиции документа (XML wrapper)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingInvoiceItems {
    #[serde(rename = "item", default)]
    pub items: Vec<IncomingInvoiceItemDto>,
}

/// Позиция приходной накладной (IncomingInvoiceItemDto)
///
/// # Согласно XSD:
/// Структура соответствует incomingInvoiceItemDto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingInvoiceItemDto {
    /// Является дополнительным расходом (только чтение, с версии 6.0)
    #[serde(rename = "isAdditionalExpense", default)]
    pub is_additional_expense: bool,
    /// Количество товара в его основных единицах измерения
    #[serde(rename = "amount", default)]
    pub amount: Option<f64>,
    /// Товар у поставщика (UUID)
    #[serde(rename = "supplierProduct", default)]
    pub supplier_product: Option<Uuid>,
    /// Товар у поставщика (артикул). Можно задать вместо guid начиная с 5.0
    #[serde(rename = "supplierProductArticle", default)]
    pub supplier_product_article: Option<String>,
    /// Товар (UUID). Хотя бы одно из полей должно быть заполнено: product или productArticle
    #[serde(rename = "product", default)]
    pub product: Option<Uuid>,
    /// Товар (артикул). Можно задать вместо guid товара начиная с 5.0, guid имеет приоритет
    #[serde(rename = "productArticle", default)]
    pub product_article: Option<String>,
    /// Производитель/импортер (UUID)
    /// Должен содержаться в списке производителей/импортеров в карточке товара
    #[serde(rename = "producer", default)]
    pub producer: Option<Uuid>,
    /// Номер позиции в документе (обязательное поле)
    #[serde(rename = "num")]
    pub num: i32,
    /// Фасовка (UUID)
    #[serde(rename = "containerId", default)]
    pub container_id: Option<Uuid>,
    /// Базовая единица измерения (UUID)
    #[serde(rename = "amountUnit", default)]
    pub amount_unit: Option<Uuid>,
    /// Вес единицы измерения (не реализовано)
    #[serde(rename = "actualUnitWeight", default)]
    pub actual_unit_weight: Option<f64>,
    /// Сумма строки без учета скидки (обязательное поле)
    /// Как правило sum == amount * price / container + discountSum + vatSum
    #[serde(rename = "sum")]
    pub sum: f64,
    /// Сумма скидки (не реализовано)
    #[serde(rename = "discountSum", default)]
    pub discount_sum: Option<f64>,
    /// Величина процента НДС (с версии 5.0)
    /// Если не задана сумма, она вычисляется по проценту
    /// Если не задан процент, он берется из карточки товара
    /// Нельзя задать только сумму, не задавая процент
    #[serde(rename = "vatPercent", default)]
    pub vat_percent: Option<f64>,
    /// Сумма НДС для строки документа (с версии 5.0)
    #[serde(rename = "vatSum", default)]
    pub vat_sum: Option<f64>,
    /// Цена единицы измерения
    #[serde(rename = "priceUnit", default)]
    pub price_unit: Option<String>,
    /// Цена за единицу
    #[serde(rename = "price", default)]
    pub price: Option<f64>,
    /// Цена без НДС за фасовку с учетом скидки (с версии 6.2)
    #[serde(rename = "priceWithoutVat", default)]
    pub price_without_vat: Option<f64>,
    /// Код (не реализовано)
    #[serde(rename = "code", default)]
    pub code: Option<String>,
    /// Склад (UUID)
    #[serde(rename = "store", default)]
    pub store: Option<Uuid>,
    /// Номер государственной таможенной декларации
    #[serde(rename = "customsDeclarationNumber", default)]
    pub customs_declaration_number: Option<String>,
    /// Фактическое (подтвержденное) количество основных единиц товара
    #[serde(rename = "actualAmount", default)]
    pub actual_amount: Option<f64>,
}

/// Результат валидации документа (DocumentValidationResult)
///
/// # Согласно XSD:
/// Структура соответствует documentValidationResult
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "documentValidationResult")]
pub struct DocumentValidationResult {
    /// Результат валидации
    #[serde(rename = "valid")]
    pub valid: bool,
    /// Указывает на то, что ошибка не критичная и служит в качестве предупреждения
    #[serde(rename = "warning", default)]
    pub warning: bool,
    /// Номер валидируемого документа
    #[serde(rename = "documentNumber", default)]
    pub document_number: Option<String>,
    /// Новый номер для документа
    /// Отличен от null, если старый нарушает уникальность или не изменились влияющие на номер поля
    #[serde(rename = "otherSuggestedNumber", default)]
    pub other_suggested_number: Option<String>,
    /// Текст ошибки (или только заголовок, если задано additionalInfo)
    /// Предназначен для показа пользователю, но в REST API не всегда локализован
    #[serde(rename = "errorMessage", default)]
    pub error_message: Option<String>,
    /// Дополнительная информация, содержащая детали ошибки
    /// Например, для случая списания в минус это поле содержит детальную информацию
    /// по каждой позиции документа, приводящей к отрицательным остаткам
    #[serde(rename = "additionalInfo", default)]
    pub additional_info: Option<String>,
}

/// Расходная накладная (OutgoingInvoiceDto)
///
/// # Согласно документации iiko API v4.4:
/// Структура соответствует XSD Расходная накладная
///
/// # Формат даты:
/// - `dateIncoming`: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
///
/// # Важно:
/// - При создании накладных с проведением обязателен склад (defaultStoreId или defaultStoreCode)
/// - Склад заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
/// - Если заполнен в документе, в бекофисе будет отмечена галочка "Отгрузить со склада"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "document")]
pub struct OutgoingInvoiceDto {
    /// UUID документа (только чтение, с версии 5.4)
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Учетный номер документа
    #[serde(rename = "documentNumber", default, skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    /// Учетная дата-время документа
    /// Если не заполнено, используется дата-время сервера
    /// Формат: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
    #[serde(rename = "dateIncoming", default, skip_serializing_if = "Option::is_none")]
    pub date_incoming: Option<String>,
    /// Использовать настройки проведения документов
    /// false (по умолчанию): использовать переданные дату-время dateIncoming как есть
    /// true: использовать настройки проведения документов, заданные в подразделении
    #[serde(rename = "useDefaultDocumentTime", default)]
    pub use_default_document_time: bool,
    /// Статус документа
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DocumentStatus>,
    /// Счет для списания товаров (расходный счет)
    /// По умолчанию "5.01" ("Расход продуктов")
    #[serde(rename = "accountToCode", default, skip_serializing_if = "Option::is_none")]
    pub account_to_code: Option<String>,
    /// Счет выручки
    /// По умолчанию "4.01" ("Торговая выручка")
    #[serde(rename = "revenueAccountCode", default, skip_serializing_if = "Option::is_none")]
    pub revenue_account_code: Option<String>,
    /// Склад (UUID)
    /// При создании накладных с проведением обязателен
    /// Заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
    #[serde(rename = "defaultStoreId", default, skip_serializing_if = "Option::is_none")]
    pub default_store_id: Option<String>,
    /// Склад (код)
    /// При создании накладных с проведением обязателен
    /// Заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
    #[serde(rename = "defaultStoreCode", default, skip_serializing_if = "Option::is_none")]
    pub default_store_code: Option<String>,
    /// Контрагент (UUID)
    #[serde(rename = "counteragentId", default, skip_serializing_if = "Option::is_none")]
    pub counteragent_id: Option<String>,
    /// Контрагент (код)
    #[serde(rename = "counteragentCode", default, skip_serializing_if = "Option::is_none")]
    pub counteragent_code: Option<String>,
    /// Концепция (UUID)
    #[serde(rename = "conceptionId", default, skip_serializing_if = "Option::is_none")]
    pub conception_id: Option<String>,
    /// Концепция (код)
    #[serde(rename = "conceptionCode", default, skip_serializing_if = "Option::is_none")]
    pub conception_code: Option<String>,
    /// Комментарий
    #[serde(rename = "comment", default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// UUID связанной расходной накладной (только чтение, с версии 5.4)
    #[serde(rename = "linkedOutgoingInvoiceId", default, skip_serializing_if = "Option::is_none")]
    pub linked_outgoing_invoice_id: Option<String>,
    /// Позиции документа
    #[serde(rename = "items", default)]
    pub items: Option<OutgoingInvoiceItems>,
}

/// Позиции документа (XML wrapper)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutgoingInvoiceItems {
    #[serde(rename = "item", default)]
    pub items: Vec<OutgoingInvoiceItemDto>,
}

/// Позиция расходной накладной (OutgoingInvoiceItemDto)
///
/// # Согласно XSD:
/// Структура соответствует outgoingInvoiceItemDto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutgoingInvoiceItemDto {
    /// Элемент номенклатуры (UUID)
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Элемент номенклатуры (код/артикул)
    #[serde(rename = "productArticle", default, skip_serializing_if = "Option::is_none")]
    pub product_article: Option<String>,
    /// Склад (UUID)
    /// При создании накладных с проведением обязателен
    /// Заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
    #[serde(rename = "storeId", default, skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// Склад (код)
    /// При создании накладных с проведением обязателен
    /// Заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
    #[serde(rename = "storeCode", default, skip_serializing_if = "Option::is_none")]
    pub store_code: Option<String>,
    /// Фасовка (UUID)
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// Фасовка (код/артикул)
    #[serde(rename = "containerCode", default, skip_serializing_if = "Option::is_none")]
    pub container_code: Option<String>,
    /// Цена за фасовку с учетом скидки (обязательное поле)
    #[serde(rename = "price")]
    pub price: f64,
    /// Цена без НДС за фасовку с учетом скидки (только чтение, с версии 6.2)
    #[serde(rename = "priceWithoutVat", default, skip_serializing_if = "Option::is_none")]
    pub price_without_vat: Option<f64>,
    /// Количество в базовых единицах измерения (обязательное поле)
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Сумма строки без учета скидки (обязательное поле)
    /// Как правило sum == amount * price / container + discountSum + vatSum
    #[serde(rename = "sum")]
    pub sum: f64,
    /// Сумма скидки
    #[serde(rename = "discountSum", default, skip_serializing_if = "Option::is_none")]
    pub discount_sum: Option<f64>,
    /// Величина процента НДС (с версии 5.0)
    /// Если не задана сумма, она вычисляется по проценту
    /// Если не задан процент, он берется из карточки товара
    /// Нельзя задать только сумму, не задавая процент
    #[serde(rename = "vatPercent", default, skip_serializing_if = "Option::is_none")]
    pub vat_percent: Option<f64>,
    /// Сумма НДС для строки документа (с версии 5.0)
    #[serde(rename = "vatSum", default, skip_serializing_if = "Option::is_none")]
    pub vat_sum: Option<f64>,
}

/// Список расходных накладных (OutgoingInvoiceDtoes)
///
/// # Согласно документации iiko API v5.4:
/// Структура для экспорта расходных накладных
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "outgoingInvoiceDtoes")]
pub struct OutgoingInvoiceDtoes {
    #[serde(rename = "document", default)]
    pub documents: Vec<OutgoingInvoiceDto>,
}

/// Возвратная накладная (ReturnedInvoiceDto)
///
/// # Согласно документации iiko API v4.4:
/// Структура соответствует XSD Возвратная накладная
/// Основана на outgoingInvoiceDto с дополнительными обязательными полями
///
/// # Формат даты:
/// - `dateIncoming`: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
/// - `incomingInvoiceDate`: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
///
/// # Важно:
/// - `incomingInvoiceNumber` и `incomingInvoiceDate` - обязательные поля
/// - При создании накладных с проведением обязателен склад (defaultStoreId или defaultStoreCode)
/// - Склад заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "document")]
pub struct ReturnedInvoiceDto {
    /// Учетный номер документа
    #[serde(rename = "documentNumber", default, skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    /// Учетная дата-время документа
    /// Если не заполнено, используется дата-время сервера
    /// Формат: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
    #[serde(rename = "dateIncoming", default, skip_serializing_if = "Option::is_none")]
    pub date_incoming: Option<String>,
    /// Использовать настройки проведения документов (с версии 5.2)
    /// false (по умолчанию): использовать переданные дату-время dateIncoming как есть
    /// true: использовать настройки проведения документов, заданные в подразделении
    #[serde(rename = "useDefaultDocumentTime", default)]
    pub use_default_document_time: bool,
    /// Статус документа
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DocumentStatus>,
    /// Номер приходной накладной, для которой заводится возврат (обязательное поле)
    #[serde(rename = "incomingInvoiceNumber")]
    pub incoming_invoice_number: String,
    /// Учетная дата приходной накладной, для которой заводится возврат (обязательное поле)
    /// Формат: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
    #[serde(rename = "incomingInvoiceDate")]
    pub incoming_invoice_date: String,
    /// Способ учета себестоимости
    /// true: списывать по цене приобретения товара
    /// false: списывать по текущей (средневзвешенной скользящей) себестоимости
    #[serde(rename = "storeCostAffected", default)]
    pub store_cost_affected: bool,
    /// Счет для списания товаров (расходный счет)
    /// По умолчанию "5.01" ("Расход продуктов")
    #[serde(rename = "accountToCode", default, skip_serializing_if = "Option::is_none")]
    pub account_to_code: Option<String>,
    /// Склад (UUID)
    /// При создании накладных с проведением обязателен
    /// Заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
    #[serde(rename = "defaultStoreId", default, skip_serializing_if = "Option::is_none")]
    pub default_store_id: Option<String>,
    /// Склад (код)
    /// При создании накладных с проведением обязателен
    /// Заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
    #[serde(rename = "defaultStoreCode", default, skip_serializing_if = "Option::is_none")]
    pub default_store_code: Option<String>,
    /// Контрагент (UUID)
    #[serde(rename = "counteragentId", default, skip_serializing_if = "Option::is_none")]
    pub counteragent_id: Option<String>,
    /// Контрагент (код)
    #[serde(rename = "counteragentCode", default, skip_serializing_if = "Option::is_none")]
    pub counteragent_code: Option<String>,
    /// Концепция (UUID)
    #[serde(rename = "conceptionId", default, skip_serializing_if = "Option::is_none")]
    pub conception_id: Option<String>,
    /// Концепция (код)
    #[serde(rename = "conceptionCode", default, skip_serializing_if = "Option::is_none")]
    pub conception_code: Option<String>,
    /// Комментарий
    #[serde(rename = "comment", default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Позиции документа
    #[serde(rename = "items", default)]
    pub items: Option<ReturnedInvoiceItems>,
}

/// Позиции документа (XML wrapper)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnedInvoiceItems {
    #[serde(rename = "item", default)]
    pub items: Vec<ReturnedInvoiceItemDto>,
}

/// Позиция возвратной накладной (ReturnedInvoiceItemDto)
///
/// # Согласно XSD:
/// Структура соответствует outgoingInvoiceItemDto с дополнительными полями
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnedInvoiceItemDto {
    /// Элемент номенклатуры (UUID)
    /// Хотя бы одно из полей должно быть заполнено: productId или productArticle
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Элемент номенклатуры (код/артикул)
    /// Хотя бы одно из полей должно быть заполнено: productId или productArticle
    #[serde(rename = "productArticle", default, skip_serializing_if = "Option::is_none")]
    pub product_article: Option<String>,
    /// Товар поставщика (UUID) (с версии 5.0.2)
    #[serde(rename = "supplierProduct", default, skip_serializing_if = "Option::is_none")]
    pub supplier_product: Option<String>,
    /// Товар поставщика (код/артикул) (с версии 5.0.2)
    #[serde(rename = "supplierProductArticle", default, skip_serializing_if = "Option::is_none")]
    pub supplier_product_article: Option<String>,
    /// Номер государственной таможенной декларации (с версии 7.6)
    #[serde(rename = "customsDeclarationNumber", default, skip_serializing_if = "Option::is_none")]
    pub customs_declaration_number: Option<String>,
    /// Склад (UUID)
    /// При создании накладных с проведением обязателен
    /// Заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
    #[serde(rename = "storeId", default, skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// Склад (код)
    /// При создании накладных с проведением обязателен
    /// Заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
    #[serde(rename = "storeCode", default, skip_serializing_if = "Option::is_none")]
    pub store_code: Option<String>,
    /// Фасовка (UUID)
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// Фасовка (код/артикул)
    #[serde(rename = "containerCode", default, skip_serializing_if = "Option::is_none")]
    pub container_code: Option<String>,
    /// Цена с НДС за фасовку без учета скидки (обязательное поле)
    #[serde(rename = "price")]
    pub price: f64,
    /// Количество товара в его основных единицах измерения (обязательное поле)
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Сумма строки без учета скидки (обязательное поле)
    /// Как правило sum == amount * price / container + discountSum + vatSum
    #[serde(rename = "sum")]
    pub sum: f64,
    /// Сумма скидки
    #[serde(rename = "discountSum", default, skip_serializing_if = "Option::is_none")]
    pub discount_sum: Option<f64>,
    /// Величина процента НДС (с версии 5.0)
    /// Если не задана сумма, она вычисляется по проценту
    /// Если не задан процент, он берется из карточки товара
    /// Нельзя задать только сумму, не задавая процент
    #[serde(rename = "vatPercent", default, skip_serializing_if = "Option::is_none")]
    pub vat_percent: Option<f64>,
    /// Сумма НДС для строки документа (с версии 5.0)
    #[serde(rename = "vatSum", default, skip_serializing_if = "Option::is_none")]
    pub vat_sum: Option<f64>,
}

/// Статус строки инвентаризации
///
/// # Согласно XSD:
/// - NEW - строка не сохранена: проводки не создаются
/// - SAVE - строка сохранена: проводки создаются
/// - RECALC - строка удалена (является результатом предыдущего подсчета)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryItemStatus {
    /// Строка не сохранена: проводки не создаются
    New,
    /// Строка сохранена: проводки создаются
    Save,
    /// Строка удалена (является результатом предыдущего подсчета)
    Recalc,
}

/// Инвентаризация (IncomingInventoryDto)
///
/// # Согласно документации iiko API v5.1:
/// Структура соответствует XSD Инвентаризация
///
/// # Формат даты:
/// - `dateIncoming`: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
///
/// # Важно:
/// - Склад (storeId или storeCode) - обязателен для заполнения
/// - Для одного элемента номенклатуры можно передавать несколько строк, но статус у них должен быть одинаковым
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "document")]
pub struct IncomingInventoryDto {
    /// Учетный номер документа
    #[serde(rename = "documentNumber", default, skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    /// Учетная дата-время документа
    /// Если не заполнено, используется дата-время сервера
    /// Формат: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
    #[serde(rename = "dateIncoming", default, skip_serializing_if = "Option::is_none")]
    pub date_incoming: Option<String>,
    /// Использовать настройки проведения документов
    /// false (по умолчанию): использовать переданные дату-время dateIncoming как есть
    /// true: использовать настройки проведения документов, заданные в подразделении
    #[serde(rename = "useDefaultDocumentTime", default)]
    pub use_default_document_time: bool,
    /// Статус документа
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DocumentStatus>,
    /// Счет, на который записываются излишки
    /// По умолчанию "5.10" ("Излишки инвентаризации")
    #[serde(rename = "accountSurplusCode", default, skip_serializing_if = "Option::is_none")]
    pub account_surplus_code: Option<String>,
    /// Счет, на который записывается недостача
    /// По умолчанию "5.09" ("Недостача инвентаризации")
    #[serde(rename = "accountShortageCode", default, skip_serializing_if = "Option::is_none")]
    pub account_shortage_code: Option<String>,
    /// Склад (UUID)
    /// Обязателен для заполнения
    #[serde(rename = "storeId", default, skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// Склад (код)
    /// Обязателен для заполнения
    #[serde(rename = "storeCode", default, skip_serializing_if = "Option::is_none")]
    pub store_code: Option<String>,
    /// Концепция (UUID)
    #[serde(rename = "conceptionId", default, skip_serializing_if = "Option::is_none")]
    pub conception_id: Option<String>,
    /// Концепция (код)
    #[serde(rename = "conceptionCode", default, skip_serializing_if = "Option::is_none")]
    pub conception_code: Option<String>,
    /// Комментарий к документу
    #[serde(rename = "comment", default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Позиции документа
    #[serde(rename = "items", default)]
    pub items: Option<IncomingInventoryItems>,
}

/// Позиции документа (XML wrapper)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingInventoryItems {
    #[serde(rename = "item", default)]
    pub items: Vec<IncomingInventoryItemDto>,
}

/// Позиция инвентаризации (IncomingInventoryItemDto)
///
/// # Согласно XSD:
/// Структура соответствует incomingInventoryItemDto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingInventoryItemDto {
    /// Статус строки
    /// При импорте должен быть одинаков у всех строк с одинаковым элементом номенклатуры
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InventoryItemStatus>,
    /// Порядковый номер пересчета остатков по элементу номенклатуры
    /// Нумерация с нуля. При импорте игнорируется
    #[serde(rename = "recalculationNumber", default, skip_serializing_if = "Option::is_none")]
    pub recalculation_number: Option<i32>,
    /// Элемент номенклатуры (UUID)
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Элемент номенклатуры (код/артикул)
    #[serde(rename = "productArticle", default, skip_serializing_if = "Option::is_none")]
    pub product_article: Option<String>,
    /// Фасовка (UUID)
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// Фасовка (код/артикул)
    #[serde(rename = "containerCode", default, skip_serializing_if = "Option::is_none")]
    pub container_code: Option<String>,
    /// Количество в фасовках (containerId/containerCode)
    #[serde(rename = "amountContainer", default, skip_serializing_if = "Option::is_none")]
    pub amount_container: Option<f64>,
    /// Вес с тарой
    /// Информационное поле, используется только для отображения в бекофисе
    #[serde(rename = "amountGross", default, skip_serializing_if = "Option::is_none")]
    pub amount_gross: Option<f64>,
    /// Производитель или импортер товара (UUID)
    /// Используется в российской алкогольной декларации
    #[serde(rename = "producerId", default, skip_serializing_if = "Option::is_none")]
    pub producer_id: Option<String>,
    /// Произвольный комментарий к строке документа
    #[serde(rename = "comment", default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// Структура Id-Code-Name для различных сущностей
///
/// # Согласно XSD:
/// Используется в результатах валидации инвентаризации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdCodeNameDto {
    /// Внутренний UUID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Код, артикул, табельный номер
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Название объекта
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Результат валидации документа инвентаризации (IncomingInventoryValidationResultDto)
///
/// # Согласно документации iiko API v5.1:
/// Структура соответствует XSD Результат валидации документа инвентаризации
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "incomingInventoryValidationResult")]
pub struct IncomingInventoryValidationResultDto {
    /// Результат валидации
    #[serde(rename = "valid")]
    pub valid: bool,
    /// Указывает на то, что ошибка не критичная и служит в качестве предупреждения
    #[serde(rename = "warning", default)]
    pub warning: bool,
    /// Номер документа
    #[serde(rename = "documentNumber")]
    pub document_number: String,
    /// Предлагаемый номер документа при ошибке валидации номера
    #[serde(rename = "otherSuggestedNumber", default, skip_serializing_if = "Option::is_none")]
    pub other_suggested_number: Option<String>,
    /// Текст ошибки (или только заголовок, если задано additionalInfo)
    /// Предназначен для показа пользователю, но не всегда локализован
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Текст ошибки (подробное описание)
    /// Как правило, отсутствует
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    /// Склад
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<IdCodeNameDto>,
    /// Учетная дата-время инвентаризации
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Строки результата валидации
    #[serde(rename = "items", default)]
    pub items: Option<IncomingInventoryValidationResultItems>,
}

/// Позиции результата валидации (XML wrapper)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingInventoryValidationResultItems {
    #[serde(rename = "item", default)]
    pub items: Vec<IncomingInventoryValidationResultItemDto>,
}

/// Позиция результата валидации инвентаризации (IncomingInventoryValidationResultItemDto)
///
/// # Согласно XSD:
/// Структура соответствует incomingInventoryValidationResultItemDto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingInventoryValidationResultItemDto {
    /// Товар
    #[serde(rename = "product")]
    pub product: IdCodeNameDto,
    /// Расчетное количество товара
    /// Может иметь точность выше, чем допустима в документах (9 вместо 3 знаков после запятой)
    #[serde(rename = "expectedAmount")]
    pub expected_amount: f64,
    /// Расчетная сумма товара
    /// Строго два знака после запятой (в будущих версиях может настраиваться)
    #[serde(rename = "expectedSum")]
    pub expected_sum: f64,
    /// Фактическое количество в базовой единице измерения товара
    /// Сумма всех проведенных (SAVE) строк инвентаризации по данному товару с учетом фасовок
    /// и правил округления строк документов iiko
    #[serde(rename = "actualAmount")]
    pub actual_amount: f64,
    /// Количество излишка (+) или недостатка (-) в базовой единице измерения товара
    /// Заполнено только для проведенной инвентаризации, для непроведенных документов 0
    #[serde(rename = "differenceAmount")]
    pub difference_amount: f64,
    /// Сумма излишка или недостатка проведенной инвентаризации (для непроведенных документов 0)
    /// Знак может не совпадать со знаком количества в случае отрицательной себестоимости
    /// из-за возвратных накладных по цене покупки
    #[serde(rename = "differenceSum")]
    pub difference_sum: f64,
}
