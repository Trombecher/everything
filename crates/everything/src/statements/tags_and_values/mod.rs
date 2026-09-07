use everything_objects::{CompositeProperties, Property};

use crate::statements::AbstractProperties;

pub enum QueryTagsAndValues2 {
    Abstract(AbstractProperties),
    Composite(CompositeProperties),
}

impl Iterator for QueryTagsAndValues2 {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Abstract(iter) => iter.next().map(Into::into),
            Self::Composite(iter) => iter.next(),
        }
    }
}
