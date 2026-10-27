mod raw;
#[cfg(test)]
mod tests;

use std::sync::Arc;

use num_bigint::BigInt;

use crate::{
    compute::nodes::raw::RawNode,
    knowledge::Knowledge,
    model::{Atom, Construction, Object, Set},
};

/// A node / expression in the computation system.
#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    /// A function node. Abstracts over the function body.
    Function {
        body: Arc<Node>,
    },

    /// A literal node. Evaluates to the inner object.
    Literal(Object),

    /// A boolean conjunction.
    And {
        first: Arc<Node>,
        second: Arc<Node>,
    },

    /// A reference to an outer function.
    FunctionSelf {
        depth: u32,
    },

    /// A reference to an outer function parameter. Evaluates to the
    /// parameter value if the outer function is called with an object.
    Parameter {
        depth: u32,
    },

    /// Counts the number of values the inner has.
    Count(Arc<Node>),
    QueryAtomsWithProperty {
        property: Arc<Node>,
    },
    QueryConstructionValues {
        set: Arc<Node>,
        property_constructor: Arc<Node>,
    },
    Contains {
        element: Arc<Node>,
        set: Arc<Node>,
    },
    Equal {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    Or {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    Xor {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    Not(Arc<Node>),
    Add {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    Union {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    Map {
        set: Arc<Node>,
        mapper_function: Arc<Node>,
    },
    Filter {
        set: Arc<Node>,
        filter_function: Arc<Node>,
    },
    Less {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    If {
        condition: Arc<Node>,
        then: Arc<Node>,
        otherwise: Arc<Node>,
    },

    /// Evaluates to the only set element of the result
    /// of the evaluation of the inner node, if the result
    /// has exactly one element; otherwise it evaluates to
    /// undefined.
    Unwrap(Arc<Node>),
    Multiply {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    Call {
        callee: Arc<Node>,
        with: Arc<Node>,
    },
    IsAtom(Arc<Node>),
    IsConstruction(Arc<Node>),
    IsSet(Arc<Node>),
    Every {
        set: Arc<Node>,
        predicate: Arc<Node>,
    },
    Any {
        set: Arc<Node>,
        predicate: Arc<Node>,
    },
    Fold {
        set: Arc<Node>,
        folder: Arc<Node>,
        initial: Arc<Node>,
    },
    NewConstruction {
        constructor: Arc<Node>,
        value: Arc<Node>,
    },
    ConstructionConstructor {
        construction: Arc<Node>,
    },
    ConstructionValue {
        construction: Arc<Node>,
    },
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("invalid node error")]
pub struct InvalidNodeError;

impl Node {
    /// Tries to parse a node from an object (and a knowledge).
    ///
    /// # Errors
    ///
    /// If the object is invalid.
    #[allow(clippy::too_many_lines)]
    pub fn new(object: &Object, knowledge: &Knowledge) -> Result<Self, InvalidNodeError> {
        let raw = RawNode::new(object, knowledge).ok_or(InvalidNodeError)?;

        Ok(match raw {
            RawNode::Function { body } => Self::Function {
                body: Self::new(&body, knowledge)?.into(),
            },
            RawNode::Literal(object) => Self::Literal(object),
            RawNode::And { first, second } => Self::And {
                first: Self::new(&first, knowledge)?.into(),
                second: Self::new(&second, knowledge)?.into(),
            },
            RawNode::FunctionSelf { depth } => Self::FunctionSelf { depth },
            RawNode::Parameter { depth } => Self::Parameter { depth },
            RawNode::Count(object) => Self::Count(Self::new(&object, knowledge)?.into()),
            RawNode::QueryAtomsWithProperty { property } => Self::QueryAtomsWithProperty {
                property: Self::new(&property, knowledge)?.into(),
            },
            RawNode::QueryConstructionValues {
                set,
                property_constructor,
            } => Self::QueryConstructionValues {
                set: Self::new(&set, knowledge)?.into(),
                property_constructor: Self::new(&property_constructor, knowledge)?.into(),
            },
            RawNode::Contains { element, set } => Self::Contains {
                element: Self::new(&element, knowledge)?.into(),
                set: Self::new(&set, knowledge)?.into(),
            },
            RawNode::Equal { first, second } => Self::Equal {
                first: Self::new(&first, knowledge)?.into(),
                second: Self::new(&second, knowledge)?.into(),
            },
            RawNode::Or { first, second } => Self::Or {
                first: Self::new(&first, knowledge)?.into(),
                second: Self::new(&second, knowledge)?.into(),
            },
            RawNode::Xor { first, second } => Self::Xor {
                first: Self::new(&first, knowledge)?.into(),
                second: Self::new(&second, knowledge)?.into(),
            },
            RawNode::Not(object) => Self::Not(Self::new(&object, knowledge)?.into()),
            RawNode::Add { first, second } => Self::Add {
                first: Self::new(&first, knowledge)?.into(),
                second: Self::new(&second, knowledge)?.into(),
            },
            RawNode::Union { first, second } => Self::Union {
                first: Self::new(&first, knowledge)?.into(),
                second: Self::new(&second, knowledge)?.into(),
            },
            RawNode::Map {
                set,
                mapper_function,
            } => Self::Map {
                set: Self::new(&set, knowledge)?.into(),
                mapper_function: Self::new(&mapper_function, knowledge)?.into(),
            },
            RawNode::Filter {
                set,
                filter_function,
            } => Self::Filter {
                set: Self::new(&set, knowledge)?.into(),
                filter_function: Self::new(&filter_function, knowledge)?.into(),
            },
            RawNode::Less { first, second } => Self::Less {
                first: Self::new(&first, knowledge)?.into(),
                second: Self::new(&second, knowledge)?.into(),
            },
            RawNode::If {
                condition,
                then,
                otherwise,
            } => Self::If {
                condition: Self::new(&condition, knowledge)?.into(),
                then: Self::new(&then, knowledge)?.into(),
                otherwise: Self::new(&otherwise, knowledge)?.into(),
            },
            RawNode::Unwrap(set) => Self::Unwrap(Self::new(&set, knowledge)?.into()),
            RawNode::Multiply { first, second } => Self::Multiply {
                first: Self::new(&first, knowledge)?.into(),
                second: Self::new(&second, knowledge)?.into(),
            },
            RawNode::Call { callee, with } => Self::Call {
                callee: Self::new(&callee, knowledge)?.into(),
                with: Self::new(&with, knowledge)?.into(),
            },
            RawNode::IsAtom(object) => Self::IsAtom(Self::new(&object, knowledge)?.into()),
            RawNode::IsConstruction(object) => {
                Self::IsConstruction(Self::new(&object, knowledge)?.into())
            }
            RawNode::IsSet(object) => Self::IsSet(Self::new(&object, knowledge)?.into()),
            RawNode::Every { set, predicate } => Self::Every {
                set: Self::new(&set, knowledge)?.into(),
                predicate: Self::new(&predicate, knowledge)?.into(),
            },
            RawNode::Any { set, predicate } => Self::Any {
                set: Self::new(&set, knowledge)?.into(),
                predicate: Self::new(&predicate, knowledge)?.into(),
            },
            RawNode::Fold {
                set,
                folder,
                initial,
            } => Self::Fold {
                set: Self::new(&set, knowledge)?.into(),
                folder: Self::new(&folder, knowledge)?.into(),
                initial: Self::new(&initial, knowledge)?.into(),
            },
            RawNode::NewConstruction { constructor, value } => Self::NewConstruction {
                constructor: Self::new(&constructor, knowledge)?.into(),
                value: Self::new(&value, knowledge)?.into(),
            },
            RawNode::ConstructionConstructor { construction } => Self::ConstructionConstructor {
                construction: Self::new(&construction, knowledge)?.into(),
            },
            RawNode::ConstructionValue { construction } => Self::ConstructionValue {
                construction: Self::new(&construction, knowledge)?.into(),
            },
        })
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn into_object(&self) -> Object {
        match self {
            Self::IsConstruction(inner) => Set::new_move([Construction::new(
                Atom::NODE_IS_CONSTRUCTION.into(),
                inner.into_object(),
            )
            .into()])
            .into(),
            Self::IsSet(inner) => {
                Set::new_move([
                    Construction::new(Atom::NODE_IS_SET.into(), inner.into_object()).into(),
                ])
                .into()
            }
            Self::ConstructionConstructor { construction } => Set::new_move([Construction::new(
                Atom::NODE_CONSTRUCTOR_OF_CONSTRUCTION.into(),
                construction.into_object(),
            )
            .into()])
            .into(),
            Self::Fold {
                set,
                folder,
                initial,
            } => Set::new_move([
                Construction::new(Atom::NODE_FOLD_SET.into(), set.into_object()).into(),
                Construction::new(Atom::NODE_FOLD_FOLDER.into(), folder.into_object()).into(),
                Construction::new(Atom::NODE_FOLD_INITIAL.into(), initial.into_object()).into(),
            ])
            .into(),
            Self::NewConstruction { constructor, value } => Set::new_move([
                Construction::new(
                    Atom::NODE_NEW_CONSTRUCTION_CONSTRUCTOR.into(),
                    constructor.into_object(),
                )
                .into(),
                Construction::new(
                    Atom::NODE_NEW_CONSTRUCTION_VALUE.into(),
                    value.into_object(),
                )
                .into(),
            ])
            .into(),
            Self::Contains { element, set } => Set::new_move([
                Construction::new(Atom::NODE_CONTAINS_ELEMENT.into(), element.into_object()).into(),
                Construction::new(Atom::NODE_CONTAINS_SET.into(), set.into_object()).into(),
            ])
            .into(),
            Self::ConstructionValue { construction } => Set::new_move([Construction::new(
                Atom::NODE_VALUE_OF_CONSTRUCTION.into(),
                construction.into_object(),
            )
            .into()])
            .into(),
            Self::IsAtom(inner) => {
                Set::new_move([
                    Construction::new(Atom::NODE_IS_ATOM.into(), inner.into_object()).into(),
                ])
                .into()
            }
            Self::Call { callee, with } => Set::new_move([
                Construction::new(Atom::NODE_CALL_CALLEE.into(), callee.into_object()).into(),
                Construction::new(Atom::NODE_CALL_WITH.into(), with.into_object()).into(),
            ])
            .into(),
            Self::QueryConstructionValues {
                set,
                property_constructor,
            } => Set::new_move([
                Construction::new(Atom::NODE_CONSTRUCTION_VALUES_SET.into(), set.into_object())
                    .into(),
                Construction::new(
                    Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR.into(),
                    property_constructor.into_object(),
                )
                .into(),
            ])
            .into(),
            Self::Function { body } => {
                Set::new_move([
                    Construction::new(Atom::NODE_FUNCTION.into(), body.into_object()).into(),
                ])
                .into()
            }
            Self::Literal(literal) => {
                Set::new_move([
                    Construction::new(Atom::NODE_LITERAL.into(), literal.clone()).into(),
                ])
                .into()
            }
            Self::And { first, second } => Set::new_move([
                Construction::new(Atom::NODE_AND_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_AND_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::FunctionSelf { depth } => Set::new_move([Construction::new(
                Atom::NODE_FUNCTION_SELF.into(),
                Object::new_integer(BigInt::from(*depth)),
            )
            .into()])
            .into(),
            Self::Parameter { depth } => Set::new_move([Construction::new(
                Atom::NODE_PARAMETER.into(),
                Object::new_integer(BigInt::from(*depth)),
            )
            .into()])
            .into(),
            Self::Count(object) => {
                Set::new_move([
                    Construction::new(Atom::NODE_COUNT.into(), object.into_object()).into(),
                ])
                .into()
            }
            Self::QueryAtomsWithProperty { property } => Set::new_move([Construction::new(
                Atom::NODE_ATOMS_WITH_PROPERTY.into(),
                property.into_object(),
            )
            .into()])
            .into(),
            Self::Equal { first, second } => Set::new_move([
                Construction::new(Atom::NODE_EQUAL_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_EQUAL_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::Or { first, second } => Set::new_move([
                Construction::new(Atom::NODE_OR_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_OR_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::Xor { first, second } => Set::new_move([
                Construction::new(Atom::NODE_XOR_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_XOR_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::Not(node) => {
                Set::new_move([Construction::new(Atom::NODE_NOT.into(), node.into_object()).into()])
                    .into()
            }
            Self::Add { first, second } => Set::new_move([
                Construction::new(Atom::NODE_ADD_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_ADD_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::Union { first, second } => Set::new_move([
                Construction::new(Atom::NODE_UNION_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_UNION_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::Map {
                set,
                mapper_function,
            } => Set::new_move([
                Construction::new(Atom::NODE_MAP_SET.into(), set.into_object()).into(),
                Construction::new(Atom::NODE_MAP_MAPPER.into(), mapper_function.into_object())
                    .into(),
            ])
            .into(),
            Self::Filter {
                set,
                filter_function,
            } => Set::new_move([
                Construction::new(Atom::NODE_FILTER_SET.into(), set.into_object()).into(),
                Construction::new(
                    Atom::NODE_FILTER_FILTER.into(),
                    filter_function.into_object(),
                )
                .into(),
            ])
            .into(),
            Self::Every { set, predicate } => Set::new_move([
                Construction::new(Atom::NODE_EVERY_SET.into(), set.into_object()).into(),
                Construction::new(Atom::NODE_EVERY_PREDICATE.into(), predicate.into_object())
                    .into(),
            ])
            .into(),
            Self::Any { set, predicate } => Set::new_move([
                Construction::new(Atom::NODE_ANY_SET.into(), set.into_object()).into(),
                Construction::new(Atom::NODE_ANY_PREDICATE.into(), predicate.into_object()).into(),
            ])
            .into(),
            Self::Less { first, second } => Set::new_move([
                Construction::new(Atom::NODE_LESS_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_LESS_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::If {
                condition,
                otherwise,
                then,
            } => Set::new_move([
                Construction::new(Atom::NODE_IF_CONDITION.into(), condition.into_object()).into(),
                Construction::new(Atom::NODE_IF_THEN.into(), then.into_object()).into(),
                Construction::new(Atom::NODE_IF_ELSE.into(), otherwise.into_object()).into(),
            ])
            .into(),
            Self::Unwrap(set) => {
                Set::new_move([
                    Construction::new(Atom::NODE_UNWRAP_SET.into(), set.into_object()).into(),
                ])
                .into()
            }
            Self::Multiply { first, second } => Set::new_move([
                Construction::new(Atom::NODE_MULTIPLY_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_MULTIPLY_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
        }
    }

    #[must_use]
    pub fn set_size(&self) -> u64 {
        // Just count the properties.

        match self {
            Self::Function { .. }
            | Self::Literal(_)
            | Self::FunctionSelf { .. }
            | Self::Count(_)
            | Self::QueryAtomsWithProperty { .. }
            | Self::IsAtom(_)
            | Self::IsSet(_)
            | Self::IsConstruction(_)
            | Self::ConstructionConstructor { .. }
            | Self::ConstructionValue { .. }
            | Self::Not(_)
            | Self::Parameter { .. } => 1,

            Self::And { .. }
            | Self::QueryConstructionValues { .. }
            | Self::Contains { .. }
            | Self::Equal { .. }
            | Self::Or { .. }
            | Self::Xor { .. }
            | Self::Add { .. }
            | Self::Union { .. }
            | Self::Map { .. }
            | Self::Filter { .. }
            | Self::Less { .. }
            | Self::Multiply { .. }
            | Self::Call { .. }
            | Self::Every { .. }
            | Self::Any { .. }
            | Self::NewConstruction { .. }
            | Self::Unwrap { .. } => 2,

            Self::If { .. } | Self::Fold { .. } => 3,
        }
    }
}

impl From<Node> for Object {
    fn from(value: Node) -> Self {
        value.into_object()
    }
}
