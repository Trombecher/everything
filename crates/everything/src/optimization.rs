use std::sync::LazyLock;

use everything_objects::{Composite, Object};

use crate::{ObjectOrSetValues, base::IS_INTEGER, ext::CompositeExt, statements::Statements};

#[allow(clippy::type_complexity)]
pub static OPTIMIZED_FUNCTIONS: LazyLock<
    [(Object, fn(&Statements, ObjectOrSetValues) -> Object); 1],
> = LazyLock::new(|| {
    [(IS_INTEGER.clone(), |_statements, object_or_set_values| {
        Composite::new_bool(match object_or_set_values {
            ObjectOrSetValues::Object(object) => object.integer().is_some(),
            ObjectOrSetValues::SetValues(_) => false,
        })
        .into()
    })]
});
