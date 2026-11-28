use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Object},
};

use super::common::*;
use crate::compute::*;

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

    assert!(result.is_composite());
    assert_eq!(set(&result).item_count(&knowledge), 2);
    assert!(set(&result).has_item(&matching[0].into(), &knowledge));
    assert!(set(&result).has_item(&matching[1].into(), &knowledge));
    assert!(!set(&result).has_item(&other.into(), &knowledge));
    assert_eq!(
        set(&result).iter(&knowledge).collect::<Vec<_>>(),
        vec![matching[0].into(), matching[1].into()]
    );
}

#[test]
fn query_atoms_with_deferred_property_is_materialised() {
    let atom = Atom(42);
    let property = Object::Composite(Composite::empty());

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: property,
    });

    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));

    let result = evaluate(
        &Node::QueryAtomsWithProperty {
            property: Node::SetUnion {
                first: empty_composite.clone().into(),
                second: empty_composite.into(),
            }
            .into(),
        },
        &knowledge,
    );

    assert!(result.is_composite());
    assert_eq!(
        set(&result).iter(&knowledge).collect::<Vec<_>>(),
        vec![atom.into()]
    );
}
