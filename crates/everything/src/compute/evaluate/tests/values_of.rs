use std::assert_matches;

use num_bigint::BigInt;

use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;
use super::common::*;

#[test]
fn values_of() {
    let atom = Atom(42);
    let value = Object::new_integer(BigInt::from(1));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: value.clone(),
    });

    let set = Set::from([Object::Atom(atom)]);

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(set.clone().into()).into()),
            &knowledge,
        )
        .into_object(&knowledge),
        Object::Set(set)
    );

    assert_matches!(
        evaluate(&Node::ValuesOf(identity_function().into()), &knowledge),
        LazyObject::Node(node) if node == identity_function()
    );

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        )
        .into_object(&knowledge),
        Object::Set(Set::from([value.clone()]))
    );

    assert_matches!(
        evaluate(
            &Node::ValuesOf(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        ),
        LazyObject::AtomValues(values) if values.size() == 1
    );

    assert_eq!(
        evaluate(
            &Node::Count(Node::ValuesOf(Node::Literal(Object::Atom(atom)).into()).into()),
            &knowledge,
        )
        .into_object(&knowledge),
        Object::new_integer(BigInt::from(1))
    );

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        )
        .iter(&knowledge)
        .collect::<Vec<_>>(),
        vec![value]
    );

    let construction = Construction::new(Atom(1).into(), Atom(2).into());

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(construction.into()).into()),
            &knowledge,
        )
        .into_object(&knowledge),
        Atom::UNDEFINED.into()
    );

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(Atom::UNDEFINED.into()).into()),
            &knowledge
        )
        .into_object(&knowledge),
        Object::Set(Set::empty())
    );

    let undefined_value = Object::new_integer(BigInt::from(2));

    knowledge.add_statement(Statement {
        atom: Atom::UNDEFINED,
        object: undefined_value.clone(),
    });

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(Atom::UNDEFINED.into()).into()),
            &knowledge
        )
        .into_object(&knowledge),
        Object::Set(Set::from([undefined_value]))
    );
}
