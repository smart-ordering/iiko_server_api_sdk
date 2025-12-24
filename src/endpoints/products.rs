use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::response::products::{
    CategoryDeleteRequest, CategoryOperationResult, CategoryRestoreRequest, CategorySaveRequest,
    CategoryUpdateRequest, EntityDto, IdCodeDto, ItemsRequest, ProductDto, ProductGroupDto,
    ProductOperationResult, ProductsOperationResult,
};
use serde_json::to_string as json_to_string;
use uuid::Uuid;

pub struct ProductsEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> ProductsEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Получить список продуктов (GET)
    ///
    /// # Параметры
    /// - `include_deleted`: Включать ли удаленные элементы
    /// - `ids`: Список ID продуктов для фильтрации
    /// - `nums`: Список артикулов для фильтрации
    /// - `types`: Список типов продуктов для фильтрации
    /// - `category_ids`: Список ID категорий для фильтрации
    /// - `parent_ids`: Список ID родительских групп для фильтрации
    pub async fn list(
        &self,
        include_deleted: Option<bool>,
        ids: Option<Vec<String>>,
        nums: Option<Vec<String>>,
        types: Option<Vec<String>>,
        category_ids: Option<Vec<String>>,
        parent_ids: Option<Vec<String>>,
    ) -> Result<Vec<ProductDto>> {
        let mut params: Vec<(&str, &str)> = Vec::new();

        if let Some(inc_del) = include_deleted {
            params.push(("includeDeleted", if inc_del { "true" } else { "false" }));
        }

        if let Some(ref id_list) = ids {
            for id in id_list {
                params.push(("ids", id.as_str()));
            }
        }

        if let Some(ref num_list) = nums {
            for num in num_list {
                params.push(("nums", num.as_str()));
            }
        }

        if let Some(ref type_list) = types {
            for t in type_list {
                params.push(("types", t.as_str()));
            }
        }

        if let Some(ref cat_ids) = category_ids {
            for cat_id in cat_ids {
                params.push(("categoryIds", cat_id.as_str()));
            }
        }

        if let Some(ref par_ids) = parent_ids {
            for par_id in par_ids {
                params.push(("parentIds", par_id.as_str()));
            }
        }

        let response_json = self
            .client
            .get_with_params("v2/entities/products/list", &params)
            .await?;

        let products: Vec<ProductDto> = serde_json::from_str(&response_json)?;
        Ok(products)
    }

    /// Получить список продуктов (POST с form data)
    ///
    /// # Параметры
    /// - `include_deleted`: Включать ли удаленные элементы
    /// - `revision_from`: Ревизия, с которой запрашиваются элементы
    /// - `ids`: Список ID продуктов для фильтрации
    /// - `nums`: Список артикулов для фильтрации
    /// - `codes`: Список кодов для фильтрации
    /// - `types`: Список типов продуктов для фильтрации
    /// - `category_ids`: Список ID категорий для фильтрации
    /// - `parent_ids`: Список ID родительских групп для фильтрации
    pub async fn list_post(
        &self,
        include_deleted: Option<bool>,
        revision_from: Option<i64>,
        ids: Option<Vec<String>>,
        nums: Option<Vec<String>>,
        codes: Option<Vec<String>>,
        types: Option<Vec<String>>,
        category_ids: Option<Vec<String>>,
        parent_ids: Option<Vec<String>>,
    ) -> Result<Vec<ProductDto>> {
        let mut form_params: Vec<(&str, &str)> = Vec::new();
        let rev_string;

        if let Some(inc_del) = include_deleted {
            form_params.push(("includeDeleted", if inc_del { "true" } else { "false" }));
        }

        if let Some(rev) = revision_from {
            rev_string = rev.to_string();
            form_params.push(("revisionFrom", &rev_string));
        }

        if let Some(ref id_list) = ids {
            for id in id_list {
                form_params.push(("ids", id.as_str()));
            }
        }

        if let Some(ref num_list) = nums {
            for num in num_list {
                form_params.push(("nums", num.as_str()));
            }
        }

        if let Some(ref code_list) = codes {
            for code in code_list {
                form_params.push(("codes", code.as_str()));
            }
        }

        if let Some(ref type_list) = types {
            for t in type_list {
                form_params.push(("types", t.as_str()));
            }
        }

        if let Some(ref cat_ids) = category_ids {
            for cat_id in cat_ids {
                form_params.push(("categoryIds", cat_id.as_str()));
            }
        }

        if let Some(ref par_ids) = parent_ids {
            for par_id in par_ids {
                form_params.push(("parentIds", par_id.as_str()));
            }
        }

        let response_json = self
            .client
            .post_form("v2/entities/products/list", &form_params)
            .await?;

        let products: Vec<ProductDto> = serde_json::from_str(&response_json)?;
        Ok(products)
    }

    /// Импорт элемента номенклатуры
    ///
    /// # Параметры
    /// - `product`: Продукт для импорта
    /// - `generate_nomenclature_code`: Генерировать ли артикул (по умолчанию true)
    /// - `generate_fast_code`: Генерировать ли код быстрого поиска (по умолчанию true)
    pub async fn save(
        &self,
        product: ProductDto,
        generate_nomenclature_code: Option<bool>,
        generate_fast_code: Option<bool>,
    ) -> Result<ProductOperationResult> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        if let Some(gen_num) = generate_nomenclature_code {
            params.push((
                "generateNomenclatureCode",
                if gen_num { "true" } else { "false" },
            ));
        }
        if let Some(gen_fast) = generate_fast_code {
            params.push(("generateFastCode", if gen_fast { "true" } else { "false" }));
        }

        let json_body = json_to_string(&product)?;
        let response_json = self
            .client
            .post_json("v2/entities/products/save", &json_body, &params)
            .await?;

        let result: ProductOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Редактирование элемента номенклатуры
    ///
    /// # Важно:
    /// - `product.id` **обязательно** должен быть указан (UUID редактируемого элемента)
    /// - Структура запроса аналогична импорту, но с обязательным полем `id`
    /// - При обновлении можно передавать только изменяемые поля
    ///
    /// # Параметры
    /// - `product`: Продукт с обязательным `id` для обновления
    /// - `override_fast_code`: Перегенерировать ли код быстрого поиска (по умолчанию false)
    /// - `override_nomenclature_code`: Перегенерировать ли артикул (по умолчанию false)
    pub async fn update(
        &self,
        product: ProductDto,
        override_fast_code: Option<bool>,
        override_nomenclature_code: Option<bool>,
    ) -> Result<ProductOperationResult> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        if let Some(ovr_fast) = override_fast_code {
            params.push(("overrideFastCode", if ovr_fast { "true" } else { "false" }));
        }
        if let Some(ovr_num) = override_nomenclature_code {
            params.push((
                "overrideNomenclatureCode",
                if ovr_num { "true" } else { "false" },
            ));
        }

        let json_body = json_to_string(&product)?;
        let response_json = self
            .client
            .post_json("v2/entities/products/update", &json_body, &params)
            .await?;

        let result: ProductOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Удаление элементов номенклатуры
    ///
    /// # Важно:
    /// - Удаляет элементы номенклатуры (устанавливает флаг `deleted = true`)
    /// - Нельзя удалить уже удаленные продукты (вернется ошибка)
    /// - В ответе возвращаются удаленные продукты с `deleted = true`
    ///
    /// # Параметры
    /// - `ids`: Список UUID продуктов для удаления
    ///
    /// # Ошибки:
    /// - Если продукт уже удален, вернется ошибка: "Could not delete already deleted products: [uuid]"
    pub async fn delete(&self, ids: Vec<Uuid>) -> Result<ProductsOperationResult> {
        let items = ids.into_iter().map(|id| IdCodeDto { id }).collect();
        let request = ItemsRequest { items };

        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/entities/products/delete", &json_body, &[])
            .await?;

        let result: ProductsOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Восстановление элементов номенклатуры
    ///
    /// # Важно:
    /// - Восстанавливает удаленные элементы номенклатуры (устанавливает флаг `deleted = false`)
    /// - Нельзя восстановить не удаленные продукты (вернется ошибка)
    /// - В ответе возвращаются восстановленные продукты с `deleted = false`
    ///
    /// # Параметры
    /// - `ids`: Список UUID продуктов для восстановления
    /// - `override_nomenclature_code`: Перегенерировать артикул при конфликте (по умолчанию false)
    ///   Если у восстанавливаемого продукта артикул совпадает с одним из текущих и параметр указан
    ///   равным true, то у восстанавливаемого продукта будет сгенерирован новый артикул (версия 6.4+)
    ///
    /// # Ошибки:
    /// - Если продукт не был удален, вернется ошибка: "Could not restore not deleted products: [uuid]"
    pub async fn restore(
        &self,
        ids: Vec<Uuid>,
        override_nomenclature_code: Option<bool>,
    ) -> Result<ProductsOperationResult> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        if let Some(ovr_num) = override_nomenclature_code {
            params.push((
                "overrideNomenclatureCode",
                if ovr_num { "true" } else { "false" },
            ));
        }

        let items = ids.into_iter().map(|id| IdCodeDto { id }).collect();
        let request = ItemsRequest { items };

        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/entities/products/restore", &json_body, &params)
            .await?;

        let result: ProductsOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Получить список групп продуктов
    pub async fn list_groups(&self, include_deleted: Option<bool>) -> Result<Vec<ProductGroupDto>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        if let Some(inc_del) = include_deleted {
            params.push(("includeDeleted", if inc_del { "true" } else { "false" }));
        }

        let response_json = self
            .client
            .get_with_params("v2/entities/products/group/list", &params)
            .await?;

        let groups: Vec<ProductGroupDto> = serde_json::from_str(&response_json)?;
        Ok(groups)
    }

    /// Получить список пользовательских категорий (GET)
    ///
    /// # Параметры
    /// - `include_deleted`: Включать ли в результат удаленные элементы (по умолчанию false)
    /// - `ids`: Список UUID категорий для фильтрации
    /// - `revision_from`: Номер ревизии, начиная с которой фильтровать (по умолчанию -1)
    pub async fn list_categories(
        &self,
        include_deleted: Option<bool>,
        ids: Option<Vec<String>>,
        revision_from: Option<i64>,
    ) -> Result<Vec<EntityDto>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        if let Some(inc_del) = include_deleted {
            params.push(("includeDeleted", if inc_del { "true" } else { "false" }));
        }
        if let Some(ref id_list) = ids {
            for id in id_list {
                params.push(("ids", id.as_str()));
            }
        }
        let rev_string = revision_from.map(|rev| rev.to_string());
        if let Some(ref rev_str) = rev_string {
            params.push(("revisionFrom", rev_str.as_str()));
        }

        let response_json = self
            .client
            .get_with_params("v2/entities/products/category/list", &params)
            .await?;

        let categories: Vec<EntityDto> = serde_json::from_str(&response_json)?;
        Ok(categories)
    }

    /// Получить список пользовательских категорий (POST)
    ///
    /// # Параметры
    /// - `include_deleted`: Включать ли в результат удаленные элементы (по умолчанию false)
    /// - `ids`: Список UUID категорий для фильтрации
    /// - `revision_from`: Номер ревизии, начиная с которой фильтровать (по умолчанию -1)
    pub async fn list_categories_post(
        &self,
        include_deleted: Option<bool>,
        ids: Option<Vec<String>>,
        revision_from: Option<i64>,
    ) -> Result<Vec<EntityDto>> {
        let mut form_params: Vec<(&str, &str)> = Vec::new();
        if let Some(inc_del) = include_deleted {
            form_params.push(("includeDeleted", if inc_del { "true" } else { "false" }));
        }
        if let Some(ref id_list) = ids {
            for id in id_list {
                form_params.push(("ids", id.as_str()));
            }
        }
        let rev_string = revision_from.map(|rev| rev.to_string());
        if let Some(ref rev_str) = rev_string {
            form_params.push(("revisionFrom", rev_str.as_str()));
        }

        let response_json = self
            .client
            .post_form("v2/entities/products/category/list", &form_params)
            .await?;

        let categories: Vec<EntityDto> = serde_json::from_str(&response_json)?;
        Ok(categories)
    }

    /// Импорт пользовательской категории
    ///
    /// # Параметры
    /// - `name`: Имя категории (обязательно)
    ///
    /// # Ошибки:
    /// - Если имя не указано или состоит только из пробелов: "Category name is not specified or consist of whitespaces"
    pub async fn save_category(&self, name: String) -> Result<CategoryOperationResult> {
        let request = CategorySaveRequest { name };
        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/entities/products/category/save", &json_body, &[])
            .await?;

        let result: CategoryOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Редактирование пользовательской категории
    ///
    /// # Параметры
    /// - `id`: UUID редактируемой категории (обязательно)
    /// - `name`: Новое имя категории (обязательно)
    ///
    /// # Ошибки:
    /// - Если имя не указано или состоит только из пробелов: "Category name is not specified or consist of whitespaces"
    pub async fn update_category(&self, id: Uuid, name: String) -> Result<CategoryOperationResult> {
        let request = CategoryUpdateRequest { id, name };
        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/entities/products/category/update", &json_body, &[])
            .await?;

        let result: CategoryOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Удаление пользовательской категории
    ///
    /// # Параметры
    /// - `id`: UUID удаляемой категории (обязательно)
    ///
    /// # Важно:
    /// - Удаляет категорию (устанавливает флаг `deleted = true`)
    /// - Нельзя удалить уже удаленную категорию
    ///
    /// # Ошибки:
    /// - Если категория уже удалена: "Could not delete already deleted product category: [uuid]"
    pub async fn delete_category(&self, id: Uuid) -> Result<CategoryOperationResult> {
        let request = CategoryDeleteRequest { id };
        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/entities/products/category/delete", &json_body, &[])
            .await?;

        let result: CategoryOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Восстановление пользовательской категории
    ///
    /// # Параметры
    /// - `id`: UUID восстанавливаемой категории (обязательно)
    ///
    /// # Важно:
    /// - Восстанавливает удаленную категорию (устанавливает флаг `deleted = false`)
    /// - Нельзя восстановить не удаленную категорию
    ///
    /// # Ошибки:
    /// - Если категория не была удалена: "Could not restore not deleted product category: [uuid]"
    pub async fn restore_category(&self, id: Uuid) -> Result<CategoryOperationResult> {
        let request = CategoryRestoreRequest { id };
        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/entities/products/category/restore", &json_body, &[])
            .await?;

        let result: CategoryOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }
}
