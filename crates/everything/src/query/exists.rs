use everything_structures::{Object, Structure};

use crate::{ctx::EvaluationContext, query};

pub fn exists(
    knowledge: &Structure,
    subject: Object,
    tag: Object,
    value: Object,
    context: &mut EvaluationContext,
) -> bool {
    let result = query::values(knowledge, subject, tag, context);
    result.set_values(knowledge).find(|v| v == &value).is_some()
}
