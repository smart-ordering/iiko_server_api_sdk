use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::response::assembly_charts::{
    AssemblyChartDto, AssemblyChartOperationResult, ChartResultDto,
};
use serde::Serialize;
use serde_json::to_string as json_to_string;
use uuid::Uuid;

pub struct AssemblyChartsEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> AssemblyChartsEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Получение всех технологических карт (getAll)
    ///
    /// # Важно:
    /// - Запрашивайте данные за период не длиннее одного месяца, в идеале — за один день или неделю
    /// - `dateFrom` обязателен
    /// - Если `dateTo` не задан, возвращаются все будущие техкарты
    ///
    /// # Параметры
    /// - `date_from`: Учетный день, начиная с которого требуются техкарты (формат: yyyy-MM-dd, обязательный)
    /// - `date_to`: Учетный день, начиная с которого техкарты не требуются (формат: yyyy-MM-dd, необязательный)
    /// - `include_deleted_products`: Включать ли в результат техкарты для удаленных блюд (по умолчанию true)
    /// - `include_prepared_charts`: Включать ли в результат техкарты, разложенные до конечных ингредиентов (по умолчанию false)
    ///
    /// # Примечание:
    /// Метод списания влияет на результат при `include_prepared_charts = true`:
    /// - "Списывать готовое блюдо" (DIRECT) - в preparedCharts возвращается элемент списания
    /// - "Списывать ингредиенты" (ASSEMBLE) - в preparedCharts возвращаются ингредиенты элемента списания
    pub async fn get_all(
        &self,
        date_from: String,
        date_to: Option<String>,
        include_deleted_products: Option<bool>,
        include_prepared_charts: Option<bool>,
    ) -> Result<ChartResultDto> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("dateFrom", date_from.as_str()));
        if let Some(ref dt) = date_to {
            params.push(("dateTo", dt.as_str()));
        }
        if let Some(inc_del) = include_deleted_products {
            params.push((
                "includeDeletedProducts",
                if inc_del { "true" } else { "false" },
            ));
        }
        if let Some(inc_prep) = include_prepared_charts {
            params.push((
                "includePreparedCharts",
                if inc_prep { "true" } else { "false" },
            ));
        }

        let response_json = self
            .client
            .get_with_params("v2/assemblyCharts/getAll", &params)
            .await?;

        let result: ChartResultDto = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Получение обновления технологических карт (getAllUpdate)
    ///
    /// # Важно:
    /// - По состоянию на 6.0 получение обновлений полностью работает только в iikoChain
    /// - Получение обновлений не поддерживается для `include_deleted_products = false`
    /// - Используйте `known_revision` из предыдущего результата `getAll` или `getAllUpdate`
    ///
    /// # Параметры
    /// - `known_revision`: Значение поля knownRevision из предыдущего результата (обязательный)
    /// - `date_from`: Учетный день, начиная с которого требуются техкарты (формат: yyyy-MM-dd, обязательный)
    /// - `date_to`: Учетный день, начиная с которого техкарты не требуются (формат: yyyy-MM-dd, необязательный)
    /// - `include_deleted_products`: Включать ли в результат техкарты для удаленных блюд (по умолчанию true)
    /// - `include_prepared_charts`: Включать ли в результат техкарты, разложенные до конечных ингредиентов (по умолчанию false)
    ///
    /// # Примечание:
    /// - `deletedAssemblyChartIds` и `deletedPreparedChartIds` содержат UUID удаленных техкарт
    /// - Клиент должен забыть перечисленные техкарты и начать считать актуальными те, что действовали на даты, предшествовавшие удаленным
    /// - На iikoRMS не работает (не сообщает об удалениях, реплицированных с iikoChain)
    pub async fn get_all_update(
        &self,
        known_revision: i64,
        date_from: String,
        date_to: Option<String>,
        include_deleted_products: Option<bool>,
        include_prepared_charts: Option<bool>,
    ) -> Result<ChartResultDto> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let rev_string = known_revision.to_string();
        params.push(("knownRevision", rev_string.as_str()));
        params.push(("dateFrom", date_from.as_str()));
        if let Some(ref dt) = date_to {
            params.push(("dateTo", dt.as_str()));
        }
        if let Some(inc_del) = include_deleted_products {
            params.push((
                "includeDeletedProducts",
                if inc_del { "true" } else { "false" },
            ));
        }
        if let Some(inc_prep) = include_prepared_charts {
            params.push((
                "includePreparedCharts",
                if inc_prep { "true" } else { "false" },
            ));
        }

        let response_json = self
            .client
            .get_with_params("v2/assemblyCharts/getAllUpdate", &params)
            .await?;

        let result: ChartResultDto = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Получение дерева актуальных технологических карт для элемента номенклатуры (getTree)
    ///
    /// # Важно:
    /// - Возвращает дерево техкарт: техкарту запрошенного продукта и техкарты всех заготовок, входящих в него, рекурсивно
    /// - Если `department_id` не указан, возвращается техкарта со строками, действующими в любом из подразделений
    /// - Если `department_id` указан, фильтры storeSpecification будут урезаны до одного указанного подразделения
    ///
    /// # Параметры
    /// - `date`: Учетный день (формат: yyyy-MM-dd, обязательный)
    /// - `product_id`: UUID элемента номенклатуры (блюда, модификатора, заготовки) (обязательный)
    /// - `department_id`: UUID подразделения (необязательный)
    ///
    /// # Примечание:
    /// - `knownRevision` всегда -1, т.к. обновление дерева техкарт невозможно вычислить по одной ревизии
    pub async fn get_tree(
        &self,
        date: String,
        product_id: Uuid,
        department_id: Option<Uuid>,
    ) -> Result<ChartResultDto> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("date", date.as_str()));
        let product_id_str = product_id.to_string();
        params.push(("productId", product_id_str.as_str()));
        let dept_id_str = department_id.map(|id| id.to_string());
        if let Some(ref dept_id_str) = dept_id_str {
            params.push(("departmentId", dept_id_str.as_str()));
        }

        let response_json = self
            .client
            .get_with_params("v2/assemblyCharts/getTree", &params)
            .await?;

        let result: ChartResultDto = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Получение исходной технологической карты для элемента номенклатуры (getAssembled)
    ///
    /// # Важно:
    /// - Возвращает первый уровень актуальной техкарты (не более одного элемента в списке assemblyCharts)
    /// - Если `department_id` не указан, возвращается техкарта со строками, действующими в любом из подразделений
    ///
    /// # Параметры
    /// - `date`: Учетный день (формат: yyyy-MM-dd, обязательный)
    /// - `product_id`: UUID элемента номенклатуры (блюда, модификатора, заготовки) (обязательный)
    /// - `department_id`: UUID подразделения (необязательный)
    pub async fn get_assembled(
        &self,
        date: String,
        product_id: Uuid,
        department_id: Option<Uuid>,
    ) -> Result<ChartResultDto> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("date", date.as_str()));
        let product_id_str = product_id.to_string();
        params.push(("productId", product_id_str.as_str()));
        let dept_id_str = department_id.map(|id| id.to_string());
        if let Some(ref dept_id_str) = dept_id_str {
            params.push(("departmentId", dept_id_str.as_str()));
        }

        let response_json = self
            .client
            .get_with_params("v2/assemblyCharts/getAssembled", &params)
            .await?;

        let result: ChartResultDto = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Получение технологической карты элемента номенклатуры, разложенной до конечных ингредиентов (getPrepared)
    ///
    /// # Важно:
    /// - В конечной техкарте норма закладки приготавливаемого блюда всегда равна одной основной единице его измерения
    /// - Если `department_id` не указан, возвращается техкарта со строками, действующими в любом из подразделений
    ///
    /// # Параметры
    /// - `date`: Учетный день (формат: yyyy-MM-dd, обязательный)
    /// - `product_id`: UUID элемента номенклатуры (блюда, модификатора, заготовки) (обязательный)
    /// - `department_id`: UUID подразделения (необязательный)
    pub async fn get_prepared(
        &self,
        date: String,
        product_id: Uuid,
        department_id: Option<Uuid>,
    ) -> Result<ChartResultDto> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("date", date.as_str()));
        let product_id_str = product_id.to_string();
        params.push(("productId", product_id_str.as_str()));
        let dept_id_str = department_id.map(|id| id.to_string());
        if let Some(ref dept_id_str) = dept_id_str {
            params.push(("departmentId", dept_id_str.as_str()));
        }

        let response_json = self
            .client
            .get_with_params("v2/assemblyCharts/getPrepared", &params)
            .await?;

        let result: ChartResultDto = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Получение технологической карты по ID (byId)
    ///
    /// # Параметры
    /// - `id`: UUID технологической карты (обязательный)
    pub async fn by_id(&self, id: Uuid) -> Result<AssemblyChartDto> {
        let id_str = id.to_string();
        let params = vec![("id", id_str.as_str())];
        let response_json = self
            .client
            .get_with_params("v2/assemblyCharts/byId", &params)
            .await?;

        let result: AssemblyChartDto = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Получение истории техкарт по продукту (getHistory)
    ///
    /// # Важно:
    /// - Возвращает список всех техкарт приготавливаемого элемента номенклатуры
    /// - Если `department_id` не указан, возвращается техкарта со строками, действующими в любом из подразделений
    ///
    /// # Параметры
    /// - `product_id`: UUID приготавливаемого элемента номенклатуры (обязательный)
    /// - `department_id`: UUID подразделения (необязательный)
    pub async fn get_history(
        &self,
        product_id: Uuid,
        department_id: Option<Uuid>,
    ) -> Result<Vec<AssemblyChartDto>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let product_id_str = product_id.to_string();
        params.push(("productId", product_id_str.as_str()));
        let dept_id_str = department_id.map(|id| id.to_string());
        if let Some(ref dept_id_str) = dept_id_str {
            params.push(("departmentId", dept_id_str.as_str()));
        }

        let response_json = self
            .client
            .get_with_params("v2/assemblyCharts/getHistory", &params)
            .await?;

        let result: Vec<AssemblyChartDto> = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Создание технологической карты (save)
    ///
    /// # Важно:
    /// - `assembled_product_id` обязателен
    /// - `date_from` обязателен
    /// - `date_to` может быть null (техкарта действует бессрочно)
    /// - `items` должны содержать хотя бы одну строку
    ///
    /// # Параметры
    /// - `chart`: Технологическая карта для создания
    ///
    /// # Ограничения:
    /// - Смешивать техкарты разных типов (COMMON/SPECIFIC) в иерархии одного элемента номенклатуры не рекомендуется
    /// - Если техкарта не содержит данных для "не своего" типа списания, результат умножения равен нулю
    pub async fn save(&self, chart: AssemblyChartDto) -> Result<AssemblyChartOperationResult> {
        let json_body = json_to_string(&chart)?;
        let response_json = self
            .client
            .post_json("v2/assemblyCharts/save", &json_body, &[])
            .await?;

        let result: AssemblyChartOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }

    /// Удаление технологической карты (delete)
    ///
    /// # Параметры
    /// - `id`: UUID технологической карты (обязательный)
    ///
    /// # Что в ответе:
    /// - UUID удаленной технологической карты
    pub async fn delete(&self, id: Uuid) -> Result<AssemblyChartOperationResult> {
        #[derive(Serialize)]
        struct DeleteRequest {
            id: Uuid,
        }

        let request = DeleteRequest { id };
        let json_body = json_to_string(&request)?;
        let response_json = self
            .client
            .post_json("v2/assemblyCharts/delete", &json_body, &[])
            .await?;

        let result: AssemblyChartOperationResult = serde_json::from_str(&response_json)?;
        Ok(result)
    }
}
