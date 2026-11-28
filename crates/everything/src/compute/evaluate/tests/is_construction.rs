use crate::{
    base::BASE,
    model::{Atom, Object},
};

use crate::compute::*;

#[test]
fn undefined_is_not_a_construction() {
    assert_eq!(
        evaluate(
            &Node::IsConstruction(Node::Literal(Object::Atom(Atom::UNDEFINED)).into()),
            &BASE
        )
        .into_object(&BASE),
        Atom::FALSE.into()
    );
}
