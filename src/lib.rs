pub mod client;
pub mod config;
pub mod endpoints;
pub mod error;
pub mod xml;

pub use client::IikoClient;
pub use config::IikoConfig;
pub use error::{IikoError, Result};

// Re-export commonly used types
pub use xml::response::{
    AssemblyChartDto, AssemblyChartItemDto, BalanceCounteragent, BalanceStore, BarcodeDto,
    ChartResultDto, ChoiceBindingDto, ContainerDto, CorporateItemDto, CorporationSettings,
    BudgetPlanItemDto, BudgetPlanItemValueType, ColumnCaptions, DateDetalization,
    DateRangeFilter, DayDishValue, DeliveryConsolidatedReport, DeliveryConsolidatedRow,
    DeliveryCourierMetric, DeliveryCouriersReport, DeliveryCouriersRow,
    DeliveryHalfHourDetailedReport, DeliveryHalfHourDetailedRow, DeliveryHalfHourMetric,
    DeliveryLoyaltyRegion, DeliveryLoyaltyReport, DeliveryLoyaltyRow, DeliveryMetricType,
    DeliveryOrderCycleReport, DeliveryOrderCycleRow, DeliveryRegionsReport, DeliveryRegionsRow,
    DeliveryType, DocumentTypeList, IngredientEntryDto, KeyValue, StoreDataDirection,
    StoreDocumentType, StoreFilterList, StoreOperationsReportGrouping, StoreReportFilter,
    StoreReportItemDto, StoreReportPreset, StoreTransactionType, TransactionTypeList,
    DistributionAlgorithmType, Document, DocumentStatus, DocumentValidationResult,
    EgaisBRegDto, EgaisMarkStateDto, EgaisMarksList, EntityDto, EntityType, Event, EventAttribute,
    EventsList, FilterType, GroupDto, GroupServiceMode, GroupsList, IdCodeDto, IdCodeNameDto,
    IdListDto, ImageDto, ImageOperationResult, ImageSaveRequest, IncomingInventoryDto,
    IncomingInventoryItemDto, IncomingInventoryItems, IncomingInventoryValidationResultDto,
    IncomingInventoryValidationResultItemDto, IncomingInventoryValidationResultItems,
    IncomingInvoiceDto, IncomingInvoiceItemDto, IncomingInvoiceItems, InventoryItemStatus,
    OlapColumnInfo, OlapColumns, OlapFieldValue, OlapFilter, OlapReportRequest, OlapReportResponse, OlapReportType, OlapReportTypeV1,
    OrderServiceType, OutgoingInvoiceDto, OutgoingInvoiceItemDto, OutgoingInvoiceItems, PeriodType,
    PreparedChartDto, ReferenceEntity, ReferenceEntityDto,
    PreparedChartItemDto, ProductDto, ProductGroupDto, ProductOperationResult,
    ProductProductScaleRequest, ProductScaleDto, ProductScaleOperationResult,
    ProductScaleSaveRequest, ProductScaleUpdateRequest, ProductSizeAssemblyStrategy,
    ProductSizeDto, ProductSizeFactorDto, ProductSizeProductRequest, ProductSizeSaveDto,
    ProductType, ProductWriteoffStrategy, ProductsOperationResult, RangeFilter, ReplicationStatus,
    ReturnedInvoiceDto, ReturnedInvoiceItemDto, ReturnedInvoiceItems, ServerType,
    StoreSpecification, Supplier, SupplierContainerDto, SupplierPriceList,
    SupplierPriceListItemDto, Suppliers, TerminalDto, ValueFilter,
};

use endpoints::{
    AssemblyChartsEndpoint, AuthEndpoint, CorporationEndpoint, DocumentsEndpoint, EntitiesEndpoint,
    EventsEndpoint, ImagesEndpoint, InventoryEndpoint, ProductScalesEndpoint, ProductsEndpoint,
    ReplicationEndpoint, ReportsEndpoint, SuppliersEndpoint,
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

    pub fn product_scales(&self) -> ProductScalesEndpoint<'_> {
        ProductScalesEndpoint::new(self)
    }

    pub fn reports(&self) -> ReportsEndpoint<'_> {
        ReportsEndpoint::new(self)
    }

    pub fn entities(&self) -> EntitiesEndpoint<'_> {
        EntitiesEndpoint::new(self)
    }
}
