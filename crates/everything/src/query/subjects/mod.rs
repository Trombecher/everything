use std::borrow::Cow;

use everything_structures::{Object, Structure};

use crate::ext::ObjectExt;

pub fn subjects_axiomatically<'knowledge>(
    knowledge: &'knowledge Structure,
    tag: Object,
    value: Object,
) -> impl Iterator<Item = Cow<'knowledge, Object>> {
    knowledge.properties().filter_map(move |property| {
        if property.tag != Object::CONTAINS {
            return None;
        }

        let statement = property.value.structure().unwrap();

        let statement_tag = statement.values(Object::STATEMENT_TAG).next().unwrap();

        if statement_tag.as_ref() != &tag {
            return None;
        }

        let statement_value = statement.values(Object::STATEMENT_VALUE).next().unwrap();

        if statement_value.as_ref() != &value {
            return None;
        }

        Some(Cow::Owned(
            statement
                .values(Object::STATEMENT_SUBJECT)
                .next()
                .unwrap()
                .into_owned(),
        ))
    })
}
