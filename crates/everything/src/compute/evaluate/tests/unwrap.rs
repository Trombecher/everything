use std::assert_matches;

use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn unwrap() {
    let cases = [
        (
            Object::Composite(Composite::from([])),
            Object::Atom(Atom::UNDEFINED),
        ),
        (
            Object::Composite(Composite::from([Object::Atom(Atom(42))])),
            Object::Atom(Atom(42)),
        ),
        (
            Object::Composite(Composite::from([
                Object::Atom(Atom(42)),
                Object::Atom(Atom(56)),
            ])),
            Object::Atom(Atom::UNDEFINED),
        ),
    ];

    for (source, unwrapped) in cases {
        assert_matches!(
            evaluate(&Node::Unwrap(Node::Literal(source).into()), &BASE),
            LazyObject::Eager(object) if object == unwrapped
        );
    }
}

#[test]
fn unwrap_requires_composites() {
    let atom = Atom(42);
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));

    let unwrap = |node: Node| evaluate(&Node::Unwrap(node.into()), &BASE).into_object(&BASE);

    assert_eq!(
        unwrap(Node::Literal(Object::Atom(atom))),
        Atom::UNDEFINED.into()
    );
    assert_eq!(unwrap(Node::Literal(construction)), Atom::UNDEFINED.into());

    // A function object is a composite of its properties.
    assert_eq!(
        unwrap(identity_function()),
        Construction::new(
            Atom::NODE_FUNCTION.into(),
            Node::Parameter { depth: 0 }.into_object()
        )
        .into()
    );
}
