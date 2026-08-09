# Internal analytics reads

These modules expose a compile-time allowlist over iiko's undocumented XML `API_V3` surface.
They do not expose an arbitrary service name or raw request body.

## Public modules

| SDK accessor | Internal iiko method | Input bound | QI Tech smoke |
|---|---|---:|---|
| `internal_documents().get_abstract_document` | `DocumentService.getAbstractDocument` | one UUID | success |
| `internal_documents().get_abstract_documents` | `DocumentService.getAbstractDocuments` | 1-50 unique UUIDs | success |
| `internal_documents().get_document_item_costs` | `DocumentService.getDocumentItemsCosts` | one UUID | success |
| `internal_documents().get_document_item_pricing` | `DocumentService.getDocumentItemsPricing` | one UUID and supported document class | request contract verified; the QI Tech incoming invoice class is not supported by iiko pricing |
| `internal_documents().get_document_transactions` | `DocumentService.getDocumentTransactions` | one UUID | success |
| `internal_historical_stock().get_product_balances_for_date` | `StoreService.getProductsBalanceForDate` | one valid calendar date | success |
| `internal_line_sales().get_past_order` | `PastOrdersService.getPastOrdersById` | one UUID | success |
| `internal_line_sales().get_item_sale_event` | `PastOrdersService.getItemSaleEventsById` | one UUID | argument contract verified |
| `internal_document_index().get_documents_by_ids` | `DocumentService.getDocuments` | verified document enum + 1-50 UUIDs | success |
| `internal_document_index().get_incoming_records_by_ids` | `DocumentService.getIncomingDocumentsRecordsByIds` | verified document enum + 1-50 UUIDs | success |
| `internal_stock_movements().get_product_usage` | `StoreService.getProductUsagesByProduct` | one date + one product UUID | success |
| `internal_sales_events().get_item_sale_event` | `PastOrdersService.getItemSaleEventsById` | one UUID | argument contract verified |
| `internal_cash_sessions().get_transactions` | `SessionsService.getSessionTransactions` | one session UUID | success |
| `internal_cost_history().get_last_costs_by_stores` | `StoreService.getLastProductCostsByStores` | 1-20 stores + one timestamp | success |
| `internal_recipe_graph().get_modifiers_containing_product` | `ProductsAssemblyInfoService.getModifiersContainingProduct` | 1-50 modifier UUIDs + one product UUID | success |
| `internal_price_history().get_department_snapshot` | `ProductsService.getPriceListItemsByDepartment` | one department + one timestamp | success |
| `internal_supplier_history().get_pricelist_on_date` | stable `suppliers/{code}/pricelist` fallback | validated supplier code + one date | success, empty QI Tech result |
| `internal_production_trace().get_order_definition_ids` | stable `v2/entities/ProductionOrderBlank/ids` index | optional revision cursor | success, QI Tech has no active definitions |
| `internal_production_trace().get_order_definitions` | `EntitiesService.getEntitiesByIds` | 1-50 production-order UUIDs | contract verified; QI Tech has no ID for a positive detail read |
| `internal_entity_changes().get_changes` | stable `v2/entities/list` revision fallback | 1-8 safe kinds + verified non-negative 32-bit revision | success, empty future-cursor result |

Internal XML methods and the two stable fallback readers have a 4 MiB response limit.
Authentication retry is allowed once only because these methods are read-only.
`ServerResult.status` is validated even when HTTP returns success.

The module name describes the analytical capability, not a promise that every route is hidden.
For supplier history and reference-entity changes the internal Java argument type was not proven,
so the SDK deliberately uses the narrower stable read route. Production trace exposes verified
order definitions, not the still-unverified linked-document graph.

## Response contract

Undocumented response fields are represented as `InternalReadResult` and `InternalXmlNode`.
The tree preserves:

- element name;
- XML attributes such as `cls`, `eid`, and `null`;
- optional text;
- ordered and repeated child elements.

This is intentionally weak output typing. Request method, argument names, UUID/date types, batch
size and response size remain strongly bounded.

## Deliberately unavailable

- arbitrary `API_V3` method calls or raw XML bodies;
- `PastOrdersService.getPastOrdersByCloseTimeInterval`, because QI Tech silently ignored the
  guessed interval fields and returned the full set;
- cost-price pair/product methods until their collection wire contract is positively verified;
- `ProductsAssemblyChartService.getAssemblyChartsTree`; its internal date/key type is unresolved,
  while the SDK already has the stable typed `assembly_charts().get_tree` route;
- `SupplierPriceListService.getBySuppliersAndDate`; plausible UUID/entity collection shapes fail
  on QI Tech, so `internal_supplier_history` uses the stable supplier-code route;
- `ProductionService.getLinkedDocumentsByProductionOrders`; QI Tech has no active production-order
  definition with which to prove the detail contract;
- internal entity revision methods until their Java cursor/filter request object is proven;
- employee, payroll, customer, delivery-address, location, finance, license, backup and system
  services.

Line-level sales can contain personal or free-text data. A future assistant capability must add
server-side scope checks, field redaction, output budgets and per-attempt audit before model access.
The SDK result must not be registered directly as a model tool.

## QI Tech smoke

`examples/internal_analytics_read_smoke.rs` requires explicit `IIKO_SMOKE_*` variables, checks the
exact expected base URL before authentication, prints only response shapes, and logs out even when
a read fails. It never loads the repository `.env`. In addition to the original document, order and
date values, the expanded smoke requires product, store and department UUIDs plus a validated
supplier code. A past-order session and item ID are derived only from the already bounded order
response; they are not accepted as arbitrary XML.
