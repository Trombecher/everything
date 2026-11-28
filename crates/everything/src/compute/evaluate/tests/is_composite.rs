use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::common::*;
use crate::compute::*;

#[test]
fn function_values_are_composites() {
    assert_eq!(
        evaluate(&Node::IsComposite(identity_function().into()), &BASE).into_object(&BASE),
        Atom::TRUE.into()
    );
    assert_eq!(
        evaluate(
            &Node::IsComposite(Node::Literal(Object::Atom(Atom(42))).into()),
            &BASE
        )
        .into_object(&BASE),
        Atom::FALSE.into()
    );
    assert_eq!(
        evaluate(
            &Node::IsComposite(
                Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()).into()
            ),
            &BASE
        )
        .into_object(&BASE),
        Atom::FALSE.into()
    );
}

#[test]
fn undefined_is_not_a_composite() {
    assert_eq!(
        evaluate(
            &Node::IsComposite(Node::Literal(Object::Atom(Atom::UNDEFINED)).into()),
            &BASE
        )
        .into_object(&BASE),
        Atom::FALSE.into()
    );
}

#[test]
fn deferred_sets_are_composites() {
    let union = Node::SetUnion {
        first: Node::Literal(Object::Composite(Composite::empty())).into(),
        second: Node::Literal(Object::Composite(Composite::empty())).into(),
    };

    assert_eq!(
        evaluate(&Node::IsComposite(union.into()), &BASE).into_object(&BASE),
        Atom::TRUE.into()
    );
}
