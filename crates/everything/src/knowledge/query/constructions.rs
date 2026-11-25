use imbl::OrdMap;

use crate::{
    knowledge::{Properties, PropertiesIter},
    model::{Atom, Composite, Construction, Object},
};

/// A pair of an atom and a construction value.
///
/// It represents a statement `atom is constructor : value` in the knowledge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtomValuePair {
    pub atom: Atom,
    pub value: Object,
}

impl AtomValuePair {
    /// Converts the pair into the object `{atom : <atom>, value : <value>}`.
    #[must_use]
    pub fn into_object(self) -> Object {
        Composite::from([
            Construction::new(Atom::ATOM.into(), self.atom.into()).into(),
            Construction::new(Atom::VALUE.into(), self.value).into(),
        ])
        .into()
    }
}

/// Queries the knowledge for all pairs of an atom and a value such that the
/// atom has the construction `constructor : value` in the knowledge.
#[derive(Clone)]
pub struct QueryConstructions {
    atoms: <OrdMap<Atom, Properties> as IntoIterator>::IntoIter,
    constructor: Object,
    current: Option<(Atom, PropertiesIter)>,
}

impl QueryConstructions {
    #[must_use]
    pub fn new(
        atoms: <OrdMap<Atom, Properties> as IntoIterator>::IntoIter,
        constructor: Object,
    ) -> Self {
        Self {
            atoms,
            constructor,
            current: None,
        }
    }
}

impl Iterator for QueryConstructions {
    type Item = AtomValuePair;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (atom, properties) = if let Some((atom, properties)) = &mut self.current {
                (*atom, properties)
            } else {
                let (atom, properties) = self.atoms.next()?;
                self.current = Some((atom, properties.into_iter()));

                continue;
            };

            for property in properties {
                if let Object::Construction(Construction { constructor, value }) = property
                    && constructor.as_ref() == &self.constructor
                {
                    return Some(AtomValuePair {
                        atom,
                        value: *value,
                    });
                }
            }

            self.current = None;
        }
    }
}
