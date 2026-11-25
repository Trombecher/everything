use std::fmt::Debug;

use num_bigint::BigInt;

use crate::model::{Atom, CompositeApi, CompositeSpecialization, Construction, Object};

/// Specialization for `{code_point : i}`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharacterComposite {
    pub representing: char,
}

impl CharacterComposite {
    fn parse_char(object: &Object) -> Option<char> {
        if let Object::Construction(Construction { constructor, value }) = object
            && matches!(constructor.as_ref(), Object::Atom(Atom::CODE_POINT))
            && let Some(integer) = value.integer()
            && let Some(character) = u32::try_from(integer)
                .ok()
                .and_then(|u| char::try_from(u).ok())
        {
            Some(character)
        } else {
            None
        }
    }
}

impl Debug for CharacterComposite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.representing)
    }
}

impl CompositeSpecialization for CharacterComposite {
    fn specialize_from(objects: &[Object]) -> Option<Self> {
        match objects {
            [object] if let Some(character) = Self::parse_char(object) => Some(Self {
                representing: character,
            }),
            _ => None,
        }
    }
}

impl CompositeApi for CharacterComposite {
    fn size(&self) -> u64 {
        1
    }

    fn has(&self, object: &Object) -> bool {
        matches!(Self::parse_char(object), Some(character) if self.representing == character)
    }
}

impl IntoIterator for CharacterComposite {
    type Item = Object;
    type IntoIter = CharacterCompositeValues;

    fn into_iter(self) -> Self::IntoIter {
        CharacterCompositeValues {
            to_yield: Some(self.representing),
        }
    }
}

#[derive(Clone)]
pub struct CharacterCompositeValues {
    pub to_yield: Option<char>,
}

impl Iterator for CharacterCompositeValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        let character = self.to_yield.take()?;

        Some(Object::Construction(Construction {
            constructor: Box::new(Object::Atom(Atom::CODE_POINT)),
            value: Box::new(Object::new_integer(BigInt::from(u32::from(character)))),
        }))
    }
}
