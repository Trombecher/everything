use num_bigint::BigInt;

use crate::{
    base::BASE,
    compute::Node,
    knowledge::{Knowledge, Statement},
    model::{Atom, AtomOrComposite, Construction, Object},
};

fn with_statement(atom: Atom, property: Object) -> Knowledge {
    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: property,
    });

    knowledge
}

#[test]
fn base_knowledge_is_valid() {
    assert!(BASE.is_valid().is_ok());
}

#[test]
fn base_knowledge_accepts_integers() {
    assert!(BASE.object_is_valid(&Atom::ZERO.into()).is_ok());
    assert!(
        BASE.object_is_valid(&Object::new_integer(BigInt::ONE))
            .is_ok()
    );
    assert!(
        BASE.object_is_valid(&Object::new_integer(BigInt::NEG_ONE))
            .is_ok()
    );
}

#[test]
fn base_knowledge_accepts_the_property_constructor_constraint() {
    let constraint = BASE
        .properties_of_atom(&Atom::PROPERTY_CONSTRUCTOR)
        .into_iter()
        .next()
        .expect("property_constructor has a constraint");

    assert!(BASE.object_is_valid(&constraint).is_ok());
}

#[test]
fn base_constraints_are_nodes() {
    for constraint in [
        super::accepts_everything(),
        super::accepts_only_nodes(),
        super::property_constraint(Atom::NON_CONSTRUCTED_PROPERTY),
        super::property_constraint(Atom::PROPERTY_CONSTRUCTOR),
    ] {
        let parsed = Node::new(&constraint, &BASE).expect("base constraint is a node");

        assert_eq!(parsed.into_object(), constraint);
    }
}

#[test]
fn node_property_constraints_require_node_values() {
    let subject = AtomOrComposite::Atom(Atom(3000));
    let non_node: Object =
        Construction::new(Atom::NODE_FUNCTION.into(), Object::new_integer(BigInt::ONE)).into();
    let node: Object = Construction::new(
        Atom::NODE_FUNCTION.into(),
        Node::Literal(Object::new_integer(BigInt::ONE)).into_object(),
    )
    .into();

    let knowledge = with_statement(Atom(3000), non_node.clone());

    assert!(
        knowledge
            .object_has_valid_property(&subject, &non_node)
            .is_err()
    );

    let knowledge = with_statement(Atom(3000), node.clone());

    assert!(knowledge.object_has_valid_property(&subject, &node).is_ok());
}

#[test]
fn non_node_properties_still_accept_arbitrary_values() {
    let subject = AtomOrComposite::Atom(Atom(3001));
    let property: Object =
        Construction::new(Atom::NODE_LITERAL.into(), Object::new_integer(BigInt::ONE)).into();

    let knowledge = with_statement(Atom(3001), property.clone());

    assert!(
        knowledge
            .object_has_valid_property(&subject, &property)
            .is_ok()
    );
}

#[test]
fn property_constructor_declarations_require_node_constraints() {
    let subject = AtomOrComposite::Atom(Atom(3002));
    let non_node: Object = Construction::new(
        Atom::PROPERTY_CONSTRUCTOR.into(),
        Object::new_integer(BigInt::ONE),
    )
    .into();
    let constraint = Node::function(Node::function(Node::Literal(true.into()))).into_object();
    let node: Object = Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), constraint).into();

    let knowledge = with_statement(Atom(3002), non_node.clone());

    assert!(
        knowledge
            .object_has_valid_property(&subject, &non_node)
            .is_err()
    );

    let knowledge = with_statement(Atom(3002), node.clone());

    assert!(knowledge.object_has_valid_property(&subject, &node).is_ok());
}

#[test]
fn base_knowledge_accepts_an_is_node_object() {
    let object = Node::IsNode(Node::Literal(Object::new_integer(BigInt::ONE)).into()).into_object();

    assert!(BASE.object_is_valid(&object).is_ok());
}
