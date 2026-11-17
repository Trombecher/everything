use num_bigint::BigInt;

use crate::{
    knowledge::{Knowledge, Statement},
    model::{Atom, Construction, Object, Set},
};

use super::*;

fn integer(value: i32) -> Object {
    Object::new_integer(BigInt::from(value))
}

#[test]
fn query_atoms_with_constructor_yields_matching_atoms_once() {
    let constructor = Atom(1);
    let other_constructor = Atom(2);
    let first_atom = Atom(42);
    let second_atom = Atom(43);
    let other_atom = Atom(44);
    let plain_atom = Atom(45);

    let mut knowledge = Knowledge::empty();

    for value in [1, 2] {
        knowledge.add_statement(Statement {
            atom: first_atom,
            object: Construction::new(constructor.into(), integer(value)).into(),
        });
    }

    knowledge.add_statement(Statement {
        atom: second_atom,
        object: Construction::new(constructor.into(), integer(3)).into(),
    });
    knowledge.add_statement(Statement {
        atom: other_atom,
        object: Construction::new(other_constructor.into(), integer(4)).into(),
    });
    knowledge.add_statement(Statement {
        atom: plain_atom,
        object: integer(5),
    });

    assert_eq!(
        knowledge
            .query_atoms_with_constructor(constructor.into())
            .collect::<Vec<_>>(),
        vec![first_atom, second_atom]
    );
}

#[test]
fn query_constructions_yields_matching_atom_value_pairs() {
    let constructor = Atom(1);
    let other_constructor = Atom(2);
    let first_atom = Atom(42);
    let second_atom = Atom(43);

    let mut knowledge = Knowledge::empty();

    for value in [1, 2] {
        knowledge.add_statement(Statement {
            atom: first_atom,
            object: Construction::new(constructor.into(), integer(value)).into(),
        });
    }

    knowledge.add_statement(Statement {
        atom: first_atom,
        object: Construction::new(other_constructor.into(), integer(3)).into(),
    });
    knowledge.add_statement(Statement {
        atom: first_atom,
        object: integer(4),
    });
    knowledge.add_statement(Statement {
        atom: second_atom,
        object: Construction::new(constructor.into(), integer(5)).into(),
    });

    assert_eq!(
        knowledge
            .query_constructions(constructor.into())
            .collect::<Vec<_>>(),
        vec![
            AtomValuePair {
                atom: first_atom,
                value: integer(1),
            },
            AtomValuePair {
                atom: first_atom,
                value: integer(2),
            },
            AtomValuePair {
                atom: second_atom,
                value: integer(5),
            },
        ]
    );
}

#[test]
fn query_constructions_with_unknown_constructor_is_empty() {
    let mut knowledge = Knowledge::empty();

    knowledge.add_statement(Statement {
        atom: Atom(42),
        object: Construction::new(Atom(1).into(), integer(1)).into(),
    });

    assert_eq!(
        knowledge
            .query_constructions(Atom(2).into())
            .collect::<Vec<_>>(),
        vec![]
    );
}

#[test]
fn atom_value_pair_into_object() {
    let pair = AtomValuePair {
        atom: Atom(42),
        value: integer(7),
    };

    assert_eq!(
        pair.into_object(),
        Set::from([
            Construction::new(Atom::ATOM.into(), Atom(42).into()).into(),
            Construction::new(Atom::VALUE.into(), integer(7)).into(),
        ])
        .into()
    );
}
