pub mod client;
pub mod config;
pub mod endpoints;
pub mod error;
pub mod xml;

pub use client::IikoClient;
pub use config::IikoConfig;
pub use endpoints::{AnalyticalEntityKind, InternalDocumentKind};
pub use error::{IikoError, Result};

// Re-export commonly used types
pub use xml::response::{
    AssemblyChartDto, AssemblyChartItemDto, BalanceCounteragent, BalanceStore, BarcodeDto,
    BudgetPlanItemDto, BudgetPlanItemValueType, ChartResultDto, ChoiceBindingDto, ColumnCaptions,
    ContainerDto, CorporateItemDto, CorporationSettings, DateDetalization, DateRangeFilter,
    DayDishValue, DeliveryConsolidatedReport, DeliveryConsolidatedRow, DeliveryCourierMetric,
    DeliveryCouriersReport, DeliveryCouriersRow, DeliveryHalfHourDetailedReport,
    DeliveryHalfHourDetailedRow, DeliveryHalfHourMetric, DeliveryLoyaltyRegion,
    DeliveryLoyaltyReport, DeliveryLoyaltyRow, DeliveryMetricType, DeliveryOrderCycleReport,
    DeliveryOrderCycleRow, DeliveryRegionsReport, DeliveryRegionsRow, DeliveryType,
    DistributionAlgorithmType, Document, DocumentStatus, DocumentTypeList,
    DocumentValidationResult, EgaisBRegDto, EgaisMarkStateDto, EgaisMarksList, Employee, Employees,
    EntityDto, EntityType, Event, EventAttribute, EventsList, FilterType, GroupDto,
    GroupServiceMode, GroupsList, IdCodeDto, IdCodeNameDto, IdListDto, ImageDto,
    ImageOperationResult, ImageSaveRequest, IncomingInventoryDto, IncomingInventoryItemDto,
    IncomingInventoryItems, IncomingInventoryValidationResultDto,
    IncomingInventoryValidationResultItemDto, IncomingInventoryValidationResultItems,
    IncomingInvoiceDto, IncomingInvoiceItemDto, IncomingInvoiceItems, IngredientEntryDto,
    InternalReadResult, InternalTransferDto, InternalTransferItemDto, InternalTransferListResult,
    InternalTransferOperationResult, InternalXmlNode, InventoryItemStatus, KeyValue, KeyValueEntry,
    OlapColumnInfo, OlapColumns, OlapFieldValue, OlapFilter, OlapReportRequest, OlapReportResponse,
    OlapReportType, OlapReportTypeV1, OrderServiceType, OutgoingInvoiceDto, OutgoingInvoiceItemDto,
    OutgoingInvoiceItems, PeriodType, PreparedChartDto, PreparedChartItemDto, ProductDto,
    ProductGroupDto, ProductOperationResult, ProductProductScaleRequest, ProductScaleDto,
    ProductScaleOperationResult, ProductScaleSaveRequest, ProductScaleUpdateRequest,
    ProductSizeAssemblyStrategy, ProductSizeDto, ProductSizeFactorDto, ProductSizeProductRequest,
    ProductSizeSaveDto, ProductType, ProductWriteoffStrategy, ProductionOrderBlank,
    ProductionOrderBlankExcludedStores, ProductionOrderBlankItem, ProductionOrderBlankItems,
    ProductionOrderBlankTab, ProductionOrderBlankTabs, ProductsOperationResult, PublicExternalData,
    RangeFilter, ReferenceEntity, ReferenceEntityDto, ReplicationStatus, ReturnedInvoiceDto,
    ReturnedInvoiceItemDto, ReturnedInvoiceItems, ServerType, StoreDataDirection,
    StoreDocumentType, StoreFilterList, StoreOperationsReportGrouping, StoreReportFilter,
    StoreReportItemDto, StoreReportPreset, StoreSpecification, StoreTransactionType, Supplier,
    SupplierContainerDto, SupplierPriceList, SupplierPriceListItemDto, Suppliers, TerminalDto,
    TransactionTypeList, ValueFilter,
};

use endpoints::{
    AssemblyChartsEndpoint, AuthEndpoint, CorporationEndpoint, DocumentsEndpoint,
    EmployeesEndpoint, EntitiesEndpoint, EventsEndpoint, ImagesEndpoint,
    InternalCashSessionsEndpoint, InternalCorporationEndpoint, InternalCostHistoryEndpoint,
    InternalDocumentIndexEndpoint, InternalDocumentsEndpoint, InternalEntityChangesEndpoint,
    InternalHistoricalStockEndpoint, InternalLineSalesEndpoint, InternalPriceHistoryEndpoint,
    InternalProductionTraceEndpoint, InternalRecipeGraphEndpoint, InternalSalesEventsEndpoint,
    InternalStockMovementsEndpoint, InternalSupplierHistoryEndpoint, InventoryEndpoint,
    ProductScalesEndpoint, ProductionOrderBlanksEndpoint, ProductsEndpoint, ReplicationEndpoint,
    ReportsEndpoint, SuppliersEndpoint,
};

impl IikoClient {
    pub fn auth(&self) -> AuthEndpoint<'_> {
        AuthEndpoint::new(self)
    }

    pub fn inventory(&self) -> InventoryEndpoint<'_> {
        InventoryEndpoint::new(self)
    }

    pub fn suppliers(&self) -> SuppliersEndpoint<'_> {
        SuppliersEndpoint::new(self)
    }

    pub fn employees(&self) -> EmployeesEndpoint<'_> {
        EmployeesEndpoint::new(self)
    }

    pub fn documents(&self) -> DocumentsEndpoint<'_> {
        DocumentsEndpoint::new(self)
    }

    pub fn corporation(&self) -> CorporationEndpoint<'_> {
        CorporationEndpoint::new(self)
    }

    pub fn replication(&self) -> ReplicationEndpoint<'_> {
        ReplicationEndpoint::new(self)
    }

    pub fn events(&self) -> EventsEndpoint<'_> {
        EventsEndpoint::new(self)
    }

    pub fn products(&self) -> ProductsEndpoint<'_> {
        ProductsEndpoint::new(self)
    }

    pub fn assembly_charts(&self) -> AssemblyChartsEndpoint<'_> {
        AssemblyChartsEndpoint::new(self)
    }

    pub fn images(&self) -> ImagesEndpoint<'_> {
        ImagesEndpoint::new(self)
    }

    /// Allowlisted internal document read models. This is not a generic v3 proxy.
    pub fn internal_documents(&self) -> InternalDocumentsEndpoint<'_> {
        InternalDocumentsEndpoint::new(self)
    }

    /// Allowlisted historical stock read models with bounded date inputs.
    pub fn internal_historical_stock(&self) -> InternalHistoricalStockEndpoint<'_> {
        InternalHistoricalStockEndpoint::new(self)
    }

    /// Allowlisted line-level sales reads by stable identifiers only.
    pub fn internal_line_sales(&self) -> InternalLineSalesEndpoint<'_> {
        InternalLineSalesEndpoint::new(self)
    }

    pub fn internal_document_index(&self) -> InternalDocumentIndexEndpoint<'_> {
        InternalDocumentIndexEndpoint::new(self)
    }

    pub fn internal_stock_movements(&self) -> InternalStockMovementsEndpoint<'_> {
        InternalStockMovementsEndpoint::new(self)
    }

    pub fn internal_sales_events(&self) -> InternalSalesEventsEndpoint<'_> {
        InternalSalesEventsEndpoint::new(self)
    }

    pub fn internal_cash_sessions(&self) -> InternalCashSessionsEndpoint<'_> {
        InternalCashSessionsEndpoint::new(self)
    }

    /// Read-only identity of the corporation hierarchy served by this RMS node.
    pub fn internal_corporation(&self) -> InternalCorporationEndpoint<'_> {
        InternalCorporationEndpoint::new(self)
    }

    pub fn internal_cost_history(&self) -> InternalCostHistoryEndpoint<'_> {
        InternalCostHistoryEndpoint::new(self)
    }

    pub fn internal_recipe_graph(&self) -> InternalRecipeGraphEndpoint<'_> {
        InternalRecipeGraphEndpoint::new(self)
    }

    pub fn internal_price_history(&self) -> InternalPriceHistoryEndpoint<'_> {
        InternalPriceHistoryEndpoint::new(self)
    }

    pub fn internal_supplier_history(&self) -> InternalSupplierHistoryEndpoint<'_> {
        InternalSupplierHistoryEndpoint::new(self)
    }

    pub fn internal_production_trace(&self) -> InternalProductionTraceEndpoint<'_> {
        InternalProductionTraceEndpoint::new(self)
    }

    pub fn internal_entity_changes(&self) -> InternalEntityChangesEndpoint<'_> {
        InternalEntityChangesEndpoint::new(self)
    }

    pub fn product_scales(&self) -> ProductScalesEndpoint<'_> {
        ProductScalesEndpoint::new(self)
    }

    pub fn production_order_blanks(&self) -> ProductionOrderBlanksEndpoint<'_> {
        ProductionOrderBlanksEndpoint::new(self)
    }

    pub fn reports(&self) -> ReportsEndpoint<'_> {
        ReportsEndpoint::new(self)
    }

    pub fn entities(&self) -> EntitiesEndpoint<'_> {
        EntitiesEndpoint::new(self)
    }
}
