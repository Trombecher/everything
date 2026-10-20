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
    Contains {
        element: Object,
        set: Object,
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
    IsConstruction(Object),
    IsSet(Object),
    Every {
        set: Object,
        predicate: Object,
    },
    Any {
        set: Object,
        predicate: Object,
    },
    Fold {
        set: Object,
        folder: Object,
        initial: Object,
    },
    NewConstruction {
        constructor: Object,
        value: Object,
    },
    ConstructionConstructor {
        construction: Object,
    },
    ConstructionValue {
        construction: Object,
    },
}

pub(crate) fn node_function_body(object: &Object, knowledge: &Knowledge) -> Option<Object> {
    knowledge
        .query_construction_values(object, Atom::NODE_FUNCTION.into())
        .next_and_last()
}

fn pair_values(
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

fn triple_values(
    object: &Object,
    knowledge: &Knowledge,
    first_property_constructor: Object,
    second_property_constructor: Object,
    third_property_constructor: Object,
) -> Option<(Object, Object, Object)> {
    let first = knowledge
        .query_construction_values(object, first_property_constructor)
        .next_and_last()?;

    let second = knowledge
        .query_construction_values(object, second_property_constructor)
        .next_and_last()?;

    let third = knowledge
        .query_construction_values(object, third_property_constructor)
        .next_and_last()?;

    Some((first, second, third))
}

fn node_function_self(object: &Object, knowledge: &Knowledge) -> Option<Node> {
    let depth_i128 = knowledge
        .query_construction_values(object, Atom::NODE_FUNCTION_SELF.into())
        .next_and_last()?
        .integer()?;

    u32::try_from(depth_i128)
        .ok()
        .map(|depth| Node::FunctionSelf { depth })
}

fn node_parameter_depth(object: &Object, knowledge: &Knowledge) -> Option<Node> {
    let depth = knowledge
        .query_construction_values(object, Atom::NODE_PARAMETER.into())
        .next_and_last()?
        .integer()?;

    u32::try_from(depth)
        .ok()
        .map(|depth| Node::Parameter { depth })
}

impl Node {
    #[allow(clippy::too_many_lines)]
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

        xor_with!(node_function_body(object, knowledge).map(|body| Self::Function { body }));

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_LITERAL.into())
                .next_and_last()
                .map(Self::Literal)
        );

        xor_with!(node_function_self(object, knowledge));

        xor_with!(node_parameter_depth(object, knowledge));

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_CALL_CALLEE.into(),
                Atom::NODE_CALL_WITH.into(),
            )
            .map(|(callee, with)| Self::Call { callee, with })
        );

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_COUNT.into())
                .next_and_last()
                .map(Self::Count)
        );

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_NOT.into())
                .next_and_last()
                .map(Self::Not)
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_AND_LEFT.into(),
                Atom::NODE_AND_RIGHT.into(),
            )
            .map(|(first, second)| Self::And { first, second })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_OR_LEFT.into(),
                Atom::NODE_OR_RIGHT.into(),
            )
            .map(|(first, second)| Self::Or { first, second })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_EQUAL_LEFT.into(),
                Atom::NODE_EQUAL_RIGHT.into(),
            )
            .map(|(first, second)| Self::Equal { first, second })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_XOR_LEFT.into(),
                Atom::NODE_XOR_RIGHT.into(),
            )
            .map(|(first, second)| Self::Xor { first, second })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_ADD_LEFT.into(),
                Atom::NODE_ADD_RIGHT.into(),
            )
            .map(|(first, second)| Self::Add { first, second })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_UNION_LEFT.into(),
                Atom::NODE_UNION_RIGHT.into(),
            )
            .map(|(first, second)| Self::Union { first, second })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_MAP_SET.into(),
                Atom::NODE_MAP_MAPPER.into(),
            )
            .map(|(set, mapper_function)| Self::Map {
                set,
                mapper_function,
            })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_FILTER_SET.into(),
                Atom::NODE_FILTER_FILTER.into(),
            )
            .map(|(set, filter_function)| Self::Filter {
                set,
                filter_function,
            })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_LESS_LEFT.into(),
                Atom::NODE_LESS_RIGHT.into(),
            )
            .map(|(first, second)| Self::Less { first, second })
        );

        xor_with!(
            triple_values(
                object,
                knowledge,
                Atom::NODE_IF_CONDITION.into(),
                Atom::NODE_IF_THEN.into(),
                Atom::NODE_IF_ELSE.into(),
            )
            .map(|(condition, then, otherwise)| Self::If {
                condition,
                then,
                otherwise,
            })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_UNWRAP_OR_SET.into(),
                Atom::NODE_UNWRAP_OR_DEFAULT.into(),
            )
            .map(|(set, default)| Self::UnwrapOr { set, default })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_MULTIPLY_LEFT.into(),
                Atom::NODE_MULTIPLY_RIGHT.into(),
            )
            .map(|(first, second)| Self::Multiply { first, second })
        );

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_IS_ATOM.into())
                .next_and_last()
                .map(Self::IsAtom)
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_EVERY_SET.into(),
                Atom::NODE_EVERY_PREDICATE.into(),
            )
            .map(|(set, predicate)| Self::Every { set, predicate })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_ANY_SET.into(),
                Atom::NODE_ANY_PREDICATE.into(),
            )
            .map(|(set, predicate)| Self::Any { set, predicate })
        );

        // TODO: more recognition

        node
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn into_object(self) -> Object {
        match self {
            Self::IsConstruction(inner) => {
                Set::new_move([Construction::new(Atom::NODE_IS_CONSTRUCTION.into(), inner).into()])
                    .into()
            }
            Self::IsSet(inner) => {
                Set::new_move([Construction::new(Atom::NODE_IS_SET.into(), inner).into()]).into()
            }
            Self::ConstructionConstructor { construction } => Set::new_move([Construction::new(
                Atom::NODE_CONSTRUCTION_CONSTRUCTOR.into(),
                construction,
            )
            .into()])
            .into(),
            Self::Fold {
                set,
                folder,
                initial,
            } => Set::new_move([
                Construction::new(Atom::NODE_FOLD_SET.into(), set).into(),
                Construction::new(Atom::NODE_FOLD_FOLDER.into(), folder).into(),
                Construction::new(Atom::NODE_FOLD_INITIAL.into(), initial).into(),
            ])
            .into(),
            Self::NewConstruction { constructor, value } => Set::new_move([
                Construction::new(Atom::NODE_NEW_CONSTRUCTION_CONSTRUCTOR.into(), constructor)
                    .into(),
                Construction::new(Atom::NODE_NEW_CONSTRUCTION_VALUE.into(), value).into(),
            ])
            .into(),
            Self::Contains { element, set } => Set::new_move([
                Construction::new(Atom::NODE_CONTAINS_ELEMENT.into(), element).into(),
                Construction::new(Atom::NODE_CONTAINS_SET.into(), set).into(),
            ])
            .into(),
            Self::ConstructionValue { construction } => Set::new_move([Construction::new(
                Atom::NODE_CONSTRUCTION_VALUE.into(),
                construction,
            )
            .into()])
            .into(),
            Self::IsAtom(inner) => {
                Set::new_move([Construction::new(Atom::NODE_IS_ATOM.into(), inner).into()]).into()
            }
            Self::Call { callee, with } => Set::new_move([
                Construction::new(Atom::NODE_CALL_CALLEE.into(), callee).into(),
                Construction::new(Atom::NODE_CALL_WITH.into(), with).into(),
            ])
            .into(),
            Self::QueryConstructionValues {
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
            Self::Function { body } => {
                Set::new_move([Construction::new(Atom::NODE_FUNCTION.into(), body).into()]).into()
            }
            Self::Literal(literal) => {
                Set::new_move([Construction::new(Atom::NODE_LITERAL.into(), literal).into()]).into()
            }
            Self::And { first, second } => Set::new_move([
                Construction::new(Atom::NODE_AND_LEFT.into(), first).into(),
                Construction::new(Atom::NODE_AND_RIGHT.into(), second).into(),
            ])
            .into(),
            Self::FunctionSelf { depth } => Set::new_move([Construction::new(
                Atom::NODE_FUNCTION_SELF.into(),
                Object::new_integer(i128::from(depth)),
            )
            .into()])
            .into(),
            Self::Parameter { depth } => Set::new_move([Construction::new(
                Atom::NODE_PARAMETER.into(),
                Object::new_integer(i128::from(depth)),
            )
            .into()])
            .into(),
            Self::Count(object) => {
                Set::new_move([Construction::new(Atom::NODE_COUNT.into(), object).into()]).into()
            }
            Self::QueryAtomsWithProperty { property } => {
                Set::new_move([
                    Construction::new(Atom::NODE_ATOMS_WITH_PROPERTY.into(), property).into(),
                ])
                .into()
            }
            Self::Equal { first, second } => Set::new_move([
                Construction::new(Atom::NODE_EQUAL_LEFT.into(), first).into(),
                Construction::new(Atom::NODE_EQUAL_RIGHT.into(), second).into(),
            ])
            .into(),
            Self::Or { first, second } => Set::new_move([
                Construction::new(Atom::NODE_OR_LEFT.into(), first).into(),
                Construction::new(Atom::NODE_OR_RIGHT.into(), second).into(),
            ])
            .into(),
            Self::Xor { first, second } => Set::new_move([
                Construction::new(Atom::NODE_XOR_LEFT.into(), first).into(),
                Construction::new(Atom::NODE_XOR_RIGHT.into(), second).into(),
            ])
            .into(),
            Self::Not(node) => {
                Set::new_move([Construction::new(Atom::NODE_NOT.into(), node).into()]).into()
            }
            Self::Add { first, second } => Set::new_move([
                Construction::new(Atom::NODE_ADD_LEFT.into(), first).into(),
                Construction::new(Atom::NODE_ADD_RIGHT.into(), second).into(),
            ])
            .into(),
            Self::Union { first, second } => Set::new_move([
                Construction::new(Atom::NODE_UNION_LEFT.into(), first).into(),
                Construction::new(Atom::NODE_UNION_RIGHT.into(), second).into(),
            ])
            .into(),
            Self::Map {
                set,
                mapper_function,
            } => Set::new_move([
                Construction::new(Atom::NODE_MAP_SET.into(), set).into(),
                Construction::new(Atom::NODE_MAP_MAPPER.into(), mapper_function).into(),
            ])
            .into(),
            Self::Filter {
                set,
                filter_function,
            } => Set::new_move([
                Construction::new(Atom::NODE_FILTER_SET.into(), set).into(),
                Construction::new(Atom::NODE_FILTER_FILTER.into(), filter_function).into(),
            ])
            .into(),
            Self::Every { set, predicate } => Set::new_move([
                Construction::new(Atom::NODE_EVERY_SET.into(), set).into(),
                Construction::new(Atom::NODE_EVERY_PREDICATE.into(), predicate).into(),
            ])
            .into(),
            Self::Any { set, predicate } => Set::new_move([
                Construction::new(Atom::NODE_ANY_SET.into(), set).into(),
                Construction::new(Atom::NODE_ANY_PREDICATE.into(), predicate).into(),
            ])
            .into(),
            Self::Less { first, second } => Set::new_move([
                Construction::new(Atom::NODE_LESS_LEFT.into(), first).into(),
                Construction::new(Atom::NODE_LESS_RIGHT.into(), second).into(),
            ])
            .into(),
            Self::If {
                condition,
                otherwise,
                then,
            } => Set::new_move([
                Construction::new(Atom::NODE_IF_CONDITION.into(), condition).into(),
                Construction::new(Atom::NODE_IF_THEN.into(), then).into(),
                Construction::new(Atom::NODE_IF_ELSE.into(), otherwise).into(),
            ])
            .into(),
            Self::UnwrapOr { set, default } => Set::new_move([
                Construction::new(Atom::NODE_UNWRAP_OR_SET.into(), set).into(),
                Construction::new(Atom::NODE_UNWRAP_OR_DEFAULT.into(), default).into(),
            ])
            .into(),
            Self::Multiply { first, second } => Set::new_move([
                Construction::new(Atom::NODE_MULTIPLY_LEFT.into(), first).into(),
                Construction::new(Atom::NODE_MULTIPLY_RIGHT.into(), second).into(),
            ])
            .into(),
        }
    }
}
