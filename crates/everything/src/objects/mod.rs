//! This module defines some objects.

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

pub trait ObjectExt {
    fn is_only_natural_number(&self) -> bool;

    /// Constructs a natural number object using
    /// repeated succ.
    fn natural_number(n: u64) -> Self;

    /// Returns the number of properties this object has.
    /// For abstract objects, this returns zero.
    fn property_count(&self) -> usize;

    /// Converts a boolean to an object.
    ///
    /// ```plain
    /// true |-> {(@1, {})}
    /// false |-> {}
    /// ```
    fn from_bool(b: bool) -> Object;

    /// Checks whether `self` is knowledge.
    fn is_knowledge(&self) -> bool;

    /// Constructs a new set containing only `self`.
    fn to_set_of_self(self) -> Structure;
}

impl ObjectExt for Object {
    fn is_only_natural_number(&self) -> bool {
        match self {
            &ZERO => true,
            Object::Structure(s) => {
                if let [
                    Property {
                        tag: SUCESSOR_OF,
                        value,
                    },
                ] = s.as_ref()
                {
                    value.is_only_natural_number()
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn natural_number(n: u64) -> Self {
        if n == 0 {
            ZERO
        } else {
            Structure::new(&mut [Property {
                tag: SUCESSOR_OF,
                value: Self::natural_number(n - 1),
            }])
            .into()
        }
    }

    fn property_count(&self) -> usize {
        match self {
            Object::Abstract(_) => 0,
            Object::Structure(structure) => structure.as_ref().len(),
        }
    }

    fn from_bool(b: bool) -> Self {
        if b {
            Self::to_set_of_self(Structure::EMPTY.into())
        } else {
            Structure::EMPTY
        }
        .into()
    }

    fn is_knowledge(&self) -> bool {
        todo!()
    }

    fn to_set_of_self(self) -> Structure {
        Structure::new(&mut [Property {
            tag: CONTAINS,
            value: self,
        }])
    }
}
