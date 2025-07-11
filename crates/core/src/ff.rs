//! File format constants.

// region Values

pub const VALUE_UNIT: u8 = 0;
pub const VALUE_INTEGER: u8 = 1;
pub const VALUE_FLOAT: u8 = 2;
pub const VALUE_CHARACTER: u8 = 3;
pub const VALUE_DURATION: u8 = 4;
pub const VALUE_DATE_TIME: u8 = 5;
pub const VALUE_OBJECT_REFERENCE: u8 = 6;
pub const VALUE_SCHEMA: u8 = 7;
pub const VALUE_LANGUAGE: u8 = 8;
pub const VALUE_URI: u8 = 9;
pub const VALUE_URI_MAX: u8 = 10;
pub const VALUE_URI_SPILLED: u8 = 11;
pub const VALUE_COLOR: u8 = 12;
pub const VALUE_EMAIL: u8 = 16;
pub const VALUE_EMAIL_MAX: u8 = 17;
pub const VALUE_EMAIL_SPILLED: u8 = 18;
pub const VALUE_TEXT: u8 = 19;
pub const VALUE_TEXT_MAX: u8 = 20;
pub const VALUE_TEXT_SPILLED: u8 = 21;
pub const VALUE_BINARY: u8 = 22;
pub const VALUE_BINARY_MAX: u8 = 23;
pub const VALUE_BINARY_SPILLED: u8 = 24;
pub const VALUE_ENCRYPTED: u8 = 25;

// endregion

// region Pages

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

// endregion
