use num_bigint::BigInt;

use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn set_count_counts_items_of_the_open_set_view() {
    let atom = Atom(42);
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));
    let plain_composite = Object::Composite(Composite::from([
        Object::new_integer(BigInt::from(1)),
        Object::new_integer(BigInt::from(2)),
    ]));

    let count = |object: Object| {
        evaluate(&Node::SetCount(Node::Literal(object).into()), &BASE).into_object(&BASE)
    };

    assert_eq!(count(construction), Atom::UNDEFINED.into());
    assert_eq!(
        count(Object::Atom(atom)),
        Object::new_integer(BigInt::from(0))
    );
    assert_eq!(count(plain_composite), Object::new_integer(BigInt::from(0)));
    assert_eq!(
        count(Atom::UNDEFINED.into()),
        Object::new_integer(BigInt::from(0))
    );
    assert_eq!(
        count(Object::Composite(integer_set())),
        Object::new_integer(BigInt::from(3))
    );

    // A function object is a composite without `contains` members.
    assert_eq!(
        evaluate(&Node::SetCount(identity_function().into()), &BASE).into_object(&BASE),
        Object::new_integer(BigInt::from(0))
    );
}

#[test]
fn set_count_of_an_atom_counts_contains_statements() {
    let atom = Atom(42);
    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: integer(1),
    });
    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(Atom::CONTAINS.into(), integer(2)).into(),
    });

    assert_eq!(
        evaluate(
            &Node::SetCount(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        )
        .into_object(&knowledge),
        Object::new_integer(BigInt::from(1))
    );
}

#[test]
fn set_count_of_deferred_sets_is_deduplicated() {
    let count = |node: Node| evaluate(&Node::SetCount(node.into()), &BASE).into_object(&BASE);

    let union = Node::SetUnion {
        first: Node::Literal(Object::Composite(integer_set())).into(),
        second: Node::Literal(Object::Composite(integer_set())).into(),
    };

    assert_eq!(count(union.clone()), Object::new_integer(BigInt::from(3)));

    let map = Node::SetMap {
        set: Node::Literal(Object::Composite(integer_set())).into(),
        mapper_function: constant_predicate(integer(1)).into(),
    };

    assert_eq!(count(map), Object::new_integer(BigInt::from(1)));

    let filter = Node::SetFilter {
        set: union.into(),
        filter_function: equals_two_predicate().into(),
    };

    assert_eq!(count(filter), Object::new_integer(BigInt::from(1)));
}
