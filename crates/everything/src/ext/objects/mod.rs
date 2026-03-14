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
        match self.node_type(knowledge) {
            Some(NodeType::Call) => {
                // let query = query_values(knowledge, node, &objects::NODE_CALL);
                // let x = query.iter().next().unwrap();
                todo!()
            }
            Some(_) => todo!(),
            None => self.clone(),
        }
    }

    fn call(&self, _knowledge: &Structure, parameter: &Object) -> Object {
        todo!("impl call of {self:?} {parameter:?} = ?")
    }
}
