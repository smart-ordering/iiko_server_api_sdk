pub mod request;
pub mod response;

// Re-export request types
pub use request::{DocumentsRequest, InventoryRequest, Request, SuppliersRequest};
// Re-export response types
pub use response::{
    CashSession, CashSessionsList, CorporateItemDto, CorporateItemDtoes, CorporationSettings,
    Document, Event, EventAttribute, EventGroup, EventType, EventTypeAttribute, EventsList,
    GroupDto, GroupDtoes, GroupsList, IdName, InventoryItem, ReplicationStatus,
    ReplicationStatuses, Response, ServerType, Supplier, TerminalDto, TerminalDtoes,
};

