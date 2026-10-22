use crate::{
    compute::{LazyObject, MaybeUndefined},
    knowledge::Knowledge,
    model::Object,
};

#[must_use]
pub fn add(a: &Object, b: &Object) -> MaybeUndefined<i128> {
    if let Some(left) = a.integer()
        && let Some(right) = b.integer()
    {
        if let Some(sum) = left.checked_add(right) {
            MaybeUndefined::Defined(sum)
        } else {
            todo!("overflow / replace this with bigint")
        }
    } else {
        MaybeUndefined::Undefined
    }
}

#[must_use]
pub fn multiply(a: &Object, b: &Object) -> MaybeUndefined<i128> {
    if let Some(left) = a.integer()
        && let Some(right) = b.integer()
    {
        if let Some(product) = left.checked_mul(right) {
            MaybeUndefined::Defined(product)
        } else {
            todo!("overflow / replace this with bigint")
        }
    } else {
        MaybeUndefined::Undefined
    }
}

/// Behaviour of `first < second`.
///
/// - If both are integers, then it returns a defined `first < second`;
/// - otherwise [`MaybeUnderfined::Undefined`].
#[must_use]
pub fn less(first: &Object, second: &Object) -> MaybeUndefined<bool> {
    if let Some(first_integer) = first.integer()
        && let Some(second_integer) = second.integer()
    {
        MaybeUndefined::Defined(first_integer < second_integer)
    } else {
        MaybeUndefined::Undefined
    }
}

#[must_use]
pub fn to_boolean(object: &LazyObject, knowledge: &Knowledge) -> bool {
    object.has_at_least_one_element(knowledge)
}
