mod query;
mod values;

pub use query::*;
pub use values::*;

use imbl::OrdMap;

use crate::model::{Atom, CompositeApi, Object};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement {
    pub atom: Atom,
    pub object: Object,
}

#[derive(Clone)]
pub struct Knowledge {
    pub map: OrdMap<Atom, Values>,
}

impl Default for Knowledge {
    fn default() -> Self {
        Self::empty()
    }
}

impl Knowledge {
    #[must_use]
    pub fn known_atoms(self) -> KnownAtoms {
        KnownAtoms {
            map_iter: self.map.into_iter(),
        }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        match self.map.get_mut(&statement.atom) {
            Some(values) => values.insert(statement.object),
            None => {
                self.map
                    .insert(statement.atom, Values::from([statement.object]));
            }
        }
    }

    pub fn remove_statement(&mut self, statement: &Statement) {
        let is_empty = match self.map.get_mut(&statement.atom) {
            Some(values) => {
                values.remove(&statement.object);
                values.is_empty()
            }
            None => return,
        };

        if is_empty {
            self.map.remove(&statement.atom);
        }
    }

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

    /// Returns the values of the given atom in the knowledge.
    #[must_use]
    pub fn values_of_atom(&self, atom: &Atom) -> Values {
        self.map.get(atom).cloned().unwrap_or_default()
    }

    /// Returns an iterator over the values of the given object.
    ///
    /// * If the given object is an atom, then the values are all properties of
    ///   that atom;
    /// * if the given object is a composite, then the values are the members of that
    ///   composite;
    /// * otherwise the iterator is empty.
    #[must_use]
    pub fn values_of(&self, object: &Object) -> ObjectValues {
        match object {
            Object::Atom(atom) => ObjectValues::properties(self.values_of_atom(atom)),
            Object::Construction(_) => ObjectValues::empty(),
            Object::Composite(composite) => ObjectValues::members(composite.clone()),
        }
    }

    #[must_use]
    pub fn object_has(&self, object: &Object, element: &Object) -> bool {
        match object {
            Object::Atom(atom) => self.map.get(atom).is_some_and(|values| values.has(element)),
            Object::Construction(_) => false,
            Object::Composite(composite) => composite.has(element),
        }
    }

    #[must_use]
    pub fn query_construction_values(
        &self,
        target: &Object,
        property_constructor: Object,
    ) -> QueryConstructionValues {
        QueryConstructionValues {
            properties: self.values_of(target),
            property_constructor,
        }
    }

    /// Queries the knowledge for all atoms that have a property that is a
    /// construction with the given constructor.
    #[must_use]
    pub fn query_atoms_with_constructor(&self, constructor: Object) -> QueryAtomsWithConstructor {
        QueryAtomsWithConstructor {
            atoms: self.map.clone().into_iter(),
            constructor,
        }
    }

    /// Queries the knowledge for all pairs of an atom and a value such that the
    /// atom has the construction `constructor : value`.
    #[must_use]
    pub fn query_constructions(&self, constructor: Object) -> QueryConstructions {
        QueryConstructions::new(self.map.clone().into_iter(), constructor)
    }

    /// Determines the size of the object in this knowledge, i.e.
    /// the number of values it has. Constructions
    /// have no values.
    #[must_use]
    pub fn size_of_object(&self, object: &Object) -> u64 {
        match object {
            Object::Atom(atom) => self.map.get(atom).map_or(0, Values::size),
            Object::Construction(_) => 0,
            Object::Composite(composite) => composite.size(),
        }
    }

    /*
    fn object_is_property_in(&self, object: &Object, containing_composite: &Composite) -> bool {
        match object {
            Object::Atom(_) | Object::Composite(_) => {
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
    fn composite_is_valid(&self, composite: &Composite) -> bool {
        composite.values().all(|value| self.object_is_valid())
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.map.iter().all(|(_, composite)| {
            composite.clone()
                .values()
                .all(|value| self.object_is_property(&value))
        })
    }
     */
}

#[derive(Clone)]
pub struct KnownAtoms {
    map_iter: <OrdMap<Atom, Values> as IntoIterator>::IntoIter,
}

impl Iterator for KnownAtoms {
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next().map(|(atom, _)| atom)
    }
}
