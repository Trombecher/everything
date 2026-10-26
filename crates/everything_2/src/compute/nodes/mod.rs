mod raw;
#[cfg(test)]
mod tests;

use num_bigint::BigInt;

use crate::{
    compute::nodes::raw::RawNode,
    iter::IteratorExtNextAndLast,
    knowledge::Knowledge,
    model::{Atom, Construction, Object, Set},
};

/// A node / expression in the computation system.
#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    /// A function node. Abstracts over the function body.
    Function {
        body: Box<Node>,
    },

    /// A literal node. Evaluates to the inner object.
    Literal(Object),

    /// A boolean conjunction.
    And {
        first: Box<Node>,
        second: Box<Node>,
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
    Count(Box<Node>),
    QueryAtomsWithProperty {
        property: Box<Node>,
    },
    QueryConstructionValues {
        set: Box<Node>,
        property_constructor: Box<Node>,
    },
    Contains {
        element: Box<Node>,
        set: Box<Node>,
    },
    Equal {
        first: Box<Node>,
        second: Box<Node>,
    },
    Or {
        first: Box<Node>,
        second: Box<Node>,
    },
    Xor {
        first: Box<Node>,
        second: Box<Node>,
    },
    Not(Box<Node>),
    Add {
        first: Box<Node>,
        second: Box<Node>,
    },
    Union {
        first: Box<Node>,
        second: Box<Node>,
    },
    Map {
        set: Box<Node>,
        mapper_function: Box<Node>,
    },
    Filter {
        set: Box<Node>,
        filter_function: Box<Node>,
    },
    Less {
        first: Box<Node>,
        second: Box<Node>,
    },
    If {
        condition: Box<Node>,
        then: Box<Node>,
        otherwise: Box<Node>,
    },
    UnwrapOr {
        set: Box<Node>,
        default: Box<Node>,
    },
    Multiply {
        first: Box<Node>,
        second: Box<Node>,
    },
    Call {
        callee: Box<Node>,
        with: Box<Node>,
    },
    IsAtom(Box<Node>),
    IsConstruction(Box<Node>),
    IsSet(Box<Node>),
    Every {
        set: Box<Node>,
        predicate: Box<Node>,
    },
    Any {
        set: Box<Node>,
        predicate: Box<Node>,
    },
    Fold {
        set: Box<Node>,
        folder: Box<Node>,
        initial: Box<Node>,
    },
    NewConstruction {
        constructor: Box<Node>,
        value: Box<Node>,
    },
    ConstructionConstructor {
        construction: Box<Node>,
    },
    ConstructionValue {
        construction: Box<Node>,
    },
}

#[derive(Debug, thiserror::Error)]
#[error("invalid node error")]
pub struct InvalidNodeError;

impl Node {
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
            } => todo!(),
            RawNode::Contains { element, set } => todo!(),
            RawNode::Equal { first, second } => todo!(),
            RawNode::Or { first, second } => todo!(),
            RawNode::Xor { first, second } => todo!(),
            RawNode::Not(object) => todo!(),
            RawNode::Add { first, second } => todo!(),
            RawNode::Union { first, second } => todo!(),
            RawNode::Map {
                set,
                mapper_function,
            } => todo!(),
            RawNode::Filter {
                set,
                filter_function,
            } => todo!(),
            RawNode::Less { first, second } => todo!(),
            RawNode::If {
                condition,
                then,
                otherwise,
            } => todo!(),
            RawNode::UnwrapOr { set, default } => todo!(),
            RawNode::Multiply { first, second } => todo!(),
            RawNode::Call { callee, with } => todo!(),
            RawNode::IsAtom(object) => todo!(),
            RawNode::IsConstruction(object) => todo!(),
            RawNode::IsSet(object) => todo!(),
            RawNode::Every { set, predicate } => todo!(),
            RawNode::Any { set, predicate } => todo!(),
            RawNode::Fold {
                set,
                folder,
                initial,
            } => todo!(),
            RawNode::NewConstruction { constructor, value } => todo!(),
            RawNode::ConstructionConstructor { construction } => todo!(),
            RawNode::ConstructionValue { construction } => todo!(),
        })
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
                Atom::NODE_CONSTRUCTOR_OF_CONSTRUCTION.into(),
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
                Atom::NODE_VALUE_OF_CONSTRUCTION.into(),
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
                Construction::new(Atom::NODE_AND_FIRST.into(), first).into(),
                Construction::new(Atom::NODE_AND_SECOND.into(), second).into(),
            ])
            .into(),
            Self::FunctionSelf { depth } => Set::new_move([Construction::new(
                Atom::NODE_FUNCTION_SELF.into(),
                Object::new_integer(BigInt::from(depth)),
            )
            .into()])
            .into(),
            Self::Parameter { depth } => Set::new_move([Construction::new(
                Atom::NODE_PARAMETER.into(),
                Object::new_integer(BigInt::from(depth)),
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
                Construction::new(Atom::NODE_EQUAL_FIRST.into(), first).into(),
                Construction::new(Atom::NODE_EQUAL_SECOND.into(), second).into(),
            ])
            .into(),
            Self::Or { first, second } => Set::new_move([
                Construction::new(Atom::NODE_OR_FIRST.into(), first).into(),
                Construction::new(Atom::NODE_OR_SECOND.into(), second).into(),
            ])
            .into(),
            Self::Xor { first, second } => Set::new_move([
                Construction::new(Atom::NODE_XOR_FIRST.into(), first).into(),
                Construction::new(Atom::NODE_XOR_SECOND.into(), second).into(),
            ])
            .into(),
            Self::Not(node) => {
                Set::new_move([Construction::new(Atom::NODE_NOT.into(), node).into()]).into()
            }
            Self::Add { first, second } => Set::new_move([
                Construction::new(Atom::NODE_ADD_FIRST.into(), first).into(),
                Construction::new(Atom::NODE_ADD_SECOND.into(), second).into(),
            ])
            .into(),
            Self::Union { first, second } => Set::new_move([
                Construction::new(Atom::NODE_UNION_FIRST.into(), first).into(),
                Construction::new(Atom::NODE_UNION_SECOND.into(), second).into(),
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
                Construction::new(Atom::NODE_LESS_FIRST.into(), first).into(),
                Construction::new(Atom::NODE_LESS_SECOND.into(), second).into(),
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
                Construction::new(Atom::NODE_MULTIPLY_FIRST.into(), first).into(),
                Construction::new(Atom::NODE_MULTIPLY_SECOND.into(), second).into(),
            ])
            .into(),
        }
    }
}

impl From<Node> for Object {
    fn from(value: Node) -> Self {
        value.into_object()
    }
}
