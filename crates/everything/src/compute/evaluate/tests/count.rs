use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;
use super::common::*;

#[test]
fn count_requires_sets() {
    let atom = Atom(42);
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));
    let set = Object::Set(Set::from([Object::new_integer(BigInt::from(1))]));

    let count = |object: Object| {
        evaluate(&Node::Count(Node::Literal(object).into()), &BASE).into_object(&BASE)
    };

    assert_eq!(count(Object::Atom(atom)), Atom::UNDEFINED.into());
    assert_eq!(count(construction), Atom::UNDEFINED.into());
    assert_eq!(count(Atom::UNDEFINED.into()), Atom::UNDEFINED.into());
    assert_eq!(count(set), Object::new_integer(BigInt::from(1)));

    // A plain function object is a set with one property.
    assert_eq!(
        evaluate(&Node::Count(identity_function().into()), &BASE).into_object(&BASE),
        Object::new_integer(BigInt::from(1))
    );
}

#[test]
fn count_of_deferred_sets_is_deduplicated() {
    let count = |node: Node| evaluate(&Node::Count(node.into()), &BASE).into_object(&BASE);

    let union = Node::Union {
        first: Node::Literal(Object::Set(integer_set())).into(),
        second: Node::Literal(Object::Set(integer_set())).into(),
    };

    assert_eq!(count(union.clone()), Object::new_integer(BigInt::from(3)));

    let map = Node::Map {
        set: Node::Literal(Object::Set(integer_set())).into(),
        mapper_function: constant_predicate(integer(1)).into(),
    };

    assert_eq!(count(map), Object::new_integer(BigInt::from(1)));

    let filter = Node::Filter {
        set: union.into(),
        filter_function: equals_two_predicate().into(),
    };

    assert_eq!(count(filter), Object::new_integer(BigInt::from(1)));
}
