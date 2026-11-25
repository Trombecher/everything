use num_bigint::BigInt;

use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn has_value() {
    let atom = Atom(42);
    let value = Object::new_integer(BigInt::from(1));
    let empty_set = Object::Composite(Composite::empty());
    let plain_composite = Object::Composite(Composite::from([value.clone()]));
    let item_set = set_of([value.clone()]);
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));

    let mut knowledge = BASE.clone();

    let has_value = |object: Object, knowledge: &Knowledge| {
        evaluate(&Node::HasValue(Node::Literal(object).into()), knowledge).into_object(knowledge)
    };

    // Atoms have a value iff they have a statement in the knowledge.
    assert_eq!(
        has_value(Object::Atom(atom), &knowledge),
        Atom::FALSE.into()
    );
    assert_eq!(has_value(empty_set.clone(), &knowledge), Atom::FALSE.into());
    // Composites have a value iff they have an item.
    assert_eq!(
        has_value(plain_composite.clone(), &knowledge),
        Atom::FALSE.into()
    );
    assert_eq!(has_value(item_set, &knowledge), Atom::TRUE.into());
    assert_eq!(has_value(construction, &knowledge), Atom::UNDEFINED.into());
    assert_eq!(
        has_value(Atom::UNDEFINED.into(), &knowledge),
        Atom::FALSE.into()
    );

    // A function object is a composite without `contains` members.
    assert_eq!(
        evaluate(&Node::HasValue(identity_function().into()), &knowledge).into_object(&knowledge),
        Atom::FALSE.into()
    );

    knowledge.add_statement(Statement {
        atom,
        object: value,
    });

    assert_eq!(has_value(Object::Atom(atom), &knowledge), Atom::TRUE.into());

    knowledge.add_statement(Statement {
        atom: Atom::UNDEFINED,
        object: Atom(3).into(),
    });

    assert_eq!(
        has_value(Atom::UNDEFINED.into(), &knowledge),
        Atom::TRUE.into()
    );

    let union = Node::SetUnion {
        first: Node::Literal(empty_set).into(),
        second: Node::Literal(plain_composite).into(),
    };

    assert_eq!(
        evaluate(&Node::HasValue(union.into()), &knowledge).into_object(&knowledge),
        Atom::FALSE.into()
    );
}
