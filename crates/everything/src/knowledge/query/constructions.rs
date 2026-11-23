use imbl::OrdMap;

use crate::{
    knowledge::{Values, ValuesValues},
    model::{Atom, Construction, Object, Set},
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
        Set::from([
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
    atoms: <OrdMap<Atom, Values> as IntoIterator>::IntoIter,
    constructor: Object,
    current: Option<(Atom, ValuesValues)>,
}

impl QueryConstructions {
    #[must_use]
    pub fn new(
        atoms: <OrdMap<Atom, Values> as IntoIterator>::IntoIter,
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
            let (atom, values) = if let Some((atom, values)) = &mut self.current {
                (*atom, values)
            } else {
                let (atom, values) = self.atoms.next()?;
                self.current = Some((atom, values.into_iter()));

                continue;
            };

            for property in values {
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
