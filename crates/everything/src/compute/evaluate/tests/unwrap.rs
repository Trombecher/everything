use std::assert_matches;

use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn set_unwrap() {
    let cases = [
        (
            Object::Composite(Composite::empty()),
            Object::Atom(Atom::UNDEFINED),
        ),
        (set_of([Object::Atom(Atom(42))]), Object::Atom(Atom(42))),
        (
            set_of([Object::Atom(Atom(42)), Object::Atom(Atom(56))]),
            Object::Atom(Atom::UNDEFINED),
        ),
    ];

    for (source, unwrapped) in cases {
        assert_matches!(
            evaluate(&Node::SetUnwrap(Node::Literal(source).into()), &BASE),
            LazyObject::Eager(object) if object == unwrapped
        );
    }
}

#[test]
fn set_unwrap_requires_a_set() {
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));

    let unwrap = |node: Node| evaluate(&Node::SetUnwrap(node.into()), &BASE).into_object(&BASE);

    assert_eq!(unwrap(Node::Literal(construction)), Atom::UNDEFINED.into());

    // An atom and a function object are sets with no items.
    assert_eq!(
        unwrap(Node::Literal(Object::Atom(Atom(42)))),
        Atom::UNDEFINED.into()
    );
    assert_eq!(unwrap(identity_function()), Atom::UNDEFINED.into());
}

#[test]
fn set_unwrap_of_an_atom_with_one_item() {
    let atom = Atom(42);
    let value = integer(7);
    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(Atom::CONTAINS.into(), value.clone()).into(),
    });

    assert_eq!(
        evaluate(
            &Node::SetUnwrap(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        )
        .into_object(&knowledge),
        value
    );
}
