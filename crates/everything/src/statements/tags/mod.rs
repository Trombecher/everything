use everything_objects::{CompositeTags, Object};

use crate::statements::AbstractProperties;

#[derive(Clone)]
pub struct AbstractTags {
    properties: AbstractProperties,
}

impl Iterator for AbstractTags {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Clone)]
pub enum QueryTags2 {
    Abstract(AbstractTags),
    Composite(CompositeTags),
}

impl Iterator for QueryTags2 {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Abstract(iter) => iter.next(),
            Self::Composite(iter) => iter.next(),
        }
    }
}
