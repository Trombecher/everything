use num_bigint::BigInt;

use crate::{
    knowledge::{Knowledge, Statement},
    model::{Atom, AtomOrComposite, Composite, Construction, Object},
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
fn construction_is_not_an_atom_or_composite() {
    assert!(AtomOrComposite::try_from(construction_value(Atom(1), 1)).is_err());
}

#[test]
fn object_has_property_distinguishes_known_and_unknown_properties() {
    let property = integer(1);
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: property.clone(),
    });

    assert!(knowledge.object_has_property(&AtomOrComposite::Atom(atom), &property));
    assert!(!knowledge.object_has_property(&AtomOrComposite::Atom(Atom(43)), &property));
}

#[test]
fn object_has_item_distinguishes_contained_and_absent_items() {
    let atom = Atom(42);
    let contained = integer(1);
    let not_contained = integer(2);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: contains(contained.clone()),
    });

    let atom = AtomOrComposite::Atom(atom);

    assert!(knowledge.object_has_item(&atom, &contained));
    assert!(!knowledge.object_has_item(&atom, &not_contained));
}

#[test]
fn items_of_an_atom_yields_its_contains_values() {
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
            .items_of(&AtomOrComposite::Atom(atom))
            .collect::<Vec<_>>(),
        vec![integer(1)]
    );
}

#[test]
fn item_count_of_an_atom_counts_its_contains_values() {
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

    assert_eq!(knowledge.item_count_of(&AtomOrComposite::Atom(atom)), 2);
}

#[test]
fn size_of_object_counts_the_properties_of_an_atom() {
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

    assert_eq!(knowledge.size_of_object(&AtomOrComposite::Atom(atom)), 2);
}
