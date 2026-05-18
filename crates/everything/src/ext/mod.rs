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

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryNode {
    pub left: Object,
    pub right: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MapNode {
    pub set: Object,
    pub mapper_function: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FilterNode {
    pub set: Object,
    pub filter_function: Object,
}

#[derive(Clone, PartialEq, Debug)]
pub enum NodeType {
    Computed(Object),
    Literal(Object),
    And(BinaryNode),
    FunctionSelf(u128),
    Parameter(u128),
    Count(Object),
    Query(Object),
    Equal(BinaryNode),
    Or(BinaryNode),
    Xor(BinaryNode),
    Not(Object),
    Add(BinaryNode),
    Union(BinaryNode),
    Map(MapNode),
    Filter(FilterNode),
    Less(BinaryNode),
}
