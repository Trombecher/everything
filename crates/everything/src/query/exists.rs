use everything_objects::{Composite, Object};

use crate::query::QueryValues;

#[derive(Debug, Clone)]
pub struct QueryExists;

impl QueryExists {
    pub fn new(knowledge: &Composite, subject: Object, tag: Object, value: Object) -> bool {
        QueryValues::new(knowledge, subject, tag)
            .find(|v| v == &value)
            .is_some()
    }
}
