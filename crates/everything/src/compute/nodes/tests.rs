use imbl::OrdMap;
use num_bigint::BigInt;

use super::*;

#[test]
#[allow(clippy::too_many_lines)]
fn node_round_trip() {
    let knowledge = Knowledge { map: OrdMap::new() };

    let a = Object::new_integer(BigInt::from(1));
    let b = Object::new_integer(BigInt::from(2));
    let c = Object::new_integer(BigInt::from(3));

    let nodes = [
        Node::function(Node::Literal(a.clone())),
        Node::Literal(Node::Literal(a.clone()).into()),
        Node::And {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::FunctionSelf { depth: 0 },
        Node::Parameter { depth: 0 },
        Node::SetCount(Node::Literal(a.clone()).into()),
        Node::PropertiesOf(Node::Literal(a.clone()).into()),
        Node::HasASetItem(Node::Literal(a.clone()).into()),
        Node::HasProperty {
            subject: Node::Literal(a.clone()).into(),
            property: Node::Literal(b.clone()).into(),
        },
        Node::QueryAtomsWithProperty {
            property: Node::Literal(a.clone()).into(),
        },
        Node::QueryConstructionValues {
            object: Node::Literal(a.clone()).into(),
            property_constructor: Node::Literal(Atom::NON_CONSTRUCTED_PROPERTY.into()).into(),
        },
        Node::QueryAtomsWithConstructor {
            constructor: Node::Literal(Atom::NON_CONSTRUCTED_PROPERTY.into()).into(),
        },
        Node::QueryConstructions {
            constructor: Node::Literal(Atom::NON_CONSTRUCTED_PROPERTY.into()).into(),
        },
        Node::SetContains {
            element: Node::Literal(a.clone()).into(),
            set: Node::Literal(b.clone()).into(),
        },
        Node::Equal {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::Or {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::Xor {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::Not(Node::Literal(a.clone()).into()),
        Node::Add {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::SetUnion {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::SetIntersection {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::SetDifference {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::SetMap {
            set: Node::Literal(a.clone()).into(),
            mapper_function: Node::Literal(b.clone()).into(),
        },
        Node::SetFilter {
            set: Node::Literal(a.clone()).into(),
            filter_function: Node::Literal(b.clone()).into(),
        },
        Node::Less {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::If {
            condition: Node::Literal(a.clone()).into(),
            then: Node::Literal(b.clone()).into(),
            otherwise: Node::Literal(c.clone()).into(),
        },
        Node::SetUnwrap(Node::Literal(a.clone()).into()),
        Node::Multiply {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::Call {
            callee: Node::Literal(a.clone()).into(),
            with: Node::Literal(b.clone()).into(),
        },
        Node::IsAtom(Node::Literal(a.clone()).into()),
        Node::IsConstruction(Node::Literal(a.clone()).into()),
        Node::IsComposite(Node::Literal(a.clone()).into()),
        Node::IsNode(Node::Literal(a.clone()).into()),
        Node::SetEvery {
            set: Node::Literal(a.clone()).into(),
            predicate: Node::Literal(b.clone()).into(),
        },
        Node::SetAny {
            set: Node::Literal(a.clone()).into(),
            predicate: Node::Literal(b.clone()).into(),
        },
        Node::SetFold {
            set: Node::Literal(a.clone()).into(),
            folder: Node::Literal(b.clone()).into(),
            initial: Node::Literal(c.clone()).into(),
        },
        Node::NewConstruction {
            constructor: Node::Literal(a.clone()).into(),
            value: Node::Literal(b.clone()).into(),
        },
        Node::NewSet {
            items: vec![
                Node::Literal(a.clone()).into(),
                Node::Literal(b.clone()).into(),
            ],
        },
        Node::NewComposite {
            properties: vec![
                Node::Literal(a.clone()).into(),
                Node::Literal(b.clone()).into(),
            ],
        },
        Node::ConstructionConstructor {
            construction: Node::Literal(a.clone()).into(),
        },
        Node::ConstructionValue {
            construction: Node::Literal(a).into(),
        },
    ];

    for node in nodes {
        let object = Object::new_node(node.clone());
        let parsed = Node::new(&object, &knowledge).unwrap();

        assert_eq!(parsed.into_object(), object);
    }
}

#[test]
fn node_round_trip_preserves_extra_properties() {
    let knowledge = Knowledge { map: OrdMap::new() };
    let value = Object::new_integer(BigInt::from(1));
    let annotation = Construction::new(Atom(42).into(), Atom(7).into()).into();

    let object = Composite::from([
        Construction::new(
            Atom::NODE_ADD_FIRST.into(),
            Node::Literal(value.clone()).into_object(),
        )
        .into(),
        Construction::new(
            Atom::NODE_ADD_SECOND.into(),
            Node::Literal(value).into_object(),
        )
        .into(),
        annotation,
    ])
    .into();

    let parsed = Node::new(&object, &knowledge).unwrap();

    assert!(matches!(parsed.resolve(), Node::Add { .. }));
    assert_eq!(parsed.into_object(), object);
}

#[test]
fn node_parse_collects_all_new_composite_properties() {
    let knowledge = Knowledge { map: OrdMap::new() };
    let first = Object::new_integer(BigInt::from(1));
    let second = Object::new_integer(BigInt::from(2));

    let object = Node::NewComposite {
        properties: vec![
            Node::Literal(first.clone()).into(),
            Node::Literal(second.clone()).into(),
        ],
    }
    .into_object();

    let parsed = Node::new(&object, &knowledge).unwrap();

    let Node::NewComposite { properties } = parsed.resolve() else {
        panic!("expected a new-composite node");
    };

    assert_eq!(properties.len(), 2);
    assert_eq!(properties[0].resolve(), &Node::Literal(first));
    assert_eq!(properties[1].resolve(), &Node::Literal(second));
}

#[test]
fn node_parse_rejects_empty_object() {
    let knowledge = Knowledge { map: OrdMap::new() };
    let object = Object::Composite(Composite::empty());

    assert_eq!(
        Node::new(&object, &knowledge),
        Err(InvalidNodeError::NotANode)
    );
}

#[test]
fn node_parse_rejects_ambiguous_kind() {
    let knowledge = Knowledge { map: OrdMap::new() };
    let value = Object::new_integer(BigInt::from(1));

    let object = Object::Composite(Composite::from([
        Construction::new(Atom::NODE_LITERAL.into(), value.clone()).into(),
        Construction::new(Atom::NODE_SET_COUNT.into(), value).into(),
    ]));

    assert_eq!(
        Node::new(&object, &knowledge),
        Err(InvalidNodeError::Ambiguous {
            first: NodeKind::Literal,
            second: NodeKind::SetCount,
        })
    );
}

#[test]
fn node_parse_reports_partial_kind() {
    let knowledge = Knowledge { map: OrdMap::new() };
    let value = Object::new_integer(BigInt::from(1));

    let object = Object::Composite(Composite::from([Construction::new(
        Atom::NODE_AND_FIRST.into(),
        value,
    )
    .into()]));

    assert_eq!(
        Node::new(&object, &knowledge),
        Err(InvalidNodeError::MissingProperty {
            kind: NodeKind::And,
            property: NodeProperty::AndSecond,
        })
    );
}

#[test]
fn node_parse_prefers_a_complete_kind_over_a_partial_one() {
    let knowledge = Knowledge { map: OrdMap::new() };
    let value = Object::new_integer(BigInt::from(1));

    let object = Object::Composite(Composite::from([
        Construction::new(Atom::NODE_LITERAL.into(), value.clone()).into(),
        Construction::new(Atom::NODE_AND_FIRST.into(), value).into(),
    ]));

    let parsed = Node::new(&object, &knowledge).unwrap();

    assert_eq!(
        parsed.resolve(),
        &Node::Literal(Object::new_integer(BigInt::from(1)))
    );
    assert_eq!(parsed.into_object(), object);
}

#[test]
fn node_parse_reports_duplicate_property() {
    let knowledge = Knowledge { map: OrdMap::new() };

    let object = Object::Composite(Composite::from([
        Construction::new(
            Atom::NODE_LITERAL.into(),
            Object::new_integer(BigInt::from(1)),
        )
        .into(),
        Construction::new(
            Atom::NODE_LITERAL.into(),
            Object::new_integer(BigInt::from(2)),
        )
        .into(),
    ]));

    assert_eq!(
        Node::new(&object, &knowledge),
        Err(InvalidNodeError::DuplicateProperty {
            kind: NodeKind::Literal,
            property: NodeProperty::Literal,
        })
    );
}

#[test]
fn node_parse_reports_invalid_depth() {
    let knowledge = Knowledge { map: OrdMap::new() };
    let depth = Object::new_integer(BigInt::from(-1));

    let object = Object::Composite(Composite::from([Construction::new(
        Atom::NODE_PARAMETER.into(),
        depth,
    )
    .into()]));

    assert_eq!(
        Node::new(&object, &knowledge),
        Err(InvalidNodeError::InvalidProperty {
            kind: NodeKind::Parameter,
            property: NodeProperty::Parameter,
            expected: PropertyRequirement::Depth,
        })
    );
}

#[test]
fn node_parse_error_display_uses_node_names() {
    assert_eq!(NodeKind::And.to_string(), "and");
    assert_eq!(NodeKind::HasProperty.to_string(), "has property");
    assert_eq!(NodeKind::NewComposite.to_string(), "new composite");
    assert_eq!(NodeKind::IsNode.to_string(), "is node");
    assert_eq!(NodeProperty::AndSecond.to_string(), "node_and_second");
    assert_eq!(NodeProperty::IsNode.to_string(), "node_is_node");
    assert_eq!(
        NodeProperty::NewCompositeProperty.to_string(),
        "node_new_composite_property"
    );
    assert_eq!(
        NodeProperty::HasPropertySubject.to_string(),
        "node_has_property_subject"
    );
    assert_eq!(
        NodeProperty::HasPropertyProperty.to_string(),
        "node_has_property_property"
    );
    assert_eq!(
        PropertyRequirement::Depth.to_string(),
        "a non-negative 32-bit integer"
    );
    assert_eq!(
        InvalidNodeError::MissingProperty {
            kind: NodeKind::And,
            property: NodeProperty::AndSecond,
        }
        .to_string(),
        "object looks like a node of kind `and`, but required property `node_and_second` is missing"
    );
}

#[test]
fn node_parse_wraps_child_errors() {
    let knowledge = Knowledge { map: OrdMap::new() };
    let invalid_child = Object::Composite(Composite::empty());

    let object = Object::Composite(Composite::from([Construction::new(
        Atom::NODE_SET_COUNT.into(),
        invalid_child,
    )
    .into()]));

    assert_eq!(
        Node::new(&object, &knowledge),
        Err(InvalidNodeError::Child {
            property: NodeProperty::SetCount,
            source: Box::new(InvalidNodeError::NotANode),
        })
    );
}
