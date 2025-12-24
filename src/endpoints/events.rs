use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::request::events::{EventsFilter, EventsRequestData, OrderNumsFilter};
use crate::xml::response::events::{CashSession, CashSessionsList, Event, EventsList, GroupsList};
use quick_xml::de::from_str;
use quick_xml::se::to_string;

pub struct EventsEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> EventsEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Получить список событий
    ///
    /// # Параметры
    /// - `from_time`: Время с которого запрашиваются события (yyyy-MM-ddTHH:mm:ss.SSS)
    /// - `to_time`: Время по которое запрашиваются события (yyyy-MM-ddTHH:mm:ss.SSS)
    /// - `from_rev`: Ревизия, с которой запрашиваются события
    pub async fn get_events(
        &self,
        from_time: Option<&str>,
        to_time: Option<&str>,
        from_rev: Option<i64>,
    ) -> Result<EventsList> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let rev_string;

        if let Some(ft) = from_time {
            params.push(("from_time", ft));
        }
        if let Some(tt) = to_time {
            params.push(("to_time", tt));
        }
        if let Some(rev) = from_rev {
            rev_string = rev.to_string();
            params.push(("from_rev", &rev_string));
        }

        let response_xml = self.client.get_with_params("events", &params).await?;

        let events_list: EventsList = from_str(&response_xml)?;
        Ok(events_list)
    }

    /// Получить список событий по фильтру событий и номеру заказа
    ///
    /// # Параметры
    /// - `event_types`: Список типов событий для фильтрации
    /// - `order_nums`: Список номеров заказов для фильтрации (опционально)
    pub async fn get_events_by_filter(
        &self,
        event_types: Vec<String>,
        order_nums: Option<Vec<String>>,
    ) -> Result<EventsList> {
        let request = EventsRequestData {
            events: EventsFilter { items: event_types },
            order_nums: order_nums.map(|nums| OrderNumsFilter { items: nums }),
        };

        let xml_body = to_string(&request)?;
        let response_xml = self.client.post_xml("events", &xml_body).await?;

        let events_list: EventsList = from_str(&response_xml)?;
        Ok(events_list)
    }

    /// Сохранить события
    ///
    /// # Параметры
    /// - `events`: Список событий для сохранения
    pub async fn add_events(&self, events: Vec<Event>) -> Result<EventsList> {
        let events_list = EventsList {
            events,
            revision: None,
        };

        let xml_body = to_string(&events_list)?;
        let response_xml = self.client.post_xml("events/add", &xml_body).await?;

        let result: EventsList = from_str(&response_xml)?;
        Ok(result)
    }

    /// Получить дерево событий (метаданные)
    pub async fn get_metadata(&self) -> Result<GroupsList> {
        let response_xml = self.client.get("events/metadata").await?;

        let groups_list: GroupsList = from_str(&response_xml)?;
        Ok(groups_list)
    }

    /// Получить дерево событий по фильтру
    ///
    /// # Параметры
    /// - `event_types`: Список типов событий для фильтрации
    pub async fn get_metadata_by_filter(&self, event_types: Vec<String>) -> Result<GroupsList> {
        let request = EventsRequestData {
            events: EventsFilter { items: event_types },
            order_nums: None,
        };

        let xml_body = to_string(&request)?;
        let response_xml = self.client.post_xml("events/metadata", &xml_body).await?;

        let groups_list: GroupsList = from_str(&response_xml)?;
        Ok(groups_list)
    }

    /// Получить информацию о кассовых сменах
    ///
    /// # Параметры
    /// - `from_time`: Время с которого запрашиваются данные (yyyy-MM-ddTHH:mm:ss.SSS)
    /// - `to_time`: Время по которое запрашиваются данные (yyyy-MM-ddTHH:mm:ss.SSS)
    pub async fn get_sessions(
        &self,
        from_time: Option<&str>,
        to_time: Option<&str>,
    ) -> Result<Vec<CashSession>> {
        let mut params = Vec::new();
        if let Some(ft) = from_time {
            params.push(("from_time", ft));
        }
        if let Some(tt) = to_time {
            params.push(("to_time", tt));
        }

        let response_xml = self
            .client
            .get_with_params("events/sessions", &params)
            .await?;

        let sessions_list: CashSessionsList = from_str(&response_xml)?;
        Ok(sessions_list.sessions)
    }
}
