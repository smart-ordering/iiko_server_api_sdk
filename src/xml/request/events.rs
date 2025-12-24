use serde::Serialize;

/// Запрос событий по фильтру
#[derive(Debug, Serialize)]
#[serde(rename = "eventsRequestData")]
pub struct EventsRequestData {
    #[serde(rename = "events")]
    pub events: EventsFilter,
    #[serde(rename = "orderNums", default)]
    pub order_nums: Option<OrderNumsFilter>,
}

/// Фильтр событий
#[derive(Debug, Serialize)]
pub struct EventsFilter {
    #[serde(rename = "event", default)]
    pub items: Vec<String>,
}

/// Фильтр номеров заказов
#[derive(Debug, Serialize)]
pub struct OrderNumsFilter {
    #[serde(rename = "orderNum", default)]
    pub items: Vec<String>,
}
