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
);

#[derive(Clone, Debug, PartialEq)]
pub struct Statement<'a> {
    pub subject: &'a Object,
    pub tag: &'a Object,
    pub value: &'a Object,
}
