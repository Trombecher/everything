use everything_structures::{Abstract, Object, Structure};

use crate::ext::{AbstractExt, ObjectExt};

pub fn subjects_axiomatically(
    knowledge: &Structure,
    tag: Object,
    value: Object,
) -> impl Iterator<Item = Object> {
    // Knowledge will almost never be a specialization
    // which means that this properties() call is cheap.

    knowledge.properties().filter_map(move |property| {
        if property.tag != Object::Abstract(Abstract::CONTAINS) {
            return None;
        }

        let statement = property.value.structure().unwrap();
        let statement_tag = statement
            .values(Abstract::STATEMENT_TAG.into())
            .next()
            .unwrap();

        if statement_tag != tag {
            return None;
        }

        let statement_value = statement
            .values(Abstract::STATEMENT_VALUE.into())
            .next()
            .unwrap();

        if statement_value != value {
            return None;
        }

        Some(
            statement
                .values(Abstract::STATEMENT_SUBJECT.into())
                .next()
                .unwrap(),
        )
    })
}
