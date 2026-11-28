use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Construction, Object},
};

use super::common::*;
use crate::compute::*;

fn composite_from_set(set: Node) -> Node {
    Node::CompositeFromSet(set.into())
}

fn evaluate_composite_from_set(set: Node) -> Object {
    evaluate(&composite_from_set(set), &BASE).into_object(&BASE)
}

#[test]
fn composite_from_set_builds_a_composite_from_items() {
    assert_eq!(
        evaluate_composite_from_set(Node::Literal(set_of([integer(1), integer(2)]))),
        composite_of([integer(1), integer(2)])
    );
}

#[test]
fn composite_from_set_of_the_empty_set_is_the_empty_composite() {
    assert_eq!(
        evaluate_composite_from_set(Node::Literal(Composite::empty().into())),
        Object::Composite(Composite::empty())
    );
}

#[test]
fn composite_from_set_rejects_a_construction() {
    let construction = Construction::new(Atom(1).into(), Atom(2).into());

    assert_eq!(
        evaluate_composite_from_set(Node::Literal(construction.into())),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn undefined_items_become_members() {
    assert_eq!(
        evaluate_composite_from_set(Node::Literal(set_of([integer(1), Atom::UNDEFINED.into()]))),
        composite_of([integer(1), Atom::UNDEFINED.into()])
    );
}

#[test]
fn duplicate_items_are_merged() {
    let items = [
        Node::Literal(integer(1)),
        Node::Add {
            first: Node::Literal(integer(0)).into(),
            second: Node::Literal(integer(1)).into(),
        },
    ];

    assert_eq!(
        evaluate_composite_from_set(Node::NewSet {
            items: items.into_iter().map(Into::into).collect(),
        }),
        composite_of([integer(1)])
    );
}

#[test]
fn deferred_sets_are_materialized() {
    let union = Node::SetUnion {
        first: Node::Literal(set_of([integer(1)])).into(),
        second: Node::Literal(set_of([integer(2)])).into(),
    };

    assert_eq!(
        evaluate_composite_from_set(union),
        composite_of([integer(1), integer(2)])
    );
}

#[test]
fn composite_from_set_of_properties_of_is_the_original_composite() {
    let contains = Construction::new(Atom::CONTAINS.into(), integer(2)).into();

    let composite = composite_of([integer(1), contains, Atom(42).into()]);

    let node = composite_from_set(Node::PropertiesOf(Node::Literal(composite.clone()).into()));

    assert_eq!(evaluate(&node, &BASE).into_object(&BASE), composite);
}

#[test]
fn properties_of_composite_from_set_is_the_original_set() {
    let set = set_of([integer(1), Atom(42).into()]);

    let node = Node::PropertiesOf(composite_from_set(Node::Literal(set.clone())).into());

    assert_eq!(evaluate(&node, &BASE).into_object(&BASE), set);
}

#[test]
fn composite_from_set_uses_the_items_of_an_atom() {
    let atom = Atom(4242);
    let item = integer(7);

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(Atom::CONTAINS.into(), item.clone()).into(),
    });

    assert_eq!(
        evaluate(&composite_from_set(Node::Literal(atom.into())), &knowledge)
            .into_object(&knowledge),
        composite_of([item])
    );
}

#[test]
fn composite_from_set_of_a_deferred_properties_of_canonicalizes_the_set() {
    let deferred = Node::SetUnion {
        first: Node::Literal(set_of([integer(1)])).into(),
        second: Node::Literal(set_of([integer(2)])).into(),
    };

    // `properties_of` of a deferred set yields the `contains` constructions
    // that encode its items, so the reverse node rebuilds the canonical set.
    let node = composite_from_set(Node::PropertiesOf(deferred.into()));

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        set_of([integer(1), integer(2)])
    );
}

#[test]
fn the_result_is_not_a_canonical_set() {
    let result = evaluate(
        &composite_from_set(Node::Literal(set_of([integer(1), integer(2)]))),
        &BASE,
    );

    assert_eq!(set(&result).item_count(&BASE), 0);
    assert_eq!(
        result.into_object(&BASE),
        composite_of([integer(1), integer(2)])
    );
}

#[test]
fn composite_from_set_evaluates_in_the_evaluation_context() {
    let body = composite_from_set(Node::NewSet {
        items: [
            Node::Parameter { depth: 0 },
            Node::Add {
                first: Node::Parameter { depth: 0 }.into(),
                second: Node::Literal(integer(1)).into(),
            },
        ]
        .into_iter()
        .map(Into::into)
        .collect(),
    });

    let node = Node::Call {
        callee: Node::function(body).into(),
        with: Node::Literal(integer(5)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        composite_of([integer(5), integer(6)])
    );
}

#[test]
fn composite_from_set_captures_the_evaluation_context() {
    // `x |-> from_set({y |-> x})`; the inner function escapes the call and
    // must capture the parameter of the outer function.
    let escaped_function = Node::function(Node::Parameter { depth: 1 });

    let outer_function = Node::function(composite_from_set(Node::NewSet {
        items: [escaped_function].into_iter().map(Into::into).collect(),
    }));

    let node = Node::Call {
        callee: outer_function.into(),
        with: Node::Literal(integer(5)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        composite_of([Node::function(Node::Literal(integer(5))).into_object()])
    );
}
