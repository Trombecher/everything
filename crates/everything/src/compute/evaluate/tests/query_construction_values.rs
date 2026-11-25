use num_bigint::BigInt;

use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;

#[test]
fn query_construction_values() {
    let atom = Atom(42);
    let property_constructor = Atom(1);
    let other_constructor = Atom(2);
    let matching_values = [
        Object::new_integer(BigInt::from(1)),
        Object::new_integer(BigInt::from(2)),
    ];

    let mut knowledge = BASE.clone();

    for value in &matching_values {
        knowledge.add_statement(Statement {
            atom,
            object: Construction::new(property_constructor.into(), value.clone()).into(),
        });
    }

    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(
            other_constructor.into(),
            Object::new_integer(BigInt::from(3)),
        )
        .into(),
    });
    knowledge.add_statement(Statement {
        atom,
        object: Object::new_integer(BigInt::from(4)),
    });

    let result = evaluate(
        &Node::QueryConstructionValues {
            object: Node::Literal(Object::Atom(atom)).into(),
            property_constructor: Node::Literal(property_constructor.into()).into(),
        },
        &knowledge,
    );

    assert!(result.is_composite());
    assert_eq!(result.item_count(&knowledge), 2);
    assert!(result.has_item(&matching_values[0], &knowledge));
    assert!(result.has_item(&matching_values[1], &knowledge));
    assert!(!result.has_item(&Object::new_integer(BigInt::from(3)), &knowledge));
    assert!(!result.has_item(&Object::new_integer(BigInt::from(4)), &knowledge));
    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        matching_values.to_vec()
    );

    // An atom can be queried directly without materialising its values.
    let direct = evaluate(
        &Node::QueryConstructionValues {
            object: Node::Literal(Object::Atom(atom)).into(),
            property_constructor: Node::Literal(property_constructor.into()).into(),
        },
        &knowledge,
    );

    assert_eq!(
        direct.iter(&knowledge).collect::<Vec<_>>(),
        matching_values.to_vec()
    );
}

#[test]
fn query_construction_values_of_literal_composite() {
    let constructor = Atom(1);

    let composite = Composite::from([
        Construction::new(constructor.into(), Object::new_integer(BigInt::from(1))).into(),
        Construction::new(Atom(2).into(), Object::new_integer(BigInt::from(2))).into(),
        Construction::new(constructor.into(), Object::new_integer(BigInt::from(3))).into(),
    ]);

    let result = evaluate(
        &Node::QueryConstructionValues {
            object: Node::Literal(composite.into()).into(),
            property_constructor: Node::Literal(constructor.into()).into(),
        },
        &BASE,
    );

    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![
            Object::new_integer(BigInt::from(1)),
            Object::new_integer(BigInt::from(3)),
        ]
    );
}

#[test]
fn query_construction_values_materialises_deferred_constructor() {
    let constructor = Object::Composite(Composite::empty());
    let value = Object::new_integer(BigInt::from(7));
    let atom = Atom(42);

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(constructor, value.clone()).into(),
    });

    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let deferred_constructor = Node::SetUnion {
        first: empty_composite.clone().into(),
        second: empty_composite.into(),
    };

    let result = evaluate(
        &Node::QueryConstructionValues {
            object: Node::Literal(Object::Atom(atom)).into(),
            property_constructor: deferred_constructor.into(),
        },
        &knowledge,
    );

    assert!(result.is_composite());
    assert_eq!(result.iter(&knowledge).collect::<Vec<_>>(), vec![value]);
}

#[test]
fn query_construction_values_resolves_objects_through_the_knowledge() {
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));
    let property_constructor = Node::Literal(Atom(1).into());

    let evaluate_query = |object: Node, property_constructor: Node| {
        evaluate(
            &Node::QueryConstructionValues {
                object: object.into(),
                property_constructor: property_constructor.into(),
            },
            &BASE,
        )
    };

    // Constructions have no values and cannot be queried.
    assert_eq!(
        evaluate_query(Node::Literal(construction), property_constructor.clone())
            .into_object(&BASE),
        Atom::UNDEFINED.into()
    );

    // Atoms are resolved through the knowledge. Atoms without values yield an
    // empty deferred composite.
    let atom_result = evaluate_query(
        Node::Literal(Object::Atom(Atom(42))),
        property_constructor.clone(),
    );
    assert!(atom_result.is_composite());
    assert_eq!(atom_result.item_count(&BASE), 0);
    assert_eq!(atom_result.iter(&BASE).count(), 0);

    // `undefined` is an atom and is queried like any other atom.
    let undefined_result = evaluate_query(
        Node::Literal(Atom::UNDEFINED.into()),
        property_constructor.clone(),
    );
    assert!(undefined_result.is_composite());
    assert_eq!(undefined_result.item_count(&BASE), 0);
    assert_eq!(undefined_result.iter(&BASE).count(), 0);

    // A deferred constructor is materialised and accepted like an eager one.
    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let deferred_composite = Node::SetUnion {
        first: empty_composite.clone().into(),
        second: empty_composite.clone().into(),
    };

    let from_deferred = evaluate_query(
        Node::Literal(Object::Composite(Composite::empty())),
        deferred_composite.clone(),
    );

    assert!(from_deferred.is_composite());
    assert_eq!(from_deferred.item_count(&BASE), 0);
    assert_eq!(from_deferred.iter(&BASE).count(), 0);

    // A deferred composite target yields a deferred result.
    let result = evaluate_query(deferred_composite, Node::Literal(Atom(1).into()));

    assert!(result.is_composite());
    assert_eq!(result.item_count(&BASE), 0);
    assert_eq!(result.iter(&BASE).count(), 0);
}

#[test]
fn query_construction_values_queries_undefined_as_an_atom() {
    let constructor = Atom(1);
    let value = Object::new_integer(BigInt::from(7));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom: Atom::UNDEFINED,
        object: Construction::new(constructor.into(), value.clone()).into(),
    });

    let result = evaluate(
        &Node::QueryConstructionValues {
            object: Node::Literal(Atom::UNDEFINED.into()).into(),
            property_constructor: Node::Literal(constructor.into()).into(),
        },
        &knowledge,
    );

    assert_eq!(result.iter(&knowledge).collect::<Vec<_>>(), vec![value]);
}
