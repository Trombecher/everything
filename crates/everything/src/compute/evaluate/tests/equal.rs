use crate::{
    base::BASE,
    model::{Atom, Object},
};

use crate::compute::*;

fn equal(first: Object, second: Object) -> Object {
    evaluate(
        &Node::Equal {
            first: Node::Literal(first).into(),
            second: Node::Literal(second).into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}

#[test]
fn undefined_equals_undefined() {
    assert_eq!(
        equal(Object::Atom(Atom::UNDEFINED), Object::Atom(Atom::UNDEFINED)),
        Atom::TRUE.into()
    );
}

#[test]
fn undefined_does_not_equal_an_atom() {
    assert_eq!(
        equal(Object::Atom(Atom::UNDEFINED), Object::Atom(Atom(42))),
        Atom::FALSE.into()
    );
}
