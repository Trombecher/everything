use std::{fmt::Debug, ops::RangeBounds, slice::SliceIndex};

use arcstr::Substr;

use crate::model::{Atom, CharacterSet, Construction, Object, Set, SetApi, SetSpecialization};

#[cfg(test)]
mod tests;

/// A set object specialization for a non-empty string
/// of text. Virtual set form:
///
/// ```plain
/// {list_item <char>, list_rest <text or empty set>}
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextSet {
    string: Substr,
}

impl Debug for TextSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.string, f)
    }
}

impl TextSet {
    /// Tries to create a new text set from a string.
    /// Returns `None` if the string is empty.
    #[must_use]
    pub fn new(string: Substr) -> Option<Self> {
        if string.is_empty() {
            None
        } else {
            Some(Self { string })
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.string
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn list_item(&self) -> char {
        self.as_str()
            .chars()
            .next()
            .expect("text set invariant invalid")
    }

    #[must_use]
    pub fn list_item_as_object(&self) -> Object {
        Object::Set(Set::Character(CharacterSet {
            representing: self.list_item(),
        }))
    }

    #[must_use]
    pub fn list_tail(&self) -> Option<Self> {
        self.substring(self.list_item().len_utf8()..)
    }

    #[must_use]
    pub fn list_tail_as_object(&self) -> Object {
        self.list_tail().map_or(Set::empty(), Set::Text).into()
    }

    pub fn substring(
        &self,
        range: impl SliceIndex<str> + Clone + RangeBounds<usize>,
    ) -> Option<Self> {
        if self.string.get(range.clone()).is_some() {
            // An empty substring is not a valid text set (see `Self::new`).
            Self::new(self.string.substr(range))
        } else {
            None
        }
    }
}

impl SetSpecialization for TextSet {
    fn specialize_from(objects: &[Object]) -> Option<Self> {
        let [first, second] = objects else {
            return None;
        };

        let mut list_item: Option<&Object> = None;
        let mut list_tail: Option<&Object> = None;

        for object in [first, second] {
            let Object::Construction(Construction { constructor, value }) = object else {
                return None;
            };

            let slot = match constructor.as_ref() {
                Object::Atom(Atom::LIST_ITEM) => &mut list_item,
                Object::Atom(Atom::LIST_TAIL) => &mut list_tail,
                _ => return None,
            };

            // Each slot must be occupied by exactly one object.
            if slot.replace(value.as_ref()).is_some() {
                return None;
            }
        }

        // The item must be a (specialized) character object. Accepting
        // any other representation would break canonicalization, as
        // iterating this set always yields the specialized form.
        let Object::Set(Set::Character(CharacterSet { representing: item })) = list_item? else {
            return None;
        };

        // The tail must be text or the empty set (the base case of text).
        let tail = match list_tail? {
            Object::Set(Set::Text(tail)) => tail.as_str(),
            Object::Set(tail) if tail.size() == 0 => "",
            _ => return None,
        };

        let mut string = String::with_capacity(item.len_utf8() + tail.len());
        string.push(*item);
        string.push_str(tail);

        Self::new(string.into())
    }
}

impl SetApi for TextSet {
    fn size(&self) -> u64 {
        2
    }

    fn has(&self, object: &Object) -> bool {
        match object {
            Object::Construction(Construction { constructor, value })
                if constructor.as_ref() == &Atom::LIST_ITEM.into() =>
            {
                value.as_ref() == &self.list_item_as_object()
            }
            Object::Construction(Construction { constructor, value })
                if constructor.as_ref() == &Atom::LIST_TAIL.into() =>
            {
                value.as_ref() == &self.list_tail_as_object()
            }
            _ => false,
        }
    }
}

impl IntoIterator for TextSet {
    type Item = Object;
    type IntoIter = TextSetValues;

    fn into_iter(self) -> Self::IntoIter {
        TextSetValues {
            text_set: self,
            step: TextSetIteratorStep::YieldItem,
        }
    }
}

#[derive(Clone)]
enum TextSetIteratorStep {
    YieldItem,
    YieldRest,
    Stop,
}

#[derive(Clone)]
pub struct TextSetValues {
    text_set: TextSet,
    step: TextSetIteratorStep,
}

impl Iterator for TextSetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self.step {
            TextSetIteratorStep::YieldItem => {
                self.step = TextSetIteratorStep::YieldRest;

                Some(Object::Construction(Construction::new(
                    Atom::LIST_ITEM.into(),
                    self.text_set.list_item_as_object(),
                )))
            }
            TextSetIteratorStep::YieldRest => {
                self.step = TextSetIteratorStep::Stop;

                Some(Object::Construction(Construction::new(
                    Atom::LIST_TAIL.into(),
                    self.text_set
                        .list_tail()
                        .map_or(Set::empty(), Set::Text)
                        .into(),
                )))
            }
            TextSetIteratorStep::Stop => None,
        }
    }
}
