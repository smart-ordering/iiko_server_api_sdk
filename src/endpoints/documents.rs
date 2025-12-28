use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::request::{DocumentsRequest, Request};
use crate::xml::response::{
    Document, DocumentValidationResult, IncomingInventoryDto, IncomingInventoryValidationResultDto,
    IncomingInvoiceDto, OutgoingInvoiceDto, OutgoingInvoiceDtoes, ReturnedInvoiceDto,
};
use quick_xml::{de::from_str, se::to_string};

pub struct DocumentsEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> DocumentsEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_documents(
        &self,
        store_id: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
    ) -> Result<Vec<Document>> {
        let request = Request::new(DocumentsRequest {
            store_id,
            date_from,
            date_to,
        });
        let xml_body = request.to_xml()?;
        
        let response_xml = self.client.post_xml("documents", &xml_body).await?;
        let documents: Vec<Document> = from_str(&response_xml)?;
        
        Ok(documents)
    }

    /// Импорт приходной накладной
    ///
    /// # Версия iiko: 3.9+ (редактирование с 5.2)
    /// # Endpoint: POST `/documents/import/incomingInvoice`
    ///
    /// # Параметры запроса:
    /// - `invoice`: Приходная накладная (IncomingInvoiceDto)
    ///
    /// # Формат даты:
    /// - `dateIncoming`: dd.MM.yyyy
    /// - `dueDate`: dd.MM.yyyy
    /// - `incomingDate`: yyyy-MM-dd (с версии 7.6.1)
    ///
    /// # Что в ответе:
    /// - Результат валидации документа (DocumentValidationResult)
    ///
    /// # Важно:
    /// - Content-Type: application/xml
    /// - В каждой позиции документа должно быть указано хотя бы одно из полей: `product` или `productArticle`
    /// - Если указан `defaultStore`, то в каждой позиции накладной нужно указать этот же склад
    pub async fn import_incoming_invoice(
        &self,
        invoice: IncomingInvoiceDto,
    ) -> Result<DocumentValidationResult> {
        // Сериализуем документ в XML
        let xml_body = to_string(&invoice)?;
        
        let response_xml = self
            .client
            .post_xml("documents/import/incomingInvoice", &xml_body)
            .await?;

        // Парсим XML ответ
        let result: DocumentValidationResult = from_str(&response_xml)?;
        Ok(result)
    }

    /// Импорт расходной накладной
    ///
    /// # Версия iiko: 4.4
    /// # Endpoint: POST `/documents/import/outgoingInvoice`
    ///
    /// # Параметры запроса:
    /// - `invoice`: Расходная накладная (OutgoingInvoiceDto)
    ///
    /// # Формат даты:
    /// - `dateIncoming`: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
    ///
    /// # Что в ответе:
    /// - Результат валидации документа (DocumentValidationResult)
    ///
    /// # Важно:
    /// - Content-Type: application/xml
    /// - При создании накладных с проведением обязателен склад (defaultStoreId или defaultStoreCode)
    /// - Склад заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
    /// - Если заполнен в документе, в бекофисе будет отмечена галочка "Отгрузить со склада"
    /// - В каждой позиции документа должно быть указано хотя бы одно из полей: `productId` или `productArticle`
    pub async fn import_outgoing_invoice(
        &self,
        invoice: OutgoingInvoiceDto,
    ) -> Result<DocumentValidationResult> {
        // Сериализуем документ в XML
        let xml_body = to_string(&invoice)?;
        
        let response_xml = self
            .client
            .post_xml("documents/import/outgoingInvoice", &xml_body)
            .await?;

        // Парсим XML ответ
        let result: DocumentValidationResult = from_str(&response_xml)?;
        Ok(result)
    }

    /// Импорт возвратной накладной
    ///
    /// # Версия iiko: 4.4
    /// # Endpoint: POST `/documents/import/returnedInvoice`
    ///
    /// # Параметры запроса:
    /// - `invoice`: Возвратная накладная (ReturnedInvoiceDto)
    ///
    /// # Формат даты:
    /// - `dateIncoming`: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
    /// - `incomingInvoiceDate`: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
    ///
    /// # Что в ответе:
    /// - Результат валидации документа (DocumentValidationResult)
    ///
    /// # Важно:
    /// - Content-Type: application/xml
    /// - `incomingInvoiceNumber` и `incomingInvoiceDate` - обязательные поля
    /// - При создании накладных с проведением обязателен склад (defaultStoreId или defaultStoreCode)
    /// - Склад заполняется либо в документе, либо в каждой строке отдельно, но не одновременно
    /// - В каждой позиции документа должно быть указано хотя бы одно из полей: `productId` или `productArticle`
    pub async fn import_returned_invoice(
        &self,
        invoice: ReturnedInvoiceDto,
    ) -> Result<DocumentValidationResult> {
        // Сериализуем документ в XML
        let xml_body = to_string(&invoice)?;
        
        let response_xml = self
            .client
            .post_xml("documents/import/returnedInvoice", &xml_body)
            .await?;

        // Парсим XML ответ
        let result: DocumentValidationResult = from_str(&response_xml)?;
        Ok(result)
    }

    /// Импорт инвентаризации
    ///
    /// # Версия iiko: 5.1
    /// # Endpoint: POST `/documents/import/incomingInventory`
    ///
    /// # Параметры запроса:
    /// - `inventory`: Инвентаризация (IncomingInventoryDto)
    ///
    /// # Формат даты:
    /// - `dateIncoming`: yyyy-MM-ddTHH:mm:ss или yyyy-MM-dd
    ///
    /// # Что в ответе:
    /// - Результат валидации документа инвентаризации (IncomingInventoryValidationResultDto)
    ///
    /// # Важно:
    /// - Content-Type: application/xml
    /// - Склад (storeId или storeCode) - обязателен для заполнения
    /// - Для одного элемента номенклатуры можно передавать несколько строк, но статус у них должен быть одинаковым
    pub async fn import_incoming_inventory(
        &self,
        inventory: IncomingInventoryDto,
    ) -> Result<IncomingInventoryValidationResultDto> {
        // Сериализуем документ в XML
        let xml_body = to_string(&inventory)?;
        
        let response_xml = self
            .client
            .post_xml("documents/import/incomingInventory", &xml_body)
            .await?;

        // Парсим XML ответ
        let result: IncomingInventoryValidationResultDto = from_str(&response_xml)?;
        Ok(result)
    }

    /// Распроведение приходной накладной
    ///
    /// # Версия iiko: 7.7
    /// # Endpoint: POST `/documents/unprocess/incomingInvoice`
    ///
    /// # Параметры запроса:
    /// - `invoice`: Приходная накладная (IncomingInvoiceDto)
    ///
    /// # Что в ответе:
    /// - Результат валидации документа (DocumentValidationResult)
    ///
    /// # Важно:
    /// - Content-Type: application/xml
    /// - Структура документа соответствует XSD Приходная накладная
    pub async fn unprocess_incoming_invoice(
        &self,
        invoice: IncomingInvoiceDto,
    ) -> Result<DocumentValidationResult> {
        // Сериализуем документ в XML
        let xml_body = to_string(&invoice)?;
        
        let response_xml = self
            .client
            .post_xml("documents/unprocess/incomingInvoice", &xml_body)
            .await?;

        // Парсим XML ответ
        let result: DocumentValidationResult = from_str(&response_xml)?;
        Ok(result)
    }

    /// Распроведение расходной накладной
    ///
    /// # Версия iiko: 7.7
    /// # Endpoint: POST `/documents/unprocess/outgoingInvoice`
    ///
    /// # Параметры запроса:
    /// - `invoice`: Расходная накладная (OutgoingInvoiceDto)
    ///
    /// # Что в ответе:
    /// - Результат валидации документа (DocumentValidationResult)
    ///
    /// # Важно:
    /// - Content-Type: application/xml
    /// - Структура документа соответствует XSD Расходная накладная
    pub async fn unprocess_outgoing_invoice(
        &self,
        invoice: OutgoingInvoiceDto,
    ) -> Result<DocumentValidationResult> {
        // Сериализуем документ в XML
        let xml_body = to_string(&invoice)?;
        
        let response_xml = self
            .client
            .post_xml("documents/unprocess/outgoingInvoice", &xml_body)
            .await?;

        // Парсим XML ответ
        let result: DocumentValidationResult = from_str(&response_xml)?;
        Ok(result)
    }

    /// Экспорт расходных накладных
    ///
    /// # Версия iiko: 5.4
    /// # Endpoint: GET `/documents/export/outgoingInvoice`
    ///
    /// # Параметры запроса:
    /// - `from`: Начальная дата в формате YYYY-MM-DD (входит в интервал)
    /// - `to`: Конечная дата в формате YYYY-MM-DD (входит в интервал, время не учитывается)
    /// - `supplier_id`: Опциональный UUID поставщика
    ///
    /// # Что в ответе:
    /// - Список расходных накладных (Vec<OutgoingInvoiceDto>)
    ///
    /// # Важно:
    /// - При запросе без поставщика возвращает все расходные накладные, попавшие в интервал
    /// - Формат даты: YYYY-MM-DD
    pub async fn export_outgoing_invoice(
        &self,
        from: String,
        to: String,
        supplier_id: Option<String>,
    ) -> Result<Vec<OutgoingInvoiceDto>> {
        // Используем get_with_params для передачи параметров
        let response_xml = if let Some(ref id) = supplier_id {
            self.client
                .get_with_params(
                    "documents/export/outgoingInvoice",
                    &[
                        ("from", from.as_str()),
                        ("to", to.as_str()),
                        ("supplierId", id.as_str()),
                    ],
                )
                .await?
        } else {
            self.client
                .get_with_params(
                    "documents/export/outgoingInvoice",
                    &[("from", from.as_str()), ("to", to.as_str())],
                )
                .await?
        };

        // Парсим XML ответ
        let result: OutgoingInvoiceDtoes = from_str(&response_xml)?;
        Ok(result.documents)
    }

    /// Экспорт расходных накладных по номеру документа
    ///
    /// # Версия iiko: 5.4
    /// # Endpoint: GET `/documents/export/outgoingInvoice/byNumber`
    ///
    /// # Параметры запроса:
    /// - `number`: Номер документа (String)
    /// - `current_year`: Только за текущий год (bool, обязательный)
    /// - `from`: Начальная дата в формате YYYY-MM-DD (опционально, только если current_year = false)
    /// - `to`: Конечная дата в формате YYYY-MM-DD (опционально, только если current_year = false)
    ///
    /// # Что в ответе:
    /// - Список расходных накладных (Vec<OutgoingInvoiceDto>)
    ///
    /// # Важно:
    /// - `current_year` - обязательный параметр
    /// - При `current_year = true`: возвращает документы с указанным номером только за текущий год, параметры `from` и `to` должны отсутствовать
    /// - При `current_year = false`: параметры `from` и `to` должны быть указаны
    /// - Формат даты: YYYY-MM-DD
    pub async fn export_outgoing_invoice_by_number(
        &self,
        number: String,
        current_year: bool,
        from: Option<String>,
        to: Option<String>,
    ) -> Result<Vec<OutgoingInvoiceDto>> {
        let mut params = vec![
            ("number", number.as_str()),
            ("currentYear", if current_year { "true" } else { "false" }),
        ];

        // Если current_year = false, добавляем from и to
        if !current_year {
            if let Some(ref from_date) = from {
                params.push(("from", from_date.as_str()));
            }
            if let Some(ref to_date) = to {
                params.push(("to", to_date.as_str()));
            }
        }

        let response_xml = self
            .client
            .get_with_params("documents/export/outgoingInvoice/byNumber", &params)
            .await?;

        // Парсим XML ответ
        let result: OutgoingInvoiceDtoes = from_str(&response_xml)?;
        Ok(result.documents)
    }
}

