use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Поставщик (Employee/Supplier)
///
/// # Согласно документации iiko API v3.9+:
/// Поставщик представлен как employee с полем `supplier="true"`
/// Структура соответствует XSD Сотрудники (employee)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    /// UUID поставщика
    #[serde(rename = "id")]
    pub id: Uuid,
    /// Табельный номер/Код поставщика
    #[serde(rename = "code")]
    pub code: String,
    /// Имя поставщика
    #[serde(rename = "name")]
    pub name: String,
    /// Логин для входа в бекофис
    #[serde(rename = "login", default)]
    pub login: Option<String>,
    /// Основная должность сотрудника
    #[serde(rename = "mainRoleCode", default)]
    pub main_role_code: Option<String>,
    /// Коды ролей
    #[serde(rename = "roleCodes", default)]
    pub role_codes: Option<Vec<String>>,
    /// Телефон
    #[serde(rename = "phone", default)]
    pub phone: Option<String>,
    /// Мобильный телефон
    #[serde(rename = "cellPhone", default)]
    pub cell_phone: Option<String>,
    /// Имя
    #[serde(rename = "firstName", default)]
    pub first_name: Option<String>,
    /// Отчество
    #[serde(rename = "middleName", default)]
    pub middle_name: Option<String>,
    /// Фамилия
    #[serde(rename = "lastName", default)]
    pub last_name: Option<String>,
    /// Дата рождения
    #[serde(rename = "birthday", default)]
    pub birthday: Option<String>,
    /// Email
    #[serde(rename = "email", default)]
    pub email: Option<String>,
    /// Адрес
    #[serde(rename = "address", default)]
    pub address: Option<String>,
    /// Дата приема на работу
    #[serde(rename = "hireDate", default)]
    pub hire_date: Option<String>,
    /// Номер документа приема
    #[serde(rename = "hireDocumentNumber", default)]
    pub hire_document_number: Option<String>,
    /// Дата увольнения (с версии 5.4)
    #[serde(rename = "fireDate", default)]
    pub fire_date: Option<String>,
    /// Примечание
    #[serde(rename = "note", default)]
    pub note: Option<String>,
    /// Номер карты сотрудника
    #[serde(rename = "cardNumber", default)]
    pub card_number: Option<String>,
    /// ИНН
    #[serde(rename = "taxpayerIdNumber", default)]
    pub taxpayer_id_number: Option<String>,
    /// СНИЛС (с версии 5.4)
    #[serde(rename = "snils", default)]
    pub snils: Option<String>,
    /// Global Location Number для поставщиков (с версии 6.0)
    #[serde(rename = "gln", default)]
    pub gln: Option<String>,
    /// Дата активации
    #[serde(rename = "activationDate", default)]
    pub activation_date: Option<String>,
    /// Дата деактивации
    #[serde(rename = "deactivationDate", default)]
    pub deactivation_date: Option<String>,
    /// Предпочтительное подразделение (с версии 5.0)
    #[serde(rename = "preferredDepartmentCode", default)]
    pub preferred_department_code: Option<String>,
    /// Назначенные подразделения
    /// Если null - сотруднику назначены все подразделения
    #[serde(rename = "departmentCodes", default)]
    pub department_codes: Option<Vec<String>>,
    /// Подразделения, в которых сотрудник является ответственным
    #[serde(rename = "responsibilityDepartmentCodes", default)]
    pub responsibility_department_codes: Option<Vec<String>>,
    /// Удален
    #[serde(rename = "deleted", default)]
    pub deleted: Option<String>,
    /// Признак поставщика
    #[serde(rename = "supplier", default)]
    pub supplier: Option<String>,
    /// Признак сотрудника
    #[serde(rename = "employee", default)]
    pub employee: Option<String>,
    /// Признак клиента
    #[serde(rename = "client", default)]
    pub client: Option<String>,
}

/// Список поставщиков (XML wrapper)
///
/// # Формат XML:
/// ```xml
/// <employees>
///   <employee>...</employee>
///   <employee>...</employee>
/// </employees>
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "employees")]
pub struct Suppliers {
    #[serde(rename = "employee", default)]
    pub items: Vec<Supplier>,
}

/// Элемент прайс-листа поставщика (SupplierPriceListItemDto)
///
/// # Согласно документации iiko API v3.9+:
/// Структура соответствует XSD Прайс-лист
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierPriceListItemDto {
    /// Товар у нас (UUID)
    #[serde(rename = "nativeProduct", default)]
    pub native_product: Option<Uuid>,
    /// Код товара у нас
    #[serde(rename = "nativeProductCode", default)]
    pub native_product_code: Option<String>,
    /// Артикул товара у нас
    #[serde(rename = "nativeProductNum", default)]
    pub native_product_num: Option<String>,
    /// Название товара у нас
    #[serde(rename = "nativeProductName", default)]
    pub native_product_name: Option<String>,
    /// Товар у поставщика (UUID)
    #[serde(rename = "supplierProduct", default)]
    pub supplier_product: Option<Uuid>,
    /// Код товара у поставщика
    #[serde(rename = "supplierProductCode", default)]
    pub supplier_product_code: Option<String>,
    /// Артикул товара у поставщика
    #[serde(rename = "supplierProductNum", default)]
    pub supplier_product_num: Option<String>,
    /// Название товара у поставщика
    #[serde(rename = "supplierProductName", default)]
    pub supplier_product_name: Option<String>,
    /// Стоимость товара
    #[serde(rename = "costPrice", default)]
    pub cost_price: Option<f64>,
    /// Допустимое отклонение от цены (%)
    #[serde(rename = "allowablePriceDeviation", default)]
    pub allowable_price_deviation: Option<f64>,
    /// Фасовка
    #[serde(rename = "container", default)]
    pub container: Option<SupplierContainerDto>,
}

/// Фасовка для прайс-листа поставщика (ContainerDto)
///
/// # Согласно XSD Прайс-лист:
/// Структура соответствует containerDto из XSD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierContainerDto {
    /// UUID фасовки
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Название фасовки
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// Количество базовых единиц измерения товара в фасовке
    #[serde(rename = "count", default)]
    pub count: Option<f64>,
    /// Вес тары (если товар продается на вес)
    #[serde(rename = "containerWeight", default)]
    pub container_weight: Option<f64>,
    /// Вес с тарой (если товар продается на вес)
    #[serde(rename = "fullContainerWeight", default)]
    pub full_container_weight: Option<f64>,
    /// Обратный пересчет (true/false)
    #[serde(rename = "backwardRecalculation", default)]
    pub backward_recalculation: Option<bool>,
    /// Признак удаления (true/false)
    #[serde(rename = "deleted", default)]
    pub deleted: Option<bool>,
    /// Использовать во фронте (true/false)
    #[serde(rename = "useInFront", default)]
    pub use_in_front: Option<bool>,
}

/// Список элементов прайс-листа (XML wrapper)
///
/// # Формат XML:
/// ```xml
/// <supplierPriceList>
///   <supplierPriceListItemDto>...</supplierPriceListItemDto>
///   <supplierPriceListItemDto>...</supplierPriceListItemDto>
/// </supplierPriceList>
/// ```
/// Или может быть просто список элементов без обертки
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "supplierPriceList")]
pub struct SupplierPriceList {
    #[serde(rename = "supplierPriceListItemDto", default)]
    pub items: Vec<SupplierPriceListItemDto>,
}

