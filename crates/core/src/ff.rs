pub const UNIT: u8 = 0;
pub const INTEGER: u8 = 1;
pub const FLOAT: u8 = 2;
pub const CHARACTER: u8 = 3;
pub const DURATION: u8 = 4;
pub const DATE_TIME: u8 = 5;
pub const OBJECT_REFERENCE: u8 = 6;
pub const SCHEMA: u8 = 7;
pub const LANGUAGE: u8 = 8;
pub const URL: u8 = 9;
pub const URL_MAX: u8 = 10;
pub const URL_REFERENCE: u8 = 11;
pub const COLOR: u8 = 12;
pub const EMAIL: u8 = 16;
pub const EMAIL_MAX: u8 = 17;
pub const EMAIL_REFERENCE: u8 = 18;
pub const TEXT: u8 = 19;
pub const TEXT_MAX: u8 = 20;
pub const TEXT_REFERENCE: u8 = 21;
pub const BINARY: u8 = 22;
pub const BINARY_MAX: u8 = 23;
pub const BINARY_REFERENCE: u8 = 24;
pub const ENCRYPTED: u8 = 25;

// ------------------ Pages ---------------------

/// A page in a free list, ready to be used.
pub const PAGE_KIND_FREE_LIST: u8 = 0;

/// A page in a value spill.
pub const PAGE_KIND_SPILL: u8 = 1;

/// B-tree node of the root tree `ObjectId -> (TagId -> Value, Value -> TagId)`.
pub const PAGE_KIND_B_TREE_NODE_O_TV: u8 = 2;

/// B-tree node of `TagId -> Value` (child of [PAGE_KIND_B_TREE_NODE_O_TV]).
pub const PAGE_KIND_B_TREE_NODE_T_V: u8 = 3;

/// B-tree node of `Value -> TagId` (child of [PAGE_KIND_B_TREE_NODE_O_TV]).
pub const PAGE_KIND_B_TREE_NODE_V_T: u8 = 4;

/// B-tree node of the root tree `TagId -> (ObjectId, Value -> ObjectId)`.
pub const PAGE_KIND_B_TREE_NODE_T_OV: u8 = 5;

/// B-tree node of `Value -> ObjectId` (child of [PAGE_KIND_B_TREE_NODE_T_OV]).
pub const PAGE_KIND_B_TREE_NODE_V_O: u8 = 6;

/// B-tree node of the root tree `Value -> (ObjectId, TagId)`.
pub const PAGE_KIND_B_TREE_NODE_V_OT: u8 = 7;

/// Page of the write-ahead-log.
pub const PAGE_KIND_WAL: u8 = 8;
