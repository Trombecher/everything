use num_bigint::BigInt;

use crate::{
    knowledge::{Knowledge, Properties, Statement},
    model::{Atom, Composite, Construction, Object},
};

use super::*;

fn integer(value: i32) -> Object {
    Object::new_integer(BigInt::from(value))
}

#[test]
fn undefined_is_not_a_node() {
    assert_eq!(
        LazyObject::Eager(Object::Atom(Atom::UNDEFINED)).try_into_node(&Knowledge::empty()),
        None
    );
}

#[test]
fn is_lazy_distinguishes_materialized_results() {
    let eager_composite = LazyObject::Eager(Object::Composite(Composite::from([integer(1)])));
    let eager_atom = LazyObject::Eager(Atom(42).into());
    let lazy_properties = LazyObject::AtomProperties(Properties::from([integer(1)]));

    assert!(!eager_composite.is_lazy());
    assert!(!eager_atom.is_lazy());
    assert!(lazy_properties.is_lazy());

    let lazy_union = LazyObject::Union {
        first: Box::new(lazy_properties.clone()),
        second: Box::new(lazy_properties),
    };

    assert!(lazy_union.is_lazy());
    assert!(!LazyObject::Node(Node::Literal(integer(1))).is_lazy());
}

#[test]
fn into_iter_of_an_eager_composite_yields_its_members() {
    let composite = Composite::from([integer(1), integer(2)]);

    assert_eq!(
        LazyObject::Eager(Object::Composite(composite))
            .into_iter(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(1), integer(2)]
    );
}

#[test]
fn into_composite_iter_yields_each_value_at_most_once() {
    let first = LazyObject::Eager(Object::Composite(Composite::from([integer(1), integer(2)])));
    let second = LazyObject::Eager(Object::Composite(Composite::from([integer(2), integer(3)])));

    let union = LazyObject::Union {
        first: Box::new(first),
        second: Box::new(second),
    };

    assert_eq!(
        union
            .into_composite_iter(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(1), integer(2), integer(3)]
    );
}

#[test]
fn into_iter_of_a_union_yields_each_value_at_most_once() {
    let first = LazyObject::Eager(Object::Composite(Composite::from([integer(1), integer(2)])));
    let second = LazyObject::Eager(Object::Composite(Composite::from([integer(2), integer(3)])));

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
fn into_iter_captures_atom_properties() {
    let atom = Atom(42);
    let first = integer(1);
    let second = integer(2);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: first.clone(),
    });

    let lazy = LazyObject::AtomProperties(knowledge.properties_of_atom(&atom));
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
            Composite::from([
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

fn eager_composite(values: impl IntoIterator<Item = Object>) -> LazyObject {
    let mut values = values.into_iter().collect::<Vec<_>>();

    LazyObject::Eager(Object::Composite(Composite::from(values.as_mut_slice())))
}

#[test]
fn size_of_deferred_composite_operations_is_deduplicated() {
    let knowledge = Knowledge::empty();

    let union = LazyObject::Union {
        first: Box::new(eager_composite([integer(1), integer(2)])),
        second: Box::new(eager_composite([integer(2), integer(3)])),
    };
    let intersection = LazyObject::Intersection {
        first: Box::new(eager_composite([integer(1), integer(2)])),
        second: Box::new(eager_composite([integer(2), integer(3)])),
    };
    let difference = LazyObject::Difference {
        source: Box::new(eager_composite([integer(1), integer(2), integer(3)])),
        without: Box::new(eager_composite([integer(2)])),
    };

    assert_eq!(union.size(&knowledge), 3);
    assert_eq!(intersection.size(&knowledge), 1);
    assert_eq!(difference.size(&knowledge), 2);
}

#[test]
fn size_of_map_counts_distinct_values() {
    // `_ |-> 1` maps every element to the same value.
    let mapper = Node::function(Node::Literal(integer(1)));
    let map = LazyObject::Map {
        target: Box::new(eager_composite([integer(1), integer(2)])),
        mapper_function: mapper,
    };

    assert_eq!(map.size(&Knowledge::empty()), 1);
}

#[test]
fn into_iter_of_map_deduplicates() {
    // `_ |-> 1` maps every element to the same value.
    let mapper = Node::function(Node::Literal(integer(1)));
    let map = LazyObject::Map {
        target: Box::new(eager_composite([integer(1), integer(2)])),
        mapper_function: mapper,
    };

    assert_eq!(
        map.into_iter(&Knowledge::empty()).collect::<Vec<_>>(),
        vec![integer(1)]
    );
}

#[test]
fn size_of_filter_counts_distinct_retained_values() {
    // `x |-> x == 1`, applied to a target that yields `1` twice.
    let predicate = Node::function(Node::Equal {
        first: Node::Parameter { depth: 0 }.into(),
        second: Node::Literal(integer(1)).into(),
    });
    let filter = LazyObject::Filter {
        target: Box::new(LazyObject::Union {
            first: Box::new(eager_composite([integer(1), integer(2)])),
            second: Box::new(eager_composite([integer(1), integer(2)])),
        }),
        filter_function: predicate,
    };

    assert_eq!(filter.size(&Knowledge::empty()), 1);
}

#[test]
fn into_iter_of_intersection_yields_only_members_of_both_composites() {
    let intersection = LazyObject::Intersection {
        first: Box::new(eager_composite([integer(1), integer(2)])),
        second: Box::new(eager_composite([integer(2), integer(3)])),
    };

    assert_eq!(
        intersection
            .into_iter(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(2)]
    );
}

#[test]
fn into_iter_of_difference_excludes_members_of_the_other_composite() {
    let difference = LazyObject::Difference {
        source: Box::new(eager_composite([integer(1), integer(2), integer(3)])),
        without: Box::new(eager_composite([integer(2)])),
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
        target: Box::new(eager_composite([integer(1), integer(2)])),
        mapper_function: Node::function(Node::Literal(integer(1))),
    };

    let intersection = LazyObject::Intersection {
        first: Box::new(colliding_map()),
        second: Box::new(eager_composite([integer(1)])),
    };
    let difference = LazyObject::Difference {
        source: Box::new(colliding_map()),
        without: Box::new(eager_composite([])),
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
