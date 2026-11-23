use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Object, Set},
};

use super::super::*;

#[test]
fn query_atoms_with_property() {
    let property = Atom(1);
    let matching = [Atom(42), Atom(43)];
    let other = Atom(44);

    let mut knowledge = BASE.clone();

    for atom in matching {
        knowledge.add_statement(Statement {
            atom,
            object: property.into(),
        });
    }

    knowledge.add_statement(Statement {
        atom: other,
        object: Atom(2).into(),
    });

    let result = evaluate(
        &Node::QueryAtomsWithProperty {
            property: Node::Literal(property.into()).into(),
        },
        &knowledge,
    );

    assert!(result.is_set());
    assert_eq!(result.size(&knowledge), 2);
    assert!(result.has(&matching[0].into(), &knowledge));
    assert!(result.has(&matching[1].into(), &knowledge));
    assert!(!result.has(&other.into(), &knowledge));
    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        vec![matching[0].into(), matching[1].into()]
    );
}

#[test]
fn query_atoms_with_deferred_property_is_materialised() {
    let atom = Atom(42);
    let property = Object::Set(Set::empty());

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: property,
    });

    let empty_set = Node::Literal(Object::Set(Set::empty()));

    let result = evaluate(
        &Node::QueryAtomsWithProperty {
            property: Node::Union {
                first: empty_set.clone().into(),
                second: empty_set.into(),
            }
            .into(),
        },
        &knowledge,
    );

    assert!(result.is_set());
    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        vec![atom.into()]
    );
}
