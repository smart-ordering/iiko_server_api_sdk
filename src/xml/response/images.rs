use serde::{Deserialize, Serialize};
use uuid::Uuid;

// IdCodeDto переиспользуется из products модуля
use crate::xml::response::products::IdCodeDto;

/// Изображение (ImageDto)
///
/// # Согласно документации iiko API v6.2:
/// - `id`: UUID изображения (обязательное поле)
/// - `data`: Изображение в Base64 (byte[], обязательное поле)
///
/// # Важно:
/// - `data` содержит изображение в формате Base64
/// - Размер изображения не должен превышать максимальный размер, установленный в настройках сервера
/// - Настройка "saved-image-max-size-mb". По умолчанию 512Мб
///
/// # Примечание:
/// - Поля сделаны опциональными для обработки edge cases, когда API может вернуть пустой объект
///   или ответ в нестандартном формате (например, просто Base64 строку)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDto {
    /// UUID изображения
    /// По документации обязательное, но сделано опциональным для обработки edge cases
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Изображение в Base64 (byte[])
    /// По документации обязательное, но сделано опциональным для обработки edge cases
    #[serde(rename = "data", default)]
    pub data: Option<String>,
}

/// Запрос на сохранение изображения (ImageSaveRequest)
///
/// # Согласно документации iiko API v6.2:
/// - `id`: UUID изображения (необязательное, генерируется при создании)
/// - `data`: Изображение в Base64 (byte[], обязательное)
///
/// # Важно:
/// - Размер изображения не должен превышать максимальный размер, установленный в настройках сервера
/// - Настройка "saved-image-max-size-mb". По умолчанию 512Мб
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSaveRequest {
    /// UUID изображения (необязательно, генерируется при создании)
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Изображение в Base64 (byte[], обязательное)
    #[serde(rename = "data")]
    pub data: String,
}

/// Список UUID (IdListDto)
///
/// # Согласно документации iiko API v6.2:
/// Используется для ответа при удалении изображений
///
/// # Структура:
/// - `items`: Список UUID удаленных изображений (List<IdCodeDto>)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdListDto {
    /// Список UUID удаленных изображений
    #[serde(rename = "items")]
    pub items: Vec<IdCodeDto>, // IdCodeDto переиспользуется из products модуля
}

/// Результат операции с изображением (ImageOperationResult)
///
/// # Согласно документации iiko API v6.2:
/// Используется для ответов операций сохранения и удаления изображений
///
/// # Структура:
/// - `result`: Результат операции - "SUCCESS" или "ERROR" (Enum)
/// - `errors`: Список ошибок валидации (List<ErrorDto>, null при успешной операции)
/// - `response`: Результат операции:
///   - При сохранении (`save`): сохраненное изображение (ImageDto)
///   - При удалении (`delete`): список UUID удаленных изображений (IdListDto)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageOperationResult {
    /// Результат операции: "SUCCESS" или "ERROR"
    #[serde(rename = "result")]
    pub result: String,
    /// Список ошибок валидации (null при успешной операции)
    #[serde(rename = "errors", default)]
    pub errors: Option<Vec<crate::xml::response::products::ErrorDto>>,
    /// Результат операции:
    /// - При сохранении: сохраненное изображение (ImageDto)
    /// - При удалении: список UUID удаленных изображений (IdListDto)
    #[serde(rename = "response")]
    pub response: serde_json::Value,
}
