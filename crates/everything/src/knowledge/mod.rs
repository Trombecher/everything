mod properties;
mod query;

pub use query::*;

use imbl::OrdMap;

use crate::model::{Atom, Object, Set, SetApi};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement {
    pub atom: Atom,
    pub is_object: Object,
}

#[derive(Clone)]
pub struct Knowledge {
    pub map: OrdMap<Atom, Set>,
}

impl Default for Knowledge {
    fn default() -> Self {
        Self::empty()
    }
}

impl Knowledge {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            map: OrdMap::default(),
        }
    }

    #[must_use]
    pub fn query_atoms_with_property(&self, property: Object) -> QueryAtomsWithProperty {
        QueryAtomsWithProperty {
            atoms: self.map.clone().into_iter(),
            property,
        }
    }

    /// Returns the set of values of the given object.
    ///
    /// * If the given object is an atom,
    ///   then the set of all properties of that atom will be returned;
    /// * if the given object is a set, then this set is returned;
    /// * otherwise the empty set is returned.
    #[must_use]
    pub fn values_of(&self, object: &Object) -> Set {
        self.values_of_by_ref(object)
            .cloned()
            .unwrap_or(Set::empty())
    }

    #[must_use]
    pub fn values_of_by_ref<'a>(&'a self, object: &'a Object) -> Option<&'a Set> {
        match object {
            Object::Atom(atom) => self.map.get(atom),
            Object::Construction(_) => None,
            Object::Set(set) => Some(set),
        }
    }

    #[must_use]
    pub fn object_has(&self, object: &Object, element: &Object) -> bool {
        if let Some(set) = self.values_of_by_ref(object) {
            set.has(element)
        } else {
            false
        }
    }

    #[must_use]
    pub fn query_construction_values(
        &self,
        target: &Object,
        property_constructor: Object,
    ) -> QueryConstructionValues {
        QueryConstructionValues {
            properties: self.values_of(target).into_iter(),
            property_constructor,
        }
    }

    /// Determines the size of the object in this knowledge, i.e.
    /// the number of values it has. Constructions
    /// have no values.
    #[must_use]
    pub fn size_of_object(&self, object: &Object) -> u64 {
        self.values_of_by_ref(object).map_or(0, SetApi::size)
    }

    /*
    fn object_is_property_in(&self, object: &Object, containing_set: &Set) -> bool {
        match object {
            Object::Atom(_) | Object::Set(_) => {
                let f = self
                    .query_construction_values(object, Atom::PROPERTY.into())
                    .next_and_last();

                if let Some(constraint_function) = f {
                } else {
                    false
                }
            }
            Object::Construction(construction) => todo!(),
        }
    }

    #[must_use]
    fn set_is_valid(&self, set: &Set) -> bool {
        set.values().all(|value| self.object_is_valid())
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.map.iter().all(|(_, set)| {
            set.clone()
                .values()
                .all(|value| self.object_is_property(&value))
        })
    }
     */
}
