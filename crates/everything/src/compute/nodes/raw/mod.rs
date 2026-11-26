//! This module handles node kind detection and first-level parsing.

mod diagnose;

use super::{InvalidNodeError, NodeKind};
use crate::{
    iter::IteratorExtNextAndLast,
    knowledge::Knowledge,
    model::{Atom, Object},
};
use diagnose::diagnose;

/// Returns the value of the single construction of `object` with the given
/// `property_constructor`, if there is exactly one.
fn construction_value(
    object: &Object,
    knowledge: &Knowledge,
    property_constructor: Object,
) -> Option<Object> {
    knowledge
        .query_construction_values(object, property_constructor)?
        .next_and_last()
}

pub(crate) fn node_function_body(object: &Object, knowledge: &Knowledge) -> Option<Object> {
    construction_value(object, knowledge, Atom::NODE_FUNCTION.into())
}

fn pair_values(
    object: &Object,
    knowledge: &Knowledge,
    first_property_constructor: Object,
    second_property_constructor: Object,
) -> Option<(Object, Object)> {
    let first = construction_value(object, knowledge, first_property_constructor)?;

    let second = construction_value(object, knowledge, second_property_constructor)?;

    Some((first, second))
}

fn triple_values(
    object: &Object,
    knowledge: &Knowledge,
    first_property_constructor: Object,
    second_property_constructor: Object,
    third_property_constructor: Object,
) -> Option<(Object, Object, Object)> {
    let first = construction_value(object, knowledge, first_property_constructor)?;

    let second = construction_value(object, knowledge, second_property_constructor)?;

    let third = construction_value(object, knowledge, third_property_constructor)?;

    Some((first, second, third))
}

/// Collects all values of a property that may occur more than once. Returns
/// `None` if the property has no values.
fn many_values(
    object: &Object,
    knowledge: &Knowledge,
    property_constructor: Object,
) -> Option<Vec<Object>> {
    let values = knowledge
        .query_construction_values(object, property_constructor)?
        .collect::<Vec<_>>();

    (!values.is_empty()).then_some(values)
}

fn node_function_self(object: &Object, knowledge: &Knowledge) -> Option<RawNode> {
    let depth_i128 =
        construction_value(object, knowledge, Atom::NODE_FUNCTION_SELF.into())?.integer()?;

    u32::try_from(depth_i128)
        .ok()
        .map(|depth| RawNode::FunctionSelf { depth })
}

fn node_parameter_depth(object: &Object, knowledge: &Knowledge) -> Option<RawNode> {
    let depth = construction_value(object, knowledge, Atom::NODE_PARAMETER.into())?.integer()?;

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
    SetCount(Object),
    PropertiesOf(Object),
    HasValue(Object),
    HasProperty {
        subject: Object,
        property: Object,
    },
    QueryAtomsWithProperty {
        property: Object,
    },
    QueryAtomsWithConstructor {
        constructor: Object,
    },
    QueryConstructionValues {
        object: Object,
        property_constructor: Object,
    },
    QueryConstructions {
        constructor: Object,
    },
    SetContains {
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
    SetUnion {
        first: Object,
        second: Object,
    },
    SetIntersection {
        first: Object,
        second: Object,
    },
    SetDifference {
        first: Object,
        second: Object,
    },
    SetMap {
        set: Object,
        mapper_function: Object,
    },
    SetFilter {
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
    SetUnwrap(Object),
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
    IsComposite(Object),
    SetEvery {
        set: Object,
        predicate: Object,
    },
    SetAny {
        set: Object,
        predicate: Object,
    },
    SetFold {
        set: Object,
        folder: Object,
        initial: Object,
    },
    NewConstruction {
        constructor: Object,
        value: Object,
    },
    NewSet {
        items: Vec<Object>,
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
    pub fn new(object: &Object, knowledge: &Knowledge) -> Result<Self, InvalidNodeError> {
        let mut node: Option<Self> = None;
        let mut kind: Option<NodeKind> = None;

        macro_rules! xor_with {
            ($kind:expr, $e:expr) => {{
                let variant = $e;

                if variant.is_some() {
                    if let Some(first) = kind {
                        return Err(InvalidNodeError::Ambiguous {
                            first,
                            second: $kind,
                        });
                    }

                    kind = Some($kind);
                    node = variant;
                }
            }};
        }

        xor_with!(
            NodeKind::Function,
            node_function_body(object, knowledge).map(|body| Self::Function { body })
        );

        xor_with!(
            NodeKind::Literal,
            construction_value(object, knowledge, Atom::NODE_LITERAL.into()).map(Self::Literal)
        );

        xor_with!(
            NodeKind::FunctionSelfReference,
            node_function_self(object, knowledge)
        );

        xor_with!(NodeKind::Parameter, node_parameter_depth(object, knowledge));

        xor_with!(
            NodeKind::Call,
            pair_values(
                object,
                knowledge,
                Atom::NODE_CALL_CALLEE.into(),
                Atom::NODE_CALL_WITH.into(),
            )
            .map(|(callee, with)| Self::Call { callee, with })
        );

        xor_with!(
            NodeKind::SetCount,
            construction_value(object, knowledge, Atom::NODE_SET_COUNT.into()).map(Self::SetCount)
        );

        xor_with!(
            NodeKind::PropertiesOf,
            construction_value(object, knowledge, Atom::NODE_PROPERTIES_OF.into())
                .map(Self::PropertiesOf)
        );

        xor_with!(
            NodeKind::HasValue,
            construction_value(object, knowledge, Atom::NODE_HAS_VALUE.into()).map(Self::HasValue)
        );

        xor_with!(
            NodeKind::HasProperty,
            pair_values(
                object,
                knowledge,
                Atom::NODE_HAS_PROPERTY_SUBJECT.into(),
                Atom::NODE_HAS_PROPERTY_PROPERTY.into(),
            )
            .map(|(subject, property)| Self::HasProperty { subject, property })
        );

        xor_with!(
            NodeKind::Not,
            construction_value(object, knowledge, Atom::NODE_NOT.into()).map(Self::Not)
        );

        xor_with!(
            NodeKind::And,
            pair_values(
                object,
                knowledge,
                Atom::NODE_AND_FIRST.into(),
                Atom::NODE_AND_SECOND.into(),
            )
            .map(|(first, second)| Self::And { first, second })
        );

        xor_with!(
            NodeKind::Or,
            pair_values(
                object,
                knowledge,
                Atom::NODE_OR_FIRST.into(),
                Atom::NODE_OR_SECOND.into(),
            )
            .map(|(first, second)| Self::Or { first, second })
        );

        xor_with!(
            NodeKind::Equal,
            pair_values(
                object,
                knowledge,
                Atom::NODE_EQUAL_FIRST.into(),
                Atom::NODE_EQUAL_SECOND.into(),
            )
            .map(|(first, second)| Self::Equal { first, second })
        );

        xor_with!(
            NodeKind::Xor,
            pair_values(
                object,
                knowledge,
                Atom::NODE_XOR_FIRST.into(),
                Atom::NODE_XOR_SECOND.into(),
            )
            .map(|(first, second)| Self::Xor { first, second })
        );

        xor_with!(
            NodeKind::Add,
            pair_values(
                object,
                knowledge,
                Atom::NODE_ADD_FIRST.into(),
                Atom::NODE_ADD_SECOND.into(),
            )
            .map(|(first, second)| Self::Add { first, second })
        );

        xor_with!(
            NodeKind::SetUnion,
            pair_values(
                object,
                knowledge,
                Atom::NODE_SET_UNION_FIRST.into(),
                Atom::NODE_SET_UNION_SECOND.into(),
            )
            .map(|(first, second)| Self::SetUnion { first, second })
        );

        xor_with!(
            NodeKind::SetIntersection,
            pair_values(
                object,
                knowledge,
                Atom::NODE_SET_INTERSECTION_FIRST.into(),
                Atom::NODE_SET_INTERSECTION_SECOND.into(),
            )
            .map(|(first, second)| Self::SetIntersection { first, second })
        );

        xor_with!(
            NodeKind::SetDifference,
            pair_values(
                object,
                knowledge,
                Atom::NODE_SET_DIFFERENCE_FIRST.into(),
                Atom::NODE_SET_DIFFERENCE_SECOND.into(),
            )
            .map(|(first, second)| Self::SetDifference { first, second })
        );

        xor_with!(
            NodeKind::SetMap,
            pair_values(
                object,
                knowledge,
                Atom::NODE_SET_MAP_SET.into(),
                Atom::NODE_SET_MAP_MAPPER.into(),
            )
            .map(|(set, mapper_function)| Self::SetMap {
                set,
                mapper_function,
            })
        );

        xor_with!(
            NodeKind::SetFilter,
            pair_values(
                object,
                knowledge,
                Atom::NODE_SET_FILTER_SET.into(),
                Atom::NODE_SET_FILTER_FILTER.into(),
            )
            .map(|(set, filter_function)| Self::SetFilter {
                set,
                filter_function,
            })
        );

        xor_with!(
            NodeKind::Less,
            pair_values(
                object,
                knowledge,
                Atom::NODE_LESS_FIRST.into(),
                Atom::NODE_LESS_SECOND.into(),
            )
            .map(|(first, second)| Self::Less { first, second })
        );

        xor_with!(
            NodeKind::If,
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
            NodeKind::SetUnwrap,
            construction_value(object, knowledge, Atom::NODE_SET_UNWRAP_SET.into())
                .map(Self::SetUnwrap)
        );

        xor_with!(
            NodeKind::Multiply,
            pair_values(
                object,
                knowledge,
                Atom::NODE_MULTIPLY_FIRST.into(),
                Atom::NODE_MULTIPLY_SECOND.into(),
            )
            .map(|(first, second)| Self::Multiply { first, second })
        );

        xor_with!(
            NodeKind::IsAtom,
            construction_value(object, knowledge, Atom::NODE_IS_ATOM.into()).map(Self::IsAtom)
        );

        xor_with!(
            NodeKind::SetEvery,
            pair_values(
                object,
                knowledge,
                Atom::NODE_SET_EVERY_SET.into(),
                Atom::NODE_SET_EVERY_PREDICATE.into(),
            )
            .map(|(set, predicate)| Self::SetEvery { set, predicate })
        );

        xor_with!(
            NodeKind::SetAny,
            pair_values(
                object,
                knowledge,
                Atom::NODE_SET_ANY_SET.into(),
                Atom::NODE_SET_ANY_PREDICATE.into(),
            )
            .map(|(set, predicate)| Self::SetAny { set, predicate })
        );

        xor_with!(
            NodeKind::QueryAtomsWithProperty,
            construction_value(object, knowledge, Atom::NODE_ATOMS_WITH_PROPERTY.into())
                .map(|property| Self::QueryAtomsWithProperty { property })
        );

        xor_with!(
            NodeKind::QueryAtomsWithConstructor,
            construction_value(object, knowledge, Atom::NODE_ATOMS_WITH_CONSTRUCTOR.into())
                .map(|constructor| Self::QueryAtomsWithConstructor { constructor })
        );

        xor_with!(
            NodeKind::QueryConstructionValues,
            pair_values(
                object,
                knowledge,
                Atom::NODE_CONSTRUCTION_VALUES_OBJECT.into(),
                Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR.into(),
            )
            .map(
                |(object, property_constructor)| Self::QueryConstructionValues {
                    object,
                    property_constructor,
                }
            )
        );

        xor_with!(
            NodeKind::QueryConstructions,
            construction_value(object, knowledge, Atom::NODE_QUERY_CONSTRUCTIONS.into())
                .map(|constructor| Self::QueryConstructions { constructor })
        );

        xor_with!(
            NodeKind::SetContains,
            pair_values(
                object,
                knowledge,
                Atom::NODE_SET_CONTAINS_ELEMENT.into(),
                Atom::NODE_SET_CONTAINS_SET.into(),
            )
            .map(|(element, set)| Self::SetContains { element, set })
        );

        xor_with!(
            NodeKind::IsConstruction,
            construction_value(object, knowledge, Atom::NODE_IS_CONSTRUCTION.into())
                .map(Self::IsConstruction)
        );

        xor_with!(
            NodeKind::IsComposite,
            construction_value(object, knowledge, Atom::NODE_IS_COMPOSITE.into())
                .map(Self::IsComposite)
        );

        xor_with!(
            NodeKind::SetFold,
            triple_values(
                object,
                knowledge,
                Atom::NODE_SET_FOLD_SET.into(),
                Atom::NODE_SET_FOLD_FOLDER.into(),
                Atom::NODE_SET_FOLD_INITIAL.into(),
            )
            .map(|(set, folder, initial)| Self::SetFold {
                set,
                folder,
                initial,
            })
        );

        xor_with!(
            NodeKind::NewConstruction,
            pair_values(
                object,
                knowledge,
                Atom::NODE_NEW_CONSTRUCTION_CONSTRUCTOR.into(),
                Atom::NODE_NEW_CONSTRUCTION_VALUE.into(),
            )
            .map(|(constructor, value)| Self::NewConstruction { constructor, value })
        );

        xor_with!(
            NodeKind::NewSet,
            many_values(object, knowledge, Atom::NODE_NEW_SET_ITEM.into())
                .map(|items| Self::NewSet { items })
        );

        xor_with!(
            NodeKind::ConstructionConstructor,
            construction_value(
                object,
                knowledge,
                Atom::NODE_CONSTRUCTOR_OF_CONSTRUCTION.into()
            )
            .map(|construction| Self::ConstructionConstructor { construction })
        );

        xor_with!(
            NodeKind::ConstructionValue,
            construction_value(object, knowledge, Atom::NODE_VALUE_OF_CONSTRUCTION.into())
                .map(|construction| Self::ConstructionValue { construction })
        );

        debug_assert_eq!(kind.is_some(), node.is_some());

        match node {
            Some(node) => Ok(node),
            None => Err(diagnose(object, knowledge)),
        }
    }
}
