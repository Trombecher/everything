use num_bigint::BigInt;

use crate::{
    knowledge::{Knowledge, Statement, Values},
    model::{Atom, Object, Set},
};

use super::*;

fn integer(value: i32) -> Object {
    Object::new_integer(BigInt::from(value))
}

#[test]
fn is_lazy_distinguishes_materialized_results() {
    let eager_set = LazyObject::Eager(Object::Set(Set::from([integer(1)])));
    let eager_atom = LazyObject::Eager(Atom(42).into());
    let lazy_values = LazyObject::AtomValues(Values::from([integer(1)]));

    assert!(!eager_set.is_lazy());
    assert!(!eager_atom.is_lazy());
    assert!(lazy_values.is_lazy());

    let lazy_union = LazyObject::Union {
        first: Box::new(lazy_values.clone()),
        second: Box::new(lazy_values),
    };

    assert!(lazy_union.is_lazy());
    assert!(!LazyObject::Node(Node::Literal(integer(1))).is_lazy());
}

#[test]
fn into_iter_of_an_eager_set_yields_its_members() {
    let set = Set::from([integer(1), integer(2)]);

    assert_eq!(
        LazyObject::Eager(Object::Set(set))
            .into_iter(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(1), integer(2)]
    );
}

#[test]
fn into_set_iter_yields_each_value_at_most_once() {
    let first = LazyObject::Eager(Object::Set(Set::from([integer(1), integer(2)])));
    let second = LazyObject::Eager(Object::Set(Set::from([integer(2), integer(3)])));

    let union = LazyObject::Union {
        first: Box::new(first),
        second: Box::new(second),
    };

    assert_eq!(
        union.into_set_iter(&Knowledge::empty()).collect::<Vec<_>>(),
        vec![integer(1), integer(2), integer(3)]
    );
}

#[test]
fn into_iter_captures_atom_values() {
    let atom = Atom(42);
    let first = integer(1);
    let second = integer(2);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: first.clone(),
    });

    let lazy = LazyObject::AtomValues(knowledge.values_of_atom(&atom));
    let iterator = lazy.into_iter(&knowledge);

    // Changes to the knowledge after creating the iterator must not be visible.
    knowledge.add_statement(Statement {
        atom,
        object: second,
    });

    assert_eq!(iterator.collect::<Vec<_>>(), vec![first]);
}

#[test]
fn into_iter_captures_atoms_with_property() {
    let property = Atom(1);
    let atom = Atom(42);
    let statement = Statement {
        atom,
        object: property.into(),
    };

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(statement.clone());

    let lazy = LazyObject::AtomsWithProperty {
        property: property.into(),
    };
    let iterator = lazy.into_iter(&knowledge);

    // Removing the statement after creating the iterator must not be visible.
    knowledge.remove_statement(&statement);

    assert_eq!(iterator.collect::<Vec<_>>(), vec![Object::Atom(atom)]);
}
