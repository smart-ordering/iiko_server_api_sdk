use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::response::product_scales::{
    ProductProductScaleRequest, ProductScaleDto, ProductScaleOperationResult,
    ProductScaleSaveRequest, ProductScaleUpdateRequest,
};
use crate::xml::response::products::{IdCodeDto, ItemsRequest};
use serde_json::to_string as json_to_string;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ProductScalesEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> ProductScalesEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Получение шкал размеров (GET)
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: GET `/v2/entities/productScales`
    ///
    /// # Параметры запроса:
    /// - `ids`: Список UUID шкал для фильтрации (необязательный)
    /// - `include_deleted`: Включать ли в ответ удаленные элементы. По умолчанию false
    ///
    /// # Что в ответе:
    /// - Список шкал с размерами (Vec<ProductScaleDto>)
    pub async fn list(
        &self,
        ids: Option<Vec<Uuid>>,
        include_deleted: Option<bool>,
    ) -> Result<Vec<ProductScaleDto>> {
        let mut param_strings = Vec::new();
        let mut params = Vec::new();

        if let Some(include_del) = include_deleted {
            param_strings.push(include_del.to_string());
        }

        if let Some(ref id_list) = ids {
            for id in id_list {
                param_strings.push(id.to_string());
            }
        }

        let mut idx = 0;
        if let Some(include_del) = include_deleted {
            params.push(("includeDeleted", param_strings[idx].as_str()));
            idx += 1;
        }

        if let Some(ref id_list) = ids {
            for _ in id_list {
                params.push(("ids", param_strings[idx].as_str()));
                idx += 1;
            }
        }

        let response_json = if params.is_empty() {
            self.client.get("v2/entities/productScales").await?
        } else {
            self.client
                .get_with_params("v2/entities/productScales", &params)
                .await?
        };

        let scales: Vec<ProductScaleDto> = serde_json::from_str(&response_json)?;
        Ok(scales)
    }

    /// Получение шкал размеров (POST)
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: POST `/v2/entities/productScales`
    ///
    /// # Параметры запроса:
    /// - `ids`: Список UUID шкал для фильтрации (необязательный)
    /// - `include_deleted`: Включать ли в ответ удаленные элементы. По умолчанию false
    ///
    /// # Что в ответе:
    /// - Список шкал с размерами (Vec<ProductScaleDto>)
    pub async fn list_post(
        &self,
        ids: Option<Vec<Uuid>>,
        include_deleted: Option<bool>,
    ) -> Result<Vec<ProductScaleDto>> {
        let mut form_strings = Vec::new();
        let mut form_data = Vec::new();

        if let Some(include_del) = include_deleted {
            form_strings.push(include_del.to_string());
        }

        if let Some(ref id_list) = ids {
            for id in id_list {
                form_strings.push(id.to_string());
            }
        }

        let mut idx = 0;
        if let Some(_) = include_deleted {
            form_data.push(("includeDeleted", form_strings[idx].as_str()));
            idx += 1;
        }

        if let Some(ref id_list) = ids {
            for _ in id_list {
                form_data.push(("ids", form_strings[idx].as_str()));
                idx += 1;
            }
        }

        let response_json = self
            .client
            .post_form("v2/entities/productScales", &form_data)
            .await?;

        let scales: Vec<ProductScaleDto> = serde_json::from_str(&response_json)?;
        Ok(scales)
    }

    /// Получение шкалы размеров по id
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: GET `/v2/entities/productScales/{productScaleId}`
    ///
    /// # Параметры запроса:
    /// - `product_scale_id`: UUID шкалы (обязательный)
    ///
    /// # Что в ответе:
    /// - Шкала с размерами (ProductScaleDto)
    pub async fn by_id(&self, product_scale_id: Uuid) -> Result<ProductScaleDto> {
        let scale_id_str = product_scale_id.to_string();
        let endpoint = format!("v2/entities/productScales/{}", scale_id_str);
        let response_json = self.client.get(&endpoint).await?;

        let scale: ProductScaleDto = serde_json::from_str(&response_json)?;
        Ok(scale)
    }

    /// Создание шкалы размеров
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: POST `/v2/entities/productScales/save`
    ///
    /// # Параметры запроса:
    /// - `request`: Запрос на создание шкалы (ProductScaleSaveRequest)
    ///
    /// # Что в ответе:
    /// - Созданная шкала с размерами (ProductScaleDto)
    pub async fn save(&self, request: ProductScaleSaveRequest) -> Result<ProductScaleDto> {
        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/entities/productScales/save", &json_body, &[])
            .await?;

        let scale: ProductScaleDto = serde_json::from_str(&response_json)?;
        Ok(scale)
    }

    /// Редактирование шкалы размеров
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: POST `/v2/entities/productScales/update`
    ///
    /// # Параметры запроса:
    /// - `request`: Запрос на обновление шкалы (ProductScaleUpdateRequest)
    ///
    /// # Что в ответе:
    /// - Отредактированная шкала с размерами (ProductScaleDto)
    pub async fn update(&self, request: ProductScaleUpdateRequest) -> Result<ProductScaleDto> {
        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/entities/productScales/update", &json_body, &[])
            .await?;

        let scale: ProductScaleDto = serde_json::from_str(&response_json)?;
        Ok(scale)
    }

    /// Удаление шкал размеров
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: POST `/v2/entities/productScales/delete`
    ///
    /// # Параметры запроса:
    /// - `ids`: Список UUID шкал для удаления (обязательный)
    ///
    /// # Что в ответе:
    /// - Удалённая шкала с размерами (ProductScaleDto) - возвращается первая удаленная шкала
    pub async fn delete(&self, ids: Vec<Uuid>) -> Result<ProductScaleDto> {
        let items = ids.into_iter().map(|id| IdCodeDto { id }).collect();
        let request = ItemsRequest { items };

        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/entities/productScales/delete", &json_body, &[])
            .await?;

        let scale: ProductScaleDto = serde_json::from_str(&response_json)?;
        Ok(scale)
    }

    /// Восстановление шкал размеров
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: POST `/v2/entities/productScales/restore`
    ///
    /// # Параметры запроса:
    /// - `ids`: Список UUID шкал для восстановления (обязательный)
    ///
    /// # Что в ответе:
    /// - Восстановленная шкала с размерами (ProductScaleDto) - возвращается первая восстановленная шкала
    pub async fn restore(&self, ids: Vec<Uuid>) -> Result<ProductScaleDto> {
        let items = ids.into_iter().map(|id| IdCodeDto { id }).collect();
        let request = ItemsRequest { items };

        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/entities/productScales/restore", &json_body, &[])
            .await?;

        let scale: ProductScaleDto = serde_json::from_str(&response_json)?;
        Ok(scale)
    }

    /// Получение шкалы с коэффициентами и доступностью размеров для продукта
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: GET `/v2/entities/products/{productId}/productScale`
    ///
    /// # Параметры запроса:
    /// - `product_id`: UUID продукта (обязательный)
    ///
    /// # Что в ответе:
    /// - Результат операции (ProductScaleOperationResult):
    ///   - `result`: "SUCCESS" или "ERROR"
    ///   - `errors`: Список ошибок валидации (null при успешной операции)
    ///   - `response`: Шкала с коэффициентами и доступностью размеров (ProductScaleDto) или null
    pub async fn get_for_product(&self, product_id: Uuid) -> Result<ProductScaleOperationResult> {
        let product_id_str = product_id.to_string();
        let endpoint = format!("v2/entities/products/{}/productScale", product_id_str);
        let response_json = self.client.get(&endpoint).await?;

        let result: ProductScaleOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Получение шкал с коэффициентами и доступностью размеров по списку продуктов (GET)
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: GET `/v2/entities/products/productScales`
    ///
    /// # Параметры запроса:
    /// - `product_ids`: Список UUID продуктов (необязательный, если не задать - возвращаются шкалы для всех не удалённых продуктов)
    /// - `include_deleted_products`: Включать ли в результат шкалы для удалённых продуктов. По умолчанию false
    ///
    /// # Что в ответе:
    /// - Список пар (productId : шкала) - HashMap<Uuid, Option<ProductScaleDto>>
    pub async fn get_for_products(
        &self,
        product_ids: Option<Vec<Uuid>>,
        include_deleted_products: Option<bool>,
    ) -> Result<HashMap<Uuid, Option<ProductScaleDto>>> {
        let mut param_strings = Vec::new();
        let mut params = Vec::new();

        if let Some(include_del) = include_deleted_products {
            param_strings.push(include_del.to_string());
        }

        if let Some(ref id_list) = product_ids {
            for id in id_list {
                param_strings.push(id.to_string());
            }
        }

        let mut idx = 0;
        if let Some(_) = include_deleted_products {
            params.push(("includeDeletedProducts", param_strings[idx].as_str()));
            idx += 1;
        }

        if let Some(ref id_list) = product_ids {
            for _ in id_list {
                params.push(("productId", param_strings[idx].as_str()));
                idx += 1;
            }
        }

        let response_json = if params.is_empty() {
            self.client
                .get("v2/entities/products/productScales")
                .await?
        } else {
            self.client
                .get_with_params("v2/entities/products/productScales", &params)
                .await?
        };

        // Парсим как HashMap, где ключи - строки UUID, значения - Option<ProductScaleDto>
        let mut result: HashMap<String, Option<ProductScaleDto>> =
            serde_json::from_str(&response_json)?;

        // Конвертируем ключи из String в Uuid
        let mut converted: HashMap<Uuid, Option<ProductScaleDto>> = HashMap::new();
        for (key, value) in result.drain() {
            if let Ok(uuid) = Uuid::parse_str(&key) {
                converted.insert(uuid, value);
            }
        }

        Ok(converted)
    }

    /// Получение шкал с коэффициентами и доступностью размеров по списку продуктов (POST)
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: POST `/v2/entities/products/productScales`
    ///
    /// # Параметры запроса:
    /// - `product_ids`: Список UUID продуктов (необязательный, если не задать - возвращаются шкалы для всех не удалённых продуктов)
    /// - `include_deleted_products`: Включать ли в результат шкалы для удалённых продуктов. По умолчанию false
    ///
    /// # Что в ответе:
    /// - Список пар (productId : шкала) - HashMap<Uuid, Option<ProductScaleDto>>
    pub async fn get_for_products_post(
        &self,
        product_ids: Option<Vec<Uuid>>,
        include_deleted_products: Option<bool>,
    ) -> Result<HashMap<Uuid, Option<ProductScaleDto>>> {
        let mut form_strings = Vec::new();
        let mut form_data = Vec::new();

        if let Some(include_del) = include_deleted_products {
            form_strings.push(include_del.to_string());
        }

        if let Some(ref id_list) = product_ids {
            for id in id_list {
                form_strings.push(id.to_string());
            }
        }

        let mut idx = 0;
        if let Some(_) = include_deleted_products {
            form_data.push(("includeDeletedProducts", form_strings[idx].as_str()));
            idx += 1;
        }

        if let Some(ref id_list) = product_ids {
            for _ in id_list {
                form_data.push(("productId", form_strings[idx].as_str()));
                idx += 1;
            }
        }

        let response_json = self
            .client
            .post_form("v2/entities/products/productScales", &form_data)
            .await?;

        // Парсим как HashMap, где ключи - строки UUID, значения - Option<ProductScaleDto>
        let mut result: HashMap<String, Option<ProductScaleDto>> =
            serde_json::from_str(&response_json)?;

        // Конвертируем ключи из String в Uuid
        let mut converted: HashMap<Uuid, Option<ProductScaleDto>> = HashMap::new();
        for (key, value) in result.drain() {
            if let Ok(uuid) = Uuid::parse_str(&key) {
                converted.insert(uuid, value);
            }
        }

        Ok(converted)
    }

    /// Задание/редактирование шкалы с доступностью и коэффициентами для размеров
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: POST `/v2/entities/products/{productId}/productScale`
    ///
    /// # Параметры запроса:
    /// - `product_id`: UUID продукта (обязательный)
    /// - `request`: Запрос на задание/редактирование шкалы (ProductProductScaleRequest)
    ///
    /// # Что в ответе:
    /// - Шкала с коэффициентами и доступностью размеров (ProductScaleDto)
    pub async fn set_for_product(
        &self,
        product_id: Uuid,
        request: ProductProductScaleRequest,
    ) -> Result<ProductScaleDto> {
        let product_id_str = product_id.to_string();
        let endpoint = format!("v2/entities/products/{}/productScale", product_id_str);
        let json_body = json_to_string(&request)?;
        let response_json = self.client.post_json(&endpoint, &json_body, &[]).await?;

        let scale: ProductScaleDto = serde_json::from_str(&response_json)?;
        Ok(scale)
    }

    /// Удаление шкалы размеров у продукта
    ///
    /// # Версия iiko: 6.2+
    /// # Endpoint: DELETE `/v2/entities/products/{productId}/productScale`
    ///
    /// # Параметры запроса:
    /// - `product_id`: UUID продукта (обязательный)
    ///
    /// # Что в ответе:
    /// - UUID шкалы (Uuid)
    pub async fn delete_for_product(&self, product_id: Uuid) -> Result<Uuid> {
        let product_id_str = product_id.to_string();
        let endpoint = format!("v2/entities/products/{}/productScale", product_id_str);

        let response_text = self.client.delete(&endpoint).await?;

        let scale_id = Uuid::parse_str(response_text.trim())
            .map_err(|e| crate::error::IikoError::Api(format!("Failed to parse UUID: {}", e)))?;
        Ok(scale_id)
    }
}
