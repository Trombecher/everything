use imbl::OrdMap;

use super::*;

#[test]
#[allow(clippy::too_many_lines)]
fn node_round_trip() {
    let knowledge = Knowledge { map: OrdMap::new() };

    let a = Object::new_integer(BigInt::from(1));
    let b = Object::new_integer(BigInt::from(2));
    let c = Object::new_integer(BigInt::from(3));

    let nodes = [
        Node::Function { body: a.clone() },
        Node::Literal(a.clone()),
        Node::And {
            first: a.clone(),
            second: b.clone(),
        },
        Node::FunctionSelf { depth: 0 },
        Node::Parameter { depth: 0 },
        Node::Count(a.clone()),
        Node::QueryAtomsWithProperty {
            property: a.clone(),
        },
        Node::QueryConstructionValues {
            set: a.clone(),
            property_constructor: Atom::PROPERTY.into(),
        },
        Node::Contains {
            element: a.clone(),
            set: b.clone(),
        },
        Node::Equal {
            first: a.clone(),
            second: b.clone(),
        },
        Node::Or {
            first: a.clone(),
            second: b.clone(),
        },
        Node::Xor {
            first: a.clone(),
            second: b.clone(),
        },
        Node::Not(a.clone()),
        Node::Add {
            first: a.clone(),
            second: b.clone(),
        },
        Node::Union {
            first: a.clone(),
            second: b.clone(),
        },
        Node::Map {
            set: a.clone(),
            mapper_function: b.clone(),
        },
        Node::Filter {
            set: a.clone(),
            filter_function: b.clone(),
        },
        Node::Less {
            first: a.clone(),
            second: b.clone(),
        },
        Node::If {
            condition: a.clone(),
            then: b.clone(),
            otherwise: c.clone(),
        },
        Node::UnwrapOr {
            set: a.clone(),
            default: b.clone(),
        },
        Node::Multiply {
            first: a.clone(),
            second: b.clone(),
        },
        Node::Call {
            callee: a.clone(),
            with: b.clone(),
        },
        Node::IsAtom(a.clone()),
        Node::IsConstruction(a.clone()),
        Node::IsSet(a.clone()),
        Node::Every {
            set: a.clone(),
            predicate: b.clone(),
        },
        Node::Any {
            set: a.clone(),
            predicate: b.clone(),
        },
        Node::Fold {
            set: a.clone(),
            folder: b.clone(),
            initial: c.clone(),
        },
        Node::NewConstruction {
            constructor: a.clone(),
            value: b.clone(),
        },
        Node::ConstructionConstructor {
            construction: a.clone(),
        },
        Node::ConstructionValue { construction: a },
    ];

    for node in nodes {
        let object = Object::new_node(node.clone());

        assert_eq!(Node::new(&object, &knowledge), Some(node));
    }
}
