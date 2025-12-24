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
    AssemblyChartDto, AssemblyChartItemDto, BarcodeDto, ChartResultDto, ChoiceBindingDto,
    ContainerDto, CorporateItemDto, CorporationSettings, DistributionAlgorithmType, Document,
    DocumentStatus, DocumentValidationResult, EntityDto, Event, EventAttribute, EventsList,
    GroupDto, GroupServiceMode, GroupsList, IdCodeDto, IdListDto, ImageDto, ImageOperationResult,
    ImageSaveRequest, IncomingInvoiceDto, IncomingInvoiceItemDto, IncomingInvoiceItems,
    OutgoingInvoiceDto, OutgoingInvoiceItemDto, OutgoingInvoiceItems, PreparedChartDto,
    PreparedChartItemDto, ProductDto, ProductGroupDto, ProductOperationResult,
    ProductProductScaleRequest, ProductScaleDto, ProductScaleOperationResult,
    ProductScaleSaveRequest, ProductScaleUpdateRequest, ProductSizeAssemblyStrategy,
    ProductSizeDto, ProductSizeFactorDto, ProductSizeProductRequest, ProductSizeSaveDto,
    ProductType, ProductWriteoffStrategy, ProductsOperationResult, ReplicationStatus, ServerType,
    StoreSpecification, Supplier, SupplierContainerDto, SupplierPriceList,
    SupplierPriceListItemDto, Suppliers, TerminalDto,
};

use endpoints::{
    AssemblyChartsEndpoint, AuthEndpoint, CorporationEndpoint, DocumentsEndpoint, EventsEndpoint,
    ImagesEndpoint, InventoryEndpoint, ProductScalesEndpoint, ProductsEndpoint,
    ReplicationEndpoint, SuppliersEndpoint,
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
}
