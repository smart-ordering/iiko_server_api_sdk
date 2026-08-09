use uuid::Uuid;

use crate::{
    client::IikoClient,
    endpoints::internal_support::{
        DEFAULT_INTERNAL_RESPONSE_BYTES, read_internal, unique_bounded_ids, uuid_items,
    },
    error::Result,
    xml::response::InternalReadResult,
};

const MAX_DOCUMENT_IDS: usize = 50;

/// Document classes whose internal wire value was positively verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum InternalDocumentKind {
    IncomingInvoice,
}

impl InternalDocumentKind {
    fn as_wire_value(self) -> &'static str {
        match self {
            Self::IncomingInvoice => "INCOMING_INVOICE",
        }
    }
}

/// Bounded document record reads for building analytical document indexes.
pub struct InternalDocumentIndexEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalDocumentIndexEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_documents_by_ids(
        &self,
        kind: InternalDocumentKind,
        document_ids: &[Uuid],
    ) -> Result<InternalReadResult> {
        self.read_ids("v3/DocumentService.getDocuments", kind, document_ids)
            .await
    }

    pub async fn get_incoming_records_by_ids(
        &self,
        kind: InternalDocumentKind,
        document_ids: &[Uuid],
    ) -> Result<InternalReadResult> {
        self.read_ids(
            "v3/DocumentService.getIncomingDocumentsRecordsByIds",
            kind,
            document_ids,
        )
        .await
    }

    async fn read_ids(
        &self,
        endpoint: &str,
        kind: InternalDocumentKind,
        document_ids: &[Uuid],
    ) -> Result<InternalReadResult> {
        let ids = unique_bounded_ids(document_ids, MAX_DOCUMENT_IDS, "document")?;
        let request = format!(
            "<request><docType>{}</docType><ids>{}</ids></request>",
            kind.as_wire_value(),
            uuid_items(&ids)
        );
        read_internal(
            self.client,
            endpoint,
            &request,
            DEFAULT_INTERNAL_RESPONSE_BYTES,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    #[test]
    fn incoming_invoice_uses_verified_enum_and_collection_shape() {
        let ids = unique_bounded_ids(
            &[uuid!("11111111-1111-1111-1111-111111111111")],
            MAX_DOCUMENT_IDS,
            "document",
        )
        .unwrap();
        assert_eq!(
            format!(
                "<request><docType>{}</docType><ids>{}</ids></request>",
                InternalDocumentKind::IncomingInvoice.as_wire_value(),
                uuid_items(&ids)
            ),
            "<request><docType>INCOMING_INVOICE</docType><ids><i>11111111-1111-1111-1111-111111111111</i></ids></request>"
        );
    }
}
