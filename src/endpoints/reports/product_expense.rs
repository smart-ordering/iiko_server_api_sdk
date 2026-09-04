use super::ReportsEndpoint;
use crate::error::Result;
use crate::xml::response::reports::{DayDishValue, DayDishValues};
use quick_xml::de::from_str;

// A slow but valid one-day production response took 128s. Bound only this
// read report; document writes and all other endpoint deadlines stay unchanged.
const PRODUCT_EXPENSE_RESPONSE_BYTES: usize = 64 * 1024 * 1024;

impl ReportsEndpoint<'_> {
    /// Расход продуктов по продажам
    ///
    /// Версия iiko: 3.9
    /// Endpoint: GET `/reports/productExpense`
    ///
    /// # Параметры запроса
    /// - `department`: Подразделение (GUID)
    /// - `date_from`: Начальная дата в формате DD.MM.YYYY
    /// - `date_to`: Конечная дата в формате DD.MM.YYYY
    /// - `hour_from`: Час начала интервала выборки в сутках (по умолчанию -1, все время)
    /// - `hour_to`: Час окончания интервала выборки в сутках (по умолчанию -1, все время)
    ///
    /// # Что в ответе
    /// Структура dayDishValue (см. XSD Расход продуктов по продажам)
    /// - `date`: Дата
    /// - `productId`: ID продукта
    /// - `productName`: Название продукта
    /// - `value`: Значение (количество) в формате decimal
    pub async fn get_product_expense(
        &self,
        department: &str,
        date_from: &str,
        date_to: &str,
        hour_from: Option<i32>,
        hour_to: Option<i32>,
    ) -> Result<Vec<DayDishValue>> {
        let mut params = vec![
            ("department", department),
            ("dateFrom", date_from),
            ("dateTo", date_to),
        ];

        let hour_from_str;
        let hour_to_str;

        if let Some(hf) = hour_from {
            hour_from_str = hf.to_string();
            params.push(("hourFrom", &hour_from_str));
        }

        if let Some(ht) = hour_to {
            hour_to_str = ht.to_string();
            params.push(("hourTo", &hour_to_str));
        }

        let response_xml = self
            .client
            .get_readonly_bounded_with_timeout(
                "reports/productExpense",
                &params,
                PRODUCT_EXPENSE_RESPONSE_BYTES,
                Some(std::time::Duration::from_secs(300)),
            )
            .await?;

        // XML может быть:
        // - списком элементов внутри обертки <dayDishValues>
        // - списком элементов без обертки
        // - одним элементом <dayDishValue>
        let items: Vec<DayDishValue> =
            // Пытаемся сначала распарсить обертку <dayDishValues>...</dayDishValues>
            if let Ok(wrapper) = from_str::<DayDishValues>(&response_xml) {
                wrapper.items
            } else if let Ok(list) = from_str::<Vec<DayDishValue>>(&response_xml) {
                // Падаем обратно на "голый" список элементов
                list
            } else {
                // И в самом крайнем случае пробуем один элемент
                let item: DayDishValue = from_str(&response_xml)?;
                vec![item]
            };
        eprintln!(
            "iiko productExpense response: date_from={}, date_to={}, rows={}, response_bytes={}",
            date_from,
            date_to,
            items.len(),
            response_xml.len(),
        );
        Ok(items)
    }
}
