use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::response::{
    EntityType, ReferenceEntity, ReferenceEntityDto,
};
use serde_json;
use uuid::Uuid;

pub struct EntitiesEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> EntitiesEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Получение справочной информации
    ///
    /// Версия iiko: 5.0
    /// Endpoint: GET `/v2/entities/list`
    ///
    /// # Параметры
    /// - `root_types`: Список типов справочников для получения (можно указать несколько)
    /// - `include_deleted`: Включать ли удаленные элементы справочника (по умолчанию true)
    /// - `revision_from`: Номер ревизии, начиная с которой необходимо отфильтровать сущности (по умолчанию -1)
    ///
    /// # Что в ответе
    /// Возвращает общую справочную информацию без привязки к подразделениям, срокам действия.
    /// Результат вызова может содержать записи (например, типы оплат), запрещенные к применению в каких-то подразделениях.
    /// Данный метод следует использовать только для получения названий объектов в целях отображения отчетов.
    ///
    /// # Примечание
    /// Параметр `format` (SHORT) начиная с версии 6.2.2 не используется, но формат вывода остался прежним.
    pub async fn list(
        &self,
        root_types: &[EntityType],
        include_deleted: Option<bool>,
        revision_from: Option<i64>,
    ) -> Result<Vec<ReferenceEntityDto>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let revision_str;

        for root_type in root_types {
            params.push(("rootType", root_type.as_str()));
        }

        if let Some(inc_del) = include_deleted {
            params.push(("includeDeleted", if inc_del { "true" } else { "false" }));
        }

        if let Some(rev) = revision_from {
            revision_str = rev.to_string();
            params.push(("revisionFrom", revision_str.as_str()));
        }

        let response_json = self
            .client
            .get_with_params("v2/entities/list", &params)
            .await?;

        // Парсим как базовые сущности, так как дополнительные поля зависят от типа
        let entities: Vec<ReferenceEntityDto> = serde_json::from_str(&response_json)?;
        Ok(entities)
    }

    /// Получение справочной информации с дополнительными полями
    ///
    /// Версия iiko: 5.0
    /// Endpoint: GET `/v2/entities/list`
    ///
    /// # Параметры
    /// - `root_types`: Список типов справочников для получения (можно указать несколько)
    /// - `include_deleted`: Включать ли удаленные элементы справочника (по умолчанию true)
    /// - `revision_from`: Номер ревизии, начиная с которой необходимо отфильтровать сущности (по умолчанию -1)
    ///
    /// # Что в ответе
    /// Возвращает справочную информацию с дополнительными полями для типов OrderType, ProductSize, TaxCategory.
    /// Для остальных типов возвращает базовую информацию.
    pub async fn list_with_extended_fields(
        &self,
        root_types: &[EntityType],
        include_deleted: Option<bool>,
        revision_from: Option<i64>,
    ) -> Result<Vec<ReferenceEntity>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let revision_str;

        for root_type in root_types {
            params.push(("rootType", root_type.as_str()));
        }

        if let Some(inc_del) = include_deleted {
            params.push(("includeDeleted", if inc_del { "true" } else { "false" }));
        }

        if let Some(rev) = revision_from {
            revision_str = rev.to_string();
            params.push(("revisionFrom", revision_str.as_str()));
        }

        let response_json = self
            .client
            .get_with_params("v2/entities/list", &params)
            .await?;

        // Парсим как enum для поддержки дополнительных полей
        let entities: Vec<ReferenceEntity> = serde_json::from_str(&response_json)?;
        Ok(entities)
    }

    /// Получение идентификаторов сущностей
    ///
    /// Версия iiko: 9.1
    /// Endpoint: GET `/v2/entities/{entityType}/ids`
    ///
    /// # Параметры
    /// - `entity_type`: Название справочника
    /// - `include_deleted`: Включать ли удаленные элементы справочника (по умолчанию true)
    /// - `revision_from`: Номер ревизии, начиная с которой необходимо отфильтровать сущности (по умолчанию -1)
    ///
    /// # Что в ответе
    /// Возвращает список уникальных идентификаторов (UUID) сущностей указанного типа.
    /// Метод используется для получения идентификаторов сущностей, которые могут быть обработаны в дальнейших операциях.
    ///
    /// # Важно
    /// Этот эндпойнт требует версию iiko 9.1 или выше. На более старых версиях будет возвращаться 404 Not Found.
    pub async fn get_ids(
        &self,
        entity_type: EntityType,
        include_deleted: Option<bool>,
        revision_from: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let revision_str;

        // entityType уже в пути, не добавляем в параметры
        if let Some(inc_del) = include_deleted {
            params.push(("includeDeleted", if inc_del { "true" } else { "false" }));
        }

        if let Some(rev) = revision_from {
            revision_str = rev.to_string();
            params.push(("revisionFrom", revision_str.as_str()));
        }

        let endpoint = format!("v2/entities/{}/ids", entity_type.as_str());
        let response_json = if params.is_empty() {
            self.client.get(&endpoint).await?
        } else {
            self.client.get_with_params(&endpoint, &params).await?
        };

        let ids: Vec<Uuid> = serde_json::from_str(&response_json)?;
        Ok(ids)
    }
}

