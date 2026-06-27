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
    InternalTransferDto, InternalTransferItemDto, InternalTransferListResult,
    InternalTransferOperationResult, InventoryItemStatus, KeyValue, KeyValueEntry, OlapColumnInfo,
    OlapColumns, OlapFieldValue, OlapFilter, OlapReportRequest, OlapReportResponse, OlapReportType,
    OlapReportTypeV1, OrderServiceType, OutgoingInvoiceDto, OutgoingInvoiceItemDto,
    OutgoingInvoiceItems, PeriodType, PreparedChartDto, PreparedChartItemDto, ProductDto,
    ProductGroupDto, ProductOperationResult, ProductProductScaleRequest, ProductScaleDto,
    ProductScaleOperationResult, ProductScaleSaveRequest, ProductScaleUpdateRequest,
    ProductSizeAssemblyStrategy, ProductSizeDto, ProductSizeFactorDto, ProductSizeProductRequest,
    ProductSizeSaveDto, ProductType, ProductWriteoffStrategy, ProductsOperationResult,
    PublicExternalData, RangeFilter, ReferenceEntity, ReferenceEntityDto, ReplicationStatus,
    ReturnedInvoiceDto, ReturnedInvoiceItemDto, ReturnedInvoiceItems, ServerType,
    StoreDataDirection, StoreDocumentType, StoreFilterList, StoreOperationsReportGrouping,
    StoreReportFilter, StoreReportItemDto, StoreReportPreset, StoreSpecification,
    StoreTransactionType, Supplier, SupplierContainerDto, SupplierPriceList,
    SupplierPriceListItemDto, Suppliers, TerminalDto, TransactionTypeList, ValueFilter,
};

use endpoints::{
    AssemblyChartsEndpoint, AuthEndpoint, CorporationEndpoint, DocumentsEndpoint,
    EmployeesEndpoint, EntitiesEndpoint, EventsEndpoint, ImagesEndpoint, InventoryEndpoint,
    ProductScalesEndpoint, ProductsEndpoint, ReplicationEndpoint, ReportsEndpoint,
    SuppliersEndpoint,
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
