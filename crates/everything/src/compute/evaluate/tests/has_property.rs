use crate::{
    base::BASE,
    knowledge::{Knowledge, Statement},
    model::{Atom, Construction, Object},
};

use super::common::*;
use crate::compute::*;

fn has_property(subject: Node, property: Node, knowledge: &Knowledge) -> Object {
    evaluate(
        &Node::HasProperty {
            subject: subject.into(),
            property: property.into(),
        },
        knowledge,
    )
    .into_object(knowledge)
}

fn contains(item: Object) -> Object {
    Construction::new(Atom::CONTAINS.into(), item).into()
}

#[test]
fn has_property_checks_atoms_against_the_knowledge() {
    let atom = Atom(42);
    let property: Object = Construction::new(Atom(7).into(), Atom(8).into()).into();

    let mut knowledge = BASE.clone();

    assert_eq!(
        has_property(
            Node::Literal(atom.into()),
            Node::Literal(property.clone()),
            &knowledge
        ),
        Atom::FALSE.into()
    );

    knowledge.add_statement(Statement {
        atom,
        object: property.clone(),
    });

    assert_eq!(
        has_property(
            Node::Literal(atom.into()),
            Node::Literal(property),
            &knowledge
        ),
        Atom::TRUE.into()
    );
    assert_eq!(
        has_property(
            Node::Literal(atom.into()),
            Node::Literal(integer(1)),
            &knowledge
        ),
        Atom::FALSE.into()
    );
}

#[test]
fn has_property_checks_composite_members() {
    let property = integer(1);
    let subject = composite_of([property.clone(), integer(2)]);

    assert_eq!(
        has_property(
            Node::Literal(subject.clone()),
            Node::Literal(property.clone()),
            &BASE
        ),
        Atom::TRUE.into()
    );
    assert_eq!(
        has_property(Node::Literal(subject), Node::Literal(integer(3)), &BASE),
        Atom::FALSE.into()
    );

    // A set encodes its items as `contains` constructions. The item itself is
    // not a property of the set.
    let set = set_of([property.clone()]);

    assert_eq!(
        has_property(
            Node::Literal(set.clone()),
            Node::Literal(property.clone()),
            &BASE
        ),
        Atom::FALSE.into()
    );
    assert_eq!(
        has_property(Node::Literal(set), Node::Literal(contains(property)), &BASE),
        Atom::TRUE.into()
    );
}

#[test]
fn has_property_accepts_deferred_sets_as_subjects() {
    let subject = Node::SetUnion {
        first: Node::Literal(set_of([integer(1)])).into(),
        second: Node::Literal(set_of([integer(2)])).into(),
    };

    assert_eq!(
        has_property(subject.clone(), Node::Literal(contains(integer(2))), &BASE),
        Atom::TRUE.into()
    );
    assert_eq!(
        has_property(subject, Node::Literal(integer(2)), &BASE),
        Atom::FALSE.into()
    );
}

#[test]
fn has_property_accepts_function_values_as_subjects() {
    let function_property = Construction::new(
        Atom::NODE_FUNCTION.into(),
        Node::Parameter { depth: 0 }.into_object(),
    )
    .into();

    assert_eq!(
        has_property(identity_function(), Node::Literal(function_property), &BASE),
        Atom::TRUE.into()
    );
    assert_eq!(
        has_property(
            identity_function(),
            Node::Literal(contains(integer(1))),
            &BASE
        ),
        Atom::FALSE.into()
    );
}

#[test]
fn has_property_requires_an_atom_or_composite_subject() {
    let construction = Construction::new(Atom(1).into(), Atom(2).into()).into();

    // The property child would not terminate if it were evaluated.
    assert_eq!(
        has_property(Node::Literal(construction), diverging_term(), &BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn undefined_is_a_first_class_subject() {
    let mut knowledge = BASE.clone();

    assert_eq!(
        has_property(
            Node::Literal(Atom::UNDEFINED.into()),
            Node::Literal(integer(1)),
            &knowledge
        ),
        Atom::FALSE.into()
    );

    knowledge.add_statement(Statement {
        atom: Atom::UNDEFINED,
        object: integer(1),
    });

    assert_eq!(
        has_property(
            Node::Literal(Atom::UNDEFINED.into()),
            Node::Literal(integer(1)),
            &knowledge
        ),
        Atom::TRUE.into()
    );
}

#[test]
fn has_property_evaluates_and_materializes_the_property() {
    let property = Node::SetUnion {
        first: Node::Literal(set_of([integer(1)])).into(),
        second: Node::Literal(set_of([integer(2)])).into(),
    };
    let materialized = set_of([integer(1), integer(2)]);

    assert_eq!(
        has_property(Node::Literal(composite_of([materialized])), property, &BASE),
        Atom::TRUE.into()
    );
}
