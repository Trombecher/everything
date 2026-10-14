use std::fmt::Debug;

use crate::model::{Atom, Construction, Object, SetApi, SetSpecialization};

/// Specialization for `{code_point i}`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharacterSet {
    pub representing: char,
}

impl Debug for CharacterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.representing)
    }
}

impl SetSpecialization for CharacterSet {
    fn specialize_from(objects: &[Object]) -> Option<Self> {
        match objects {
            [Object::Construction(Construction { constructor, value })]
                if matches!(constructor.as_ref(), Object::Atom(Atom::CODE_POINT))
                    && let Some(integer) = value.integer()
                    && let Some(character) = u32::try_from(integer)
                        .ok()
                        .and_then(|u| char::try_from(u).ok()) =>
            {
                Some(Self {
                    representing: character,
                })
            }
            _ => None,
        }
    }
}

impl SetApi for CharacterSet {
    fn size(&self) -> u64 {
        1
    }

    fn has(&self, object: &Object) -> bool {
        todo!()
    }
}

impl IntoIterator for CharacterSet {
    type Item = Object;
    type IntoIter = CharacterSetValues;

    fn into_iter(self) -> Self::IntoIter {
        CharacterSetValues {
            to_yield: Some(self.representing),
        }
    }
}

#[derive(Clone)]
pub struct CharacterSetValues {
    pub to_yield: Option<char>,
}

impl Iterator for CharacterSetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        let character = self.to_yield.take()?;

        Some(Object::Construction(Construction {
            constructor: Box::new(Object::Atom(Atom::CODE_POINT)),
            value: Box::new(Object::new_integer(i128::from(u32::from(character)))),
        }))
    }
}
