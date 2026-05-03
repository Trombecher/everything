//! Extension traits and implementations [ObjectExt] and [StructureExt]
//! for [Object] and [Structure].

mod abstracts;
mod iter;
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

#[derive(Clone, PartialEq, Debug)]
pub enum NodeType {
    Computed(Object),
    Literal(Object),
    And,
    FunctionSelf(Object),
    Exists(Object),
    Parameter(u128),
    Count(Object),
    Query(Object),
    Equal,
    Or,
    Xor,
    Not(Object),
    Add { left: Object, right: Object },
}
