use num_bigint::BigInt;

use crate::{
    base::BASE,
    compute::Node,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;

pub const PERSON: Atom = Atom(1000);

pub fn integer(value: i32) -> Object {
    Object::new_integer(BigInt::from(value))
}

pub fn construction_value(constructor: Atom, value: i32) -> Object {
    Construction::new(constructor.into(), integer(value)).into()
}

pub fn construction(constructor: impl Into<Object>, value: impl Into<Object>) -> Object {
    Construction::new(constructor.into(), value.into()).into()
}

pub fn property(constructor: Atom, value: Object) -> Object {
    construction(constructor, value)
}

pub fn unary_constant(value: bool) -> Object {
    Node::function(Node::Literal(value.into())).into()
}

pub fn binary_constant(value: bool) -> Object {
    Node::function(Node::function(Node::Literal(value.into()))).into()
}

pub fn declare_property_constructor(knowledge: &mut Knowledge, atom: Atom, constraint: Object) {
    knowledge.add_statement(Statement {
        atom,
        object: property(Atom::PROPERTY_CONSTRUCTOR, constraint),
    });
}

/// Returns the base knowledge extended with a permissive `PERSON` property.
pub fn base_knowledge() -> Knowledge {
    let mut knowledge = BASE.clone();

    knowledge.add_statement(Statement {
        atom: PERSON,
        object: property(Atom::NON_CONSTRUCTED_PROPERTY, unary_constant(true)),
    });

    knowledge
}

pub fn contains(item: Object) -> Object {
    construction(Atom::CONTAINS, item)
}

/// Returns a composite whose members include a unique `property` constraint.
pub fn composite_with_property_constraint<const N: usize>(
    constraint: Object,
    items: [Object; N],
) -> Object {
    let mut members = items.into_iter().collect::<Vec<_>>();
    members.push(property(Atom::NON_CONSTRUCTED_PROPERTY, constraint));

    Composite::from(members.as_mut_slice()).into()
}
