use uuid::Uuid;

use crate::{
    client::IikoClient,
    endpoints::internal_support::{
        DEFAULT_INTERNAL_RESPONSE_BYTES, read_internal, unique_bounded_ids, uuid_items,
    },
    error::Result,
    xml::response::InternalReadResult,
};

const MAX_MODIFIER_IDS: usize = 50;

/// Focused recipe-graph edge queries not exposed by the public recipe endpoints.
pub struct InternalRecipeGraphEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalRecipeGraphEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Finds which members of `modifier_ids` contain `product_id` in their assembly chart.
    pub async fn get_modifiers_containing_product(
        &self,
        modifier_ids: &[Uuid],
        product_id: Uuid,
    ) -> Result<InternalReadResult> {
        let modifiers = unique_bounded_ids(modifier_ids, MAX_MODIFIER_IDS, "modifier")?;
        let request = format!(
            "<request><modifiers>{}</modifiers><product>{product_id}</product></request>",
            uuid_items(&modifiers)
        );
        read_internal(
            self.client,
            "v3/ProductsAssemblyInfoService.getModifiersContainingProduct",
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
    fn modifier_graph_requires_both_verified_arguments() {
        let product = uuid!("11111111-1111-1111-1111-111111111111");
        let modifiers = unique_bounded_ids(&[product], MAX_MODIFIER_IDS, "modifier").unwrap();
        assert_eq!(
            format!(
                "<request><modifiers>{}</modifiers><product>{product}</product></request>",
                uuid_items(&modifiers)
            ),
            "<request><modifiers><i>11111111-1111-1111-1111-111111111111</i></modifiers><product>11111111-1111-1111-1111-111111111111</product></request>"
        );
    }
}
