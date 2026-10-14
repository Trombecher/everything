use imbl::OrdMap;

use crate::model::{Atom, Object, Set, SetApi};

#[derive(Clone)]
pub struct QueryAtomsWithProperty {
    pub(crate) atoms: <OrdMap<Atom, Set> as IntoIterator>::IntoIter,
    pub(crate) property: Object,
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
