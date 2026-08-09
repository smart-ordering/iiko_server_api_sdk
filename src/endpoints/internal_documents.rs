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

/// Allowlisted, read-only access to undocumented iiko document read models.
pub struct InternalDocumentsEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalDocumentsEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_abstract_document(&self, document_id: Uuid) -> Result<InternalReadResult> {
        self.read(
            "v3/DocumentService.getAbstractDocument",
            &format!("<request><id>{document_id}</id></request>"),
        )
        .await
    }

    pub async fn get_abstract_documents(
        &self,
        document_ids: &[Uuid],
    ) -> Result<InternalReadResult> {
        let document_ids = unique_bounded_ids(document_ids, MAX_DOCUMENT_IDS, "document")?;
        let ids = uuid_items(&document_ids);
        self.read(
            "v3/DocumentService.getAbstractDocuments",
            &format!("<request><documentIds>{ids}</documentIds></request>"),
        )
        .await
    }

    pub async fn get_document_item_costs(&self, document_id: Uuid) -> Result<InternalReadResult> {
        self.read(
            "v3/DocumentService.getDocumentItemsCosts",
            &document_id_request(document_id),
        )
        .await
    }

    /// Available only for document classes supported by iiko's pricing service.
    pub async fn get_document_item_pricing(&self, document_id: Uuid) -> Result<InternalReadResult> {
        self.read(
            "v3/DocumentService.getDocumentItemsPricing",
            &document_id_request(document_id),
        )
        .await
    }

    pub async fn get_document_transactions(&self, document_id: Uuid) -> Result<InternalReadResult> {
        self.read(
            "v3/DocumentService.getDocumentTransactions",
            &document_id_request(document_id),
        )
        .await
    }

    async fn read(&self, endpoint: &str, request: &str) -> Result<InternalReadResult> {
        read_internal(
            self.client,
            endpoint,
            request,
            DEFAULT_INTERNAL_RESPONSE_BYTES,
        )
        .await
    }
}

fn document_id_request(document_id: Uuid) -> String {
    format!("<request><documentId>{document_id}</documentId></request>")
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    #[test]
    fn document_cost_request_uses_verified_document_id_argument() {
        assert_eq!(
            document_id_request(uuid!("11111111-1111-1111-1111-111111111111")),
            "<request><documentId>11111111-1111-1111-1111-111111111111</documentId></request>"
        );
    }

    #[test]
    fn document_batch_rejects_empty_and_oversized_inputs() {
        assert!(unique_bounded_ids(&[], MAX_DOCUMENT_IDS, "document").is_err());
        let ids = (0..=MAX_DOCUMENT_IDS)
            .map(|index| Uuid::from_u128(index as u128 + 1))
            .collect::<Vec<_>>();
        assert!(unique_bounded_ids(&ids, MAX_DOCUMENT_IDS, "document").is_err());
    }

    #[test]
    fn document_batch_deduplicates_ids_before_enforcing_limit() {
        let id = uuid!("11111111-1111-1111-1111-111111111111");
        assert_eq!(
            unique_bounded_ids(&[id, id], MAX_DOCUMENT_IDS, "document").unwrap(),
            vec![id]
        );
    }
}
