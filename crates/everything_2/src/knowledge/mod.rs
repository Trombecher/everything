mod properties;
mod query_atoms;

use imbl::OrdMap;
pub use query_atoms::*;

use crate::model::{Atom, Object, Set};

pub struct Statement {
    pub atom: Atom,
    pub is_object: Object,
}

#[derive(Clone)]
pub struct Knowledge {
    pub map: OrdMap<Atom, Set>,
}

impl Knowledge {
    pub fn query_atoms_with_property(&self, property: Object) -> QueryAtomsWithProperty {
        QueryAtomsWithProperty {
            atoms: self.map.clone().into_iter(),
            property,
        }
    }
}
