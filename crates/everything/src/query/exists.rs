use everything_structures::{Object, Structure};

use crate::query::QueryValues;

#[derive(Debug, Clone)]
pub struct QueryExists;

impl QueryExists {
    pub fn new(knowledge: &Structure, subject: Object, tag: Object, value: Object) -> bool {
        QueryValues::new(knowledge, subject, tag)
            .find(|v| v == &value)
            .is_some()
    }
}
