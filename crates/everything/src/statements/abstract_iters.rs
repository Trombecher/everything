use everything_objects::{Object, Property};
use imbl::HashSet;

use crate::statements::StatementProperty;

type AbstractExtendedProperties = <HashSet<StatementProperty> as IntoIterator>::IntoIter;

#[derive(Clone)]
pub struct AbstractProperties {
    pub(super) extended: AbstractExtendedProperties,
}

impl Iterator for AbstractProperties {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        self.extended.next().map(Into::into)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.extended.size_hint()
    }
}

/// An iterator over all values with a given tag of an abstract object
/// under the knowledge.
#[derive(Clone)]
pub struct AbstractValues {
    pub(super) properties: AbstractProperties,
    pub(super) tag: Object,
}

impl Iterator for AbstractValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.properties.find_map(|property| {
            if property.tag == self.tag {
                Some(property.value)
            } else {
                None
            }
        })
    }
}

/// An iterator over all tags with a given value of an abstract object
/// under the knowledge.
#[derive(Clone)]
pub struct AbstractTags {
    pub(super) properties: AbstractProperties,
    pub(super) value: Object,
}

impl Iterator for AbstractTags {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.properties.find_map(|property| {
            if property.value == self.value {
                Some(property.tag)
            } else {
                None
            }
        })
    }
}
