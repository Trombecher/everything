mod error;
mod kind;
mod raw;
#[cfg(test)]
mod tests;

pub use error::InvalidNodeError;
pub use kind::{NodeKind, NodeProperty, PropertyRequirement};

use std::sync::Arc;

use num_bigint::BigInt;

use crate::{
    compute::nodes::raw::RawNode,
    knowledge::Knowledge,
    model::{Atom, Composite, CompositeApi, Construction, Object},
};

/// A node / expression in the computation system.
#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    /// A parsed node together with the exact object it was parsed from.
    ///
    /// Preserving the source object keeps all properties, including ones that
    /// are not part of the node kind, such as annotations. This makes
    /// node-to-object conversion lossless. Use [`Node::resolve`] to inspect
    /// the structural node and [`Node::into_plain`] to unwrap it.
    WithSource {
        node: Arc<Self>,
        source: Arc<Object>,
    },

    /// A function node. Abstracts over the function body.
    Function {
        body: Arc<Node>,
        /// The object form of the function, with all non-body properties.
        ///
        /// Atom-defined functions are materialised into composites here. The body is
        /// replaced on materialisation, so this may still contain an older body.
        object: Arc<Composite>,
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

    /// Counts the number of elements of the inner composite. Evaluates to
    /// undefined if the inner result is not a composite.
    Count(Arc<Node>),

    /// Converts the inner result to a composite of its properties: composites
    /// are returned unchanged, atoms evaluate to the composite of their
    /// properties in the knowledge, and constructions evaluate to undefined.
    PropertiesOf(Arc<Node>),

    /// Evaluates to `true` iff the inner result has at least one value.
    /// Composites are tested directly, atoms are tested against their values
    /// in the knowledge, and constructions evaluate to undefined.
    HasValue(Arc<Node>),
    QueryAtomsWithProperty {
        property: Arc<Node>,
    },
    /// Queries the knowledge for all atoms that have a property that is a
    /// construction with the given constructor.
    QueryAtomsWithConstructor {
        constructor: Arc<Node>,
    },
    /// Queries the values of constructions with the given property
    /// constructor among the values of an object. Atoms are resolved
    /// through the knowledge, composites through their members, and
    /// constructions evaluate to undefined.
    QueryConstructionValues {
        object: Arc<Node>,
        property_constructor: Arc<Node>,
    },
    /// Queries the knowledge for all pairs of an atom and a value such
    /// that the atom has a construction with the given constructor.
    QueryConstructions {
        constructor: Arc<Node>,
    },
    Contains {
        element: Arc<Node>,
        composite: Arc<Node>,
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
    /// A composite intersection. Evaluates to a deferred composite containing the values
    /// that are members of both results. If either result is not a composite, the
    /// result is undefined.
    Intersection {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    /// A composite difference. Evaluates to a deferred composite containing the values of
    /// the first result without the values of the second result. If the first
    /// result is not a composite, the result is undefined. If the first result is
    /// empty, the second result is not evaluated and the empty composite is returned.
    /// Otherwise, if the second result is not a composite, the result is undefined.
    Difference {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    Map {
        composite: Arc<Node>,
        mapper_function: Arc<Node>,
    },
    Filter {
        composite: Arc<Node>,
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

    /// Evaluates to the only composite element of the result
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
    IsComposite(Arc<Node>),
    Every {
        composite: Arc<Node>,
        predicate: Arc<Node>,
    },
    Any {
        composite: Arc<Node>,
        predicate: Arc<Node>,
    },
    Fold {
        composite: Arc<Node>,
        folder: Arc<Node>,
        initial: Arc<Node>,
    },
    NewConstruction {
        constructor: Arc<Node>,
        value: Arc<Node>,
    },
    /// Constructs a composite from the results of the given child nodes.
    ///
    /// There is always at least one child. The children are evaluated in an
    /// unspecified order; see the computation documentation for the resulting
    /// well-formedness requirement.
    NewComposite {
        values: Vec<Arc<Node>>,
    },
    ConstructionConstructor {
        construction: Arc<Node>,
    },
    ConstructionValue {
        construction: Arc<Node>,
    },
}

/// Materialises the source object into a composite so that its properties can be
/// transformed. Composites are returned unchanged; atoms are expanded through the
/// knowledge; constructions have no properties.
pub(crate) fn source_composite(object: &Object, knowledge: &Knowledge) -> Composite {
    match object {
        Object::Composite(composite) => composite.clone(),
        Object::Atom(atom) => {
            let mut values = knowledge
                .properties_of_atom(atom)
                .into_iter()
                .collect::<Vec<_>>();

            Composite::from(values.as_mut_slice())
        }
        Object::Construction(_) => Composite::empty(),
    }
}

fn function_object(object: &Composite, body: &Node) -> Object {
    let mut values = object
        .clone()
        .into_iter()
        .filter(|value| {
            !matches!(
                value,
                Object::Construction(Construction { constructor, .. })
                    if matches!(&**constructor, Object::Atom(atom) if *atom == Atom::NODE_FUNCTION)
            )
        })
        .collect::<Vec<_>>();

    values.push(Construction::new(Atom::NODE_FUNCTION.into(), body.into_object()).into());

    Composite::from(values.as_mut_slice()).into()
}

impl Node {
    /// Creates a function node with the minimal function object form.
    #[must_use]
    pub fn function(body: Self) -> Self {
        let object =
            Composite::from([
                Construction::new(Atom::NODE_FUNCTION.into(), body.into_object()).into(),
            ]);

        Self::Function {
            body: Arc::new(body),
            object: Arc::new(object),
        }
    }

    /// Tries to parse a node from an object (and a knowledge).
    ///
    /// # Errors
    ///
    /// If the object is invalid.
    #[allow(clippy::too_many_lines)]
    pub fn new(object: &Object, knowledge: &Knowledge) -> Result<Self, InvalidNodeError> {
        let raw = RawNode::new(object, knowledge)?;

        macro_rules! child {
            ($property:expr, $object:expr) => {
                Self::child($object, knowledge, $property)?
            };
        }

        let node = match raw {
            RawNode::Function { body } => Self::Function {
                body: child!(NodeProperty::Function, &body),
                object: Arc::new(source_composite(object, knowledge)),
            },
            RawNode::Literal(object) => Self::Literal(object),
            RawNode::And { first, second } => Self::And {
                first: child!(NodeProperty::AndFirst, &first),
                second: child!(NodeProperty::AndSecond, &second),
            },
            RawNode::FunctionSelf { depth } => Self::FunctionSelf { depth },
            RawNode::Parameter { depth } => Self::Parameter { depth },
            RawNode::Count(object) => Self::Count(child!(NodeProperty::Count, &object)),
            RawNode::PropertiesOf(object) => {
                Self::PropertiesOf(child!(NodeProperty::PropertiesOf, &object))
            }
            RawNode::HasValue(object) => Self::HasValue(child!(NodeProperty::HasValue, &object)),
            RawNode::QueryAtomsWithProperty { property } => Self::QueryAtomsWithProperty {
                property: child!(NodeProperty::AtomsWithProperty, &property),
            },
            RawNode::QueryAtomsWithConstructor { constructor } => Self::QueryAtomsWithConstructor {
                constructor: child!(NodeProperty::AtomsWithConstructor, &constructor),
            },
            RawNode::QueryConstructionValues {
                object,
                property_constructor,
            } => Self::QueryConstructionValues {
                object: child!(NodeProperty::ConstructionValuesObject, &object),
                property_constructor: child!(
                    NodeProperty::ConstructionValuesPropertyConstructor,
                    &property_constructor
                ),
            },
            RawNode::QueryConstructions { constructor } => Self::QueryConstructions {
                constructor: child!(NodeProperty::QueryConstructions, &constructor),
            },
            RawNode::Contains { element, composite } => Self::Contains {
                element: child!(NodeProperty::ContainsElement, &element),
                composite: child!(NodeProperty::ContainsComposite, &composite),
            },
            RawNode::Equal { first, second } => Self::Equal {
                first: child!(NodeProperty::EqualFirst, &first),
                second: child!(NodeProperty::EqualSecond, &second),
            },
            RawNode::Or { first, second } => Self::Or {
                first: child!(NodeProperty::OrFirst, &first),
                second: child!(NodeProperty::OrSecond, &second),
            },
            RawNode::Xor { first, second } => Self::Xor {
                first: child!(NodeProperty::XorFirst, &first),
                second: child!(NodeProperty::XorSecond, &second),
            },
            RawNode::Not(object) => Self::Not(child!(NodeProperty::Not, &object)),
            RawNode::Add { first, second } => Self::Add {
                first: child!(NodeProperty::AddFirst, &first),
                second: child!(NodeProperty::AddSecond, &second),
            },
            RawNode::Union { first, second } => Self::Union {
                first: child!(NodeProperty::UnionFirst, &first),
                second: child!(NodeProperty::UnionSecond, &second),
            },
            RawNode::Intersection { first, second } => Self::Intersection {
                first: child!(NodeProperty::IntersectionFirst, &first),
                second: child!(NodeProperty::IntersectionSecond, &second),
            },
            RawNode::Difference { first, second } => Self::Difference {
                first: child!(NodeProperty::DifferenceFirst, &first),
                second: child!(NodeProperty::DifferenceSecond, &second),
            },
            RawNode::Map {
                composite,
                mapper_function,
            } => Self::Map {
                composite: child!(NodeProperty::MapComposite, &composite),
                mapper_function: child!(NodeProperty::MapMapper, &mapper_function),
            },
            RawNode::Filter {
                composite,
                filter_function,
            } => Self::Filter {
                composite: child!(NodeProperty::FilterComposite, &composite),
                filter_function: child!(NodeProperty::FilterFilter, &filter_function),
            },
            RawNode::Less { first, second } => Self::Less {
                first: child!(NodeProperty::LessFirst, &first),
                second: child!(NodeProperty::LessSecond, &second),
            },
            RawNode::If {
                condition,
                then,
                otherwise,
            } => Self::If {
                condition: child!(NodeProperty::IfCondition, &condition),
                then: child!(NodeProperty::IfThen, &then),
                otherwise: child!(NodeProperty::IfElse, &otherwise),
            },
            RawNode::Unwrap(composite) => {
                Self::Unwrap(child!(NodeProperty::UnwrapComposite, &composite))
            }
            RawNode::Multiply { first, second } => Self::Multiply {
                first: child!(NodeProperty::MultiplyFirst, &first),
                second: child!(NodeProperty::MultiplySecond, &second),
            },
            RawNode::Call { callee, with } => Self::Call {
                callee: child!(NodeProperty::CallCallee, &callee),
                with: child!(NodeProperty::CallWith, &with),
            },
            RawNode::IsAtom(object) => Self::IsAtom(child!(NodeProperty::IsAtom, &object)),
            RawNode::IsConstruction(object) => {
                Self::IsConstruction(child!(NodeProperty::IsConstruction, &object))
            }
            RawNode::IsComposite(object) => {
                Self::IsComposite(child!(NodeProperty::IsComposite, &object))
            }
            RawNode::Every {
                composite,
                predicate,
            } => Self::Every {
                composite: child!(NodeProperty::EveryComposite, &composite),
                predicate: child!(NodeProperty::EveryPredicate, &predicate),
            },
            RawNode::Any {
                composite,
                predicate,
            } => Self::Any {
                composite: child!(NodeProperty::AnyComposite, &composite),
                predicate: child!(NodeProperty::AnyPredicate, &predicate),
            },
            RawNode::Fold {
                composite,
                folder,
                initial,
            } => Self::Fold {
                composite: child!(NodeProperty::FoldComposite, &composite),
                folder: child!(NodeProperty::FoldFolder, &folder),
                initial: child!(NodeProperty::FoldInitial, &initial),
            },
            RawNode::NewConstruction { constructor, value } => Self::NewConstruction {
                constructor: child!(NodeProperty::NewConstructionConstructor, &constructor),
                value: child!(NodeProperty::NewConstructionValue, &value),
            },
            RawNode::NewComposite { values } => Self::NewComposite {
                values: values
                    .iter()
                    .map(|value| Self::child(value, knowledge, NodeProperty::NewCompositeValue))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            RawNode::ConstructionConstructor { construction } => Self::ConstructionConstructor {
                construction: child!(NodeProperty::ConstructorOfConstruction, &construction),
            },
            RawNode::ConstructionValue { construction } => Self::ConstructionValue {
                construction: child!(NodeProperty::ValueOfConstruction, &construction),
            },
        };

        Ok(Self::WithSource {
            node: Arc::new(node),
            source: Arc::new(object.clone()),
        })
    }

    fn child(
        object: &Object,
        knowledge: &Knowledge,
        property: NodeProperty,
    ) -> Result<Arc<Self>, InvalidNodeError> {
        Self::new(object, knowledge)
            .map(Arc::new)
            .map_err(|source| InvalidNodeError::Child {
                property,
                source: Box::new(source),
            })
    }

    /// Returns the structural node, bypassing any source wrappers.
    #[must_use]
    pub fn resolve(&self) -> &Self {
        let mut node = self;

        while let Self::WithSource { node: inner, .. } = node {
            node = inner;
        }

        node
    }

    /// Consumes the node and returns its structural form, dropping the
    /// preserved source object.
    #[must_use]
    pub fn into_plain(self) -> Self {
        match self {
            Self::WithSource { node, .. } => (*node).clone().into_plain(),
            node => node,
        }
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn into_object(&self) -> Object {
        match self {
            Self::WithSource { source, .. } => (**source).clone(),
            Self::IsConstruction(inner) => Composite::from([Construction::new(
                Atom::NODE_IS_CONSTRUCTION.into(),
                inner.into_object(),
            )
            .into()])
            .into(),
            Self::IsComposite(inner) => Composite::from([Construction::new(
                Atom::NODE_IS_COMPOSITE.into(),
                inner.into_object(),
            )
            .into()])
            .into(),
            Self::ConstructionConstructor { construction } => Composite::from([Construction::new(
                Atom::NODE_CONSTRUCTOR_OF_CONSTRUCTION.into(),
                construction.into_object(),
            )
            .into()])
            .into(),
            Self::Fold {
                composite,
                folder,
                initial,
            } => Composite::from([
                Construction::new(Atom::NODE_FOLD_COMPOSITE.into(), composite.into_object()).into(),
                Construction::new(Atom::NODE_FOLD_FOLDER.into(), folder.into_object()).into(),
                Construction::new(Atom::NODE_FOLD_INITIAL.into(), initial.into_object()).into(),
            ])
            .into(),
            Self::NewConstruction { constructor, value } => Composite::from([
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
            Self::NewComposite { values } => {
                let mut properties = values
                    .iter()
                    .map(|value| {
                        Construction::new(
                            Atom::NODE_NEW_COMPOSITE_VALUE.into(),
                            value.into_object(),
                        )
                        .into()
                    })
                    .collect::<Vec<Object>>();

                Composite::from(properties.as_mut_slice()).into()
            }
            Self::Contains { element, composite } => Composite::from([
                Construction::new(Atom::NODE_CONTAINS_ELEMENT.into(), element.into_object()).into(),
                Construction::new(
                    Atom::NODE_CONTAINS_COMPOSITE.into(),
                    composite.into_object(),
                )
                .into(),
            ])
            .into(),
            Self::ConstructionValue { construction } => Composite::from([Construction::new(
                Atom::NODE_VALUE_OF_CONSTRUCTION.into(),
                construction.into_object(),
            )
            .into()])
            .into(),
            Self::IsAtom(inner) => {
                Composite::from([
                    Construction::new(Atom::NODE_IS_ATOM.into(), inner.into_object()).into(),
                ])
                .into()
            }
            Self::Call { callee, with } => Composite::from([
                Construction::new(Atom::NODE_CALL_CALLEE.into(), callee.into_object()).into(),
                Construction::new(Atom::NODE_CALL_WITH.into(), with.into_object()).into(),
            ])
            .into(),
            Self::QueryConstructionValues {
                object,
                property_constructor,
            } => Composite::from([
                Construction::new(
                    Atom::NODE_CONSTRUCTION_VALUES_OBJECT.into(),
                    object.into_object(),
                )
                .into(),
                Construction::new(
                    Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR.into(),
                    property_constructor.into_object(),
                )
                .into(),
            ])
            .into(),
            Self::Function { body, object } => function_object(object, body),
            Self::Literal(literal) => {
                Composite::from([
                    Construction::new(Atom::NODE_LITERAL.into(), literal.clone()).into(),
                ])
                .into()
            }
            Self::And { first, second } => Composite::from([
                Construction::new(Atom::NODE_AND_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_AND_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::FunctionSelf { depth } => Composite::from([Construction::new(
                Atom::NODE_FUNCTION_SELF.into(),
                Object::new_integer(BigInt::from(*depth)),
            )
            .into()])
            .into(),
            Self::Parameter { depth } => Composite::from([Construction::new(
                Atom::NODE_PARAMETER.into(),
                Object::new_integer(BigInt::from(*depth)),
            )
            .into()])
            .into(),
            Self::Count(object) => {
                Composite::from([
                    Construction::new(Atom::NODE_COUNT.into(), object.into_object()).into(),
                ])
                .into()
            }
            Self::PropertiesOf(object) => Composite::from([Construction::new(
                Atom::NODE_PROPERTIES_OF.into(),
                object.into_object(),
            )
            .into()])
            .into(),
            Self::HasValue(object) => Composite::from([Construction::new(
                Atom::NODE_HAS_VALUE.into(),
                object.into_object(),
            )
            .into()])
            .into(),
            Self::QueryAtomsWithProperty { property } => Composite::from([Construction::new(
                Atom::NODE_ATOMS_WITH_PROPERTY.into(),
                property.into_object(),
            )
            .into()])
            .into(),
            Self::QueryAtomsWithConstructor { constructor } => {
                Composite::from([Construction::new(
                    Atom::NODE_ATOMS_WITH_CONSTRUCTOR.into(),
                    constructor.into_object(),
                )
                .into()])
                .into()
            }
            Self::Equal { first, second } => Composite::from([
                Construction::new(Atom::NODE_EQUAL_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_EQUAL_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::Or { first, second } => Composite::from([
                Construction::new(Atom::NODE_OR_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_OR_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::Xor { first, second } => Composite::from([
                Construction::new(Atom::NODE_XOR_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_XOR_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::Not(node) => {
                Composite::from([
                    Construction::new(Atom::NODE_NOT.into(), node.into_object()).into()
                ])
                .into()
            }
            Self::Add { first, second } => Composite::from([
                Construction::new(Atom::NODE_ADD_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_ADD_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::Union { first, second } => Composite::from([
                Construction::new(Atom::NODE_UNION_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_UNION_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::Intersection { first, second } => Composite::from([
                Construction::new(Atom::NODE_INTERSECTION_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_INTERSECTION_SECOND.into(), second.into_object())
                    .into(),
            ])
            .into(),
            Self::Difference { first, second } => Composite::from([
                Construction::new(Atom::NODE_DIFFERENCE_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_DIFFERENCE_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::QueryConstructions { constructor } => Composite::from([Construction::new(
                Atom::NODE_QUERY_CONSTRUCTIONS.into(),
                constructor.into_object(),
            )
            .into()])
            .into(),
            Self::Map {
                composite,
                mapper_function,
            } => Composite::from([
                Construction::new(Atom::NODE_MAP_COMPOSITE.into(), composite.into_object()).into(),
                Construction::new(Atom::NODE_MAP_MAPPER.into(), mapper_function.into_object())
                    .into(),
            ])
            .into(),
            Self::Filter {
                composite,
                filter_function,
            } => Composite::from([
                Construction::new(Atom::NODE_FILTER_COMPOSITE.into(), composite.into_object())
                    .into(),
                Construction::new(
                    Atom::NODE_FILTER_FILTER.into(),
                    filter_function.into_object(),
                )
                .into(),
            ])
            .into(),
            Self::Every {
                composite,
                predicate,
            } => Composite::from([
                Construction::new(Atom::NODE_EVERY_COMPOSITE.into(), composite.into_object())
                    .into(),
                Construction::new(Atom::NODE_EVERY_PREDICATE.into(), predicate.into_object())
                    .into(),
            ])
            .into(),
            Self::Any {
                composite,
                predicate,
            } => Composite::from([
                Construction::new(Atom::NODE_ANY_COMPOSITE.into(), composite.into_object()).into(),
                Construction::new(Atom::NODE_ANY_PREDICATE.into(), predicate.into_object()).into(),
            ])
            .into(),
            Self::Less { first, second } => Composite::from([
                Construction::new(Atom::NODE_LESS_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_LESS_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
            Self::If {
                condition,
                otherwise,
                then,
            } => Composite::from([
                Construction::new(Atom::NODE_IF_CONDITION.into(), condition.into_object()).into(),
                Construction::new(Atom::NODE_IF_THEN.into(), then.into_object()).into(),
                Construction::new(Atom::NODE_IF_ELSE.into(), otherwise.into_object()).into(),
            ])
            .into(),
            Self::Unwrap(composite) => Composite::from([Construction::new(
                Atom::NODE_UNWRAP_COMPOSITE.into(),
                composite.into_object(),
            )
            .into()])
            .into(),
            Self::Multiply { first, second } => Composite::from([
                Construction::new(Atom::NODE_MULTIPLY_FIRST.into(), first.into_object()).into(),
                Construction::new(Atom::NODE_MULTIPLY_SECOND.into(), second.into_object()).into(),
            ])
            .into(),
        }
    }

    #[must_use]
    pub fn composite_size(&self) -> u64 {
        // Just count the properties.

        match self {
            Self::WithSource { node, source } => match &**source {
                Object::Composite(composite) => composite.size(),
                Object::Atom(_) | Object::Construction(_) => node.composite_size(),
            },

            Self::Function { object, .. } => object.size(),

            Self::Literal(_)
            | Self::FunctionSelf { .. }
            | Self::Count(_)
            | Self::PropertiesOf(_)
            | Self::HasValue(_)
            | Self::QueryAtomsWithProperty { .. }
            | Self::QueryAtomsWithConstructor { .. }
            | Self::QueryConstructions { .. }
            | Self::IsAtom(_)
            | Self::IsComposite(_)
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
            | Self::Intersection { .. }
            | Self::Difference { .. }
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

            Self::NewComposite { values } => values.len() as u64,
        }
    }
}

impl From<Node> for Object {
    fn from(value: Node) -> Self {
        value.into_object()
    }
}
