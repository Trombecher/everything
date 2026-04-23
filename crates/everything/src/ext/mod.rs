//! Extension traits and implementations [ObjectExt] and [StructureExt]
//! for [Object] and [Structure].

mod abstracts;
mod objects;
mod properties;
mod structures;

pub use abstracts::*;
use everything_structures::Abstract;
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

impl From<NodeType> for Abstract {
    fn from(value: NodeType) -> Abstract {
        match value {
            NodeType::Computed => Abstract::COMPUTED,
            NodeType::Literal => Abstract::NODE_LITERAL,
            NodeType::And => Abstract::NODE_AND,
            NodeType::FunctionSelf => Abstract::NODE_FUNCTION_SELF,
            NodeType::Exists => Abstract::NODE_EXISTS,
            NodeType::Parameter => Abstract::NODE_PARAMETER,
            NodeType::Count => Abstract::NODE_COUNT,
            NodeType::Query => Abstract::NODE_QUERY,
            NodeType::Equal => Abstract::NODE_EQUAL,
            NodeType::Or => Abstract::NODE_OR,
            NodeType::XOr => Abstract::NODE_XOR,
            NodeType::Not => Abstract::NODE_NOT,
        }
    }
}
