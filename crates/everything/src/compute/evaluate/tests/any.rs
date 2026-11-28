use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::common::*;
use crate::compute::*;

#[test]
fn any_of_an_empty_composite_is_false() {
    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let invalid_predicates = [
        Node::Literal(Atom::UNDEFINED.into()),
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::SetUnion {
            first: empty_composite.clone().into(),
            second: empty_composite.clone().into(),
        },
    ];

    assert_eq!(
        any_result(empty_composite.clone(), always_true_predicate()),
        Atom::FALSE.into()
    );

    for predicate in invalid_predicates {
        assert_eq!(
            any_result(empty_composite.clone(), predicate),
            Atom::FALSE.into()
        );
    }
}

#[test]
fn any_of_an_empty_deferred_composite_is_false() {
    let drop_everything = Node::function(Node::Literal(Atom::FALSE.into()));
    let empty_composite = Node::SetFilter {
        set: Node::Literal(integer_set().into()).into(),
        filter_function: drop_everything.into(),
    };

    assert_eq!(
        any_result(empty_composite, Node::Literal(Atom::UNDEFINED.into())),
        Atom::FALSE.into()
    );
}

#[test]
fn any_requires_a_set() {
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    assert_eq!(
        any_result(construction, always_true_predicate()),
        Atom::UNDEFINED.into()
    );

    // Atoms are sets with no items, so any is false.
    for set in [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Atom::UNDEFINED.into()),
    ] {
        assert_eq!(any_result(set, always_true_predicate()), Atom::FALSE.into());
    }
}

#[test]
fn any_requires_a_function_predicate() {
    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let invalid_predicates = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
        Node::SetUnion {
            first: empty_composite.clone().into(),
            second: empty_composite.into(),
        },
    ];

    for predicate in invalid_predicates {
        assert_eq!(
            any_result(Node::Literal(integer_set().into()), predicate),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn any_returns_true_when_a_call_is_true() {
    let set = Node::Literal(set_of([integer(2)]));

    assert_eq!(any_result(set, equals_two_predicate()), Atom::TRUE.into());
    assert_eq!(
        any_result(Node::Literal(integer_set().into()), equals_two_predicate()),
        Atom::TRUE.into()
    );
}

#[test]
fn any_returns_false_when_all_calls_are_false() {
    assert_eq!(
        any_result(
            Node::Literal(integer_set().into()),
            always_false_predicate()
        ),
        Atom::FALSE.into()
    );
}

#[test]
fn any_returns_undefined_when_a_call_is_undefined() {
    assert_eq!(
        any_result(
            Node::Literal(integer_set().into()),
            constant_predicate(Atom::UNDEFINED.into()),
        ),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn any_returns_undefined_when_a_call_is_not_boolean() {
    let non_boolean_results = [
        integer(0),
        Object::Composite(Composite::empty()),
        Construction::new(Atom(1).into(), Atom(2).into()).into(),
        Object::new_node(identity_function()),
    ];

    for result in non_boolean_results {
        assert_eq!(
            any_result(
                Node::Literal(integer_set().into()),
                constant_predicate(result)
            ),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn any_short_circuits_on_true() {
    assert_eq!(
        any_result(
            Node::Literal(integer_set().into()),
            true_on_one_undefined_otherwise_predicate(),
        ),
        Atom::TRUE.into()
    );
}

#[test]
fn any_short_circuits_on_undefined() {
    assert_eq!(
        any_result(
            Node::Literal(integer_set().into()),
            undefined_on_one_true_otherwise_predicate(),
        ),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn any_iterates_deferred_composites() {
    let doubled = Node::SetMap {
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

    assert_eq!(any_result(doubled, is_positive), Atom::TRUE.into());
}

#[test]
fn any_captures_the_evaluation_context() {
    let outer_function = Node::function(Node::SetAny {
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
    assert_eq!(call(0), Atom::FALSE.into());
}

#[test]
fn any_matches_or_folding() {
    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let integer_set_node = || Node::Literal(integer_set().into());
    let cases = [
        (empty_composite.clone(), always_true_predicate()),
        (
            empty_composite.clone(),
            Node::Literal(Atom::UNDEFINED.into()),
        ),
        (integer_set_node(), always_true_predicate()),
        (integer_set_node(), always_false_predicate()),
        (integer_set_node(), equals_two_predicate()),
        (
            integer_set_node(),
            constant_predicate(Atom::UNDEFINED.into()),
        ),
        (integer_set_node(), constant_predicate(integer(0))),
        (
            integer_set_node(),
            true_on_one_undefined_otherwise_predicate(),
        ),
        (
            integer_set_node(),
            undefined_on_one_true_otherwise_predicate(),
        ),
    ];

    for (composite, predicate) in cases {
        let any = evaluate(
            &Node::SetAny {
                set: composite.clone().into(),
                predicate: predicate.clone().into(),
            },
            &BASE,
        )
        .into_object(&BASE);

        assert_eq!(any, any_by_or_fold(composite, &predicate));
    }
}
