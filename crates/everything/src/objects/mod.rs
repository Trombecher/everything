//! This module defines some objects.

#[cfg(test)]
mod tests;

use everything_structures::{Object, Property, Structure};

use crate::{
    inference::{compute, query_values},
    objects,
};

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

pub struct Statement<'a> {
    pub subject: &'a Object,
    pub tag: &'a Object,
    pub value: &'a Object,
}

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

    /// Constructs a new set containing only `self`.
    fn to_set_of_self(self) -> Structure;

    fn structure(&self) -> Option<&Structure>;

    fn is_truthy(&self) -> bool;
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

    fn to_set_of_self(self) -> Structure {
        Structure::new(&mut [Property {
            tag: CONTAINS,
            value: self,
        }])
    }

    fn structure(&self) -> Option<&Structure> {
        match self {
            Self::Abstract(_) => None,
            Self::Structure(structure) => Some(structure),
        }
    }

    // TODO: discuss abstract objects
    fn is_truthy(&self) -> bool {
        match self {
            Self::Abstract(_) => false,
            Self::Structure(structure) => !structure.as_ref().is_empty(),
        }
    }
}

pub trait StructureExt {
    fn has_exactly_one_value_on(&self, tag: &Object) -> bool;

    fn is_knowledge(&self) -> bool;

    fn is_statement(&self) -> bool;

    fn parse_statement<'a>(&'a self) -> Option<Statement<'a>>;
}

impl StructureExt for Structure {
    fn has_exactly_one_value_on(&self, tag: &Object) -> bool {
        let mut values = self.values(tag);
        return values.next().is_some() && values.next().is_none();
    }

    fn is_knowledge(&self) -> bool {
        // First we validate that every object contained
        // in `self` is a statement.

        for contains_object in self.values(&CONTAINS) {
            if let Object::Structure(contains_structure) = contains_object
                && contains_structure.is_statement()
            {
            } else {
                // TODO: review this for abstracts
                return false;
            }
        }

        // Now we need to check constraints and values.

        for statement in self.values(&CONTAINS) {
            // TODO: better panic msgs
            let statement = statement.structure().unwrap().parse_statement().unwrap();

            // Get constraint function from tag for value:
            let constraint_query_result = query_values(self, &statement.tag, &AXIOMATIC);
            let constraint_function = match constraint_query_result.iter().next() {
                Some(c) => c,
                None => return false,
            };

            let inter = compute::call(constraint_function, &statement.subject);
            let result = compute::call(&inter, &statement.value);

            if !result.is_truthy() {
                return false;
            }
        }

        true
    }

    fn is_statement(&self) -> bool {
        self.has_exactly_one_value_on(&objects::STATEMENT_SUBJECT)
            && self.has_exactly_one_value_on(&objects::STATEMENT_TAG)
            && self.has_exactly_one_value_on(&objects::STATEMENT_VALUE)
    }

    fn parse_statement<'a>(&'a self) -> Option<Statement<'a>> {
        let subject = self.values(&objects::STATEMENT_SUBJECT).next()?;
        let tag = self.values(&objects::STATEMENT_TAG).next()?;
        let value = self.values(&objects::STATEMENT_TAG).next()?;

        Some(Statement {
            subject,
            tag,
            value,
        })
    }
}
