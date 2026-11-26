use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

fn contains(element: Node, set: Node, knowledge: &Knowledge) -> Object {
    evaluate(
        &Node::SetContains {
            element: element.into(),
            set: set.into(),
        },
        knowledge,
    )
    .into_object(knowledge)
}

#[test]
fn set_contains_evaluated_sets() {
    let set = Node::Literal(Object::Composite(integer_set()));

    assert_eq!(
        contains(Node::Literal(integer(2)), set.clone(), &BASE),
        Atom::TRUE.into()
    );
    assert_eq!(
        contains(Node::Literal(integer(4)), set, &BASE),
        Atom::FALSE.into()
    );
    assert_eq!(
        contains(
            Node::Literal(integer(1)),
            Node::Literal(Object::Composite(Composite::empty())),
            &BASE
        ),
        Atom::FALSE.into()
    );
}

#[test]
fn set_contains_deferred_sets() {
    let union = Node::SetUnion {
        first: Node::Literal(set_of([integer(1)])).into(),
        second: Node::Literal(set_of([integer(2), integer(3)])).into(),
    };

    assert_eq!(
        contains(Node::Literal(integer(3)), union.clone(), &BASE),
        Atom::TRUE.into()
    );
    assert_eq!(
        contains(Node::Literal(integer(4)), union, &BASE),
        Atom::FALSE.into()
    );

    let filter = Node::SetFilter {
        set: Node::Literal(Object::Composite(integer_set())).into(),
        filter_function: equals_two_predicate().into(),
    };

    assert_eq!(
        contains(Node::Literal(integer(2)), filter.clone(), &BASE),
        Atom::TRUE.into()
    );
    assert_eq!(
        contains(Node::Literal(integer(1)), filter, &BASE),
        Atom::FALSE.into()
    );
}

#[test]
fn set_contains_atom_values() {
    let atom = Atom(42);
    let value = integer(1);

    let mut knowledge = BASE.clone();

    knowledge.add_statement(Statement {
        atom,
        object: value.clone(),
    });

    let values = Node::PropertiesOf(Node::Literal(Object::Atom(atom)).into());

    assert_eq!(
        contains(Node::Literal(value), values.clone(), &knowledge),
        Atom::TRUE.into()
    );
    assert_eq!(
        contains(Node::Literal(integer(2)), values, &knowledge),
        Atom::FALSE.into()
    );
}

#[test]
fn set_contains_materializes_deferred_items() {
    let element = Node::SetUnion {
        first: Node::Literal(set_of([integer(2)])).into(),
        second: Node::Literal(set_of([integer(3)])).into(),
    };

    // The deferred element is materialized to `{2, 3}`. It is not an item of
    // the integers set, but it is an item of a set that contains it as a whole.
    assert_eq!(
        contains(
            element.clone(),
            Node::Literal(Object::Composite(integer_set())),
            &BASE
        ),
        Atom::FALSE.into()
    );
    assert_eq!(
        contains(
            element,
            Node::Literal(set_of([set_of([integer(2), integer(3)])])),
            &BASE
        ),
        Atom::TRUE.into()
    );
}

#[test]
fn set_contains_ignores_non_item_properties() {
    let node_function = Construction::new(
        Atom::NODE_FUNCTION.into(),
        Node::Parameter { depth: 0 }.into_object(),
    );

    assert_eq!(
        contains(
            Node::Literal(node_function.into()),
            identity_function(),
            &BASE
        ),
        Atom::FALSE.into()
    );
}

#[test]
fn set_contains_requires_a_set() {
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    assert_eq!(
        contains(Node::Literal(integer(1)), construction, &BASE),
        Atom::UNDEFINED.into()
    );

    // An atom is a set with no items.
    assert_eq!(
        contains(
            Node::Literal(integer(1)),
            Node::Literal(Object::Atom(Atom(42))),
            &BASE
        ),
        Atom::FALSE.into()
    );
}

#[test]
fn set_contains_does_not_evaluate_the_element_when_the_set_is_a_construction() {
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    // The element child would not terminate if it were evaluated.
    assert_eq!(
        contains(diverging_term(), construction, &BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn undefined_is_a_first_class_value() {
    assert_eq!(
        contains(
            Node::Literal(Atom::UNDEFINED.into()),
            Node::Literal(set_of([Atom::UNDEFINED.into()])),
            &BASE
        ),
        Atom::TRUE.into()
    );
    assert_eq!(
        contains(
            Node::Literal(Atom::UNDEFINED.into()),
            Node::Literal(Object::Composite(integer_set())),
            &BASE
        ),
        Atom::FALSE.into()
    );
}
