use everything_structures::{Object, Structure};

use crate::ext::ObjectExt;

pub fn subjects_axiomatically(
    knowledge: &Structure,
    tag: Object,
    value: Object,
) -> impl Iterator<Item = Object> {
    // Knowledge will almost never be a specialization
    // which means that this properties() call is cheap.

    knowledge.properties().filter_map(move |property| {
        if property.tag != Object::CONTAINS {
            return None;
        }

        let statement = property.value.structure().unwrap();
        let statement_tag = statement.values(Object::STATEMENT_TAG).next().unwrap();

        if statement_tag != tag {
            return None;
        }

        let statement_value = statement.values(Object::STATEMENT_VALUE).next().unwrap();

        if statement_value != value {
            return None;
        }

        Some(statement.values(Object::STATEMENT_SUBJECT).next().unwrap())
    })
}
