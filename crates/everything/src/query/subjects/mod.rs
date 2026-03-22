use everything_structures::{Object, Structure};

use crate::ext::ObjectExt;

pub fn subjects_axiomatically(
    knowledge: &Structure,
    tag: Object,
    value: Object,
) -> impl Iterator<Item = &Object> {
    knowledge
        .as_ref()
        .iter()
        .filter_map(|property| (property.tag == Object::CONTAINS).then_some(&property.value))
        .filter_map(move |statement| {
            let statement = statement.structure().unwrap();

            let statement_tag = statement.values(Object::STATEMENT_TAG).next().unwrap();

            if statement_tag != &tag {
                return None;
            }

            let statement_value = statement.values(Object::STATEMENT_VALUE).next().unwrap();

            if statement_value != &value {
                return None;
            }

            Some(statement.values(Object::STATEMENT_SUBJECT).next().unwrap())
        })
}
