use everything_structures::{Object, Structure};

use crate::query::{self, QueryValues};

pub fn exists(knowledge: &Structure, subject: Object, tag: Object, value: Object) -> QueryExists {
    match query::values(knowledge, subject, tag) {
        QueryValues::Axiomatically(mut values) => {
            QueryExists::Axiomatically(values.find(|v| v == &value).is_some())
        }
        QueryValues::Call {
            function_body,
            parameter,
        } => QueryExists::Call {
            function_body,
            parameter,
        },
    }
}

#[derive(Debug, Clone)]
pub enum QueryExists {
    Axiomatically(bool),
    Call {
        function_body: Object,
        parameter: Object,
    },
}
