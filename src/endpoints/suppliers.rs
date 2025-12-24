use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::response::suppliers::{Supplier, SupplierPriceListItemDto, Suppliers};
use quick_xml::de::from_str;

pub struct SuppliersEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> SuppliersEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Получение списка поставщиков
    ///
    /// # Версия iiko: 3.9+
    /// # Endpoint: GET `/suppliers`
    ///
    /// # Параметры запроса:
    /// - `revision_from`: Номер ревизии, начиная с которой необходимо отфильтровать сущности (с версии 6.4)
    ///   По умолчанию (неревизионный запрос) revisionFrom = -1
    ///
    /// # Что в ответе:
    /// - Список всех поставщиков (XML структура employees)
    pub async fn list(&self, revision_from: Option<i32>) -> Result<Vec<Supplier>> {
        let mut param_strings = Vec::new();
        let mut params = Vec::new();

        if let Some(rev) = revision_from {
            param_strings.push(rev.to_string());
            params.push(("revisionFrom", param_strings.last().unwrap().as_str()));
        }

        let response_xml = if params.is_empty() {
            self.client.get("suppliers").await?
        } else {
            self.client.get_with_params("suppliers", &params).await?
        };

        // Парсим XML ответ
        // XML формат: <employees><employee>...</employee></employees>
        let wrapper: Suppliers = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    /// Поиск поставщика
    ///
    /// # Версия iiko: 3.9+
    /// # Endpoint: GET `/suppliers/search`
    ///
    /// # Параметры запроса:
    /// Поиск по id поставщика не производится.
    /// Возможно произвести поиск по следующим полям:
    /// - `name` - поле Имя в системе
    /// - `code` - поле Таб.номер/Код
    /// - `phone` - поле Телефон
    /// - `cell_phone` - поле Мобильный телефон
    /// - `first_name` - поле Имя
    /// - `middle_name` - поле Отчество
    /// - `last_name` - поле Фамилия
    /// - `email` - поле e-mail
    /// - `card_number` - поле Номер карты
    /// - `taxpayer_id_number` - поле ИНН
    ///
    /// # Что в ответе:
    /// - Список найденных поставщиков (XML структура employees)
    pub async fn search(
        &self,
        name: Option<&str>,
        code: Option<&str>,
        phone: Option<&str>,
        cell_phone: Option<&str>,
        first_name: Option<&str>,
        middle_name: Option<&str>,
        last_name: Option<&str>,
        email: Option<&str>,
        card_number: Option<&str>,
        taxpayer_id_number: Option<&str>,
    ) -> Result<Vec<Supplier>> {
        let mut param_strings = Vec::new();
        let mut params = Vec::new();

        // Сначала собираем все строки
        if let Some(n) = name {
            param_strings.push(n.to_string());
        }
        if let Some(c) = code {
            param_strings.push(c.to_string());
        }
        if let Some(p) = phone {
            param_strings.push(p.to_string());
        }
        if let Some(cp) = cell_phone {
            param_strings.push(cp.to_string());
        }
        if let Some(fn_val) = first_name {
            param_strings.push(fn_val.to_string());
        }
        if let Some(mn) = middle_name {
            param_strings.push(mn.to_string());
        }
        if let Some(ln) = last_name {
            param_strings.push(ln.to_string());
        }
        if let Some(e) = email {
            param_strings.push(e.to_string());
        }
        if let Some(cn) = card_number {
            param_strings.push(cn.to_string());
        }
        if let Some(tin) = taxpayer_id_number {
            param_strings.push(tin.to_string());
        }

        // Теперь создаем params, используя индексы
        let mut idx = 0;
        if let Some(_) = name {
            params.push(("name", param_strings[idx].as_str()));
            idx += 1;
        }
        if let Some(_) = code {
            params.push(("code", param_strings[idx].as_str()));
            idx += 1;
        }
        if let Some(_) = phone {
            params.push(("phone", param_strings[idx].as_str()));
            idx += 1;
        }
        if let Some(_) = cell_phone {
            params.push(("cellPhone", param_strings[idx].as_str()));
            idx += 1;
        }
        if let Some(_) = first_name {
            params.push(("firstName", param_strings[idx].as_str()));
            idx += 1;
        }
        if let Some(_) = middle_name {
            params.push(("middleName", param_strings[idx].as_str()));
            idx += 1;
        }
        if let Some(_) = last_name {
            params.push(("lastName", param_strings[idx].as_str()));
            idx += 1;
        }
        if let Some(_) = email {
            params.push(("email", param_strings[idx].as_str()));
            idx += 1;
        }
        if let Some(_) = card_number {
            params.push(("cardNumber", param_strings[idx].as_str()));
            idx += 1;
        }
        if let Some(_) = taxpayer_id_number {
            params.push(("taxpayerIdNumber", param_strings[idx].as_str()));
            idx += 1;
        }

        let response_xml = self
            .client
            .get_with_params("suppliers/search", &params)
            .await?;

        // Парсим XML ответ
        // XML формат: <employees><employee>...</employee></employees>
        let wrapper: Suppliers = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    /// Получение прайс-листа поставщика
    ///
    /// # Версия iiko: 3.9+
    /// # Endpoint: GET `/suppliers/{code}/pricelist`
    ///
    /// # Параметры запроса:
    /// - `code`: Код поставщика (обязательный)
    /// - `date`: Дата начала действия прайс-листа в формате DD.MM.YYYY (необязательный)
    ///   Если параметр не указан, возвращается последний прайс-лист
    ///
    /// # Что в ответе:
    /// - Структура supplierPriceListItemDto (XML)
    pub async fn get_pricelist(
        &self,
        code: &str,
        date: Option<&str>,
    ) -> Result<Vec<SupplierPriceListItemDto>> {
        let mut param_strings = Vec::new();
        let mut params = Vec::new();

        if let Some(d) = date {
            param_strings.push(d.to_string());
            params.push(("date", param_strings.last().unwrap().as_str()));
        }

        let endpoint = format!("suppliers/{}/pricelist", code);
        let response_xml = if params.is_empty() {
            self.client.get(&endpoint).await?
        } else {
            self.client.get_with_params(&endpoint, &params).await?
        };

        // Парсим XML ответ
        // XML формат может быть:
        // 1. <supplierPriceList><supplierPriceListItemDto>...</supplierPriceListItemDto></supplierPriceList>
        // 2. Просто список элементов без обертки
        // 3. Один элемент
        use crate::xml::response::suppliers::SupplierPriceList;
        let pricelist: Vec<SupplierPriceListItemDto> = match from_str::<SupplierPriceList>(&response_xml) {
            Ok(wrapper) => wrapper.items,
            Err(_) => {
                // Пробуем как список элементов без обертки
                match from_str::<Vec<SupplierPriceListItemDto>>(&response_xml) {
                    Ok(list) => list,
                    Err(_) => {
                        // Если не список, пробуем как один элемент
                        let item: SupplierPriceListItemDto = from_str(&response_xml)?;
                        vec![item]
                    }
                }
            }
        };
        Ok(pricelist)
    }
}
