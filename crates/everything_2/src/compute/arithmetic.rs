use crate::model::{Atom, Object};

pub fn add(a: &Object, b: &Object) -> Object {
    if let Some(left) = a.integer()
        && let Some(right) = b.integer()
    {
        if let Some(sum) = left.checked_add(right) {
            Object::new_integer(sum)
        } else {
            todo!("overflow / replace this with bigint")
        }
    } else {
        Atom::UNDEFINED.into()
    }
}

pub fn multiply(a: &Object, b: &Object) -> Object {
    if let Some(left) = a.integer()
        && let Some(right) = b.integer()
    {
        if let Some(product) = left.checked_mul(right) {
            Object::new_integer(product)
        } else {
            todo!("overflow / replace this with bigint")
        }
    } else {
        Atom::UNDEFINED.into()
    }
}
