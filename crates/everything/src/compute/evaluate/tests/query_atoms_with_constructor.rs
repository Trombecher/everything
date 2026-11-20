use num_bigint::BigInt;

use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;

#[test]
fn query_atoms_with_constructor() {
    let constructor = Atom(1);
    let other_constructor = Atom(2);
    let first_atom = Atom(42);
    let second_atom = Atom(43);
    let other_atom = Atom(44);

    let mut knowledge = BASE.clone();

    for value in [1, 2] {
        knowledge.add_statement(Statement {
            atom: first_atom,
            object: Construction::new(constructor.into(), Object::new_integer(BigInt::from(value)))
                .into(),
        });
    }

    knowledge.add_statement(Statement {
        atom: second_atom,
        object: Construction::new(constructor.into(), Object::new_integer(BigInt::from(3))).into(),
    });
    knowledge.add_statement(Statement {
        atom: other_atom,
        object: Construction::new(
            other_constructor.into(),
            Object::new_integer(BigInt::from(4)),
        )
        .into(),
    });

    let result = evaluate(
        &Node::QueryAtomsWithConstructor {
            constructor: Node::Literal(constructor.into()).into(),
        },
        &knowledge,
    );

    assert!(result.is_set());
    assert_eq!(result.size(&knowledge), 2);
    assert!(result.has(&first_atom.into(), &knowledge));
    assert!(result.has(&second_atom.into(), &knowledge));
    assert!(!result.has(&other_atom.into(), &knowledge));
    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        vec![first_atom.into(), second_atom.into()]
    );
}

#[test]
fn query_atoms_with_constructor_materializes_deferred_constructor() {
    let atom = Atom(42);
    let constructor = Object::Set(Set::from([Atom(1).into(), Atom(2).into()]));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(constructor, Object::new_integer(BigInt::from(7))).into(),
    });

    let result = evaluate(
        &Node::QueryAtomsWithConstructor {
            constructor: Node::Union {
                first: Node::Literal(Object::Set(Set::from([Atom(1).into()]))).into(),
                second: Node::Literal(Object::Set(Set::from([Atom(2).into()]))).into(),
            }
            .into(),
        },
        &knowledge,
    );

    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        vec![atom.into()]
    );
}
