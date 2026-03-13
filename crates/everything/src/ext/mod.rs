//! This module defines some objects.

mod objects;
mod structures;

pub use objects::*;
pub use structures::*;

use everything_structures::Object;

macro_rules! define_abstract {
    ($($id:ident = $n:literal),* $(,)?) => {
        $(pub const $id: Object = Object::Abstract($n);)*
    };
}

// DO NOT CHANGE THESE!
define_abstract!(
    CONTAINS = 1,
    AXIOMATIC = 2,
    COMPUTED = 3,
    STATEMENT_SUBJECT = 4,
    STATEMENT_TAG = 5,
    STATEMENT_VALUE = 6,
    STATEMENT = 7,
    KNOWLEDGE = 8,
    ZERO = 9,
    SUCCESSOR_OF = 10,
    NODE_FUNCTION_BODY = 11,
    NODE_LITERAL = 12,
    NODE_AND = 13,
    NODE_EXISTS = 14,
    NODE_PARAMETER = 15,
    IS_NATURAL_NUMBER = 16,
    NODE_COUNT = 17,
    NODE_QUERY = 18,
    NODE_EQUAL = 19,
    NODE_OR = 20,
    NODE_XOR = 21,
    NODE_NOT = 22,
    NODE = 23,
    TAG = 24,
    NODE_FUNCTION_SELF = 25,
    NODE_CALL_TARGET = 26,
    NODE_CALL_PARAMETER = 27,
    NODE_CALL = 28,
);

#[derive(Clone, Debug, PartialEq)]
pub struct Statement<'a> {
    pub subject: &'a Object,
    pub tag: &'a Object,
    pub value: &'a Object,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NodeType {
    FunctionBody,
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
    Call,
}

impl NodeType {
    pub const ALL: [Self; 13] = [
        Self::FunctionBody,
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
        Self::Call,
    ];
}

impl Into<Object> for NodeType {
    fn into(self) -> Object {
        match self {
            Self::FunctionBody => NODE_FUNCTION_BODY,
            Self::Literal => NODE_LITERAL,
            Self::And => NODE_AND,
            Self::FunctionSelf => NODE_FUNCTION_SELF,
            Self::Exists => NODE_EXISTS,
            Self::Parameter => NODE_PARAMETER,
            Self::Count => NODE_COUNT,
            Self::Query => NODE_QUERY,
            Self::Equal => NODE_EQUAL,
            Self::Or => NODE_OR,
            Self::XOr => NODE_XOR,
            Self::Not => NODE_NOT,
            Self::Call => NODE_CALL,
        }
    }
}
