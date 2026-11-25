use num_bigint::BigInt;

use crate::{
    knowledge::{Knowledge, Statement},
    model::{Atom, Composite, Construction, Object},
};

fn integer(value: i32) -> Object {
    Object::new_integer(BigInt::from(value))
}

fn construction_value(constructor: Atom, value: i32) -> Object {
    Construction::new(constructor.into(), integer(value)).into()
}

fn contains(item: Object) -> Object {
    Construction::new(Atom::CONTAINS.into(), item).into()
}

#[test]
fn query_construction_values_of_atom_yields_matching_values() {
    let constructor = Atom(1);
    let other_constructor = Atom(2);
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();

    knowledge.add_statement(Statement {
        atom,
        object: construction_value(constructor, 1),
    });
    knowledge.add_statement(Statement {
        atom,
        object: construction_value(constructor, 2),
    });
    knowledge.add_statement(Statement {
        atom,
        object: construction_value(other_constructor, 3),
    });
    knowledge.add_statement(Statement {
        atom,
        object: integer(4),
    });

    let mut values = knowledge
        .query_construction_values(&atom.into(), constructor.into())
        .expect("an atom with statements has properties")
        .collect::<Vec<_>>();

    values.sort();

    assert_eq!(values, vec![integer(1), integer(2)]);
}

#[test]
fn query_construction_values_of_composite_yields_matching_member_values() {
    let constructor = Atom(1);

    let knowledge = Knowledge::empty();

    let composite = Composite::from([
        construction_value(constructor, 1),
        integer(2),
        construction_value(constructor, 3),
    ]);

    let mut values = knowledge
        .query_construction_values(&composite.into(), constructor.into())
        .expect("a composite has properties")
        .collect::<Vec<_>>();

    values.sort();

    assert_eq!(values, vec![integer(1), integer(3)]);
}

#[test]
fn query_construction_values_of_construction_is_none() {
    let knowledge = Knowledge::empty();

    let construction = construction_value(Atom(1), 1);

    assert!(
        knowledge
            .query_construction_values(&construction, Atom(2).into())
            .is_none()
    );
}

#[test]
fn object_has_property_of_construction_is_none() {
    let property = integer(1);
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: property.clone(),
    });

    assert_eq!(
        knowledge.object_has_property(&atom.into(), &property),
        Some(true)
    );
    assert_eq!(
        knowledge.object_has_property(&Atom(43).into(), &property),
        Some(false)
    );
    assert_eq!(
        knowledge.object_has_property(&construction_value(Atom(1), 1), &property),
        None
    );
}

#[test]
fn object_has_item_of_construction_is_none() {
    let atom = Atom(42);
    let contained = integer(1);
    let not_contained = integer(2);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: contains(contained.clone()),
    });

    assert_eq!(
        knowledge.object_has_item(&atom.into(), &contained),
        Some(true)
    );
    assert_eq!(
        knowledge.object_has_item(&atom.into(), &not_contained),
        Some(false)
    );
    assert_eq!(
        knowledge.object_has_item(&construction_value(Atom(1), 1), &contained),
        None
    );
}

#[test]
fn items_of_construction_is_none() {
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: contains(integer(1)),
    });
    knowledge.add_statement(Statement {
        atom,
        object: integer(2),
    });

    assert_eq!(
        knowledge
            .items_of(&atom.into())
            .expect("an atom is a set")
            .collect::<Vec<_>>(),
        vec![integer(1)]
    );
    assert!(
        knowledge
            .items_of(&construction_value(Atom(1), 1))
            .is_none()
    );
}

#[test]
fn item_count_of_construction_is_none() {
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: contains(integer(1)),
    });
    knowledge.add_statement(Statement {
        atom,
        object: contains(integer(2)),
    });

    assert_eq!(knowledge.item_count_of(&atom.into()), Some(2));
    assert_eq!(
        knowledge.item_count_of(&construction_value(Atom(1), 1)),
        None
    );
}

#[test]
fn size_of_object_of_construction_is_none() {
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: integer(1),
    });
    knowledge.add_statement(Statement {
        atom,
        object: integer(2),
    });

    assert_eq!(knowledge.size_of_object(&atom.into()), Some(2));
    assert_eq!(
        knowledge.size_of_object(&construction_value(Atom(1), 1)),
        None
    );
}
