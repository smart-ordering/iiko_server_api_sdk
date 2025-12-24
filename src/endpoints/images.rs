use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::response::images::{ImageDto, ImageOperationResult, ImageSaveRequest};
use crate::xml::response::products::{IdCodeDto, ItemsRequest};
use serde_json::to_string as json_to_string;
use uuid::Uuid;

pub struct ImagesEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> ImagesEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Выгрузка изображения (load)
    ///
    /// # Версия iiko: 6.2
    /// # Endpoint: GET `/v2/images/load?imageId={imageId}`
    ///
    /// # Параметры запроса:
    /// - `image_id`: UUID запрашиваемого изображения (обязательный)
    ///
    /// # Что в ответе:
    /// - Изображение в формате JSON с полями:
    ///   - `id`: UUID изображения
    ///   - `data`: Изображение в Base64 (byte[])
    ///
    /// # Важно:
    /// - Изображение возвращается в формате Base64
    /// - Размер изображения может быть большим, учитывайте это при обработке
    ///
    /// # Примечание:
    /// - Метод обрабатывает edge cases, когда API может вернуть пустой объект
    ///   или ответ в нестандартном формате
    pub async fn load(&self, image_id: Uuid) -> Result<ImageDto> {
        let image_id_str = image_id.to_string();
        let params = vec![("imageId", image_id_str.as_str())];
        let response_text = self
            .client
            .get_with_params("v2/images/load", &params)
            .await?;

        let trimmed = response_text.trim();

        // Если ответ пустой, возвращаем пустой ImageDto
        if trimmed.is_empty() {
            return Ok(ImageDto {
                id: Some(image_id),
                data: None,
            });
        }

        // Пытаемся распарсить как JSON
        match serde_json::from_str::<ImageDto>(trimmed) {
            Ok(mut image) => {
                // Если JSON распарсился, но data пустое, возможно это просто Base64 строка
                if image.data.is_none() && !trimmed.starts_with('{') {
                    // Если не начинается с '{', это не JSON объект
                    Ok(ImageDto {
                        id: Some(image_id),
                        data: Some(trimmed.to_string()),
                    })
                } else {
                    // Убеждаемся, что id установлен
                    if image.id.is_none() {
                        image.id = Some(image_id);
                    }
                    Ok(image)
                }
            }
            Err(_) => {
                // Если не JSON, возможно это просто Base64 строка
                // Создаем ImageDto с данными
                Ok(ImageDto {
                    id: Some(image_id),
                    data: Some(trimmed.to_string()),
                })
            }
        }
    }

    /// Импорт изображения (save)
    ///
    /// # Версия iiko: 6.2
    /// # Endpoint: POST `/v2/images/save`
    ///
    /// # Параметры запроса:
    /// - `data`: Изображение в формате Base64 (byte[], обязательный)
    /// - `id`: UUID изображения (необязательный, генерируется при создании)
    ///
    /// # Что в ответе:
    /// - Json структура результата импорта (ImageOperationResult):
    ///   - `result`: "SUCCESS" или "ERROR"
    ///   - `errors`: Список ошибок валидации (null при успешной операции)
    ///   - `response`: Сохраненное изображение (ImageDto)
    ///
    /// # Важно:
    /// - Размер изображения не должен превышать максимальный размер, установленный в настройках сервера
    /// - Настройка "saved-image-max-size-mb". По умолчанию 512Мб
    /// - `data` должен содержать валидную Base64 строку
    ///
    /// # Ошибки:
    /// - При превышении размера: ошибка валидации
    /// - При невалидном Base64: ошибка десериализации
    pub async fn save(&self, data: String, id: Option<Uuid>) -> Result<ImageOperationResult> {
        let request = ImageSaveRequest { id, data };
        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/images/save", &json_body, &[])
            .await?;

        let result: ImageOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Удаление изображений (delete)
    ///
    /// # Версия iiko: 6.2
    /// # Endpoint: POST `/v2/images/delete`
    ///
    /// # Параметры запроса:
    /// - `ids`: Список UUID изображений для удаления (обязательный)
    ///
    /// # Тело запроса:
    /// - `items`: Список UUID изображений (List<IdCodeDto>)
    ///
    /// # Что в ответе:
    /// - Json структура результата удаления (ImageOperationResult):
    ///   - `result`: "SUCCESS" или "ERROR"
    ///   - `errors`: Список ошибок валидации (null при успешной операции)
    ///   - `response`: Список UUID удаленных изображений (IdListDto)
    ///
    /// # Ошибки:
    /// - Если изображение не найдено: ошибка валидации
    pub async fn delete(&self, ids: Vec<Uuid>) -> Result<ImageOperationResult> {
        let items = ids.into_iter().map(|id| IdCodeDto { id }).collect();
        let request = ItemsRequest { items };

        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/images/delete", &json_body, &[])
            .await?;

        let result: ImageOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }
}
