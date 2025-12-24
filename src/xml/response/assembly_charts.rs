use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Метод списания продукта
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductWriteoffStrategy {
    /// Списывать ингредиенты (обычно для полуфабрикатов и блюд, которые не нужно учитывать на складе)
    Assemble,
    /// Списывать сам продукт (обычно для товаров и полуфабрикатов, остаток которых нужно учитывать на складе)
    Direct,
}

/// Способ учета размеров списываемых блюд
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductSizeAssemblyStrategy {
    /// Общая техкарта - содержит строки для продуктов и подразделений
    /// Рассчитанное по техкарте списываемое количество умножается на коэффициент списания
    Common,
    /// Отдельные техкарты для размеров - содержит строки для продуктов, размеров и подразделений
    /// Коэффициенты списания, зафиксированные в документах, не учитываются
    Specific,
}

/// Спецификация подразделений (StoreSpecification)
/// Используется для указания на подмножество подразделений, в которых действует строка техкарты
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreSpecification {
    /// Список ID подразделений
    #[serde(rename = "departments", default)]
    pub departments: Vec<Uuid>,
    /// false — фильтр включающий (строка действует для всех перечисленных подразделений)
    /// true — фильтр исключающий (строка действует для всех подразделений, КРОМЕ перечисленных)
    #[serde(rename = "inverse", default)]
    pub inverse: bool,
}

/// Строка исходной технологической карты (AssemblyChartItemDto)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyChartItemDto {
    /// UUID строки
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Порядок отображения строк в интерфейсе iikoOffice
    #[serde(rename = "sortWeight", default)]
    pub sort_weight: f64,
    /// UUID списываемого ингредиента (товара, заготовки)
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    /// UUID размера блюда. Null для строк "общей" (COMMON) техкарты
    #[serde(rename = "productSizeSpecification", default)]
    pub product_size_specification: Option<Uuid>,
    /// Список подразделений, в списаниях со складов которых применяется данная строка техкарты
    #[serde(rename = "storeSpecification", default)]
    pub store_specification: Option<StoreSpecification>,
    /// Брутто (в основных единицах измерения ингредиента). Именно это поле участвует в расчете списаний
    #[serde(rename = "amountIn", default)]
    pub amount_in: Option<f64>,
    /// Нетто (в основных единицах измерения ингредиента, а НЕ в кг, отображаемых в iikoOffice)
    #[serde(rename = "amountMiddle", default)]
    pub amount_middle: Option<f64>,
    /// Выход готового продукта (в основных единицах измерения ингредиента, а НЕ в кг, отображаемых в iikoOffice)
    #[serde(rename = "amountOut", default)]
    pub amount_out: Option<f64>,
    /// Акт проработки/Проработка 1/Брутто (в кг)
    #[serde(rename = "amountIn1", default)]
    pub amount_in1: Option<f64>,
    /// Акт проработки/Проработка 1/Нетто (в кг)
    #[serde(rename = "amountOut1", default)]
    pub amount_out1: Option<f64>,
    /// Акт проработки/Проработка 2/Брутто (в кг)
    #[serde(rename = "amountIn2", default)]
    pub amount_in2: Option<f64>,
    /// Акт проработки/Проработка 2/Нетто (в кг)
    #[serde(rename = "amountOut2", default)]
    pub amount_out2: Option<f64>,
    /// Акт проработки/Проработка 3/Брутто (в кг)
    #[serde(rename = "amountIn3", default)]
    pub amount_in3: Option<f64>,
    /// Акт проработки/Проработка 3/Нетто (в кг)
    #[serde(rename = "amountOut3", default)]
    pub amount_out3: Option<f64>,
    /// Часть от общей фасовки, которая входит в техкарту
    #[serde(rename = "packageCount", default)]
    pub package_count: Option<f64>,
    /// UUID фасовки ингредиента (product.Container)
    #[serde(rename = "packageTypeId", default)]
    pub package_type_id: Option<Uuid>,
}

/// Исходная технологическая карта (AssemblyChartDto)
///
/// # Важно:
/// - Техкарты строго привязаны к элементам номенклатуры и датам
/// - На каждый учетный день элементу номенклатуры может быть сопоставлено не более одной техкарты
/// - Ингредиентом может быть заготовка, имеющая свою собственную техкарту (образуют деревья)
/// - Срок действия техкарты не может быть искусственно ограничен, последняя техкарта действует бессрочно (dateTo = null)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyChartDto {
    /// UUID технологической карты
    #[serde(rename = "id")]
    pub id: Uuid,
    /// UUID приготавливаемого элемента номенклатуры (блюда, модификатора, заготовки)
    #[serde(rename = "assembledProductId")]
    pub assembled_product_id: Uuid,
    /// Учетный день начала действия технологической карты
    /// Все списания assembledProductId, начиная с 00:00 этого дня, проводятся по данной техкарте
    #[serde(rename = "dateFrom")]
    pub date_from: String,
    /// Учетный день прекращения действия технологической карты
    /// Начиная с 00:00 этого дня списания проводятся по СЛЕДУЮЩЕЙ техкарте
    /// null означает, что техкарта действует бессрочно
    #[serde(rename = "dateTo", default)]
    pub date_to: Option<String>,
    /// Норма закладки приготавливаемого блюда
    #[serde(rename = "assembledAmount", default)]
    pub assembled_amount: Option<f64>,
    /// Метод списания продукта
    #[serde(rename = "productWriteoffStrategy", default)]
    pub product_writeoff_strategy: Option<ProductWriteoffStrategy>,
    /// Список UUID подразделений, где элемент номенклатуры списывается целиком (а не по ингредиентам)
    #[serde(rename = "effectiveDirectWriteoffStoreSpecification", default)]
    pub effective_direct_writeoff_store_specification: Option<StoreSpecification>,
    /// Способ учета размеров списываемых блюд
    #[serde(rename = "productSizeAssemblyStrategy", default)]
    pub product_size_assembly_strategy: Option<ProductSizeAssemblyStrategy>,
    /// Строки техкарты
    #[serde(rename = "items", default)]
    pub items: Vec<AssemblyChartItemDto>,
    /// Комментарий "Технология приготовления"
    #[serde(rename = "technologyDescription", default)]
    pub technology_description: Option<String>,
    /// Комментарий "Описание"
    #[serde(rename = "description", default)]
    pub description: Option<String>,
    /// Комментарий "Требования к оформлению, подаче и реализации"
    #[serde(rename = "appearance", default)]
    pub appearance: Option<String>,
    /// Комментарий "Органолептические показатели качества"
    #[serde(rename = "organoleptic", default)]
    pub organoleptic: Option<String>,
    /// Суммарный выход
    #[serde(rename = "outputComment", default)]
    pub output_comment: Option<String>,
}

/// Строка разложенной технологической карты (PreparedChartItemDto)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedChartItemDto {
    /// UUID строки
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Порядок отображения строк в интерфейсе iikoOffice
    #[serde(rename = "sortWeight", default)]
    pub sort_weight: f64,
    /// UUID списываемого ингредиента (товара, заготовки)
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    /// UUID размера блюда. Null для строк "общей" (COMMON) техкарты
    #[serde(rename = "productSizeSpecification", default)]
    pub product_size_specification: Option<Uuid>,
    /// Список подразделений, в списаниях со складов которых применяется данная строка техкарты
    #[serde(rename = "storeSpecification", default)]
    pub store_specification: Option<StoreSpecification>,
    /// Количество списываемого ингредиента в основных единицах его измерения для нормы закладки = 1
    #[serde(rename = "amount", default)]
    pub amount: Option<f64>,
}

/// Разложенная до конечных ингредиентов технологическая карта (PreparedChartDto)
///
/// # Важно:
/// - В конечной техкарте норма закладки приготавливаемого блюда всегда равна одной основной единице его измерения
/// - Метод списания влияет на результат:
///   - "Списывать готовое блюдо" (DIRECT) - в preparedCharts возвращается элемент списания (например, полуфабрикат)
///   - "Списывать ингредиенты" (ASSEMBLE) - в preparedCharts возвращаются ингредиенты элемента списания
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedChartDto {
    /// UUID технологической карты
    #[serde(rename = "id")]
    pub id: Uuid,
    /// UUID приготавливаемого элемента номенклатуры (блюда, модификатора, заготовки)
    #[serde(rename = "assembledProductId")]
    pub assembled_product_id: Uuid,
    /// Учетный день начала действия технологической карты
    #[serde(rename = "dateFrom")]
    pub date_from: String,
    /// Учетный день прекращения действия технологической карты (null означает бессрочно)
    #[serde(rename = "dateTo", default)]
    pub date_to: Option<String>,
    /// Список UUID подразделений, где элемент номенклатуры списывается целиком
    #[serde(rename = "effectiveDirectWriteoffStoreSpecification", default)]
    pub effective_direct_writeoff_store_specification: Option<StoreSpecification>,
    /// Способ учета размеров списываемых блюд
    #[serde(rename = "productSizeAssemblyStrategy", default)]
    pub product_size_assembly_strategy: Option<ProductSizeAssemblyStrategy>,
    /// Строки разложенной техкарты
    #[serde(rename = "items", default)]
    pub items: Vec<PreparedChartItemDto>,
}

/// Результат запроса технологических карт (ChartResultDto)
///
/// # Важно:
/// - knownRevision используется для получения обновлений через getAllUpdate
/// - assemblyCharts содержит исходные техкарты
/// - preparedCharts содержит разложенные до ингредиентов техкарты
/// - deletedAssemblyChartIds и deletedPreparedChartIds содержат UUID удаленных техкарт (только в getAllUpdate)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartResultDto {
    /// Ревизия сервера, на которую валиден ответ
    /// Используется в getAll и getAllUpdate для получения обновлений
    /// В остальных методах всегда -1
    #[serde(rename = "knownRevision", default)]
    pub known_revision: i64,
    /// Список исходных технологических карт
    #[serde(rename = "assemblyCharts", default)]
    pub assembly_charts: Option<Vec<AssemblyChartDto>>,
    /// Список разложенных до ингредиентов технологических карт
    #[serde(rename = "preparedCharts", default)]
    pub prepared_charts: Option<Vec<PreparedChartDto>>,
    /// Список UUID удаленных исходных техкарт (только в getAllUpdate, null в остальных методах)
    #[serde(rename = "deletedAssemblyChartIds", default)]
    pub deleted_assembly_chart_ids: Option<Vec<Uuid>>,
    /// Список UUID удаленных разложенных техкарт (только в getAllUpdate, null в остальных методах)
    #[serde(rename = "deletedPreparedChartIds", default)]
    pub deleted_prepared_chart_ids: Option<Vec<Uuid>>,
}

/// Результат операции с техкартой (создание/удаление)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyChartOperationResult {
    /// Результат операции: "SUCCESS" или "ERROR"
    #[serde(rename = "result")]
    pub result: String,
    /// Список ошибок валидации (null при успешной операции)
    #[serde(rename = "errors", default)]
    pub errors: Option<Vec<crate::xml::response::products::ErrorDto>>,
    /// Результат операции:
    /// - При создании: созданная техкарта (AssemblyChartDto)
    /// - При удалении: UUID удаленной техкарты (String)
    #[serde(rename = "response")]
    pub response: serde_json::Value,
}
