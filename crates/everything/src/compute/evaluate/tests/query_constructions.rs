use num_bigint::BigInt;

use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn query_constructions() {
    let constructor = Atom(1);
    let other_constructor = Atom(2);
    let first_atom = Atom(42);
    let second_atom = Atom(43);

    let mut knowledge = BASE.clone();

    for value in [1, 2] {
        knowledge.add_statement(Statement {
            atom: first_atom,
            object: Construction::new(constructor.into(), Object::new_integer(BigInt::from(value)))
                .into(),
        });
    }

    knowledge.add_statement(Statement {
        atom: first_atom,
        object: Construction::new(
            other_constructor.into(),
            Object::new_integer(BigInt::from(3)),
        )
        .into(),
    });
    knowledge.add_statement(Statement {
        atom: second_atom,
        object: Construction::new(constructor.into(), Object::new_integer(BigInt::from(4))).into(),
    });

    let result = evaluate(
        &Node::QueryConstructions {
            constructor: Node::Literal(constructor.into()).into(),
        },
        &knowledge,
    );

    let expected = vec![
        pair(first_atom, Object::new_integer(BigInt::from(1))),
        pair(first_atom, Object::new_integer(BigInt::from(2))),
        pair(second_atom, Object::new_integer(BigInt::from(4))),
    ];

    assert!(result.is_composite());
    assert_eq!(result.size(&knowledge), 3);
    assert!(result.has(&expected[0], &knowledge));
    assert!(result.has(&expected[1], &knowledge));
    assert!(result.has(&expected[2], &knowledge));
    assert!(!result.has(
        &pair(first_atom, Object::new_integer(BigInt::from(3))),
        &knowledge
    ));
    assert_eq!(result.iter(&knowledge).collect::<Vec<_>>(), expected);
}

#[test]
fn query_constructions_materializes_deferred_constructor() {
    let atom = Atom(42);
    let value = Object::new_integer(BigInt::from(7));
    let constructor = Object::Composite(Composite::from([Atom(1).into(), Atom(2).into()]));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(constructor, value.clone()).into(),
    });

    let result = evaluate(
        &Node::QueryConstructions {
            constructor: Node::Union {
                first: Node::Literal(Object::Composite(Composite::from([Atom(1).into()]))).into(),
                second: Node::Literal(Object::Composite(Composite::from([Atom(2).into()]))).into(),
            }
            .into(),
        },
        &knowledge,
    );

    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        vec![pair(atom, value)]
    );
}

#[test]
fn query_constructions_queries_undefined_as_an_atom() {
    let constructor = Atom(1);
    let value = Object::new_integer(BigInt::from(7));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom: Atom::UNDEFINED,
        object: Construction::new(constructor.into(), value.clone()).into(),
    });

    let result = evaluate(
        &Node::QueryConstructions {
            constructor: Node::Literal(constructor.into()).into(),
        },
        &knowledge,
    );

    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        vec![pair(Atom::UNDEFINED, value)]
    );
}
