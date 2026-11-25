use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

fn contains(element: Node, composite: Node, knowledge: &Knowledge) -> Object {
    evaluate(
        &Node::Contains {
            element: element.into(),
            composite: composite.into(),
        },
        knowledge,
    )
    .into_object(knowledge)
}

#[test]
fn contains_evaluated_composites() {
    let composite = Node::Literal(Object::Composite(integer_composite()));

    assert_eq!(
        contains(Node::Literal(integer(2)), composite.clone(), &BASE),
        Atom::TRUE.into()
    );
    assert_eq!(
        contains(Node::Literal(integer(4)), composite, &BASE),
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
fn contains_deferred_composites() {
    let union = Node::Union {
        first: Node::Literal(Object::Composite(Composite::from([integer(1)]))).into(),
        second: Node::Literal(Object::Composite(Composite::from([integer(2), integer(3)]))).into(),
    };

    assert_eq!(
        contains(Node::Literal(integer(3)), union.clone(), &BASE),
        Atom::TRUE.into()
    );
    assert_eq!(
        contains(Node::Literal(integer(4)), union, &BASE),
        Atom::FALSE.into()
    );

    let filter = Node::Filter {
        composite: Node::Literal(Object::Composite(integer_composite())).into(),
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
fn contains_atom_values() {
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
fn contains_materializes_deferred_elements() {
    let element = Node::Union {
        first: Node::Literal(Object::Composite(Composite::from([integer(2)]))).into(),
        second: Node::Literal(Object::Composite(Composite::from([integer(3)]))).into(),
    };

    // The deferred element is materialized to `{2, 3}`; it is not a member of
    // the integers composite, but it is a member of a composite that contains it as a whole.
    assert_eq!(
        contains(
            element.clone(),
            Node::Literal(Object::Composite(integer_composite())),
            &BASE
        ),
        Atom::FALSE.into()
    );
    assert_eq!(
        contains(
            element,
            Node::Literal(Object::Composite(Composite::from([Object::Composite(
                Composite::from([integer(2), integer(3),])
            )]))),
            &BASE
        ),
        Atom::TRUE.into()
    );
}

#[test]
fn function_values_are_composites() {
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
        Atom::TRUE.into()
    );
}

#[test]
fn contains_requires_a_composite() {
    let atom = Node::Literal(Object::Atom(Atom(42)));
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    for invalid in [atom, construction] {
        assert_eq!(
            contains(Node::Literal(integer(1)), invalid, &BASE),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn undefined_is_a_first_class_value() {
    assert_eq!(
        contains(
            Node::Literal(Atom::UNDEFINED.into()),
            Node::Literal(Object::Composite(Composite::from([Atom::UNDEFINED.into()]))),
            &BASE
        ),
        Atom::TRUE.into()
    );
    assert_eq!(
        contains(
            Node::Literal(Atom::UNDEFINED.into()),
            Node::Literal(Object::Composite(integer_composite())),
            &BASE
        ),
        Atom::FALSE.into()
    );
}
