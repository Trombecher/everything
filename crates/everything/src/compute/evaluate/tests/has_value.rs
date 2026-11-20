use num_bigint::BigInt;

use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;
use super::common::*;

#[test]
fn has_value() {
    let atom = Atom(42);
    let value = Object::new_integer(BigInt::from(1));
    let empty_set = Object::Set(Set::empty());
    let non_empty_set = Object::Set(Set::from([value.clone()]));
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));

    let mut knowledge = BASE.clone();

    let has_value = |object: Object, knowledge: &Knowledge| {
        evaluate(&Node::HasValue(Node::Literal(object).into()), knowledge).into_object(knowledge)
    };

    assert_eq!(
        has_value(Object::Atom(atom), &knowledge),
        Atom::FALSE.into()
    );
    assert_eq!(has_value(empty_set.clone(), &knowledge), Atom::FALSE.into());
    assert_eq!(
        has_value(non_empty_set.clone(), &knowledge),
        Atom::TRUE.into()
    );
    assert_eq!(has_value(construction, &knowledge), Atom::UNDEFINED.into());
    assert_eq!(
        has_value(Atom::UNDEFINED.into(), &knowledge),
        Atom::UNDEFINED.into()
    );

    assert_eq!(
        evaluate(&Node::HasValue(identity_function().into()), &knowledge).into_object(&knowledge),
        Atom::TRUE.into()
    );

    knowledge.add_statement(Statement {
        atom,
        object: value,
    });

    assert_eq!(has_value(Object::Atom(atom), &knowledge), Atom::TRUE.into());

    let union = Node::Union {
        first: Node::Literal(empty_set).into(),
        second: Node::Literal(non_empty_set).into(),
    };

    assert_eq!(
        evaluate(&Node::HasValue(union.into()), &knowledge).into_object(&knowledge),
        Atom::TRUE.into()
    );
}
