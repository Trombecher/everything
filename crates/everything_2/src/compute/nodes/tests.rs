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
        Node::Function {
            body: Node::Literal(a.clone()).into(),
        },
        Node::Literal(Node::Literal(a.clone()).into()),
        Node::And {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::FunctionSelf { depth: 0 },
        Node::Parameter { depth: 0 },
        Node::Count(Node::Literal(a.clone()).into()),
        Node::QueryAtomsWithProperty {
            property: Node::Literal(a.clone()).into(),
        },
        Node::QueryConstructionValues {
            set: Node::Literal(a.clone()).into(),
            property_constructor: Node::Literal(Atom::PROPERTY.into()).into(),
        },
        Node::Contains {
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
        Node::Union {
            first: Node::Literal(a.clone()).into(),
            second: Node::Literal(b.clone()).into(),
        },
        Node::Map {
            set: Node::Literal(a.clone()).into(),
            mapper_function: Node::Literal(b.clone()).into(),
        },
        Node::Filter {
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
        Node::Unwrap(Node::Literal(a.clone()).into()),
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
        Node::IsSet(Node::Literal(a.clone()).into()),
        Node::Every {
            set: Node::Literal(a.clone()).into(),
            predicate: Node::Literal(b.clone()).into(),
        },
        Node::Any {
            set: Node::Literal(a.clone()).into(),
            predicate: Node::Literal(b.clone()).into(),
        },
        Node::Fold {
            set: Node::Literal(a.clone()).into(),
            folder: Node::Literal(b.clone()).into(),
            initial: Node::Literal(c.clone()).into(),
        },
        Node::NewConstruction {
            constructor: Node::Literal(a.clone()).into(),
            value: Node::Literal(b.clone()).into(),
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

        assert_eq!(Node::new(&object, &knowledge), Ok(node));
    }
}
