use crate::{
    base::BASE,
    compute::Node,
    model::{Atom, AtomOrComposite, Composite, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn empty_knowledge_is_valid() {
    assert!(Knowledge::empty().is_valid().is_ok());
}

#[test]
fn knowledge_with_nested_constructed_properties_is_valid() {
    let mut knowledge = base_knowledge();

    let born = Atom(1100);
    let date_time = Atom(1101);
    let constraint = binary_constant(true);

    declare_property_constructor(&mut knowledge, born, constraint.clone());
    declare_property_constructor(&mut knowledge, date_time, constraint);

    let david = Atom(1110);

    knowledge.add_statement(Statement {
        atom: david,
        object: PERSON.into(),
    });
    knowledge.add_statement(Statement {
        atom: david,
        object: construction(born, construction(date_time, integer(2))),
    });

    assert!(knowledge.is_valid().is_ok());
}

#[test]
fn composite_can_be_a_property_constructor() {
    let mut knowledge = BASE.clone();

    let constructor: Object =
        Composite::from([property(Atom::PROPERTY_CONSTRUCTOR, binary_constant(true))]).into();
    let subject = Atom(1200);

    knowledge.add_statement(Statement {
        atom: subject,
        object: construction(constructor.clone(), Atom(1201)),
    });

    assert!(knowledge.object_is_valid(&constructor).is_ok());
    assert!(knowledge.is_valid().is_ok());
}

#[test]
fn nested_composites_are_validated_with_themselves_as_subject() {
    let knowledge = BASE.clone();

    let inner_true = composite_with_property_constraint(
        unary_constant(true),
        [contains(Atom(1300).into()), contains(Atom(1301).into())],
    );
    let inner_false =
        composite_with_property_constraint(unary_constant(false), [contains(Atom(1302).into())]);
    let outer =
        |inner: Object| -> Object { Composite::from([inner, contains(Atom(1303).into())]).into() };

    assert!(knowledge.object_is_valid(&inner_true).is_ok());
    assert!(knowledge.object_is_valid(&outer(inner_true)).is_ok());

    assert!(knowledge.object_is_valid(&inner_false).is_ok());
    assert!(knowledge.object_is_valid(&outer(inner_false)).is_err());
}

#[test]
fn constraint_arguments_are_passed_as_literals() {
    let mut knowledge = BASE.clone();

    let node_like = Atom(1400);
    let constructor = Atom(1401);

    let constraint: Object = Node::function(Node::function(Node::And {
        first: Node::Equal {
            first: Node::Parameter { depth: 1 }.into(),
            second: Node::Literal(node_like.into()).into(),
        }
        .into(),
        second: Node::Equal {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Literal(node_like.into()).into(),
        }
        .into(),
    }))
    .into();

    declare_property_constructor(&mut knowledge, constructor, constraint);

    // As a node, this atom is a function instead of itself.
    knowledge.add_statement(Statement {
        atom: node_like,
        object: construction(Atom::NODE_FUNCTION, unary_constant(true)),
    });
    knowledge.add_statement(Statement {
        atom: node_like,
        object: construction(constructor, node_like),
    });

    assert!(knowledge.is_valid().is_ok());
}

#[test]
fn value_only_construction_is_valid_without_running_its_constraint() {
    let mut knowledge = BASE.clone();

    let age = Atom(1500);
    let other = Atom(1501);

    declare_property_constructor(&mut knowledge, age, binary_constant(false));
    declare_property_constructor(&mut knowledge, other, binary_constant(true));

    let age_minus_one = construction(age, integer(-1));

    // The construction is structurally valid even though `age` rejects its
    // value.
    assert!(knowledge.object_is_valid(&age_minus_one).is_ok());

    // Using it as a composite member applies the `age` constraint to that
    // composite as the subject.
    let composite = Composite::from([age_minus_one.clone()]);
    assert!(
        knowledge
            .object_is_valid(&composite.clone().into())
            .is_err()
    );

    // An outer construction does not run the nested constraint.
    let outer = construction(other, age_minus_one);
    assert!(knowledge.object_is_valid(&outer).is_ok());

    knowledge.add_statement(Statement {
        atom: Atom(1510),
        object: outer,
    });

    assert!(knowledge.is_valid().is_ok());
}

#[test]
fn statement_using_a_rejecting_constructor_is_invalid() {
    let mut knowledge = BASE.clone();

    let age = Atom(1600);
    let constraint = binary_constant(false);
    declare_property_constructor(&mut knowledge, age, constraint.clone());

    let subject = Atom(1601);
    let age_minus_one = construction(age, integer(-1));

    knowledge.add_statement(Statement {
        atom: subject,
        object: age_minus_one.clone(),
    });

    let object = AtomOrComposite::Atom(subject);

    assert_eq!(
        knowledge.object_has_valid_property(&object, &age_minus_one),
        Err(InvalidPropertyError::RejectedByConstraint {
            object: object.clone(),
            property: age_minus_one.clone(),
            constraint: constraint.clone(),
        })
    );
    assert_eq!(
        knowledge.is_valid(),
        Err(InvalidKnowledgeError::InvalidStatement {
            atom: subject,
            property: age_minus_one.clone(),
            source: Box::new(InvalidPropertyError::RejectedByConstraint {
                object,
                property: age_minus_one,
                constraint,
            }),
        })
    );
}

#[test]
fn unknown_constructor_makes_knowledge_invalid() {
    let mut knowledge = BASE.clone();

    let unknown = construction(Atom(1701), Atom(1702));

    knowledge.add_statement(Statement {
        atom: Atom(1700),
        object: unknown.clone(),
    });

    assert_eq!(
        knowledge.object_is_valid(&unknown),
        Err(InvalidObjectError::NotAPropertyConstructor {
            constructor: Atom(1701).into(),
        })
    );
    assert_eq!(
        knowledge.is_valid(),
        Err(InvalidKnowledgeError::InvalidStatement {
            atom: Atom(1700),
            property: unknown.clone(),
            source: Box::new(InvalidPropertyError::InvalidObject {
                property: unknown,
                source: Box::new(InvalidObjectError::NotAPropertyConstructor {
                    constructor: Atom(1701).into(),
                }),
            }),
        })
    );
}

#[test]
fn only_exact_true_accepts_a_property() {
    let mut knowledge = BASE.clone();

    let constructor = Atom(1800);
    let constraint: Object = Node::function(Node::function(Node::Literal(Atom(42).into()))).into();
    let subject = AtomOrComposite::Atom(Atom(1801));
    let property = construction(constructor, Atom(1802));

    declare_property_constructor(&mut knowledge, constructor, constraint.clone());
    knowledge.add_statement(Statement {
        atom: Atom(1801),
        object: property.clone(),
    });

    assert_eq!(
        knowledge.object_has_valid_property(&subject, &property),
        Err(InvalidPropertyError::RejectedByConstraint {
            object: subject,
            property,
            constraint,
        })
    );
}

#[test]
fn constraint_sees_the_statement_being_validated() {
    let mut knowledge = BASE.clone();

    let constructor = Atom(2000);
    let subject = Atom(2001);
    let value = Atom(2002);

    let constraint: Object = Node::function(Node::function(Node::SetContains {
        element: Node::NewConstruction {
            constructor: Node::Literal(constructor.into()).into(),
            value: Node::Parameter { depth: 0 }.into(),
        }
        .into(),
        set: Node::PropertiesOf(Node::Parameter { depth: 1 }.into()).into(),
    }))
    .into();

    declare_property_constructor(&mut knowledge, constructor, constraint);

    knowledge.add_statement(Statement {
        atom: subject,
        object: construction(constructor, value),
    });

    assert!(knowledge.is_valid().is_ok());
}

#[test]
fn non_constructed_property_requires_exactly_one_constraint() {
    let mut knowledge = BASE.clone();

    let subject = Atom(1900);
    let property = Atom(1901);

    knowledge.add_statement(Statement {
        atom: subject,
        object: property.into(),
    });

    let subject = AtomOrComposite::Atom(subject);
    let property = property.into();

    assert_eq!(
        knowledge.object_has_valid_property(&subject, &property),
        Err(
            InvalidPropertyError::MissingNonConstructedPropertyConstraint {
                property: AtomOrComposite::Atom(Atom(1901)),
            }
        )
    );

    knowledge.add_statement(Statement {
        atom: Atom(1901),
        object: construction(Atom::NON_CONSTRUCTED_PROPERTY, unary_constant(true)),
    });

    assert!(
        knowledge
            .object_has_valid_property(&subject, &property)
            .is_ok()
    );

    knowledge.add_statement(Statement {
        atom: Atom(1901),
        object: construction(Atom::NON_CONSTRUCTED_PROPERTY, unary_constant(false)),
    });

    assert_eq!(
        knowledge.object_has_valid_property(&subject, &property),
        Err(
            InvalidPropertyError::MissingNonConstructedPropertyConstraint {
                property: AtomOrComposite::Atom(Atom(1901)),
            }
        )
    );
}

#[test]
fn property_must_belong_to_the_subject() {
    let knowledge = base_knowledge();

    let subject = AtomOrComposite::Atom(Atom(2000));

    assert_eq!(
        knowledge.object_has_valid_property(&subject, &PERSON.into()),
        Err(InvalidPropertyError::DoesNotBelong {
            object: subject,
            property: PERSON.into(),
        })
    );
}

#[test]
fn duplicate_property_constructor_constraints_invalidate_knowledge() {
    let mut knowledge = BASE.clone();

    let constructor = Atom(2100);
    let first = binary_constant(true);
    let second: Object = Node::function(Node::function(Node::Literal(Atom(2101).into()))).into();

    knowledge.add_statement(Statement {
        atom: constructor,
        object: property(Atom::PROPERTY_CONSTRUCTOR, first),
    });
    knowledge.add_statement(Statement {
        atom: constructor,
        object: property(Atom::PROPERTY_CONSTRUCTOR, second),
    });

    assert!(knowledge.is_valid().is_err());
}

#[test]
fn duplicate_non_constructed_property_constraints_invalidate_knowledge() {
    let mut knowledge = BASE.clone();

    let subject = Atom(2200);

    knowledge.add_statement(Statement {
        atom: subject,
        object: property(Atom::NON_CONSTRUCTED_PROPERTY, unary_constant(true)),
    });
    knowledge.add_statement(Statement {
        atom: subject,
        object: property(Atom::NON_CONSTRUCTED_PROPERTY, unary_constant(false)),
    });

    assert!(knowledge.is_valid().is_err());
}

#[test]
fn permissive_base_constraints_invalidate_knowledge() {
    let mut knowledge = BASE.clone();

    knowledge.map.insert(
        Atom::PROPERTY_CONSTRUCTOR,
        Properties::from([property(Atom::PROPERTY_CONSTRUCTOR, binary_constant(true))]),
    );

    assert_eq!(
        knowledge.is_valid(),
        Err(InvalidKnowledgeError::BaseConstraintNotPinned {
            atom: Atom::PROPERTY_CONSTRUCTOR,
        })
    );
}
