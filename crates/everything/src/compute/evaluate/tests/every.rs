use crate::{
    base::BASE,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;
use super::common::*;

#[test]
fn every_of_an_empty_set_is_true() {
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let invalid_predicates = [
        Node::Literal(Atom::UNDEFINED.into()),
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Union {
            first: empty_set.clone().into(),
            second: empty_set.clone().into(),
        },
    ];

    assert_eq!(
        every_result(empty_set.clone(), always_true_predicate()),
        Atom::TRUE.into()
    );

    for predicate in invalid_predicates {
        assert_eq!(
            every_result(empty_set.clone(), predicate),
            Atom::TRUE.into()
        );
    }
}

#[test]
fn every_of_an_empty_deferred_set_is_true() {
    let drop_everything = Node::function(Node::Literal(Atom::FALSE.into()));
    let empty_set = Node::Filter {
        set: Node::Literal(integer_set().into()).into(),
        filter_function: drop_everything.into(),
    };

    assert_eq!(
        every_result(empty_set, Node::Literal(Atom::UNDEFINED.into())),
        Atom::TRUE.into()
    );
}

#[test]
fn every_requires_a_set() {
    let invalid_sets = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
    ];

    for set in invalid_sets {
        assert_eq!(
            every_result(set, always_true_predicate()),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn every_requires_a_function_predicate() {
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let invalid_predicates = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
        Node::Union {
            first: empty_set.clone().into(),
            second: empty_set.into(),
        },
    ];

    for predicate in invalid_predicates {
        assert_eq!(
            every_result(Node::Literal(integer_set().into()), predicate),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn every_returns_true_when_all_calls_are_true() {
    let set = Node::Literal(Object::Set(Set::from([integer(2)])));

    assert_eq!(every_result(set, equals_two_predicate()), Atom::TRUE.into());
    assert_eq!(
        every_result(Node::Literal(integer_set().into()), always_true_predicate()),
        Atom::TRUE.into()
    );
}

#[test]
fn every_returns_false_when_a_call_is_false() {
    assert_eq!(
        every_result(Node::Literal(integer_set().into()), equals_two_predicate()),
        Atom::FALSE.into()
    );
}

#[test]
fn every_returns_undefined_when_a_call_is_undefined() {
    assert_eq!(
        every_result(
            Node::Literal(integer_set().into()),
            constant_predicate(Atom::UNDEFINED.into()),
        ),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn every_returns_undefined_when_a_call_is_not_boolean() {
    let non_boolean_results = [
        integer(0),
        Object::Set(Set::empty()),
        Construction::new(Atom(1).into(), Atom(2).into()).into(),
        Object::new_node(identity_function()),
    ];

    for result in non_boolean_results {
        assert_eq!(
            every_result(
                Node::Literal(integer_set().into()),
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
            Node::Literal(integer_set().into()),
            false_on_one_undefined_otherwise_predicate(),
        ),
        Atom::FALSE.into()
    );
}

#[test]
fn every_short_circuits_on_undefined() {
    assert_eq!(
        every_result(
            Node::Literal(integer_set().into()),
            undefined_on_one_false_otherwise_predicate(),
        ),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn every_iterates_deferred_sets() {
    let doubled = Node::Map {
        set: Node::Literal(integer_set().into()).into(),
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
        set: Node::Literal(integer_set().into()).into(),
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
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let integer_set_node = || Node::Literal(integer_set().into());
    let cases = [
        (empty_set.clone(), always_true_predicate()),
        (empty_set.clone(), Node::Literal(Atom::UNDEFINED.into())),
        (integer_set_node(), always_true_predicate()),
        (integer_set_node(), equals_two_predicate()),
        (
            integer_set_node(),
            constant_predicate(Atom::UNDEFINED.into()),
        ),
        (integer_set_node(), constant_predicate(integer(0))),
        (
            integer_set_node(),
            false_on_one_undefined_otherwise_predicate(),
        ),
        (
            integer_set_node(),
            undefined_on_one_false_otherwise_predicate(),
        ),
    ];

    for (set, predicate) in cases {
        let every = evaluate(
            &Node::Every {
                set: set.clone().into(),
                predicate: predicate.clone().into(),
            },
            &BASE,
        )
        .into_object(&BASE);

        assert_eq!(every, every_by_and_fold(set, &predicate));
    }
}
