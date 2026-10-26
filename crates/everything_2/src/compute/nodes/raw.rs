use crate::{
    iter::IteratorExtNextAndLast,
    knowledge::Knowledge,
    model::{Atom, Object},
};

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

fn node_function_self(object: &Object, knowledge: &Knowledge) -> Option<RawNode> {
    let depth_i128 = knowledge
        .query_construction_values(object, Atom::NODE_FUNCTION_SELF.into())
        .next_and_last()?
        .integer()?;

    u32::try_from(depth_i128)
        .ok()
        .map(|depth| RawNode::FunctionSelf { depth })
}

fn node_parameter_depth(object: &Object, knowledge: &Knowledge) -> Option<RawNode> {
    let depth = knowledge
        .query_construction_values(object, Atom::NODE_PARAMETER.into())
        .next_and_last()?
        .integer()?;

    u32::try_from(depth)
        .ok()
        .map(|depth| RawNode::Parameter { depth })
}

pub enum RawNode {
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

impl RawNode {
    /// Tries to parse a raw node from an object.
    #[allow(clippy::too_many_lines)]
    pub fn new(object: &Object, knowledge: &Knowledge) -> Option<Self> {
        // TODO: maybe change return type to a result (?)

        let mut node: Option<Self> = None;

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
                Atom::NODE_AND_FIRST.into(),
                Atom::NODE_AND_SECOND.into(),
            )
            .map(|(first, second)| Self::And { first, second })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_OR_FIRST.into(),
                Atom::NODE_OR_SECOND.into(),
            )
            .map(|(first, second)| Self::Or { first, second })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_EQUAL_FIRST.into(),
                Atom::NODE_EQUAL_SECOND.into(),
            )
            .map(|(first, second)| Self::Equal { first, second })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_XOR_FIRST.into(),
                Atom::NODE_XOR_SECOND.into(),
            )
            .map(|(first, second)| Self::Xor { first, second })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_ADD_FIRST.into(),
                Atom::NODE_ADD_SECOND.into(),
            )
            .map(|(first, second)| Self::Add { first, second })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_UNION_FIRST.into(),
                Atom::NODE_UNION_SECOND.into(),
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
                Atom::NODE_LESS_FIRST.into(),
                Atom::NODE_LESS_SECOND.into(),
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
                Atom::NODE_MULTIPLY_FIRST.into(),
                Atom::NODE_MULTIPLY_SECOND.into(),
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

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_ATOMS_WITH_PROPERTY.into())
                .next_and_last()
                .map(|property| Self::QueryAtomsWithProperty { property })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_CONSTRUCTION_VALUES_SET.into(),
                Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR.into(),
            )
            .map(
                |(set, property_constructor)| Self::QueryConstructionValues {
                    set,
                    property_constructor,
                }
            )
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_CONTAINS_ELEMENT.into(),
                Atom::NODE_CONTAINS_SET.into(),
            )
            .map(|(element, set)| Self::Contains { element, set })
        );

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_IS_CONSTRUCTION.into())
                .next_and_last()
                .map(Self::IsConstruction)
        );

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_IS_SET.into())
                .next_and_last()
                .map(Self::IsSet)
        );

        xor_with!(
            triple_values(
                object,
                knowledge,
                Atom::NODE_FOLD_SET.into(),
                Atom::NODE_FOLD_FOLDER.into(),
                Atom::NODE_FOLD_INITIAL.into(),
            )
            .map(|(set, folder, initial)| Self::Fold {
                set,
                folder,
                initial,
            })
        );

        xor_with!(
            pair_values(
                object,
                knowledge,
                Atom::NODE_NEW_CONSTRUCTION_CONSTRUCTOR.into(),
                Atom::NODE_NEW_CONSTRUCTION_VALUE.into(),
            )
            .map(|(constructor, value)| Self::NewConstruction { constructor, value })
        );

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_CONSTRUCTOR_OF_CONSTRUCTION.into())
                .next_and_last()
                .map(|construction| Self::ConstructionConstructor { construction })
        );

        xor_with!(
            knowledge
                .query_construction_values(object, Atom::NODE_VALUE_OF_CONSTRUCTION.into())
                .next_and_last()
                .map(|construction| Self::ConstructionValue { construction })
        );

        node
    }
}
