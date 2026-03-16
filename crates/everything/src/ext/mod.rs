//! Extension traits and implementations [ObjectExt] and [StructureExt]
//! for [Object] and [Structure].

mod objects;
mod structures;

pub use objects::*;
pub use structures::*;

#[allow(unused_imports)]
use everything_structures::{Object, Structure};

#[derive(Clone, Debug, PartialEq)]
pub struct Statement<'a> {
    pub subject: &'a Object,
    pub tag: &'a Object,
    pub value: &'a Object,
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
    // Call,
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
        // Self::Call,
    ];
}

impl Into<Object> for NodeType {
    fn into(self) -> Object {
        match self {
            Self::Computed => Object::COMPUTED,
            Self::Literal => Object::NODE_LITERAL,
            Self::And => Object::NODE_AND,
            Self::FunctionSelf => Object::NODE_FUNCTION_SELF,
            Self::Exists => Object::NODE_EXISTS,
            Self::Parameter => Object::NODE_PARAMETER,
            Self::Count => Object::NODE_COUNT,
            Self::Query => Object::NODE_QUERY,
            Self::Equal => Object::NODE_EQUAL,
            Self::Or => Object::NODE_OR,
            Self::XOr => Object::NODE_XOR,
            Self::Not => Object::NODE_NOT,
            // Self::Call => Object::NODE_CALL,
        }
    }
}
