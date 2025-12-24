use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Тип элемента номенклатуры
///
/// # Важно:
/// Менять тип номенклатуры можно только на тип той же категории, что и исходный.
/// Категории разбиты по цветам.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    /// Товар
    Goods,
    /// Блюдо
    Dish,
    /// Заготовка (полуфабрикат)
    Prepared,
    /// Услуга
    Service,
    /// Модификатор
    Modifier,
    /// Товары поставщиков, не являющиеся товарами систем iiko
    Outer,
    /// Тариф (дочерний элемента для услуги)
    Rate,
}

/// Модификатор продукта (ChoiceBindingDto)
///
/// # Ограничения:
/// - `defaultAmount` должен лежать в пределах `minimumAmount` и `maximumAmount`
/// - У группового модификатора `defaultAmount` равен сумме значений по умолчанию дочерних элементов
/// - `freeOfChargeAmount` не должен превышать `maximumAmount`
/// - Если модификатор обязательный (`required = true`), то `minimumAmount` должен быть больше 0
/// - У дочерних и одиночных модификаторов `childModifiersHaveMinMaxRestrictions` должно быть `false`
/// - Если у группового модификатора `childModifiersHaveMinMaxRestrictions = false`, то:
///   - `freeOfChargeAmount` у дочерних модификаторов должны быть такими же как у группового
///   - `required` должен быть `false`
///   - `minimumAmount = 0` и `maximumAmount = 0`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceBindingDto {
    /// UUID модификатора либо номенклатурной группы, если это групповой модификатор
    #[serde(rename = "modifier")]
    pub modifier: Uuid,
    /// Количество по умолчанию. Должно лежать в пределах мин. и макс. значений.
    /// У группового модификатора равно сумме значений по умолчанию дочерних элементов.
    #[serde(rename = "defaultAmount", default)]
    pub default_amount: i32,
    /// Количество бесплатных модификаторов. Не более макс. количества.
    #[serde(rename = "freeOfChargeAmount", default)]
    pub free_of_charge_amount: i32,
    /// Минимальное количество. Если модификатор обязательный, то минимальное количество должно быть больше 0.
    #[serde(rename = "minimumAmount", default)]
    pub minimum_amount: i32,
    /// Максимальное количество
    #[serde(rename = "maximumAmount", default)]
    pub maximum_amount: i32,
    /// Скрывать, если количество по умолчанию
    #[serde(rename = "hideIfDefaultAmount", default)]
    pub hide_if_default_amount: bool,
    /// Является ли модификатор обязательным
    #[serde(rename = "required", default)]
    pub required: bool,
    /// Ограничения на мин. макс. количество у дочерних модификаторов.
    /// Значение флага в дочерних и одиночных модификаторов должно быть false.
    #[serde(rename = "childModifiersHaveMinMaxRestrictions", default)]
    pub child_modifiers_have_min_max_restrictions: bool,
    /// Признак делимости модификатора. Используется только в схемах модификаторов.
    #[serde(rename = "splittable", default)]
    pub splittable: bool,
    /// Дочерние модификаторы
    #[serde(rename = "childModifiers", default)]
    pub child_modifiers: Option<Vec<ChoiceBindingDto>>,
}

/// Фасовка (ContainerDto)
///
/// # Обязательные поля при импорте:
/// - `name` - Название
/// - `num` - Артикул
/// - `count` - Количество продукта в единицах измерения продукта
/// - `containerWeight` - Вес тары
/// - `fullContainerWeight` - Вес элемента номенклатуры вместе с тарой
///
/// # Примечания:
/// - `backwardRecalculation` - параметр не используется, значение будет проигнорировано
/// - `useInFront` - по умолчанию `true`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerDto {
    /// UUID фасовки (генерируется при создании)
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Артикул (обязателен при импорте)
    #[serde(rename = "num", default)]
    pub num: Option<String>,
    /// Название (обязательно при импорте)
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// Количество продукта в единицах измерения продукта (обязательно при импорте)
    #[serde(rename = "count", default)]
    pub count: Option<f64>,
    /// Минимальный вес элемента номенклатуры (по умолчанию 0)
    #[serde(rename = "minContainerWeight", default)]
    pub min_container_weight: Option<f64>,
    /// Максимальный вес элемента номенклатуры
    #[serde(rename = "maxContainerWeight", default)]
    pub max_container_weight: Option<f64>,
    /// Вес тары (обязателен при импорте)
    #[serde(rename = "containerWeight", default)]
    pub container_weight: Option<f64>,
    /// Вес элемента номенклатуры вместе с тарой (обязателен при импорте)
    #[serde(rename = "fullContainerWeight", default)]
    pub full_container_weight: Option<f64>,
    /// Параметр не используется, значение будет проигнорировано
    #[serde(rename = "backwardRecalculation", default)]
    pub backward_recalculation: bool,
    /// Использовать на фронте (по умолчанию true)
    #[serde(rename = "useInFront", default)]
    pub use_in_front: bool,
    /// Удалена или нет
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
}

/// Штрихкод (BarcodeDto)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeDto {
    #[serde(rename = "barcode")]
    pub barcode: String,
    #[serde(rename = "containerId", default)]
    pub container_id: Option<Uuid>,
}

/// Элемент номенклатуры
///
/// # Обязательные поля при импорте (save):
/// - `name` - Имя
/// - `mainUnit` - UUID основной единицы измерения продукта
/// - `type` - Тип элемента номенклатуры
///
/// # Обязательные поля при обновлении (update):
/// - `id` - UUID редактируемого элемента номенклатуры
///
/// # Условно обязательные поля:
/// - `num` - Обязателен при импорте, если `generateNomenclatureCode == false` (версия 6.2.4+)
/// - `placeType` - Обязателен, если `defaultIncludeInMenu == true`
///
/// # Важные ограничения:
/// - При `defaultIncludeInMenu=true` обязательно указывается `placeType`, независимо от типа элемента
/// - При `defaultIncludeInMenu=false` поле `excludedSections` должно быть `null`
/// - Менять тип номенклатуры можно только на тип той же категории, что и исходный
/// - При обновлении можно передавать только изменяемые поля (необязательно передавать все поля)
/// - `accountingCategory` - по умолчанию "товар" (версия 6.2.3+)
/// - `defaultSalePrice` - по умолчанию 0
/// - `unitWeight` - по умолчанию 1 (версия 6.2.4+)
/// - `unitCapacity` - по умолчанию 0
/// - `notInStoreMovement` - по умолчанию false
/// - `defaultIncludeInMenu` - по умолчанию false
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDto {
    /// UUID элемента:
    /// - Обязателен при обновлении (update)
    /// - Генерируется при создании (save)
    /// - Может быть null при ошибке импорта/обновления
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    #[serde(rename = "description", default)]
    pub description: Option<String>,
    #[serde(rename = "num", default)]
    pub num: Option<String>,
    #[serde(rename = "code", default)]
    pub code: Option<String>,
    #[serde(rename = "parent", default)]
    pub parent: Option<Uuid>,
    /// Модификаторы (версия 6.2+)
    #[serde(rename = "modifiers", default)]
    pub modifiers: Vec<ChoiceBindingDto>,
    /// UUID пользовательской категории
    #[serde(rename = "category", default)]
    pub category: Option<Uuid>,
    /// UUID бухгалтерской категории (по умолчанию "товар", версия 6.2.3+)
    #[serde(rename = "accountingCategory", default)]
    pub accounting_category: Option<Uuid>,
    /// UUID налоговой категории
    #[serde(rename = "taxCategory", default)]
    pub tax_category: Option<Uuid>,
    #[serde(rename = "color", default)]
    pub color: Option<Color>,
    #[serde(rename = "fontColor", default)]
    pub font_color: Option<Color>,
    #[serde(rename = "frontImageId", default)]
    pub front_image_id: Option<Uuid>,
    #[serde(rename = "position", default)]
    pub position: Option<i32>,
    /// UUID основной единицы измерения продукта (обязательно при импорте)
    #[serde(rename = "mainUnit", default)]
    pub main_unit: Option<Uuid>,
    /// Множество UUID отделений ресторана, в которых нельзя продавать данный продукт
    /// (поле имеет смысл только для блюд).
    /// При `defaultIncludeInMenu=false` должно быть `null`
    #[serde(rename = "excludedSections", default)]
    pub excluded_sections: Option<Vec<Uuid>>,
    /// Цена, по которой по умолчанию продаётся продукт (если для него нет приказов о меню).
    /// По умолчанию 0
    #[serde(rename = "defaultSalePrice", default)]
    pub default_sale_price: Option<f64>,
    /// UUID места приготовления блюда (поле имеет смысл только для блюд).
    /// Обязателен, если `defaultIncludeInMenu == true`
    #[serde(rename = "placeType", default)]
    pub place_type: Option<Uuid>,
    /// Включать ли по умолчанию (если нет приказов о меню) позицию в меню.
    /// По умолчанию false.
    /// При `defaultIncludeInMenu=true` обязательно указывается `placeType`
    #[serde(rename = "defaultIncludedInMenu", default)]
    pub default_included_in_menu: Option<bool>,
    /// Тип элемента номенклатуры (обязательно при импорте).
    /// Менять тип номенклатуры можно только на тип той же категории, что и исходный.
    #[serde(rename = "type", default)]
    pub r#type: Option<String>,
    /// Вес одной единицы в килограммах (по умолчанию 1, версия 6.2.4+)
    #[serde(rename = "unitWeight", default)]
    pub unit_weight: Option<f64>,
    /// Объем одной единицы в литрах. По умолчанию 0
    #[serde(rename = "unitCapacity", default)]
    pub unit_capacity: Option<f64>,
    /// Участвует ли в перемещениях по складу. По умолчанию false.
    /// Услуга с такой настройкой, если указана в ПН, на склад никогда не приходуется ни количеством, ни суммой.
    #[serde(rename = "notInStoreMovement", default)]
    pub not_in_store_movement: Option<bool>,
    /// Фасовки (версия 6.2.4+)
    #[serde(rename = "containers", default)]
    pub containers: Option<Vec<ContainerDto>>,
    #[serde(rename = "modifierSchemaId", default)]
    pub modifier_schema_id: Option<Uuid>,
    #[serde(rename = "productScaleId", default)]
    pub product_scale_id: Option<Uuid>,
    /// Потери при холодной обработке (%) (по умолчанию 0, версия 7.1.2+)
    #[serde(rename = "coldLossPercent", default)]
    pub cold_loss_percent: Option<f64>,
    /// Потери при горячей обработке (%) (по умолчанию 0, версия 7.1.2+)
    #[serde(rename = "hotLossPercent", default)]
    pub hot_loss_percent: Option<f64>,
    /// Набор идентификаторов групп аллергенов, которые присутствуют в данном элементе номенклатуры
    /// (версия 7.1.2+)
    #[serde(rename = "allergenGroups", default)]
    pub allergen_groups: Option<Vec<String>>,
    /// Оценочная себестоимость. Поле используется для расчёта себестоимости в случае,
    /// если по данному товару не было приходов (по умолчанию 0, версия 7.1.5+)
    #[serde(rename = "estimatedPurchasePrice", default)]
    pub estimated_purchase_price: Option<f64>,
    /// Свободная цена (версия 7.4.4+)
    #[serde(rename = "canSetOpenPrice", default)]
    pub can_set_open_price: Option<bool>,
    #[serde(rename = "barcodes", default)]
    pub barcodes: Option<Vec<BarcodeDto>>,
}

/// Цвет (RGB)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    #[serde(rename = "red")]
    pub red: u8,
    #[serde(rename = "green")]
    pub green: u8,
    #[serde(rename = "blue")]
    pub blue: u8,
}

/// Группа продуктов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductGroupDto {
    #[serde(rename = "id")]
    pub id: Uuid,
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    #[serde(rename = "description", default)]
    pub description: Option<String>,
    #[serde(rename = "num", default)]
    pub num: Option<String>,
    #[serde(rename = "code", default)]
    pub code: Option<String>,
    #[serde(rename = "parent", default)]
    pub parent: Option<Uuid>,
    #[serde(rename = "color", default)]
    pub color: Option<Color>,
    #[serde(rename = "fontColor", default)]
    pub font_color: Option<Color>,
    #[serde(rename = "frontImageId", default)]
    pub front_image_id: Option<Uuid>,
    #[serde(rename = "position", default)]
    pub position: Option<i32>,
}

/// Результат операции (SUCCESS или ERROR)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OperationResult {
    Success,
    Error,
}

/// Ошибка валидации
///
/// # Примечания:
/// - `value` может содержать UUID объекта (модификатора, продукта) или имя поля (например, "placeType")
/// - `code` содержит код ошибки (например: "NO_RESTRICTION_AND_MIN_MAX_NOT_ZERO",
///   "PARENT_AMOUNT_BY_DEFAULT_NOT_EQUAL_SUM_OF_CHILDREN", "COOKING_PLACE_EMPTY_FOR_SALE_DISH")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDto {
    /// Код ошибки (например: "NO_RESTRICTION_AND_MIN_MAX_NOT_ZERO",
    /// "PARENT_AMOUNT_BY_DEFAULT_NOT_EQUAL_SUM_OF_CHILDREN", "COOKING_PLACE_EMPTY_FOR_SALE_DISH")
    #[serde(rename = "code", default)]
    pub code: Option<String>,
    /// Значение, связанное с ошибкой:
    /// - UUID объекта (модификатора, продукта) при ошибках валидации модификаторов
    /// - Имя поля (например, "placeType") при ошибках валидации полей продукта
    #[serde(rename = "value", default)]
    pub value: Option<String>,
    /// Текст ошибки (может отсутствовать, обычно используется `code` и `value`)
    #[serde(rename = "message", default)]
    pub message: Option<String>,
}

/// Результат импорта/обновления продукта
///
/// # Примечания:
/// - При успешной операции `response` содержит сохраненный/обновленный продукт
/// - При ошибке `response` содержит импортируемый/обновляемый объект (с `id = null` при импорте)
/// - `errors` содержит список ошибок валидации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductOperationResult {
    /// Результат операции: "SUCCESS" или "ERROR"
    #[serde(rename = "result")]
    pub result: String,
    /// Список ошибок валидации (null при успешной операции)
    #[serde(rename = "errors", default)]
    pub errors: Option<Vec<ErrorDto>>,
    /// Продукт:
    /// - При успешном импорте/обновлении: сохраненный объект с `id`
    /// - При ошибке: импортируемый/обновляемый объект (может быть с `id = null` при импорте)
    #[serde(rename = "response")]
    pub response: ProductDto,
}

/// Результат удаления/восстановления продуктов
///
/// # Примечания:
/// - При успешном удалении/восстановлении `response` содержит массив продуктов с обновленным флагом `deleted`
/// - При ошибке `response` может быть `null` или содержать частично обработанные элементы
/// - `errors` содержит список ошибок валидации (например, попытка удалить уже удаленный продукт)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductsOperationResult {
    /// Результат операции: "SUCCESS" или "ERROR"
    #[serde(rename = "result")]
    pub result: String,
    /// Список ошибок валидации (null при успешной операции)
    #[serde(rename = "errors", default)]
    pub errors: Option<Vec<ErrorDto>>,
    /// Массив обработанных продуктов:
    /// - При удалении: продукты с `deleted = true`
    /// - При восстановлении: продукты с `deleted = false`
    /// - Может быть `null` при ошибке
    #[serde(rename = "response", default)]
    pub response: Option<Vec<ProductDto>>,
}

/// ID элемента для операций удаления/восстановления
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdCodeDto {
    /// UUID элемента номенклатуры
    #[serde(rename = "id")]
    pub id: Uuid,
}

/// Запрос на удаление/восстановление
///
/// # Примечания:
/// - Используется для операций `delete` и `restore`
/// - Содержит список UUID элементов для обработки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemsRequest {
    /// Список UUID элементов для удаления/восстановления
    #[serde(rename = "items")]
    pub items: Vec<IdCodeDto>,
}

/// Базовая структура сущности (EntityDto)
/// Используется для пользовательских категорий продуктов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDto {
    /// UUID категории
    #[serde(rename = "id")]
    pub id: Uuid,
    /// Тип корневой сущности (например, "ProductCategory")
    #[serde(rename = "rootType", default)]
    pub root_type: Option<String>,
    /// Удалена ли данная категория
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    /// Код категории
    #[serde(rename = "code", default)]
    pub code: Option<String>,
    /// Имя категории
    #[serde(rename = "name", default)]
    pub name: Option<String>,
}

/// Запрос на создание категории
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySaveRequest {
    /// Имя категории (обязательно)
    #[serde(rename = "name")]
    pub name: String,
}

/// Запрос на обновление категории
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryUpdateRequest {
    /// UUID редактируемой категории (обязательно)
    #[serde(rename = "id")]
    pub id: Uuid,
    /// Новое имя категории (обязательно)
    #[serde(rename = "name")]
    pub name: String,
}

/// Запрос на удаление категории
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDeleteRequest {
    /// UUID удаляемой категории (обязательно)
    #[serde(rename = "id")]
    pub id: Uuid,
}

/// Запрос на восстановление категории
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRestoreRequest {
    /// UUID восстанавливаемой категории (обязательно)
    #[serde(rename = "id")]
    pub id: Uuid,
}

/// Результат операции с категорией (импорт/обновление/удаление/восстановление)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryOperationResult {
    /// Результат операции: "SUCCESS" или "ERROR"
    #[serde(rename = "result")]
    pub result: String,
    /// Список ошибок валидации (null при успешной операции)
    #[serde(rename = "errors", default)]
    pub errors: Option<Vec<ErrorDto>>,
    /// Категория:
    /// - При успешной операции: сохраненная/обновленная категория
    /// - При ошибке: импортируемая/обновляемая категория
    #[serde(rename = "response")]
    pub response: EntityDto,
}
