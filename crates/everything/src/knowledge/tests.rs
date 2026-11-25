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
