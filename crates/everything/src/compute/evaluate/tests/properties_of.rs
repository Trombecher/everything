use std::assert_matches;

use num_bigint::BigInt;

use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

fn contains(item: Object) -> Object {
    Construction::new(Atom::CONTAINS.into(), item).into()
}

#[test]
fn properties_of_atom() {
    let atom = Atom(42);
    let value = Object::new_integer(BigInt::from(1));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: value.clone(),
    });

    assert_eq!(
        evaluate(
            &Node::PropertiesOf(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        )
        .into_object(&knowledge),
        set_of([value.clone()])
    );

    assert_matches!(
        evaluate(
            &Node::PropertiesOf(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        ),
        LazyObject::Set(LazySet::PropertiesOf { .. })
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
}

#[test]
fn properties_of_composite_lists_all_members_as_items() {
    let value = Object::new_integer(BigInt::from(1));
    let annotation = Object::Atom(Atom(43));
    let member = contains(value);

    let composite = Composite::from([annotation.clone(), member.clone()]);

    assert_eq!(
        evaluate(
            &Node::PropertiesOf(Node::Literal(composite.into()).into()),
            &BASE,
        )
        .into_object(&BASE),
        set_of([annotation, member])
    );
}

#[test]
fn properties_of_composite_without_contains_members_is_not_empty() {
    let annotation = Object::Atom(Atom(43));

    let properties = evaluate(
        &Node::PropertiesOf(Node::Literal(composite_of([annotation.clone()])).into()),
        &BASE,
    );

    assert_eq!(set(&properties).item_count(&BASE), 1);
    assert_eq!(
        set(&properties).iter(&BASE).collect::<Vec<_>>(),
        vec![annotation]
    );
}

#[test]
fn properties_of_function_value_lists_its_members() {
    let function = identity_function();
    let function_object = function.clone().into_object();

    let Object::Composite(function_composite) = function_object else {
        panic!("a function value materialises as a composite");
    };

    let members = function_composite.into_iter().collect::<Vec<_>>();

    let properties = evaluate(&Node::PropertiesOf(function.into()), &BASE);

    assert_eq!(
        properties.clone().into_object(&BASE),
        set_of(members.clone())
    );
    assert_eq!(set(&properties).item_count(&BASE), members.len() as u64);
    assert_eq!(set(&properties).iter(&BASE).collect::<Vec<_>>(), members);
}

#[test]
fn properties_of_deferred_set_lists_its_contains_properties() {
    let first = integer(1);
    let second = integer(2);

    let deferred_set = Node::SetUnion {
        first: Node::Literal(set_of([first.clone()])).into(),
        second: Node::Literal(set_of([second.clone()])).into(),
    };

    let properties = evaluate(&Node::PropertiesOf(deferred_set.into()), &BASE);

    let expected_items = vec![contains(first), contains(second)];

    assert_eq!(
        properties.clone().into_object(&BASE),
        set_of(expected_items.clone())
    );
    assert_eq!(
        set(&properties).iter(&BASE).collect::<Vec<_>>(),
        expected_items
    );
}

#[test]
fn properties_of_construction_is_undefined() {
    let construction = Construction::new(Atom(1).into(), Atom(2).into());

    assert_eq!(
        evaluate(
            &Node::PropertiesOf(Node::Literal(construction.into()).into()),
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn properties_of_atom_without_statements_is_empty() {
    assert_eq!(
        evaluate(
            &Node::PropertiesOf(Node::Literal(Atom::UNDEFINED.into()).into()),
            &BASE
        )
        .into_object(&BASE),
        Object::Composite(Composite::empty())
    );

    let undefined_value = Object::new_integer(BigInt::from(2));

    let mut knowledge = BASE.clone();
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
        set_of([undefined_value])
    );
}
