use crate::{
    iter::IteratorExtNextAndLast,
    knowledge::Knowledge,
    model::{Atom, Construction, Object, Set},
};

#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Function {
        body: Object,
    },
    Literal(Object),
    And {
        first: Object,
        second: Object,
    },
    FunctionSelf {
        depth: u32,
    },
    Parameter {
        depth: u32,
    },
    Count(Object),
    QueryAtomsWithProperty {
        property: Object,
    },
    QueryConstructionValues {
        set: Object,
        property_constructor: Object,
    },
    Equal {
        first: Object,
        second: Object,
    },
    Or {
        first: Object,
        second: Object,
    },
    Xor {
        first: Object,
        second: Object,
    },
    Not(Object),
    Add {
        first: Object,
        second: Object,
    },
    Union {
        first: Object,
        second: Object,
    },
    Map {
        set: Object,
        mapper_function: Object,
    },
    Filter {
        set: Object,
        filter_function: Object,
    },
    Less {
        first: Object,
        second: Object,
    },
    If {
        condition: Object,
        then: Object,
        otherwise: Object,
    },
    UnwrapOr {
        set: Object,
        default: Object,
    },
    Multiply {
        first: Object,
        second: Object,
    },
    Call {
        callee: Object,
        with: Object,
    },
    IsAtom(Object),
    Every {
        set: Object,
        predicate: Object,
    },
    Any {
        set: Object,
        predicate: Object,
    },
    // Please also add new nodes to the array of nodes
    // in the tests (function `node_parsing()`).
}

impl Node {
    pub fn new(object: &Object, knowledge: &Knowledge) -> Option<Self> {
        let mut node: Option<Node> = None;

        macro_rules! xor_with {
            ($e:expr) => {{
                let variant = $e;

                if variant.is_some() {
                    if node.is_some() {
                        return None;
                    }

                    node = variant;
                }
            }};
        }

        fn both_values(
            object: &Object,
            knowledge: &Knowledge,
            first_property_constructor: Object,
            second_property_constructor: Object,
        ) -> Option<(Object, Object)> {
            let first = knowledge
                .query_construction_values(object, first_property_constructor)
                .next_and_last()?;

            let second = knowledge
                .query_construction_values(object, second_property_constructor)
                .next_and_last()?;

            Some((first, second))
        }

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_FUNCTION.into())
                .next_and_last()
                .map(|body| Node::Function { body })
        );

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_LITERAL.into())
                .next_and_last()
                .map(Node::Literal)
        );

        fn node_function_self(object: &Object, knowledge: &Knowledge) -> Option<Node> {
            let depth_i128 = knowledge
                .query_construction_values(object, Atom::NODE_FUNCTION_SELF.into())
                .next_and_last()?
                .integer()?;

            u32::try_from(depth_i128)
                .ok()
                .map(|depth| Node::FunctionSelf { depth })
        }
        xor_with!(node_function_self(object, knowledge));

        fn node_parameter_depth(object: &Object, knowledge: &Knowledge) -> Option<Node> {
            let depth = knowledge
                .query_construction_values(object, Atom::NODE_PARAMETER.into())
                .next_and_last()?
                .integer()?;

            u32::try_from(depth)
                .ok()
                .map(|depth| Node::Parameter { depth })
        }
        xor_with!(node_parameter_depth(object, knowledge));

        fn node_call(object: &Object, knowledge: &Knowledge) -> Option<Node> {
            let callee = knowledge
                .query_construction_values(object, Atom::NODE_CALL_CALLEE.into())
                .next_and_last()?;

            let with = knowledge
                .query_construction_values(object, Atom::NODE_CALL_WITH.into())
                .next_and_last()?;

            Some(Node::Call { callee, with })
        }
        xor_with!(node_call(object, knowledge));

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_COUNT.into())
                .next_and_last()
                .map(Node::Count)
        );

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_NOT.into())
                .next_and_last()
                .map(Node::Not)
        );

        xor_with!(
            both_values(
                object,
                knowledge,
                Atom::NODE_AND_LEFT.into(),
                Atom::NODE_AND_RIGHT.into(),
            )
            .map(|(first, second)| Node::And { first, second })
        );

        xor_with!(
            both_values(
                object,
                knowledge,
                Atom::NODE_OR_LEFT.into(),
                Atom::NODE_OR_RIGHT.into(),
            )
            .map(|(first, second)| Node::Or { first, second })
        );

        xor_with!(
            both_values(
                object,
                knowledge,
                Atom::NODE_EQUAL_LEFT.into(),
                Atom::NODE_EQUAL_RIGHT.into(),
            )
            .map(|(first, second)| Node::Equal { first, second })
        );

        xor_with!(
            both_values(
                object,
                knowledge,
                Atom::NODE_XOR_LEFT.into(),
                Atom::NODE_XOR_RIGHT.into(),
            )
            .map(|(first, second)| Node::Xor { first, second })
        );

        xor_with!(
            both_values(
                object,
                knowledge,
                Atom::NODE_ADD_LEFT.into(),
                Atom::NODE_ADD_RIGHT.into(),
            )
            .map(|(first, second)| Node::Add { first, second })
        );

        xor_with!(
            both_values(
                object,
                knowledge,
                Atom::NODE_UNION_LEFT.into(),
                Atom::NODE_UNION_RIGHT.into(),
            )
            .map(|(first, second)| Node::Union { first, second })
        );

        fn node_map(object: &Object, knowledge: &Knowledge) -> Option<Node> {
            let set_expression = knowledge
                .query_construction_values(object, Atom::NODE_MAP_SET.into())
                .next()?;
            let mapper_function_expression = knowledge
                .query_construction_values(object, Atom::NODE_MAP_MAPPER.into())
                .next()?;

            Some(Node::Map {
                set: set_expression,
                mapper_function: mapper_function_expression,
            })
        }
        xor_with!(node_map(object, knowledge));

        fn node_filter(object: &Object, knowledge: &Knowledge) -> Option<Node> {
            let set = knowledge
                .query_construction_values(object, Atom::NODE_FILTER_SET.into())
                .next_and_last()?;

            let filter = knowledge
                .query_construction_values(object, Atom::NODE_FILTER_FILTER.into())
                .next_and_last()?;

            Some(Node::Filter {
                set,
                filter_function: filter,
            })
        }
        xor_with!(node_filter(object, knowledge));

        xor_with!(
            both_values(
                object,
                knowledge,
                Atom::NODE_LESS_LEFT.into(),
                Atom::NODE_LESS_RIGHT.into(),
            )
            .map(|(first, second)| Node::Less { first, second })
        );

        fn node_if(object: &Object, knowledge: &Knowledge) -> Option<Node> {
            let condition = knowledge
                .query_construction_values(object, Atom::NODE_IF_CONDITION.into())
                .next_and_last()?;

            let then = knowledge
                .query_construction_values(object, Atom::NODE_IF_THEN.into())
                .next_and_last()?;

            let otherwise = knowledge
                .query_construction_values(object, Atom::NODE_IF_ELSE.into())
                .next_and_last()?;

            Some(Node::If {
                condition,
                then,
                otherwise,
            })
        }
        xor_with!(node_if(object, knowledge));

        fn node_unwrap_or(object: &Object, knowledge: &Knowledge) -> Option<Node> {
            let set = knowledge
                .query_construction_values(object, Atom::NODE_UNWRAP_OR_SET.into())
                .next_and_last()?;

            let default = knowledge
                .query_construction_values(object, Atom::NODE_UNWRAP_OR_DEFAULT.into())
                .next_and_last()?;

            Some(Node::UnwrapOr { set, default })
        }
        xor_with!(node_unwrap_or(object, knowledge));

        xor_with!(
            both_values(
                object,
                knowledge,
                Atom::NODE_MULTIPLY_LEFT.into(),
                Atom::NODE_MULTIPLY_RIGHT.into(),
            )
            .map(|(first, second)| Node::Multiply { first, second })
        );

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_IS_ATOM.into())
                .next_and_last()
                .map(Node::IsAtom)
        );

        fn node_every(object: &Object, knowledge: &Knowledge) -> Option<Node> {
            let set = knowledge
                .query_construction_values(object, Atom::NODE_EVERY_SET.into())
                .next_and_last()?;

            let predicate = knowledge
                .query_construction_values(object, Atom::NODE_EVERY_PREDICATE.into())
                .next_and_last()?;

            Some(Node::Every { set, predicate })
        }
        xor_with!(node_every(object, knowledge));

        fn node_any(object: &Object, knowledge: &Knowledge) -> Option<Node> {
            let set = knowledge
                .query_construction_values(object, Atom::NODE_ANY_SET.into())
                .next_and_last()?;

            let predicate = knowledge
                .query_construction_values(object, Atom::NODE_ANY_PREDICATE.into())
                .next_and_last()?;

            Some(Node::Any { set, predicate })
        }
        xor_with!(node_any(object, knowledge));

        // TODO: more recognition

        node
    }
}

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn object_from_node(node: Node) -> Object {
    match node {
        Node::IsAtom(inner) => {
            Set::new_move([Construction::new(Atom::NODE_IS_ATOM.into(), inner).into()]).into()
        }
        Node::Call { callee, with } => Set::new_move([
            Construction::new(Atom::NODE_CALL_CALLEE.into(), callee).into(),
            Construction::new(Atom::NODE_CALL_WITH.into(), with).into(),
        ])
        .into(),
        Node::QueryConstructionValues {
            set,
            property_constructor,
        } => Set::new_move([
            Construction::new(Atom::NODE_CONSTRUCTION_VALUES_SET.into(), set).into(),
            Construction::new(
                Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR.into(),
                property_constructor,
            )
            .into(),
        ])
        .into(),
        Node::Function { body } => {
            Set::new_move([Construction::new(Atom::NODE_FUNCTION.into(), body).into()]).into()
        }
        Node::Literal(literal) => {
            Set::new_move([Construction::new(Atom::NODE_LITERAL.into(), literal).into()]).into()
        }
        Node::And { first, second } => Set::new_move([
            Construction::new(Atom::NODE_AND_LEFT.into(), first).into(),
            Construction::new(Atom::NODE_AND_RIGHT.into(), second).into(),
        ])
        .into(),
        Node::FunctionSelf { depth } => Set::new_move([Construction::new(
            Atom::NODE_FUNCTION_SELF.into(),
            Object::new_integer(i128::from(depth)),
        )
        .into()])
        .into(),
        Node::Parameter { depth } => Set::new_move([Construction::new(
            Atom::NODE_PARAMETER.into(),
            Object::new_integer(i128::from(depth)),
        )
        .into()])
        .into(),
        Node::Count(object) => {
            Set::new_move([Construction::new(Atom::NODE_COUNT.into(), object).into()]).into()
        }
        Node::QueryAtomsWithProperty { property } => {
            Set::new_move([
                Construction::new(Atom::NODE_ATOMS_WITH_PROPERTY.into(), property).into(),
            ])
            .into()
        }
        Node::Equal { first, second } => Set::new_move([
            Construction::new(Atom::NODE_EQUAL_LEFT.into(), first).into(),
            Construction::new(Atom::NODE_EQUAL_RIGHT.into(), second).into(),
        ])
        .into(),
        Node::Or { first, second } => Set::new_move([
            Construction::new(Atom::NODE_OR_LEFT.into(), first).into(),
            Construction::new(Atom::NODE_OR_RIGHT.into(), second).into(),
        ])
        .into(),
        Node::Xor { first, second } => Set::new_move([
            Construction::new(Atom::NODE_XOR_LEFT.into(), first).into(),
            Construction::new(Atom::NODE_XOR_RIGHT.into(), second).into(),
        ])
        .into(),
        Node::Not(node) => {
            Set::new_move([Construction::new(Atom::NODE_NOT.into(), node).into()]).into()
        }
        Node::Add { first, second } => Set::new_move([
            Construction::new(Atom::NODE_ADD_LEFT.into(), first).into(),
            Construction::new(Atom::NODE_ADD_RIGHT.into(), second).into(),
        ])
        .into(),
        Node::Union { first, second } => Set::new_move([
            Construction::new(Atom::NODE_UNION_LEFT.into(), first).into(),
            Construction::new(Atom::NODE_UNION_RIGHT.into(), second).into(),
        ])
        .into(),
        Node::Map {
            set,
            mapper_function,
        } => Set::new_move([
            Construction::new(Atom::NODE_MAP_SET.into(), set).into(),
            Construction::new(Atom::NODE_MAP_MAPPER.into(), mapper_function).into(),
        ])
        .into(),
        Node::Filter {
            set,
            filter_function,
        } => Set::new_move([
            Construction::new(Atom::NODE_FILTER_SET.into(), set).into(),
            Construction::new(Atom::NODE_FILTER_FILTER.into(), filter_function).into(),
        ])
        .into(),
        Node::Every { set, predicate } => Set::new_move([
            Construction::new(Atom::NODE_EVERY_SET.into(), set).into(),
            Construction::new(Atom::NODE_EVERY_PREDICATE.into(), predicate).into(),
        ])
        .into(),
        Node::Any { set, predicate } => Set::new_move([
            Construction::new(Atom::NODE_ANY_SET.into(), set).into(),
            Construction::new(Atom::NODE_ANY_PREDICATE.into(), predicate).into(),
        ])
        .into(),
        Node::Less { first, second } => Set::new_move([
            Construction::new(Atom::NODE_LESS_LEFT.into(), first).into(),
            Construction::new(Atom::NODE_LESS_RIGHT.into(), second).into(),
        ])
        .into(),
        Node::If {
            condition,
            otherwise,
            then,
        } => Set::new_move([
            Construction::new(Atom::NODE_IF_CONDITION.into(), condition).into(),
            Construction::new(Atom::NODE_IF_THEN.into(), then).into(),
            Construction::new(Atom::NODE_IF_ELSE.into(), otherwise).into(),
        ])
        .into(),
        Node::UnwrapOr { set, default } => Set::new_move([
            Construction::new(Atom::NODE_UNWRAP_OR_SET.into(), set).into(),
            Construction::new(Atom::NODE_UNWRAP_OR_DEFAULT.into(), default).into(),
        ])
        .into(),
        Node::Multiply { first, second } => Set::new_move([
            Construction::new(Atom::NODE_MULTIPLY_LEFT.into(), first).into(),
            Construction::new(Atom::NODE_MULTIPLY_RIGHT.into(), second).into(),
        ])
        .into(),
    }
}
