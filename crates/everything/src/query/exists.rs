use everything_structures::{Object, Structure};

use crate::query::QueryValues;

#[derive(Debug, Clone)]
pub enum QueryExists {
    Axiomatically(bool),
    Call {
        function_body: Object,
        parameter: Object,
    },
}

impl QueryExists {
    pub fn new(knowledge: &Structure, subject: Object, tag: Object, value: Object) -> Self {
        match QueryValues::new(knowledge, subject, tag) {
            QueryValues::Axiomatically(mut values) => {
                Self::Axiomatically(values.find(|v| v == &value).is_some())
            }
            QueryValues::Call {
                function_body,
                parameter,
            } => Self::Call {
                function_body,
                parameter,
            },
        }
    }
}
