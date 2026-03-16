#[cfg(test)]
mod tests;

use everything_structures::{Object, Property, Structure};

use crate::{ext::NodeType, query::query_values};

macro_rules! define_abstract {
    ($($id:ident = $n:literal),* $(,)?) => {
        $(const $id: Object = Object::Abstract($n);)*
    };
}

pub trait ObjectExt {
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
        // NODE_FUNCTION_BODY = 11,
        NODE_LITERAL = 12,
        NODE_AND = 13,
        NODE_EXISTS = 14,
        NODE_PARAMETER = 15,
        // IS_NATURAL_NUMBER = 16,
        NODE_COUNT = 17,
        NODE_QUERY = 18,
        NODE_EQUAL = 19,
        NODE_OR = 20,
        NODE_XOR = 21,
        NODE_NOT = 22,
        NODE = 23,
        TAG = 24,
        NODE_FUNCTION_SELF = 25,
        // NODE_CALL_TARGET = 26,
        // NODE_CALL_PARAMETER = 27,
        // NODE_CALL = 28,
    );

    fn eval(&self, knowledge: &Structure) -> Object;

    fn node_type(&self, knowledge: &Structure) -> Option<NodeType>;

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

    fn call(&self, knowledge: &Structure, parameter: &Object) -> Object;
}

impl ObjectExt for Object {
    fn is_only_natural_number(&self) -> bool {
        match self {
            &Object::ZERO => true,
            Object::Structure(s) => {
                if let [
                    Property {
                        tag: Object::SUCCESSOR_OF,
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
            Object::ZERO
        } else {
            Structure::new(&mut [Property {
                tag: Object::SUCCESSOR_OF,
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
            tag: Object::CONTAINS,
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

    fn node_type(&self, knowledge: &Structure) -> Option<NodeType> {
        let mut current_pick = None;

        for node_type in NodeType::ALL {
            let node_type_object: Object = node_type.into();
            let query = query_values(knowledge, self, node_type_object);
            let there_are_values = query.iter().next().is_some();

            println!("there are values: {there_are_values}");

            if there_are_values {
                if current_pick.is_some() {
                    // multiple node types apply
                    return None;
                }

                current_pick = Some(node_type);
            }
        }

        current_pick
    }

    fn eval(&self, knowledge: &Structure) -> Object {
        // TODO: better panic msgs

        match self.node_type(knowledge) {
            Some(NodeType::Count) => {
                let qr = query_values(knowledge, self, Object::NODE_COUNT);
                let value = qr.iter().next().unwrap();

                Object::natural_number(value.property_count() as u64)
            }
            Some(NodeType::Query) => {
                // TODO: adjust constraint for query

                let qr = query_values(knowledge, self, Object::NODE_QUERY);
                let query_form = qr.iter().next().unwrap();

                let subject_qr = query_values(knowledge, query_form, Object::STATEMENT_SUBJECT);
                let subject = subject_qr
                    .iter()
                    .next()
                    .expect("cannot query with no subject");

                let tag_qr = query_values(knowledge, query_form, Object::STATEMENT_TAG);
                let tag = tag_qr.iter().next().expect("cannot query with no tag");

                let value_qr = query_values(knowledge, query_form, Object::STATEMENT_VALUE);
                let value = value_qr.iter().next();

                let actual_qr = query_values(knowledge, subject, tag.clone());

                if let Some(value) = value {
                    // This is just equal to `NODE_EXISTS`.

                    for item in actual_qr.iter() {
                        if item == value {
                            return Object::from_bool(true);
                        }
                    }

                    Object::from_bool(false)
                } else {
                    // Collect all values into a set.
                    actual_qr.collect_to_set()
                }
            }
            Some(_) => todo!(),
            None => self.clone(),
        }
    }

    // TODO: remove this fn
    fn call(&self, _knowledge: &Structure, parameter: &Object) -> Object {
        todo!("impl call of {self:?} {parameter:?} = ?")
    }
}
