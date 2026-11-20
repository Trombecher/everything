use num_bigint::BigInt;

use crate::{
    knowledge::{Knowledge, Statement, Values},
    model::{Atom, Construction, Object, Set},
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
fn into_iter_of_a_union_yields_each_value_at_most_once() {
    let first = LazyObject::Eager(Object::Set(Set::from([integer(1), integer(2)])));
    let second = LazyObject::Eager(Object::Set(Set::from([integer(2), integer(3)])));

    let union = LazyObject::Union {
        first: Box::new(first),
        second: Box::new(second),
    };

    assert_eq!(
        union.into_iter(&Knowledge::empty()).collect::<Vec<_>>(),
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
fn into_iter_yields_constructions_as_atom_value_objects() {
    let constructor = Atom(1);
    let atom = Atom(42);
    let value = integer(1);
    let statement = Statement {
        atom,
        object: Construction::new(constructor.into(), value.clone()).into(),
    };

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(statement.clone());

    let lazy = LazyObject::Constructions {
        constructor: constructor.into(),
    };
    let iterator = lazy.into_iter(&knowledge);

    // Removing the statement after creating the iterator must not be visible.
    knowledge.remove_statement(&statement);

    assert_eq!(
        iterator.collect::<Vec<_>>(),
        vec![
            Set::from([
                Construction::new(Atom::ATOM.into(), atom.into()).into(),
                Construction::new(Atom::VALUE.into(), value).into(),
            ])
            .into()
        ]
    );
}

#[test]
fn into_iter_captures_atoms_with_constructor() {
    let constructor = Atom(1);
    let atom = Atom(42);
    let statement = Statement {
        atom,
        object: Construction::new(constructor.into(), integer(1)).into(),
    };

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(statement.clone());

    let lazy = LazyObject::AtomsWithConstructor {
        constructor: constructor.into(),
    };
    let iterator = lazy.into_iter(&knowledge);

    // Removing the statement after creating the iterator must not be visible.
    knowledge.remove_statement(&statement);

    assert_eq!(iterator.collect::<Vec<_>>(), vec![Object::Atom(atom)]);
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

fn eager_set(values: impl IntoIterator<Item = Object>) -> LazyObject {
    let mut values = values.into_iter().collect::<Vec<_>>();

    LazyObject::Eager(Object::Set(Set::from(values.as_mut_slice())))
}

#[test]
fn size_of_deferred_set_operations_is_deduplicated() {
    let knowledge = Knowledge::empty();

    let union = LazyObject::Union {
        first: Box::new(eager_set([integer(1), integer(2)])),
        second: Box::new(eager_set([integer(2), integer(3)])),
    };
    let intersection = LazyObject::Intersection {
        first: Box::new(eager_set([integer(1), integer(2)])),
        second: Box::new(eager_set([integer(2), integer(3)])),
    };
    let difference = LazyObject::Difference {
        source: Box::new(eager_set([integer(1), integer(2), integer(3)])),
        without: Box::new(eager_set([integer(2)])),
    };

    assert_eq!(union.size(&knowledge), 3);
    assert_eq!(intersection.size(&knowledge), 1);
    assert_eq!(difference.size(&knowledge), 2);
}

#[test]
fn size_of_map_counts_distinct_values() {
    // `_ |-> 1` maps every element to the same value.
    let mapper = Node::Function {
        body: Node::Literal(integer(1)).into(),
    };
    let map = LazyObject::Map {
        target: Box::new(eager_set([integer(1), integer(2)])),
        mapper_function: mapper,
    };

    assert_eq!(map.size(&Knowledge::empty()), 1);
}

#[test]
fn size_of_filter_counts_distinct_retained_values() {
    // `x |-> x == 1`, applied to a target that yields `1` twice.
    let predicate = Node::Function {
        body: Node::Equal {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Literal(integer(1)).into(),
        }
        .into(),
    };
    let filter = LazyObject::Filter {
        target: Box::new(LazyObject::Union {
            first: Box::new(eager_set([integer(1), integer(2)])),
            second: Box::new(eager_set([integer(1), integer(2)])),
        }),
        filter_function: predicate,
    };

    assert_eq!(filter.size(&Knowledge::empty()), 1);
}

#[test]
fn into_iter_of_intersection_yields_only_members_of_both_sets() {
    let intersection = LazyObject::Intersection {
        first: Box::new(eager_set([integer(1), integer(2)])),
        second: Box::new(eager_set([integer(2), integer(3)])),
    };

    assert_eq!(
        intersection
            .into_iter(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(2)]
    );
}

#[test]
fn into_iter_of_difference_excludes_members_of_the_other_set() {
    let difference = LazyObject::Difference {
        source: Box::new(eager_set([integer(1), integer(2), integer(3)])),
        without: Box::new(eager_set([integer(2)])),
    };

    assert_eq!(
        difference
            .into_iter(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(1), integer(3)]
    );
}

#[test]
fn into_iter_of_intersection_and_difference_deduplicates() {
    let colliding_map = || LazyObject::Map {
        target: Box::new(eager_set([integer(1), integer(2)])),
        mapper_function: Node::Function {
            body: Node::Literal(integer(1)).into(),
        },
    };

    let intersection = LazyObject::Intersection {
        first: Box::new(colliding_map()),
        second: Box::new(eager_set([integer(1)])),
    };
    let difference = LazyObject::Difference {
        source: Box::new(colliding_map()),
        without: Box::new(eager_set([])),
    };

    assert_eq!(
        intersection
            .into_iter(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(1)]
    );
    assert_eq!(
        difference
            .into_iter(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(1)]
    );
}
