use imbl::OrdMap;

use crate::{
    knowledge::Values,
    model::{Atom, Object},
};

#[derive(Clone)]
pub struct QueryAtomsWithProperty {
    pub atoms: <OrdMap<Atom, Values> as IntoIterator>::IntoIter,
    pub property: Object,
}

impl Iterator for QueryAtomsWithProperty {
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        self.atoms.find_map(|(atom, set)| {
            if set.has(&self.property) {
                Some(atom)
            } else {
                None
            }
        })
    }
}
