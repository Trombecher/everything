use num_bigint::BigInt;

use crate::{
    knowledge::{Knowledge, Properties, Statement},
    model::{Atom, AtomOrComposite, Composite, Construction, Object},
};

use super::*;

fn integer(value: i32) -> Object {
    Object::new_integer(BigInt::from(value))
}

fn contains(item: Object) -> Object {
    Construction::new(Atom::CONTAINS.into(), item).into()
}

fn eager_set(items: impl IntoIterator<Item = Object>) -> SetValue {
    let items = items.into_iter().collect::<Vec<_>>();

    SetValue::Eager(AtomOrComposite::Composite(Composite::from_set_items(
        &items,
    )))
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
    let eager = LazyObject::from(eager_set([integer(1), integer(2)]));
    let eager_atom = LazyObject::Eager(Atom(42).into());
    let lazy_properties = LazyObject::Set(LazySet::AtomProperties(Properties::from([integer(1)])));

    assert!(!eager.is_lazy());
    assert!(!eager_atom.is_lazy());
    assert!(lazy_properties.is_lazy());

    let lazy_union = LazyObject::Set(LazySet::Union {
        first: Box::new(SetValue::Set(LazySet::AtomProperties(Properties::from([
            integer(1),
        ])))),
        second: Box::new(eager_set([integer(2)])),
    });

    assert!(lazy_union.is_lazy());
    assert!(!LazyObject::Node(Node::Literal(integer(1))).is_lazy());
}

#[test]
fn every_atom_and_composite_is_a_set() {
    assert!(LazyObject::Eager(Atom(42).into()).is_set());
    assert!(LazyObject::Eager(Object::Composite(Composite::empty())).is_set());
    assert!(LazyObject::Node(Node::Literal(integer(1))).is_set());
    assert!(!LazyObject::Eager(contains(integer(1))).is_set());
}

#[test]
fn into_items_of_a_set_yields_its_items() {
    assert_eq!(
        eager_set([integer(1), integer(2)])
            .into_items(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(1), integer(2)]
    );
}

#[test]
fn into_items_of_a_plain_composite_is_empty() {
    let composite = Composite::from([integer(1), integer(2), integer(3)]);

    assert_eq!(
        SetValue::Eager(AtomOrComposite::Composite(composite))
            .into_items(&Knowledge::empty())
            .count(),
        0
    );
}

#[test]
fn into_set_of_a_construction_is_none() {
    assert!(LazyObject::Eager(contains(integer(1))).into_set().is_none());
}

#[test]
fn into_properties_of_an_atom_yields_its_statements() {
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: integer(1),
    });

    let properties = LazyObject::Eager(Object::Atom(atom))
        .into_set()
        .expect("an atom is a set")
        .into_properties(&knowledge)
        .collect::<Vec<_>>();

    assert_eq!(properties, vec![integer(1)]);
}

#[test]
fn into_properties_of_a_deferred_set_yields_contains_constructions() {
    let set = SetValue::Set(LazySet::AtomProperties(Properties::from([integer(1)])));

    let properties = set.into_properties(&Knowledge::empty()).collect::<Vec<_>>();

    assert_eq!(properties, vec![contains(integer(1))]);
}

#[test]
fn properties_of_lists_composite_members() {
    let knowledge = Knowledge::empty();
    let annotation = integer(1);
    let member = contains(integer(2));

    let target = SetValue::Eager(AtomOrComposite::Composite(Composite::from([
        annotation.clone(),
        member.clone(),
    ])));
    let properties = LazySet::PropertiesOf {
        target: Box::new(target),
    };

    assert_eq!(properties.item_count(&knowledge), 2);
    assert!(properties.has_at_least_one_item(&knowledge));
    assert!(properties.has_item(&annotation, &knowledge));
    assert!(properties.has_item(&member, &knowledge));
    assert!(!properties.has_item(&integer(3), &knowledge));

    let mut items = properties.into_items(&knowledge).collect::<Vec<_>>();
    items.sort();

    let mut expected = vec![annotation, member];
    expected.sort();

    assert_eq!(items, expected);
}

#[test]
fn properties_of_atom_target_uses_its_statements() {
    let atom = Atom(42);
    let first = integer(1);
    let second = integer(2);
    let late = integer(3);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: first.clone(),
    });
    knowledge.add_statement(Statement {
        atom,
        object: second.clone(),
    });

    let properties = LazySet::PropertiesOf {
        target: Box::new(SetValue::Eager(AtomOrComposite::Atom(atom))),
    };

    assert_eq!(properties.item_count(&knowledge), 2);
    assert!(properties.has_at_least_one_item(&knowledge));
    assert!(properties.has_item(&first, &knowledge));
    assert!(properties.has_item(&second, &knowledge));
    assert!(!properties.has_item(&contains(first.clone()), &knowledge));

    let iterator = properties.into_items(&knowledge);

    // Changes to the knowledge after creating the iterator must not be visible.
    knowledge.add_statement(Statement { atom, object: late });

    let mut items = iterator.collect::<Vec<_>>();
    items.sort();

    let mut expected = vec![first, second];
    expected.sort();

    assert_eq!(items, expected);
}

#[test]
fn properties_of_deferred_target_wraps_items_in_contains() {
    let knowledge = Knowledge::empty();
    let first = integer(1);
    let second = integer(2);

    let target = SetValue::Set(LazySet::Union {
        first: Box::new(eager_set([first.clone()])),
        second: Box::new(eager_set([second.clone()])),
    });
    let properties = LazySet::PropertiesOf {
        target: Box::new(target),
    };

    assert_eq!(properties.item_count(&knowledge), 2);
    assert!(properties.has_at_least_one_item(&knowledge));
    assert!(properties.has_item(&contains(first.clone()), &knowledge));
    assert!(!properties.has_item(&first, &knowledge));

    let mut items = properties.into_items(&knowledge).collect::<Vec<_>>();
    items.sort();

    let mut expected = vec![contains(first), contains(second)];
    expected.sort();

    assert_eq!(items, expected);
}

#[test]
fn properties_of_empty_target_has_no_items() {
    let knowledge = Knowledge::empty();
    let target = SetValue::Eager(AtomOrComposite::Composite(Composite::empty()));
    let properties = LazySet::PropertiesOf {
        target: Box::new(target),
    };

    assert_eq!(properties.item_count(&knowledge), 0);
    assert!(!properties.has_at_least_one_item(&knowledge));
    assert_eq!(properties.into_items(&knowledge).count(), 0);
}

#[test]
fn into_items_of_an_atom_yields_contains_values() {
    let atom = Atom(42);
    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: integer(1),
    });
    knowledge.add_statement(Statement {
        atom,
        object: contains(integer(2)),
    });
    knowledge.add_statement(Statement {
        atom,
        object: contains(integer(3)),
    });

    assert_eq!(
        SetValue::Eager(AtomOrComposite::Atom(atom))
            .into_items(&knowledge)
            .collect::<Vec<_>>(),
        vec![integer(2), integer(3)]
    );
}

#[test]
fn into_items_captures_atom_properties() {
    let atom = Atom(42);
    let first = integer(1);
    let second = integer(2);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: first.clone(),
    });

    let set = SetValue::Set(LazySet::AtomProperties(knowledge.properties_of_atom(&atom)));
    let iterator = set.into_items(&knowledge);

    // Changes to the knowledge after creating the iterator must not be visible.
    knowledge.add_statement(Statement {
        atom,
        object: second,
    });

    assert_eq!(iterator.collect::<Vec<_>>(), vec![first]);
}

#[test]
fn into_items_yields_constructions_as_atom_value_objects() {
    let constructor = Atom(1);
    let atom = Atom(42);
    let value = integer(1);
    let statement = Statement {
        atom,
        object: Construction::new(constructor.into(), value.clone()).into(),
    };

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(statement.clone());

    let set = SetValue::Set(LazySet::Constructions {
        constructor: constructor.into(),
    });
    let iterator = set.into_items(&knowledge);

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
fn into_items_captures_atoms_with_constructor() {
    let constructor = Atom(1);
    let atom = Atom(42);
    let statement = Statement {
        atom,
        object: Construction::new(constructor.into(), integer(1)).into(),
    };

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(statement.clone());

    let set = SetValue::Set(LazySet::AtomsWithConstructor {
        constructor: constructor.into(),
    });
    let iterator = set.into_items(&knowledge);

    // Removing the statement after creating the iterator must not be visible.
    knowledge.remove_statement(&statement);

    assert_eq!(iterator.collect::<Vec<_>>(), vec![Object::Atom(atom)]);
}

#[test]
fn into_items_captures_atoms_with_property() {
    let property = Atom(1);
    let atom = Atom(42);
    let statement = Statement {
        atom,
        object: property.into(),
    };

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(statement.clone());

    let set = SetValue::Set(LazySet::AtomsWithProperty {
        property: property.into(),
    });
    let iterator = set.into_items(&knowledge);

    // Removing the statement after creating the iterator must not be visible.
    knowledge.remove_statement(&statement);

    assert_eq!(iterator.collect::<Vec<_>>(), vec![Object::Atom(atom)]);
}

#[test]
fn union_yields_each_item_once() {
    let union = LazySet::Union {
        first: Box::new(eager_set([integer(1), integer(2)])),
        second: Box::new(eager_set([integer(2), integer(3)])),
    };

    let mut items = union.into_items(&Knowledge::empty()).collect::<Vec<_>>();
    items.sort();

    assert_eq!(items, vec![integer(1), integer(2), integer(3)]);
}

#[test]
fn into_items_of_intersection_yields_only_items_of_both_sets() {
    let intersection = LazySet::Intersection {
        first: Box::new(eager_set([integer(1), integer(2)])),
        second: Box::new(eager_set([integer(2), integer(3)])),
    };

    assert_eq!(
        intersection
            .into_items(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(2)]
    );
}

#[test]
fn into_items_of_difference_excludes_items_of_the_other_set() {
    let difference = LazySet::Difference {
        source: Box::new(eager_set([integer(1), integer(2), integer(3)])),
        without: Box::new(eager_set([integer(2)])),
    };

    assert_eq!(
        difference
            .into_items(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(1), integer(3)]
    );
}

#[test]
fn item_count_of_deferred_set_operations_is_deduplicated() {
    let knowledge = Knowledge::empty();

    let union = LazySet::Union {
        first: Box::new(eager_set([integer(1), integer(2)])),
        second: Box::new(eager_set([integer(2), integer(3)])),
    };
    let intersection = LazySet::Intersection {
        first: Box::new(eager_set([integer(1), integer(2)])),
        second: Box::new(eager_set([integer(2), integer(3)])),
    };
    let difference = LazySet::Difference {
        source: Box::new(eager_set([integer(1), integer(2), integer(3)])),
        without: Box::new(eager_set([integer(2)])),
    };

    assert_eq!(union.item_count(&knowledge), 3);
    assert_eq!(intersection.item_count(&knowledge), 1);
    assert_eq!(difference.item_count(&knowledge), 2);
}

#[test]
fn item_count_of_plain_composites_is_zero() {
    let composite = Composite::from([integer(1), integer(2), integer(3)]);

    assert_eq!(
        SetValue::Eager(AtomOrComposite::Composite(composite)).item_count(&Knowledge::empty()),
        0
    );
}

#[test]
fn item_count_of_map_counts_distinct_items() {
    // `_ |-> 1` maps every item to the same value.
    let mapper = Node::function(Node::Literal(integer(1)));
    let map = LazySet::Map {
        target: Box::new(eager_set([integer(1), integer(2)])),
        mapper_function: mapper,
    };

    assert_eq!(map.item_count(&Knowledge::empty()), 1);
}

#[test]
fn into_items_of_map_deduplicates() {
    // `_ |-> 1` maps every item to the same value.
    let mapper = Node::function(Node::Literal(integer(1)));
    let map = LazySet::Map {
        target: Box::new(eager_set([integer(1), integer(2)])),
        mapper_function: mapper,
    };

    assert_eq!(
        map.into_items(&Knowledge::empty()).collect::<Vec<_>>(),
        vec![integer(1)]
    );
}

#[test]
fn item_count_of_filter_counts_distinct_retained_items() {
    // `x |-> x == 1`, applied to a target that yields `1` twice.
    let predicate = Node::function(Node::Equal {
        first: Node::Parameter { depth: 0 }.into(),
        second: Node::Literal(integer(1)).into(),
    });
    let filter = LazySet::Filter {
        target: Box::new(SetValue::Set(LazySet::Union {
            first: Box::new(eager_set([integer(1), integer(2)])),
            second: Box::new(eager_set([integer(1), integer(2)])),
        })),
        filter_function: predicate,
    };

    assert_eq!(filter.item_count(&Knowledge::empty()), 1);
}

#[test]
fn into_items_of_intersection_and_difference_deduplicate() {
    let colliding_map = || {
        SetValue::Set(LazySet::Map {
            target: Box::new(eager_set([integer(1), integer(2)])),
            mapper_function: Node::function(Node::Literal(integer(1))),
        })
    };

    let intersection = LazySet::Intersection {
        first: Box::new(colliding_map()),
        second: Box::new(eager_set([integer(1)])),
    };
    let difference = LazySet::Difference {
        source: Box::new(colliding_map()),
        without: Box::new(eager_set([])),
    };

    assert_eq!(
        intersection
            .into_items(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(1)]
    );
    assert_eq!(
        difference
            .into_items(&Knowledge::empty())
            .collect::<Vec<_>>(),
        vec![integer(1)]
    );
}

#[test]
fn has_item_uses_the_open_set_view() {
    let knowledge = Knowledge::empty();
    let set = eager_set([integer(1), integer(2)]);
    let composite = SetValue::Eager(AtomOrComposite::Composite(Composite::from([
        integer(3),
        contains(integer(4)),
    ])));

    assert!(set.has_item(&integer(1), &knowledge));
    assert!(!set.has_item(&integer(3), &knowledge));
    assert!(composite.has_item(&integer(4), &knowledge));
    assert!(!composite.has_item(&integer(3), &knowledge));
}

#[test]
fn materialising_a_deferred_set_produces_contains_members() {
    let union = LazyObject::Set(LazySet::Union {
        first: Box::new(eager_set([integer(1)])),
        second: Box::new(eager_set([integer(2)])),
    });

    assert_eq!(
        union.into_object(&Knowledge::empty()),
        Object::Composite(Composite::from_set_items(&[integer(1), integer(2)]))
    );
}
