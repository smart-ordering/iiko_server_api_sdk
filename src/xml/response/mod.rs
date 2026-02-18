//! - `common` - общие структуры (IdName, Response)
//! - `corporation` - корпоративные данные (подразделения, группы, терминалы, настройки)
//! - `replication` - репликация (статусы, тип сервера)
//! - `events` - события (журнал событий, метаданные, кассовые смены)
//! - `products` - номенклатура (продукты, группы продуктов, операции CRUD)
//! - `assembly_charts` - технологические карты (рецепты, ингредиенты, списания)
//! - `images` - изображения (загрузка, сохранение, удаление)
//! - `product_scales` - шкалы размеров продуктов
//! - `inventory` - инвентаризация
//! - `suppliers` - поставщики
//! - `employees` - сотрудники
//! - `documents` - документы
//! - `reports` - отчеты (балансы, остатки, ЕГАИС)
//! - `entities` - справочники (типы оплат, скидок, категории и т.д.)
//!
pub mod assembly_charts;
pub mod common;
pub mod corporation;
pub mod documents;
pub mod employees;
pub mod entities;
pub mod events;
pub mod images;
pub mod inventory;
pub mod product_scales;
pub mod products;
pub mod replication;
pub mod reports;
pub mod suppliers;

// Re-export common types
pub use common::{IdName, Response};

// Re-export corporation types
pub use corporation::{
    CorporateItemDto, CorporateItemDtoes, CorporationSettings, DepartmentType, GroupDto,
    GroupDtoes, GroupServiceMode, JurPersonAdditionalPropertiesDto, LegalAddressDto,
    OfficialEmployeeDto, OfficialEmployees, PointOfSaleDto, PointOfSaleDtoes, RestaurantSectionIds,
    RestaurantSectionInfos, TerminalDto, TerminalDtoes,
};

// Re-export replication types
pub use replication::{ReplicationStatus, ReplicationStatuses, ServerType};

// Re-export inventory types
pub use inventory::InventoryItem;

// Re-export suppliers types
pub use suppliers::{
    Supplier, SupplierContainerDto, SupplierPriceList, SupplierPriceListItemDto, Suppliers,
};

// Re-export employees types
pub use employees::{
    Employee, Employees, KeyValueEntry, PublicExternalData,
};

// Re-export documents types
pub use documents::{
    DistributionAlgorithmType, Document, DocumentStatus, DocumentValidationResult,
    IdCodeNameDto, IncomingInventoryDto, IncomingInventoryItemDto, IncomingInventoryItems,
    IncomingInventoryValidationResultDto, IncomingInventoryValidationResultItemDto,
    IncomingInventoryValidationResultItems, IncomingInvoiceDto, IncomingInvoiceItemDto,
    IncomingInvoiceItems, InventoryItemStatus, OutgoingInvoiceDto, OutgoingInvoiceDtoes,
    OutgoingInvoiceItemDto, OutgoingInvoiceItems, ReturnedInvoiceDto, ReturnedInvoiceItemDto,
    ReturnedInvoiceItems,
};

// Re-export events types
pub use events::{
    CashSession, CashSessionsList, Event, EventAttribute, EventGroup, EventType,
    EventTypeAttribute, EventsList, GroupsList,
};

// Re-export products types
pub use products::{
    BarcodeDto, CategoryDeleteRequest, CategoryOperationResult, CategoryRestoreRequest,
    CategorySaveRequest, CategoryUpdateRequest, ChoiceBindingDto, Color, ContainerDto, EntityDto,
    ErrorDto, IdCodeDto, ItemsRequest, OperationResult, ProductDto, ProductGroupDto,
    ProductOperationResult, ProductType, ProductsOperationResult,
};

// Re-export assembly charts types
pub use assembly_charts::{
    AssemblyChartDto, AssemblyChartItemDto, AssemblyChartOperationResult, ChartResultDto,
    PreparedChartDto, PreparedChartItemDto, ProductSizeAssemblyStrategy, ProductWriteoffStrategy,
    StoreSpecification,
};

// Re-export images types
pub use images::{IdListDto, ImageDto, ImageOperationResult, ImageSaveRequest};

// Re-export product scales types
pub use product_scales::{
    ProductProductScaleRequest, ProductScaleDto, ProductScaleOperationResult,
    ProductScaleSaveRequest, ProductScaleUpdateRequest, ProductSizeDto, ProductSizeFactorDto,
    ProductSizeProductRequest, ProductSizeSaveDto,
};

// Re-export reports types
pub use reports::{
    BalanceCounteragent, BalanceStore, BudgetPlanItemDto, BudgetPlanItemValueType,
    ColumnCaptions, DateDetalization, DateRangeFilter, DayDishValue, DeliveryConsolidatedReport,
    DeliveryConsolidatedRow, DeliveryConsolidatedRows, DeliveryCourierMetric,
    DeliveryCourierMetrics, DeliveryCouriersReport, DeliveryCouriersRow, DeliveryCouriersRows,
    DeliveryHalfHourDetailedReport, DeliveryHalfHourDetailedRow, DeliveryHalfHourDetailedRows,
    DeliveryHalfHourMetric, DeliveryHalfHourMetrics, DeliveryLoyaltyRegion,
    DeliveryLoyaltyRegions, DeliveryLoyaltyReport, DeliveryLoyaltyRow, DeliveryLoyaltyRows,
    DeliveryMetricType, DeliveryOrderCycleReport, DeliveryOrderCycleRow,
    DeliveryOrderCycleRows, DeliveryRegionsReport, DeliveryRegionsRow, DeliveryRegionsRows,
    DeliveryType, DocumentTypeList, EgaisBRegDto, EgaisMarkStateDto, EgaisMarksList, FilterType,
    IngredientEntryDto, KeyValue, OlapColumnInfo, OlapColumns, OlapFieldValue, OlapFilter,
    OlapReportRequest, OlapReportResponse, OlapReportType, OlapReportTypeV1, PeriodType,
    RangeFilter, StoreDataDirection, StoreDocumentType, StoreFilterList,
    StoreOperationsReportGrouping, StoreReportFilter, StoreReportItemDto, StoreReportPreset,
    StoreTransactionType, TransactionTypeList, ValueFilter,
};

// Re-export entities types
pub use entities::{
    EntityType, OrderServiceType, OrderTypeEntityDto, ProductSizeEntityDto,
    ReferenceEntity, ReferenceEntityDto, TaxCategoryEntityDto,
};
