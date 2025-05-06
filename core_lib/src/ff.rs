//! File format constants
//!
//! DO NOT CHANGE WITHOUT REASON.
//! Existing deployments depend on these to match the version.

pub const META_FILE_NAME: &str = "everything";
pub const RESOURCES_PATH: &str = "r";
pub const OBJECTS_TAGS_PATH: &str = "o";
pub const TAGS_OBJECTS_PATH: &str = "t";

pub const MAGIC_BYTES: [u8; 12] = *b"EVERYTHINGKB";

/// Offsets of the feature bits in the features meta field.
pub mod features {
    pub const CREATION: u8 = 0;
    pub const BIN: u8 = 1;
    pub const NAMING: u8 = 2;
    pub const INTERNATIONALIZATION: u8 = 3;
    pub const FILE_SYSTEM: u8 = 4;
    pub const FILE_TYPES: u8 = 5;
    pub const NODE_COUNT: u8 = 6;
    pub const IMAGES: u8 = 7;
    pub const FAVOURITES: u8 = 8;
    pub const TEMPORARY_OBJECTS: u8 = 9;
    pub const USERS: u8 = 10;
    pub const REFERENCES: u8 = 11;
}

/// Used to indicate the empty (unit) schema.
pub const NONE: u8 = 0;

/// Indicates a variable in a constraint.
pub const VAR: u8 = 1;

/// Indicates an `i64`
pub const INTEGER: u8 = 4;

/// Indicates an `f64`.
pub const FLOAT: u8 = 5;

/// Indicates a [crate::values::time::Duration].
pub const DURATION: u8 = 6;

/// Indicates a [crate::values::time::DateTime].
pub const DATE_TIME: u8 = 7;

/// Indicates an object **reference**, [crate::ObjectId].
pub const OBJECT: u8 = 9;

/// Indicates a [crate::values::lang::Language].
pub const LANGUAGE: u8 = 10;

/// Indicates a [crate::url::URL].
pub const URL: u8 = 11;

/// Indicates a [crate::color::Color].
pub const COLOR: u8 = 12;

/// Indicates a [crate::values::schema::Schema].
pub const SCHEMA: u8 = 13;

/// Indicates a [crate::values::constraints::Constraint].
pub const CONSTRAINT: u8 = 14;

/// Indicates a [crate::values::email::Email].
pub const EMAIL: u8 = 15;

/// Indicates text, `str`.
pub const TEXT: u8 = 16;

/// Indicates binary data, `[u8]`.
pub const BINARY: u8 = 17;
pub const ENC_EMAIL: u8 = 18;
pub const ENC_TEXT: u8 = 19;
pub const ENC_BINARY: u8 = 20;
pub const CHARACTER: u8 = 21;
pub const NEG: u8 = 22;
pub const ADD: u8 = 23;
pub const SUB: u8 = 24;
pub const MUL: u8 = 25;
pub const DIV: u8 = 26;
pub const MOD: u8 = 27;
pub const EQ: u8 = 28;
pub const NEQ: u8 = 29;
pub const LTH: u8 = 30;
pub const LE: u8 = 31;
pub const GTH: u8 = 32;
pub const GE: u8 = 33;
pub const OPT_OBJECT: u8 = 34;

/// 15 bytes of inlined text.
pub const TEXT_MAX: u8 = 35;

/// External text resource.
pub const TEXT_RES: u8 = 36;

/// 15 bytes of inlined binary data.
pub const BINARY_MAX: u8 = 37;

/// External binary resource.
pub const BINARY_RES: u8 = 38;

/// 15 bytes of inlined email text.
pub const EMAIL_MAX: u8 = 39;

/// External email text resource (that must be a long email address).
pub const EMAIL_RES: u8 = 40;

/// 15 bytes of inlined, encrypted text.
pub const ENC_TEXT_MAX: u8 = 41;

/// External resource containing encrypted text.
pub const ENC_TEXT_RES: u8 = 42;

/// 15 bytes of inlined, encrypted binary data.
pub const ENC_BINARY_MAX: u8 = 43;

/// External resource containing encrypted binary data.
pub const ENC_BINARY_RES: u8 = 44;

/// 15 bytes of an encrypted, inlined email address.
pub const ENC_EMAIL_MAX: u8 = 45;

/// External resource containing an encrypted email address.
pub const ENC_EMAIL_RES: u8 = 46;

/// External resource containing a [crate::values::time::Duration].
pub const BIG_DURATION: u8 = 47;

/// External resource containing a [crate::values::time::DateTime].
pub const BIG_DATE_TIME: u8 = 48;