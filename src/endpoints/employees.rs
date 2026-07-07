use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::response::employees::{Employee, Employees};
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use uuid::Uuid;

pub struct EmployeesEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> EmployeesEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Получение списка активных сотрудников
    ///
    /// # Версия API: 1.0
    /// # Версия iiko: 4.0
    /// # Endpoint: GET `/employees`
    ///
    /// # Параметры запроса:
    /// - `include_deleted`: Возвращать и действующих, и удаленных сотрудников (с версии 5.0)
    /// - `revision_from`: Номер ревизии, начиная с которой необходимо отфильтровать сущности (с версии 6.4)
    ///   По умолчанию (неревизионный запрос) revisionFrom = -1
    ///
    /// # Что в ответе:
    /// - Список сотрудников. Все сотрудники (включая встроенные системные аккаунты), которые активны (не удалены)
    pub async fn list(
        &self,
        include_deleted: Option<bool>,
        revision_from: Option<i64>,
    ) -> Result<Vec<Employee>> {
        let mut param_strings = Vec::new();
        let mut params = Vec::new();

        // Сначала собираем все строки
        if let Some(inc_del) = include_deleted {
            param_strings.push(if inc_del { "true" } else { "false" }.to_string());
        }
        if let Some(rev) = revision_from {
            param_strings.push(rev.to_string());
        }

        // Затем создаем ссылки на них
        let mut idx = 0;
        if include_deleted.is_some() {
            params.push(("includeDeleted", param_strings[idx].as_str()));
            idx += 1;
        }
        if revision_from.is_some() {
            params.push(("revisionFrom", param_strings[idx].as_str()));
        }

        let response_xml = if params.is_empty() {
            self.client.get("employees").await?
        } else {
            self.client.get_with_params("employees", &params).await?
        };

        let wrapper: Employees = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    /// Получение списка сотрудников по подразделению
    ///
    /// # Версия API: 1.0
    /// # Версия iiko: 4.0
    /// # Endpoint: GET `/employees/byDepartment/{departmentCode}`
    ///
    /// # Параметры запроса:
    /// - `department_code`: Код подразделения (обязательный)
    /// - `include_deleted`: Возвращать и действующих, и удаленных сотрудников (с версии 5.0)
    ///
    /// # Что в ответе:
    /// - Список сотрудников указанного подразделения. Все сотрудники (включая встроенные системные аккаунты), которые активны (не удалены).
    ///   Для RMS идентично обычному списку, для Chain - только список сотрудников указанного подразделения.
    pub async fn list_by_department(
        &self,
        department_code: &str,
        include_deleted: Option<bool>,
    ) -> Result<Vec<Employee>> {
        let mut param_strings = Vec::new();
        let mut params = Vec::new();

        // Сначала собираем все строки
        if let Some(inc_del) = include_deleted {
            param_strings.push(if inc_del { "true" } else { "false" }.to_string());
        }

        // Затем создаем ссылки на них
        if include_deleted.is_some() {
            params.push(("includeDeleted", param_strings[0].as_str()));
        }

        let endpoint = format!("employees/byDepartment/{}", department_code);
        let response_xml = if params.is_empty() {
            self.client.get(&endpoint).await?
        } else {
            self.client.get_with_params(&endpoint, &params).await?
        };

        let wrapper: Employees = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    /// Получение сотрудника по ID
    ///
    /// # Версия API: 1.0
    /// # Версия iiko: 4.0
    /// # Endpoint: GET `/employees/byId/{employeeUUID}`
    ///
    /// # Параметры запроса:
    /// - `employee_uuid`: UUID сотрудника (обязательный)
    ///
    /// # Что в ответе:
    /// - Сотрудник с указанным GUID
    pub async fn get_by_id(&self, employee_uuid: Uuid) -> Result<Employee> {
        let endpoint = format!("employees/byId/{}", employee_uuid);
        let response_xml = self.client.get(&endpoint).await?;

        let employee: Employee = from_str(&response_xml)?;
        Ok(employee)
    }

    /// Получение сотрудника по коду
    ///
    /// # Версия API: 1.0
    /// # Версия iiko: 4.0
    /// # Endpoint: GET `/employees/byCode/{employeeCode}`
    ///
    /// # Параметры запроса:
    /// - `employee_code`: Код сотрудника (обязательный)
    ///
    /// # Что в ответе:
    /// - Сотрудник с указанным кодом
    pub async fn get_by_code(&self, employee_code: &str) -> Result<Employee> {
        let endpoint = format!("employees/byCode/{}", employee_code);
        let response_xml = self.client.get(&endpoint).await?;

        let employee: Employee = from_str(&response_xml)?;
        Ok(employee)
    }

    /// Поиск сотрудника
    ///
    /// # Версия API: 1.0
    /// # Версия iiko: 4.0
    /// # Endpoint: GET `/employees/search`
    ///
    /// # Параметры запроса:
    /// Поиск по регулярному выражению по любому из текстовых или булевых полей в dto:
    /// - `address`, `card_number`, `cell_phone`, `client`, `code`, `email`, `employee`,
    ///   `first_name`, `last_name`, `login`, `main_role_code`, `middle_name`, `name`,
    ///   `note`, `phone`, `supplier`
    ///   Параметры необязательные. Если отсутствуют, вернет всех активных.
    /// - `include_deleted`: Возвращать и действующих, и удаленных сотрудников (с версии 5.0)
    ///
    /// # Что в ответе:
    /// - Список найденных сотрудников
    pub async fn search(
        &self,
        address: Option<&str>,
        card_number: Option<&str>,
        cell_phone: Option<&str>,
        client: Option<bool>,
        code: Option<&str>,
        email: Option<&str>,
        employee: Option<bool>,
        first_name: Option<&str>,
        last_name: Option<&str>,
        login: Option<&str>,
        main_role_code: Option<&str>,
        middle_name: Option<&str>,
        name: Option<&str>,
        note: Option<&str>,
        phone: Option<&str>,
        supplier: Option<bool>,
        include_deleted: Option<bool>,
    ) -> Result<Vec<Employee>> {
        let mut param_strings = Vec::new();
        let mut params = Vec::new();

        // Собираем все строки
        if let Some(a) = address {
            param_strings.push(a.to_string());
        }
        if let Some(cn) = card_number {
            param_strings.push(cn.to_string());
        }
        if let Some(cp) = cell_phone {
            param_strings.push(cp.to_string());
        }
        if let Some(c) = client {
            param_strings.push(if c { "true" } else { "false" }.to_string());
        }
        if let Some(c) = code {
            param_strings.push(c.to_string());
        }
        if let Some(e) = email {
            param_strings.push(e.to_string());
        }
        if let Some(e) = employee {
            param_strings.push(if e { "true" } else { "false" }.to_string());
        }
        if let Some(fn_val) = first_name {
            param_strings.push(fn_val.to_string());
        }
        if let Some(ln) = last_name {
            param_strings.push(ln.to_string());
        }
        if let Some(l) = login {
            param_strings.push(l.to_string());
        }
        if let Some(mrc) = main_role_code {
            param_strings.push(mrc.to_string());
        }
        if let Some(mn) = middle_name {
            param_strings.push(mn.to_string());
        }
        if let Some(n) = name {
            param_strings.push(n.to_string());
        }
        if let Some(n) = note {
            param_strings.push(n.to_string());
        }
        if let Some(p) = phone {
            param_strings.push(p.to_string());
        }
        if let Some(s) = supplier {
            param_strings.push(if s { "true" } else { "false" }.to_string());
        }
        if let Some(inc_del) = include_deleted {
            param_strings.push(if inc_del { "true" } else { "false" }.to_string());
        }

        // Создаем params, используя индексы
        let mut idx = 0;
        if address.is_some() {
            params.push(("address", param_strings[idx].as_str()));
            idx += 1;
        }
        if card_number.is_some() {
            params.push(("cardNumber", param_strings[idx].as_str()));
            idx += 1;
        }
        if cell_phone.is_some() {
            params.push(("cellPhone", param_strings[idx].as_str()));
            idx += 1;
        }
        if client.is_some() {
            params.push(("client", param_strings[idx].as_str()));
            idx += 1;
        }
        if code.is_some() {
            params.push(("code", param_strings[idx].as_str()));
            idx += 1;
        }
        if email.is_some() {
            params.push(("email", param_strings[idx].as_str()));
            idx += 1;
        }
        if employee.is_some() {
            params.push(("employee", param_strings[idx].as_str()));
            idx += 1;
        }
        if first_name.is_some() {
            params.push(("firstName", param_strings[idx].as_str()));
            idx += 1;
        }
        if last_name.is_some() {
            params.push(("lastName", param_strings[idx].as_str()));
            idx += 1;
        }
        if login.is_some() {
            params.push(("login", param_strings[idx].as_str()));
            idx += 1;
        }
        if main_role_code.is_some() {
            params.push(("mainRoleCode", param_strings[idx].as_str()));
            idx += 1;
        }
        if middle_name.is_some() {
            params.push(("middleName", param_strings[idx].as_str()));
            idx += 1;
        }
        if name.is_some() {
            params.push(("name", param_strings[idx].as_str()));
            idx += 1;
        }
        if note.is_some() {
            params.push(("note", param_strings[idx].as_str()));
            idx += 1;
        }
        if phone.is_some() {
            params.push(("phone", param_strings[idx].as_str()));
            idx += 1;
        }
        if supplier.is_some() {
            params.push(("supplier", param_strings[idx].as_str()));
            idx += 1;
        }
        if include_deleted.is_some() {
            params.push(("includeDeleted", param_strings[idx].as_str()));
        }

        let response_xml = self
            .client
            .get_with_params("employees/search", &params)
            .await?;

        let wrapper: Employees = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    /// Добавить или заменить сотрудника
    ///
    /// # Версия API: 1.0
    /// # Версия iiko: 4.0
    /// # Endpoint: PUT `/employees/byId/{UUID}`
    ///
    /// # Параметры запроса:
    /// - `employee_uuid`: UUID сотрудника (обязательный)
    /// - `employee`: Данные сотрудника (обязательный)
    ///
    /// # Что в ответе:
    /// - Если передан новый id, то будет создан новый сотрудник (код возврата 201 Created).
    /// - Если передан id существующего сотрудника, то произойдет полное замещение всех полей сотрудника (код возврата 200 OK).
    ///   При этом если не указать какое-либо из необязательных полей, то значение этого поля сбросится.
    /// - Для обновления частичного набора полей используйте метод `update_partial`
    pub async fn create_or_replace(
        &self,
        employee_uuid: Uuid,
        employee: &Employee,
    ) -> Result<Employee> {
        let endpoint = format!("employees/byId/{}", employee_uuid);
        let xml_body = to_string(employee)?;
        let response_xml = self.client.put_xml(&endpoint, &xml_body).await?;

        let employee: Employee = from_str(&response_xml)?;
        Ok(employee)
    }

    /// Частичное обновление сотрудника
    ///
    /// # Версия API: 1.0
    /// # Версия iiko: 4.0
    /// # Endpoint: POST `/employees/byId/{employeeUUID}`
    ///
    /// # Параметры запроса:
    /// - `employee_uuid`: UUID сотрудника (обязательный)
    /// - `employee`: Данные сотрудника для частичного обновления (обязательный)
    ///
    /// # Что в ответе:
    /// - Обновленный сотрудник
    pub async fn update_partial(
        &self,
        employee_uuid: Uuid,
        employee: &Employee,
    ) -> Result<Employee> {
        let endpoint = format!("employees/byId/{}", employee_uuid);
        let xml_body = to_string(employee)?;
        let response_xml = self.client.post_xml(&endpoint, &xml_body).await?;

        let employee: Employee = from_str(&response_xml)?;
        Ok(employee)
    }

    /// Удаление сотрудника
    ///
    /// # Версия API: 1.0
    /// # Версия iiko: 4.0
    /// # Endpoint: DELETE `/employees/byId/{employeeUUID}`
    ///
    /// # Параметры запроса:
    /// - `employee_uuid`: UUID сотрудника (обязательный)
    ///
    /// # Что в ответе:
    /// - Пустой ответ если сотрудник удален (или уже был удален).
    /// - Entity of class User not found by id (employeeUUID), если передан несуществующий guid.
    pub async fn delete(&self, employee_uuid: Uuid) -> Result<String> {
        let endpoint = format!("employees/byId/{}", employee_uuid);
        self.client.delete(&endpoint).await
    }
}
