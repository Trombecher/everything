use num_bigint::BigInt;

use crate::{
    base::BASE,
    compute::Node,
    model::{Atom, Object},
};

#[test]
fn base_knowledge_is_valid() {
    assert!(BASE.is_valid());
}

#[test]
fn base_knowledge_accepts_integers() {
    assert!(BASE.object_is_valid(&Atom::ZERO.into()));
    assert!(BASE.object_is_valid(&Object::new_integer(BigInt::ONE)));
    assert!(BASE.object_is_valid(&Object::new_integer(BigInt::NEG_ONE)));
}

#[test]
fn base_knowledge_accepts_the_property_constructor_constraint() {
    let constraint = BASE
        .properties_of_atom(&Atom::PROPERTY_CONSTRUCTOR)
        .into_iter()
        .next()
        .expect("property_constructor has a constraint");

    assert!(BASE.object_is_valid(&constraint));
}

#[test]
fn base_constraints_are_nodes() {
    for constraint in [
        super::accepts_everything(),
        super::property_constraint(Atom::NON_CONSTRUCTED_PROPERTY),
        super::property_constraint(Atom::PROPERTY_CONSTRUCTOR),
    ] {
        let parsed = Node::new(&constraint, &BASE).expect("base constraint is a node");

        assert_eq!(parsed.into_object(), constraint);
    }
}
