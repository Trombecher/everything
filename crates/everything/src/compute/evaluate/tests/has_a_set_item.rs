use num_bigint::BigInt;

use crate::{
    base::BASE,
    knowledge::{Knowledge, Statement},
    model::{Atom, Composite, Construction, Object},
};

use super::common::*;
use crate::compute::*;

#[test]
fn has_a_set_item() {
    let atom = Atom(42);
    let value = Object::new_integer(BigInt::from(1));
    let empty_set = Object::Composite(Composite::empty());
    let plain_composite = Object::Composite(Composite::from([value.clone()]));
    let item_set = set_of([value.clone()]);
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));

    let mut knowledge = BASE.clone();

    let has_a_set_item = |object: Object, knowledge: &Knowledge| {
        evaluate(&Node::HasASetItem(Node::Literal(object).into()), knowledge).into_object(knowledge)
    };

    // Atoms have a set item iff they have a `contains` statement in the
    // knowledge.
    assert_eq!(
        has_a_set_item(Object::Atom(atom), &knowledge),
        Atom::FALSE.into()
    );
    assert_eq!(
        has_a_set_item(empty_set.clone(), &knowledge),
        Atom::FALSE.into()
    );
    // Composites have a set item iff they have a `contains` item.
    assert_eq!(
        has_a_set_item(plain_composite.clone(), &knowledge),
        Atom::FALSE.into()
    );
    assert_eq!(has_a_set_item(item_set, &knowledge), Atom::TRUE.into());
    assert_eq!(
        has_a_set_item(construction, &knowledge),
        Atom::UNDEFINED.into()
    );
    assert_eq!(
        has_a_set_item(Atom::UNDEFINED.into(), &knowledge),
        Atom::FALSE.into()
    );

    // A function object is a composite without `contains` members.
    assert_eq!(
        evaluate(&Node::HasASetItem(identity_function().into()), &knowledge)
            .into_object(&knowledge),
        Atom::FALSE.into()
    );

    // A plain property is not a set item.
    knowledge.add_statement(Statement {
        atom,
        object: value.clone(),
    });

    assert_eq!(
        has_a_set_item(Object::Atom(atom), &knowledge),
        Atom::FALSE.into()
    );

    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(Atom::CONTAINS.into(), value).into(),
    });

    assert_eq!(
        has_a_set_item(Object::Atom(atom), &knowledge),
        Atom::TRUE.into()
    );

    knowledge.add_statement(Statement {
        atom: Atom::UNDEFINED,
        object: Construction::new(Atom::CONTAINS.into(), Atom(3).into()).into(),
    });

    assert_eq!(
        has_a_set_item(Atom::UNDEFINED.into(), &knowledge),
        Atom::TRUE.into()
    );

    let union = Node::SetUnion {
        first: Node::Literal(empty_set).into(),
        second: Node::Literal(plain_composite).into(),
    };

    assert_eq!(
        evaluate(&Node::HasASetItem(union.into()), &knowledge).into_object(&knowledge),
        Atom::FALSE.into()
    );
}
