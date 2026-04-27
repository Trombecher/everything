//! Extension traits and implementations [ObjectExt] and [StructureExt]
//! for [Object] and [Structure].

mod abstracts;
mod objects;
mod properties;
mod structures;

pub use abstracts::*;
pub use objects::*;
pub use properties::*;
pub use structures::*;

#[allow(unused_imports)]
use everything_structures::{Object, Structure};

#[derive(Clone, Debug, PartialEq)]
pub struct Statement {
    pub subject: Object,
    pub tag: Object,
    pub value: Object,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NodeType {
    Computed,
    Literal,
    And,
    FunctionSelf,
    Exists,
    Parameter,
    Count,
    Query,
    Equal,
    Or,
    XOr,
    Not,
    Add,
}

impl NodeType {
    pub const ALL: [Self; 13] = [
        Self::Computed,
        Self::Literal,
        Self::And,
        Self::FunctionSelf,
        Self::Exists,
        Self::Parameter,
        Self::Count,
        Self::Query,
        Self::Equal,
        Self::Or,
        Self::XOr,
        Self::Not,
        Self::Add,
    ];
}
