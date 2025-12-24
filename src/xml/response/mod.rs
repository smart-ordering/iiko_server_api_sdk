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
//! - `documents` - документы
//!
pub mod assembly_charts;
pub mod common;
pub mod corporation;
pub mod documents;
pub mod events;
pub mod images;
pub mod inventory;
pub mod product_scales;
pub mod products;
pub mod replication;
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

// Re-export documents types
pub use documents::{
    DistributionAlgorithmType, Document, DocumentStatus, DocumentValidationResult,
    IncomingInvoiceDto, IncomingInvoiceItemDto, IncomingInvoiceItems, OutgoingInvoiceDto,
    OutgoingInvoiceDtoes, OutgoingInvoiceItemDto, OutgoingInvoiceItems,
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
