use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn every_of_an_empty_composite_is_true() {
    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let invalid_predicates = [
        Node::Literal(Atom::UNDEFINED.into()),
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Union {
            first: empty_composite.clone().into(),
            second: empty_composite.clone().into(),
        },
    ];

    assert_eq!(
        every_result(empty_composite.clone(), always_true_predicate()),
        Atom::TRUE.into()
    );

    for predicate in invalid_predicates {
        assert_eq!(
            every_result(empty_composite.clone(), predicate),
            Atom::TRUE.into()
        );
    }
}

#[test]
fn every_of_an_empty_deferred_composite_is_true() {
    let drop_everything = Node::function(Node::Literal(Atom::FALSE.into()));
    let empty_composite = Node::Filter {
        composite: Node::Literal(integer_composite().into()).into(),
        filter_function: drop_everything.into(),
    };

    assert_eq!(
        every_result(empty_composite, Node::Literal(Atom::UNDEFINED.into())),
        Atom::TRUE.into()
    );
}

#[test]
fn every_requires_a_composite() {
    let invalid_composites = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
    ];

    for composite in invalid_composites {
        assert_eq!(
            every_result(composite, always_true_predicate()),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn every_requires_a_function_predicate() {
    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let invalid_predicates = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
        Node::Union {
            first: empty_composite.clone().into(),
            second: empty_composite.into(),
        },
    ];

    for predicate in invalid_predicates {
        assert_eq!(
            every_result(Node::Literal(integer_composite().into()), predicate),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn every_returns_true_when_all_calls_are_true() {
    let composite = Node::Literal(Object::Composite(Composite::from([integer(2)])));

    assert_eq!(
        every_result(composite, equals_two_predicate()),
        Atom::TRUE.into()
    );
    assert_eq!(
        every_result(
            Node::Literal(integer_composite().into()),
            always_true_predicate()
        ),
        Atom::TRUE.into()
    );
}

#[test]
fn every_returns_false_when_a_call_is_false() {
    assert_eq!(
        every_result(
            Node::Literal(integer_composite().into()),
            equals_two_predicate()
        ),
        Atom::FALSE.into()
    );
}

#[test]
fn every_returns_undefined_when_a_call_is_undefined() {
    assert_eq!(
        every_result(
            Node::Literal(integer_composite().into()),
            constant_predicate(Atom::UNDEFINED.into()),
        ),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn every_returns_undefined_when_a_call_is_not_boolean() {
    let non_boolean_results = [
        integer(0),
        Object::Composite(Composite::empty()),
        Construction::new(Atom(1).into(), Atom(2).into()).into(),
        Object::new_node(identity_function()),
    ];

    for result in non_boolean_results {
        assert_eq!(
            every_result(
                Node::Literal(integer_composite().into()),
                constant_predicate(result)
            ),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn every_short_circuits_on_false() {
    assert_eq!(
        every_result(
            Node::Literal(integer_composite().into()),
            false_on_one_undefined_otherwise_predicate(),
        ),
        Atom::FALSE.into()
    );
}

#[test]
fn every_short_circuits_on_undefined() {
    assert_eq!(
        every_result(
            Node::Literal(integer_composite().into()),
            undefined_on_one_false_otherwise_predicate(),
        ),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn every_iterates_deferred_composites() {
    let doubled = Node::Map {
        composite: Node::Literal(integer_composite().into()).into(),
        mapper_function: Node::function(Node::Add {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Parameter { depth: 0 }.into(),
        })
        .into(),
    };
    let is_positive = Node::function(Node::Less {
        first: Node::Literal(integer(0)).into(),
        second: Node::Parameter { depth: 0 }.into(),
    });

    assert_eq!(every_result(doubled, is_positive), Atom::TRUE.into());
}

#[test]
fn every_captures_the_evaluation_context() {
    let outer_function = Node::function(Node::Every {
        composite: Node::Literal(integer_composite().into()).into(),
        predicate: Node::function(Node::Less {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Parameter { depth: 1 }.into(),
        })
        .into(),
    });

    let call = |value| {
        evaluate(
            &Node::Call {
                callee: outer_function.clone().into(),
                with: Node::Literal(integer(value)).into(),
            },
            &BASE,
        )
        .into_object(&BASE)
    };

    assert_eq!(call(5), Atom::TRUE.into());
    assert_eq!(call(2), Atom::FALSE.into());
}

#[test]
fn every_matches_and_folding() {
    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let integer_composite_node = || Node::Literal(integer_composite().into());
    let cases = [
        (empty_composite.clone(), always_true_predicate()),
        (
            empty_composite.clone(),
            Node::Literal(Atom::UNDEFINED.into()),
        ),
        (integer_composite_node(), always_true_predicate()),
        (integer_composite_node(), equals_two_predicate()),
        (
            integer_composite_node(),
            constant_predicate(Atom::UNDEFINED.into()),
        ),
        (integer_composite_node(), constant_predicate(integer(0))),
        (
            integer_composite_node(),
            false_on_one_undefined_otherwise_predicate(),
        ),
        (
            integer_composite_node(),
            undefined_on_one_false_otherwise_predicate(),
        ),
    ];

    for (composite, predicate) in cases {
        let every = evaluate(
            &Node::Every {
                composite: composite.clone().into(),
                predicate: predicate.clone().into(),
            },
            &BASE,
        )
        .into_object(&BASE);

        assert_eq!(every, every_by_and_fold(composite, &predicate));
    }
}
