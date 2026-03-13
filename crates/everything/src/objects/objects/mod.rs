#[cfg(test)]
mod tests;

use everything_structures::{Object, Property, Structure};

use crate::{
    inference::query_values,
    objects::{self, NodeType},
};

pub trait ObjectExt {
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
}

impl ObjectExt for Object {
    fn is_only_natural_number(&self) -> bool {
        match self {
            &objects::ZERO => true,
            Object::Structure(s) => {
                if let [
                    Property {
                        tag: objects::SUCCESSOR_OF,
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
            objects::ZERO
        } else {
            Structure::new(&mut [Property {
                tag: objects::SUCCESSOR_OF,
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
            tag: objects::CONTAINS,
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
            let query = query_values(knowledge, self, &node_type_object);
            let there_are_values = query.iter().next().is_some();

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
}
