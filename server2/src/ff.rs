//! # File format constants
//! 
//! DO NOT CHANGE WITHOUT REASON.
//! Existing deployments depend on these to match the version.

pub mod schema {
    pub const NONE: u8 = 0;
    pub const OPT_OBJECT: u8 = 34;
}

pub mod expr {
    pub const VAR: u8 = 1;
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
}

pub mod value {
    pub const TRUE: u8 = 2;
    pub const FALSE: u8 = 3;
    pub const INTEGER: u8 = 4;
    pub const FLOAT: u8 = 5;
    pub const DURATION: u8 = 6;
    pub const DATE_TIME: u8 = 7;
    pub const OBJECT: u8 = 9;
    pub const LANGUAGE: u8 = 10;
    pub const URL: u8 = 11;
    pub const COLOR: u8 = 12;
    pub const SCHEMA: u8 = 13;
    pub const CONSTRAINT: u8 = 14;
    pub const EMAIL: u8 = 15;
    pub const TEXT: u8 = 16;
    pub const BINARY: u8 = 17;
    pub const ENC_EMAIL: u8 = 18;
    pub const ENC_TEXT: u8 = 19;
    pub const ENC_BINARY: u8 = 20;
    pub const CHARACTER: u8 = 21;
}