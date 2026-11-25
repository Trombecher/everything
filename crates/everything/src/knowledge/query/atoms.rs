use imbl::OrdMap;

use crate::{
    knowledge::Properties,
    model::{Atom, Construction, Object},
};

#[derive(Clone)]
pub struct QueryAtomsWithProperty {
    pub atoms: <OrdMap<Atom, Properties> as IntoIterator>::IntoIter,
    pub property: Object,
}

impl Iterator for QueryAtomsWithProperty {
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        self.atoms.find_map(|(atom, properties)| {
            if properties.has(&self.property) {
                Some(atom)
            } else {
                None
            }
        })
    }
}

/// Queries the knowledge for all atoms that have a property that is a
/// construction with the given constructor.
#[derive(Clone)]
pub struct QueryAtomsWithConstructor {
    pub atoms: <OrdMap<Atom, Properties> as IntoIterator>::IntoIter,
    pub constructor: Object,
}

impl Iterator for QueryAtomsWithConstructor {
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        self.atoms.find_map(|(atom, properties)| {
            properties
                .into_iter()
                .any(|property| {
                    matches!(
                        property,
                        Object::Construction(Construction { constructor, .. })
                            if constructor.as_ref() == &self.constructor
                    )
                })
                .then_some(atom)
        })
    }
}
