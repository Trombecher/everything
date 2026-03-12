//! This module defines structures.

#[cfg(test)]
mod tests;

use everything_structures::{Object, Property, Structure};

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
    SUCESSOR_OF = 10,
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
);

pub fn is_only_natural_number(o: &Object) -> bool {
    match o {
        &ZERO => true,
        Object::Structure(s) => {
            if let [
                Property {
                    tag: SUCESSOR_OF,
                    value,
                },
            ] = s.as_ref()
            {
                is_only_natural_number(value)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Constructs a natural number object using
/// repeated succ.
pub fn natural_number(n: u64) -> Object {
    if n == 0 {
        ZERO
    } else {
        Structure::EMPTY
            .change(
                &mut [],
                &mut [Property {
                    tag: SUCESSOR_OF,
                    value: natural_number(n - 1),
                }],
            )
            .into()
    }
}
