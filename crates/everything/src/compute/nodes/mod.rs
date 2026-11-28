mod error;
mod kind;
mod raw;
#[cfg(test)]
mod tests;

pub use error::InvalidNodeError;
pub use kind::{NodeKind, NodeProperty, PropertyRequirement};
pub(crate) use raw::RawNode;

use std::sync::Arc;

use crate::{
    compute::evaluate::Thunk,
    knowledge::Knowledge,
    model::{Atom, Composite, Construction, Object},
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

    /// An internal node that evaluates a shared argument thunk.
    ///
    /// This node is not parsed from an object. Capture inserts it instead of
    /// copying an unevaluated argument expression, so that several references
    /// to a captured parameter share one evaluation (call-by-need). It
    /// materialises as the thunk's expression object.
    Thunk(Arc<Thunk>),

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

    /// Counts the number of items of the inner set. Atoms and composites are
    /// sets in the open set model; constructions evaluate to undefined.
    SetCount(Arc<Node>),

    /// Converts the inner result to a set of its properties: composites
    /// are returned unchanged, atoms evaluate to the set of their
    /// properties in the knowledge, and constructions evaluate to undefined.
    PropertiesOf(Arc<Node>),

    /// Evaluates to `true` iff the inner result has at least one set item.
    /// Atoms and composites are tested through their set view, and
    /// constructions evaluate to undefined.
    HasASetItem(Arc<Node>),
    /// Evaluates to `true` iff the property result is a property of the
    /// subject result. Atoms are resolved through the knowledge, composites
    /// through their members, and constructions evaluate to undefined as the
    /// subject. The property child is not evaluated if the subject is a
    /// construction.
    HasProperty {
        subject: Arc<Node>,
        property: Arc<Node>,
    },
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
    SetContains {
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
    SetUnion {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    /// A set intersection. Evaluates to a deferred set containing the items
    /// that are items of both results. If either result is a construction, the
    /// result is undefined.
    SetIntersection {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    /// A set difference. Evaluates to a deferred set containing the items of
    /// the first result without the items of the second result. If the first
    /// result is a construction, the result is undefined. If the first result
    /// has no items, the second result is not evaluated and the empty set is
    /// returned. Otherwise, if the second result is a construction, the result
    /// is undefined.
    SetDifference {
        first: Arc<Node>,
        second: Arc<Node>,
    },
    SetMap {
        set: Arc<Node>,
        mapper_function: Arc<Node>,
    },
    SetFilter {
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

    /// Evaluates to the only item of the result of the evaluation of the
    /// inner node, if the result has exactly one item; otherwise it evaluates
    /// to undefined.
    SetUnwrap(Arc<Node>),
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
    SetEvery {
        set: Arc<Node>,
        predicate: Arc<Node>,
    },
    SetAny {
        set: Arc<Node>,
        predicate: Arc<Node>,
    },
    SetFold {
        set: Arc<Node>,
        folder: Arc<Node>,
        initial: Arc<Node>,
    },
    NewConstruction {
        constructor: Arc<Node>,
        value: Arc<Node>,
    },
    /// Constructs a set from the results of the given child nodes.
    ///
    /// There is always at least one child. The children are evaluated in an
    /// unspecified order; see the computation documentation for the resulting
    /// well-formedness requirement.
    NewSet {
        items: Vec<Arc<Node>>,
    },
    /// Constructs a composite from the results of the given child nodes.
    ///
    /// There is always at least one child. The results become members of the
    /// composite directly; unlike [`Node::NewSet`], they are not wrapped in
    /// `contains` constructions. The children are evaluated in an unspecified
    /// order; see the computation documentation for the resulting
    /// well-formedness requirement.
    NewComposite {
        properties: Vec<Arc<Node>>,
    },
    ConstructionConstructor {
        construction: Arc<Node>,
    },
    ConstructionValue {
        construction: Arc<Node>,
    },
}

/// Materialises the source object into a composite so that its properties can
/// be transformed. Composites are returned unchanged; atoms are expanded through
/// the knowledge; constructions have no properties.
pub(crate) fn source_composite(object: &Object, knowledge: &Knowledge) -> Composite {
    match object {
        Object::Composite(set) => set.clone(),
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
            RawNode::SetCount(object) => Self::SetCount(child!(NodeProperty::SetCount, &object)),
            RawNode::PropertiesOf(object) => {
                Self::PropertiesOf(child!(NodeProperty::PropertiesOf, &object))
            }
            RawNode::HasASetItem(object) => {
                Self::HasASetItem(child!(NodeProperty::HasASetItem, &object))
            }
            RawNode::HasProperty { subject, property } => Self::HasProperty {
                subject: child!(NodeProperty::HasPropertySubject, &subject),
                property: child!(NodeProperty::HasPropertyProperty, &property),
            },
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
            RawNode::SetContains { element, set } => Self::SetContains {
                element: child!(NodeProperty::SetContainsElement, &element),
                set: child!(NodeProperty::SetContainsSet, &set),
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
            RawNode::SetUnion { first, second } => Self::SetUnion {
                first: child!(NodeProperty::SetUnionFirst, &first),
                second: child!(NodeProperty::SetUnionSecond, &second),
            },
            RawNode::SetIntersection { first, second } => Self::SetIntersection {
                first: child!(NodeProperty::SetIntersectionFirst, &first),
                second: child!(NodeProperty::SetIntersectionSecond, &second),
            },
            RawNode::SetDifference { first, second } => Self::SetDifference {
                first: child!(NodeProperty::SetDifferenceFirst, &first),
                second: child!(NodeProperty::SetDifferenceSecond, &second),
            },
            RawNode::SetMap {
                set,
                mapper_function,
            } => Self::SetMap {
                set: child!(NodeProperty::SetMapSet, &set),
                mapper_function: child!(NodeProperty::SetMapMapper, &mapper_function),
            },
            RawNode::SetFilter {
                set,
                filter_function,
            } => Self::SetFilter {
                set: child!(NodeProperty::SetFilterSet, &set),
                filter_function: child!(NodeProperty::SetFilterFilter, &filter_function),
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
            RawNode::SetUnwrap(set) => Self::SetUnwrap(child!(NodeProperty::SetUnwrapSet, &set)),
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
            RawNode::SetEvery { set, predicate } => Self::SetEvery {
                set: child!(NodeProperty::SetEverySet, &set),
                predicate: child!(NodeProperty::SetEveryPredicate, &predicate),
            },
            RawNode::SetAny { set, predicate } => Self::SetAny {
                set: child!(NodeProperty::SetAnySet, &set),
                predicate: child!(NodeProperty::SetAnyPredicate, &predicate),
            },
            RawNode::SetFold {
                set,
                folder,
                initial,
            } => Self::SetFold {
                set: child!(NodeProperty::SetFoldSet, &set),
                folder: child!(NodeProperty::SetFoldFolder, &folder),
                initial: child!(NodeProperty::SetFoldInitial, &initial),
            },
            RawNode::NewConstruction { constructor, value } => Self::NewConstruction {
                constructor: child!(NodeProperty::NewConstructionConstructor, &constructor),
                value: child!(NodeProperty::NewConstructionValue, &value),
            },
            RawNode::NewSet { items } => Self::NewSet {
                items: items
                    .iter()
                    .map(|item| Self::child(item, knowledge, NodeProperty::NewSetItem))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            RawNode::NewComposite { properties } => Self::NewComposite {
                properties: properties
                    .iter()
                    .map(|property| {
                        Self::child(property, knowledge, NodeProperty::NewCompositeProperty)
                    })
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
    pub fn into_object(&self) -> Object {
        match self {
            Self::WithSource { source, .. } => (**source).clone(),
            Self::Thunk(thunk) => thunk.expression().into_object(),
            Self::Function { body, object } => function_object(object, body),
            node => node.to_raw().into_object(),
        }
    }

    /// Maps an ordinary node to its raw node object representation.
    ///
    /// The runtime-only variants are handled by [`Self::into_object`] before
    /// this method is called.
    #[allow(clippy::too_many_lines)]
    fn to_raw(&self) -> RawNode {
        match self {
            Self::WithSource { .. } | Self::Thunk(_) | Self::Function { .. } => {
                unreachable!("runtime-only node variants are serialized separately")
            }
            Self::Literal(object) => RawNode::Literal(object.clone()),
            Self::And { first, second } => RawNode::And {
                first: first.into_object(),
                second: second.into_object(),
            },
            Self::FunctionSelf { depth } => RawNode::FunctionSelf { depth: *depth },
            Self::Parameter { depth } => RawNode::Parameter { depth: *depth },
            Self::SetCount(object) => RawNode::SetCount(object.into_object()),
            Self::PropertiesOf(object) => RawNode::PropertiesOf(object.into_object()),
            Self::HasASetItem(object) => RawNode::HasASetItem(object.into_object()),
            Self::HasProperty { subject, property } => RawNode::HasProperty {
                subject: subject.into_object(),
                property: property.into_object(),
            },
            Self::QueryAtomsWithProperty { property } => RawNode::QueryAtomsWithProperty {
                property: property.into_object(),
            },
            Self::QueryAtomsWithConstructor { constructor } => RawNode::QueryAtomsWithConstructor {
                constructor: constructor.into_object(),
            },
            Self::QueryConstructionValues {
                object,
                property_constructor,
            } => RawNode::QueryConstructionValues {
                object: object.into_object(),
                property_constructor: property_constructor.into_object(),
            },
            Self::QueryConstructions { constructor } => RawNode::QueryConstructions {
                constructor: constructor.into_object(),
            },
            Self::SetContains { element, set } => RawNode::SetContains {
                element: element.into_object(),
                set: set.into_object(),
            },
            Self::Equal { first, second } => RawNode::Equal {
                first: first.into_object(),
                second: second.into_object(),
            },
            Self::Or { first, second } => RawNode::Or {
                first: first.into_object(),
                second: second.into_object(),
            },
            Self::Xor { first, second } => RawNode::Xor {
                first: first.into_object(),
                second: second.into_object(),
            },
            Self::Not(object) => RawNode::Not(object.into_object()),
            Self::Add { first, second } => RawNode::Add {
                first: first.into_object(),
                second: second.into_object(),
            },
            Self::SetUnion { first, second } => RawNode::SetUnion {
                first: first.into_object(),
                second: second.into_object(),
            },
            Self::SetIntersection { first, second } => RawNode::SetIntersection {
                first: first.into_object(),
                second: second.into_object(),
            },
            Self::SetDifference { first, second } => RawNode::SetDifference {
                first: first.into_object(),
                second: second.into_object(),
            },
            Self::SetMap {
                set,
                mapper_function,
            } => RawNode::SetMap {
                set: set.into_object(),
                mapper_function: mapper_function.into_object(),
            },
            Self::SetFilter {
                set,
                filter_function,
            } => RawNode::SetFilter {
                set: set.into_object(),
                filter_function: filter_function.into_object(),
            },
            Self::Less { first, second } => RawNode::Less {
                first: first.into_object(),
                second: second.into_object(),
            },
            Self::If {
                condition,
                then,
                otherwise,
            } => RawNode::If {
                condition: condition.into_object(),
                then: then.into_object(),
                otherwise: otherwise.into_object(),
            },
            Self::SetUnwrap(object) => RawNode::SetUnwrap(object.into_object()),
            Self::Multiply { first, second } => RawNode::Multiply {
                first: first.into_object(),
                second: second.into_object(),
            },
            Self::Call { callee, with } => RawNode::Call {
                callee: callee.into_object(),
                with: with.into_object(),
            },
            Self::IsAtom(object) => RawNode::IsAtom(object.into_object()),
            Self::IsConstruction(object) => RawNode::IsConstruction(object.into_object()),
            Self::IsComposite(object) => RawNode::IsComposite(object.into_object()),
            Self::SetEvery { set, predicate } => RawNode::SetEvery {
                set: set.into_object(),
                predicate: predicate.into_object(),
            },
            Self::SetAny { set, predicate } => RawNode::SetAny {
                set: set.into_object(),
                predicate: predicate.into_object(),
            },
            Self::SetFold {
                set,
                folder,
                initial,
            } => RawNode::SetFold {
                set: set.into_object(),
                folder: folder.into_object(),
                initial: initial.into_object(),
            },
            Self::NewConstruction { constructor, value } => RawNode::NewConstruction {
                constructor: constructor.into_object(),
                value: value.into_object(),
            },
            Self::NewSet { items } => RawNode::NewSet {
                items: items.iter().map(|item| item.into_object()).collect(),
            },
            Self::NewComposite { properties } => RawNode::NewComposite {
                properties: properties
                    .iter()
                    .map(|property| property.into_object())
                    .collect(),
            },
            Self::ConstructionConstructor { construction } => RawNode::ConstructionConstructor {
                construction: construction.into_object(),
            },
            Self::ConstructionValue { construction } => RawNode::ConstructionValue {
                construction: construction.into_object(),
            },
        }
    }
}

impl From<Node> for Object {
    fn from(value: Node) -> Self {
        value.into_object()
    }
}
