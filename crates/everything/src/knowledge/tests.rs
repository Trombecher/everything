use num_bigint::BigInt;

use crate::{
    base::BASE,
    compute::Node,
    knowledge::{Knowledge, Properties, Statement},
    model::{Atom, AtomOrComposite, Composite, Construction, Object},
};

const PERSON: Atom = Atom(1000);

fn integer(value: i32) -> Object {
    Object::new_integer(BigInt::from(value))
}

fn construction_value(constructor: Atom, value: i32) -> Object {
    Construction::new(constructor.into(), integer(value)).into()
}

fn construction(constructor: impl Into<Object>, value: impl Into<Object>) -> Object {
    Construction::new(constructor.into(), value.into()).into()
}

fn property(constructor: Atom, value: Object) -> Object {
    construction(constructor, value)
}

fn unary_constant(value: bool) -> Object {
    Node::function(Node::Literal(value.into())).into()
}

fn binary_constant(value: bool) -> Object {
    Node::function(Node::function(Node::Literal(value.into()))).into()
}

fn declare_property_constructor(knowledge: &mut Knowledge, atom: Atom, constraint: Object) {
    knowledge.add_statement(Statement {
        atom,
        object: property(Atom::PROPERTY_CONSTRUCTOR, constraint),
    });
}

/// Returns the base knowledge extended with a permissive `PERSON` property.
fn base_knowledge() -> Knowledge {
    let mut knowledge = BASE.clone();

    knowledge.add_statement(Statement {
        atom: PERSON,
        object: property(Atom::NON_CONSTRUCTED_PROPERTY, unary_constant(true)),
    });

    knowledge
}

fn contains(item: Object) -> Object {
    construction(Atom::CONTAINS, item)
}

/// Returns a composite whose members include a unique `property` constraint.
fn composite_with_property_constraint<const N: usize>(
    constraint: Object,
    items: [Object; N],
) -> Object {
    let mut members = items.into_iter().collect::<Vec<_>>();
    members.push(property(Atom::NON_CONSTRUCTED_PROPERTY, constraint));

    Composite::from(members.as_mut_slice()).into()
}

#[test]
fn query_construction_values_of_atom_yields_matching_values() {
    let constructor = Atom(1);
    let other_constructor = Atom(2);
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();

    knowledge.add_statement(Statement {
        atom,
        object: construction_value(constructor, 1),
    });
    knowledge.add_statement(Statement {
        atom,
        object: construction_value(constructor, 2),
    });
    knowledge.add_statement(Statement {
        atom,
        object: construction_value(other_constructor, 3),
    });
    knowledge.add_statement(Statement {
        atom,
        object: integer(4),
    });

    let mut values = knowledge
        .query_construction_values(&atom.into(), constructor.into())
        .expect("an atom with statements has properties")
        .collect::<Vec<_>>();

    values.sort();

    assert_eq!(values, vec![integer(1), integer(2)]);
}

#[test]
fn query_construction_values_of_composite_yields_matching_member_values() {
    let constructor = Atom(1);

    let knowledge = Knowledge::empty();

    let composite = Composite::from([
        construction_value(constructor, 1),
        integer(2),
        construction_value(constructor, 3),
    ]);

    let mut values = knowledge
        .query_construction_values(&composite.into(), constructor.into())
        .expect("a composite has properties")
        .collect::<Vec<_>>();

    values.sort();

    assert_eq!(values, vec![integer(1), integer(3)]);
}

#[test]
fn query_construction_values_of_construction_is_none() {
    let knowledge = Knowledge::empty();

    let construction = construction_value(Atom(1), 1);

    assert!(
        knowledge
            .query_construction_values(&construction, Atom(2).into())
            .is_none()
    );
}

#[test]
fn construction_is_not_an_atom_or_composite() {
    assert!(AtomOrComposite::try_from(construction_value(Atom(1), 1)).is_err());
}

#[test]
fn object_has_property_distinguishes_known_and_unknown_properties() {
    let property = integer(1);
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: property.clone(),
    });

    assert!(knowledge.object_has_property(&AtomOrComposite::Atom(atom), &property));
    assert!(!knowledge.object_has_property(&AtomOrComposite::Atom(Atom(43)), &property));
}

#[test]
fn object_has_item_distinguishes_contained_and_absent_items() {
    let atom = Atom(42);
    let contained = integer(1);
    let not_contained = integer(2);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: contains(contained.clone()),
    });

    let atom = AtomOrComposite::Atom(atom);

    assert!(knowledge.object_has_item(&atom, &contained));
    assert!(!knowledge.object_has_item(&atom, &not_contained));
}

#[test]
fn items_of_an_atom_yields_its_contains_values() {
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: contains(integer(1)),
    });
    knowledge.add_statement(Statement {
        atom,
        object: integer(2),
    });

    assert_eq!(
        knowledge
            .items_of(&AtomOrComposite::Atom(atom))
            .collect::<Vec<_>>(),
        vec![integer(1)]
    );
}

#[test]
fn item_count_of_an_atom_counts_its_contains_values() {
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: contains(integer(1)),
    });
    knowledge.add_statement(Statement {
        atom,
        object: contains(integer(2)),
    });

    assert_eq!(knowledge.item_count_of(&AtomOrComposite::Atom(atom)), 2);
}

#[test]
fn size_of_object_counts_the_properties_of_an_atom() {
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: integer(1),
    });
    knowledge.add_statement(Statement {
        atom,
        object: integer(2),
    });

    assert_eq!(knowledge.size_of_object(&AtomOrComposite::Atom(atom)), 2);
}

#[test]
fn empty_knowledge_is_valid() {
    assert!(Knowledge::empty().is_valid());
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

    assert!(knowledge.is_valid());
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

    assert!(knowledge.object_is_valid(&constructor));
    assert!(knowledge.is_valid());
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

    assert!(knowledge.object_is_valid(&inner_true));
    assert!(knowledge.object_is_valid(&outer(inner_true)));

    assert!(knowledge.object_is_valid(&inner_false));
    assert!(!knowledge.object_is_valid(&outer(inner_false)));
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

    assert!(knowledge.is_valid());
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
    assert!(knowledge.object_is_valid(&age_minus_one));

    // Using it as a composite member applies the `age` constraint to that
    // composite as the subject.
    let composite = Composite::from([age_minus_one.clone()]);
    assert!(!knowledge.object_is_valid(&composite.clone().into()));

    // An outer construction does not run the nested constraint.
    let outer = construction(other, age_minus_one);
    assert!(knowledge.object_is_valid(&outer));

    knowledge.add_statement(Statement {
        atom: Atom(1510),
        object: outer,
    });

    assert!(knowledge.is_valid());
}

#[test]
fn statement_using_a_rejecting_constructor_is_invalid() {
    let mut knowledge = BASE.clone();

    let age = Atom(1600);
    declare_property_constructor(&mut knowledge, age, binary_constant(false));

    let subject = Atom(1601);
    let age_minus_one = construction(age, integer(-1));

    knowledge.add_statement(Statement {
        atom: subject,
        object: age_minus_one.clone(),
    });

    assert!(!knowledge.object_has_valid_property(&AtomOrComposite::Atom(subject), &age_minus_one));
    assert!(!knowledge.is_valid());
}

#[test]
fn unknown_constructor_makes_knowledge_invalid() {
    let mut knowledge = BASE.clone();

    knowledge.add_statement(Statement {
        atom: Atom(1700),
        object: construction(Atom(1701), Atom(1702)),
    });

    assert!(!knowledge.is_valid());
}

#[test]
fn only_exact_true_accepts_a_property() {
    let mut knowledge = BASE.clone();

    let constructor = Atom(1800);
    let constraint: Object = Node::function(Node::function(Node::Literal(Atom(42).into()))).into();
    let subject = Atom(1801);
    let property = construction(constructor, Atom(1802));

    declare_property_constructor(&mut knowledge, constructor, constraint);
    knowledge.add_statement(Statement {
        atom: subject,
        object: property.clone(),
    });

    assert!(!knowledge.object_has_valid_property(&AtomOrComposite::Atom(subject), &property));
    assert!(!knowledge.is_valid());
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

    assert!(knowledge.is_valid());
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

    assert!(!knowledge.object_has_valid_property(&subject, &property));

    knowledge.add_statement(Statement {
        atom: Atom(1901),
        object: construction(Atom::NON_CONSTRUCTED_PROPERTY, unary_constant(true)),
    });

    assert!(knowledge.object_has_valid_property(&subject, &property));

    knowledge.add_statement(Statement {
        atom: Atom(1901),
        object: construction(Atom::NON_CONSTRUCTED_PROPERTY, unary_constant(false)),
    });

    assert!(!knowledge.object_has_valid_property(&subject, &property));
}

#[test]
fn property_must_belong_to_the_subject() {
    let knowledge = base_knowledge();

    assert!(
        !knowledge.object_has_valid_property(&AtomOrComposite::Atom(Atom(2000)), &PERSON.into())
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

    assert!(!knowledge.is_valid());
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

    assert!(!knowledge.is_valid());
}

#[test]
fn permissive_base_constraints_invalidate_knowledge() {
    let mut knowledge = BASE.clone();

    knowledge.map.insert(
        Atom::PROPERTY_CONSTRUCTOR,
        Properties::from([property(Atom::PROPERTY_CONSTRUCTOR, binary_constant(true))]),
    );

    assert!(!knowledge.is_valid());
}
