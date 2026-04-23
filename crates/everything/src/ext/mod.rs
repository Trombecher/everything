//! Extension traits and implementations [ObjectExt] and [StructureExt]
//! for [Object] and [Structure].

mod objects;
mod structures;

pub use objects::*;
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
}

impl NodeType {
    pub const ALL: [Self; 12] = [
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
    ];
}

impl From<NodeType> for Object {
    fn from(value: NodeType) -> Object {
        match value {
            NodeType::Computed => Object::COMPUTED,
            NodeType::Literal => Object::NODE_LITERAL,
            NodeType::And => Object::NODE_AND,
            NodeType::FunctionSelf => Object::NODE_FUNCTION_SELF,
            NodeType::Exists => Object::NODE_EXISTS,
            NodeType::Parameter => Object::NODE_PARAMETER,
            NodeType::Count => Object::NODE_COUNT,
            NodeType::Query => Object::NODE_QUERY,
            NodeType::Equal => Object::NODE_EQUAL,
            NodeType::Or => Object::NODE_OR,
            NodeType::XOr => Object::NODE_XOR,
            NodeType::Not => Object::NODE_NOT,
        }
    }
}
