use std::assert_matches;

use num_bigint::BigInt;

use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn properties_of() {
    let atom = Atom(42);
    let value = Object::new_integer(BigInt::from(1));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: value.clone(),
    });

    let composite = Composite::from([Object::Atom(atom)]);

    assert_eq!(
        evaluate(
            &Node::PropertiesOf(Node::Literal(composite.clone().into()).into()),
            &knowledge,
        )
        .into_object(&knowledge),
        Object::Composite(composite)
    );

    assert_matches!(
        evaluate(&Node::PropertiesOf(identity_function().into()), &knowledge),
        LazyObject::Node(node) if node == identity_function()
    );

    assert_eq!(
        evaluate(
            &Node::PropertiesOf(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        )
        .into_object(&knowledge),
        Object::Composite(Composite::from_set_items(std::slice::from_ref(&value)))
    );

    assert_matches!(
        evaluate(
            &Node::PropertiesOf(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        ),
        LazyObject::Set(LazySet::AtomProperties(properties)) if properties.size() == 1
    );

    assert_eq!(
        evaluate(
            &Node::SetCount(Node::PropertiesOf(Node::Literal(Object::Atom(atom)).into()).into()),
            &knowledge,
        )
        .into_object(&knowledge),
        Object::new_integer(BigInt::from(1))
    );

    let properties_of = evaluate(
        &Node::PropertiesOf(Node::Literal(Object::Atom(atom)).into()),
        &knowledge,
    );

    assert_eq!(
        set(&properties_of).iter(&knowledge).collect::<Vec<_>>(),
        vec![value]
    );

    let construction = Construction::new(Atom(1).into(), Atom(2).into());

    assert_eq!(
        evaluate(
            &Node::PropertiesOf(Node::Literal(construction.into()).into()),
            &knowledge,
        )
        .into_object(&knowledge),
        Atom::UNDEFINED.into()
    );

    assert_eq!(
        evaluate(
            &Node::PropertiesOf(Node::Literal(Atom::UNDEFINED.into()).into()),
            &knowledge
        )
        .into_object(&knowledge),
        Object::Composite(Composite::empty())
    );

    let undefined_value = Object::new_integer(BigInt::from(2));

    knowledge.add_statement(Statement {
        atom: Atom::UNDEFINED,
        object: undefined_value.clone(),
    });

    assert_eq!(
        evaluate(
            &Node::PropertiesOf(Node::Literal(Atom::UNDEFINED.into()).into()),
            &knowledge
        )
        .into_object(&knowledge),
        Object::Composite(Composite::from_set_items(&[undefined_value]))
    );
}
