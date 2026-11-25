#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use crate::{
    compute::{self, MaybeUndefined, Node},
    knowledge::{
        AtomValuePair, Knowledge, ObjectProperties, Properties, QueryAtomsWithConstructor,
        QueryAtomsWithProperty, QueryConstructions,
    },
    model::{Composite, Construction, Object},
};

#[derive(Clone, Debug)]
pub enum LazyObject {
    /// Just an object.
    Eager(Object),

    /// A node.
    Node(Node),

    /// The composite of properties of an atom in the knowledge.
    ///
    /// The properties are kept in their knowledge container, so iterating
    /// them does not require materialising a model [`Composite`].
    AtomProperties(Properties),

    /// Represents the composite that is the union of two [`LazyObject`]s.
    Union { first: Box<Self>, second: Box<Self> },

    /// Represents the composite that is the intersection of two [`LazyObject`]s.
    Intersection { first: Box<Self>, second: Box<Self> },

    /// Represents the composite that is the difference of two [`LazyObject`]s.
    Difference {
        source: Box<Self>,
        without: Box<Self>,
    },

    /// The composite of atoms with the given property.
    AtomsWithProperty { property: Object },

    /// The composite of atoms that have a property that is a construction with the
    /// given constructor.
    AtomsWithConstructor { constructor: Object },

    /// The values of constructions with the given constructor among the values
    /// of a given object.
    ///
    /// The target is iterated lazily and only constructions whose constructor
    /// equals `property_constructor` contribute their value. Atoms are resolved
    /// through the knowledge and composites through their members.
    ConstructionValues {
        target: Box<Self>,
        property_constructor: Object,
    },

    /// The pairs of an atom and a value such that the atom has a construction
    /// with the given constructor in the knowledge.
    ///
    /// Each pair is yielded as the object `{atom : <atom>, value : <value>}`.
    Constructions { constructor: Object },

    /// Maps every value of a given composite with a mapper function.
    Map {
        target: Box<Self>,
        /// This function is captured.
        mapper_function: Node,
    },

    /// Retains all values for that the filter function returns a truthy value.
    Filter {
        target: Box<Self>,
        /// This function is captured.
        filter_function: Node,
    },
}

impl From<Object> for LazyObject {
    fn from(object: Object) -> Self {
        Self::Eager(object)
    }
}

impl From<Node> for LazyObject {
    fn from(value: Node) -> Self {
        Self::Node(value)
    }
}

impl LazyObject {
    /// Returns `true` iff `self` is a composite result: an evaluated composite, atom
    /// values, a function object, or a deferred composite operation.
    #[must_use]
    pub fn is_composite(&self) -> bool {
        match self {
            Self::Eager(Object::Composite(_))
            | Self::AtomProperties(_)
            | Self::Node(_)
            | Self::Union { .. }
            | Self::Intersection { .. }
            | Self::Difference { .. }
            | Self::AtomsWithProperty { .. }
            | Self::AtomsWithConstructor { .. }
            | Self::ConstructionValues { .. }
            | Self::Constructions { .. }
            | Self::Map { .. }
            | Self::Filter { .. } => true,
            Self::Eager(Object::Atom(_) | Object::Construction(_)) => false,
        }
    }

    /// Tries to convert `self` into a node, returning `None` if the value
    /// cannot be materialised as an object.
    #[must_use]
    pub fn try_into_node(self, knowledge: &Knowledge) -> Option<Node> {
        match self {
            Self::Node(node) => Some(node),
            object => Node::new(&object.into_object(knowledge), knowledge).ok(),
        }
    }

    /// Tries to convert `self` into an evaluated object, returning `None`
    /// for deferred values that cannot be materialised yet.
    #[must_use]
    pub fn try_into_object(self) -> Option<Object> {
        match self {
            Self::Eager(object) => Some(object),
            Self::Node(node) => Some(node.into_object()),
            Self::AtomProperties(properties) => Some(atom_properties_into_object(properties)),
            Self::Union { .. }
            | Self::Intersection { .. }
            | Self::Difference { .. }
            | Self::AtomsWithProperty { .. }
            | Self::AtomsWithConstructor { .. }
            | Self::ConstructionValues { .. }
            | Self::Constructions { .. }
            | Self::Map { .. }
            | Self::Filter { .. } => None,
        }
    }

    /// Converts the lazy object into an evaluated object, materializing a
    /// deferred composite by iterating it.
    #[must_use]
    pub fn into_object(self, knowledge: &Knowledge) -> Object {
        match self {
            Self::Eager(object) => object,
            Self::Node(node) => node.into_object(),
            Self::AtomProperties(properties) => atom_properties_into_object(properties),
            lazy_composite => {
                let mut values = lazy_composite
                    .into_composite_iter(knowledge)
                    .collect::<Vec<_>>();

                Composite::from(values.as_mut_slice()).into()
            }
        }
    }

    #[must_use]
    pub fn size(&self, knowledge: &Knowledge) -> u64 {
        match self {
            Self::Node(node) => node.composite_size(),
            Self::Eager(object) => knowledge.size_of_object(object),
            Self::AtomProperties(properties) => properties.size(),
            Self::Union { .. }
            | Self::Intersection { .. }
            | Self::Difference { .. }
            | Self::Map { .. }
            | Self::Filter { .. } => {
                // Deferred composites may yield an element more than once, so the
                // values are deduplicated while counting.
                self.clone().into_composite_iter(knowledge).count() as u64
            }
            Self::AtomsWithProperty { property } => knowledge
                .query_atoms_with_property(property.clone())
                .count() as u64,
            Self::AtomsWithConstructor { constructor } => knowledge
                .query_atoms_with_constructor(constructor.clone())
                .count() as u64,
            Self::ConstructionValues { .. } | Self::Constructions { .. } => {
                self.iter(knowledge).count() as u64
            }
        }
    }

    /// Returns `true` iff `self` has at least one element; `false` otherwise.
    #[must_use]
    pub fn has_at_least_one_element(&self, knowledge: &Knowledge) -> bool {
        match self {
            Self::Node(node) => node.composite_size() != 0,
            Self::Eager(object) => knowledge.size_of_object(object) != 0,
            Self::AtomProperties(properties) => !properties.is_empty(),
            Self::Union { first, second } => {
                first.has_at_least_one_element(knowledge)
                    || second.has_at_least_one_element(knowledge)
            }
            Self::Map { target, .. } => target.has_at_least_one_element(knowledge),
            Self::AtomsWithProperty { property } => knowledge
                .query_atoms_with_property(property.clone())
                .next()
                .is_some(),
            Self::AtomsWithConstructor { constructor } => knowledge
                .query_atoms_with_constructor(constructor.clone())
                .next()
                .is_some(),
            Self::Intersection { .. }
            | Self::Difference { .. }
            | Self::ConstructionValues { .. }
            | Self::Constructions { .. }
            | Self::Filter { .. } => self.iter(knowledge).next().is_some(),
        }
    }

    #[must_use]
    pub fn truthy(&self) -> MaybeUndefined<bool> {
        match self {
            LazyObject::Eager(object) => object.truthy().into(),
            _ => MaybeUndefined::Undefined,
        }
    }

    #[must_use]
    pub fn has(&self, element: &Object, knowledge: &Knowledge) -> bool {
        match self {
            Self::Node(node) => knowledge.object_has(&node.into_object(), element),
            Self::Eager(object) => knowledge.object_has(object, element),
            Self::AtomProperties(properties) => properties.has(element),
            Self::Union { first, second } => {
                first.has(element, knowledge) || second.has(element, knowledge)
            }
            Self::Difference { source, without } => {
                source.has(element, knowledge) && !without.has(element, knowledge)
            }
            Self::Intersection { first, second } => {
                first.has(element, knowledge) && second.has(element, knowledge)
            }
            Self::AtomsWithProperty { property } => {
                if let Object::Atom(_) = element {
                    knowledge.object_has(element, property)
                } else {
                    false
                }
            }
            Self::ConstructionValues {
                target,
                property_constructor,
            } => target.has(
                &Construction::new(property_constructor.clone(), element.clone()).into(),
                knowledge,
            ),
            Self::Filter {
                target,
                filter_function,
            } => {
                // `element` is member of the composite which all elements satisfy the constraint
                // iff it is in that composite and matches the constraint.

                if !target.has(element, knowledge) {
                    return false;
                }

                compute::evaluate(
                    &Node::Call {
                        callee: filter_function.clone().into(),
                        with: Node::Literal(element.clone()).into(),
                    },
                    knowledge,
                )
                .truthy()
                    == MaybeUndefined::Defined(true)
            }
            LazyObject::AtomsWithConstructor { .. }
            | LazyObject::Map { .. }
            | LazyObject::Constructions { .. } => {
                self.iter(knowledge).any(|object| element == &object)
            }
        }
    }

    /// Returns `true` iff `self` is a composite that is computed lazily.
    ///
    /// Eagerly represented objects and nodes that can be converted into a model
    /// object without further computation are not lazy. Lazy composite results should
    /// be iterated with [`Self::into_composite_iter`] instead of being materialized
    /// into a [`Composite`].
    #[must_use]
    pub fn is_lazy(&self) -> bool {
        self.is_composite() && !matches!(self, Self::Eager(_) | Self::Node(_))
    }

    /// Iterate over the lazy object.
    ///
    /// The returned iterator owns a snapshot of everything it needs, so it
    /// stays valid if the knowledge changes after this call. Use
    /// [`Self::into_iter`] to avoid cloning the lazy object.
    #[must_use]
    pub fn iter(&self, knowledge: &Knowledge) -> LazyObjectIter {
        match self {
            Self::Eager(object) => {
                let properties = knowledge
                    .properties_of(object)
                    .unwrap_or_else(ObjectProperties::empty);

                LazyObjectIter::ObjectProperties(properties)
            }
            Self::Node(node) => {
                let object = node.into_object();
                let properties = knowledge
                    .properties_of(&object)
                    .unwrap_or_else(ObjectProperties::empty);

                LazyObjectIter::ObjectProperties(properties)
            }
            lazy_object => lazy_object.clone().into_iter(knowledge),
        }
    }

    /// Consumes the lazy object and returns an iterator over its values.
    ///
    /// The iterator captures the parts of the knowledge it needs. Changing the
    /// knowledge after this call therefore does not affect the iteration.
    #[must_use]
    pub fn into_iter(self, knowledge: &Knowledge) -> LazyObjectIter {
        match self {
            Self::Node(node) => {
                let object = node.into_object();
                let properties = knowledge
                    .properties_of(&object)
                    .unwrap_or_else(ObjectProperties::empty);

                LazyObjectIter::ObjectProperties(properties)
            }
            Self::Eager(object) => {
                let properties = knowledge
                    .properties_of(&object)
                    .unwrap_or_else(ObjectProperties::empty);

                LazyObjectIter::ObjectProperties(properties)
            }
            Self::AtomProperties(properties) => {
                LazyObjectIter::ObjectProperties(ObjectProperties::properties(properties))
            }
            Self::Union { first, second } => LazyObjectIter::Deduplicated {
                values: Box::new(LazyObjectIter::Union {
                    first: Box::new(first.into_iter(knowledge)),
                    second: Box::new(second.into_iter(knowledge)),
                }),
                yielded: BTreeSet::new(),
            },
            Self::Intersection { first, second } => LazyObjectIter::Deduplicated {
                values: Box::new(LazyObjectIter::Intersection {
                    first: Box::new(first.into_iter(knowledge)),
                    second,
                    knowledge: knowledge.clone(),
                }),
                yielded: BTreeSet::new(),
            },
            Self::Difference { source, without } => LazyObjectIter::Deduplicated {
                values: Box::new(LazyObjectIter::Difference {
                    source: Box::new(source.into_iter(knowledge)),
                    without,
                    knowledge: knowledge.clone(),
                }),
                yielded: BTreeSet::new(),
            },
            Self::AtomsWithProperty { property } => {
                LazyObjectIter::AtomsWithProperty(knowledge.query_atoms_with_property(property))
            }
            Self::AtomsWithConstructor { constructor } => LazyObjectIter::AtomsWithConstructor(
                knowledge.query_atoms_with_constructor(constructor),
            ),
            Self::ConstructionValues {
                target,
                property_constructor,
            } => LazyObjectIter::ConstructionValues {
                target: Box::new(target.into_iter(knowledge)),
                property_constructor,
            },
            Self::Constructions { constructor } => {
                LazyObjectIter::Constructions(knowledge.query_constructions(constructor))
            }
            Self::Map {
                target,
                mapper_function,
            } => LazyObjectIter::Deduplicated {
                values: Box::new(LazyObjectIter::Map {
                    target: Box::new(target.into_iter(knowledge)),
                    mapper_function,
                    knowledge: knowledge.clone(),
                }),
                yielded: BTreeSet::new(),
            },
            Self::Filter {
                target,
                filter_function,
            } => LazyObjectIter::Filter {
                target: Box::new(target.into_iter(knowledge)),
                filter_function,
                knowledge: knowledge.clone(),
            },
        }
    }

    /// Consumes the lazy object and returns an iterator over its values that
    /// respects composite semantics, i.e. yields every value at most once.
    ///
    /// Like [`Self::into_iter`], the returned iterator captures the parts of the
    /// knowledge it needs.
    #[must_use]
    pub fn into_composite_iter(self, knowledge: &Knowledge) -> LazyCompositeIterator {
        LazyCompositeIterator::new(self.into_iter(knowledge))
    }
}

#[must_use]
fn atom_properties_into_object(properties: Properties) -> Object {
    let mut properties = properties.into_iter().collect::<Vec<_>>();

    Composite::from(properties.as_mut_slice()).into()
}

#[derive(Clone)]
pub enum LazyObjectIter {
    ObjectProperties(ObjectProperties),
    AtomsWithProperty(QueryAtomsWithProperty),
    AtomsWithConstructor(QueryAtomsWithConstructor),
    ConstructionValues {
        target: Box<Self>,
        property_constructor: Object,
    },
    Constructions(QueryConstructions),
    Union {
        first: Box<Self>,
        second: Box<Self>,
    },
    /// Yields the values of `values` that have not been yielded before.
    Deduplicated {
        values: Box<Self>,
        yielded: BTreeSet<Object>,
    },
    /// Yields the values of `first` that are members of `second`.
    Intersection {
        first: Box<Self>,
        second: Box<LazyObject>,
        knowledge: Knowledge,
    },
    /// Yields the values of `source` that are not members of `without`.
    Difference {
        source: Box<Self>,
        without: Box<LazyObject>,
        knowledge: Knowledge,
    },
    Filter {
        target: Box<Self>,
        filter_function: Node,
        knowledge: Knowledge,
    },
    Map {
        target: Box<Self>,
        mapper_function: Node,
        knowledge: Knowledge,
    },
}

impl std::fmt::Debug for LazyObjectIter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.clone()).finish()
    }
}

impl Iterator for LazyObjectIter {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::ObjectProperties(properties) => properties.next(),
            Self::AtomsWithProperty(atoms) => atoms.next().map(Object::Atom),
            Self::AtomsWithConstructor(atoms) => atoms.next().map(Object::Atom),
            Self::ConstructionValues {
                target,
                property_constructor,
            } => target.find_map(|object| match object {
                Object::Construction(Construction { constructor, value })
                    if constructor.as_ref() == property_constructor =>
                {
                    Some(*value)
                }
                _ => None,
            }),
            Self::Constructions(pairs) => pairs.next().map(AtomValuePair::into_object),
            Self::Union { first, second } => first.next().or_else(|| second.next()),
            Self::Deduplicated { values, yielded } => loop {
                let object = values.next()?;

                if yielded.insert(object.clone()) {
                    return Some(object);
                }
            },
            Self::Intersection {
                first,
                second,
                knowledge,
            } => first.find(|object| second.has(object, knowledge)),
            Self::Difference {
                source,
                without,
                knowledge,
            } => source.find(|object| !without.has(object, knowledge)),
            Self::Filter {
                target,
                filter_function,
                knowledge,
            } => target.find(|object| {
                compute::evaluate(
                    &Node::Call {
                        callee: filter_function.clone().into(),
                        with: Node::Literal(object.clone()).into(),
                    },
                    knowledge,
                )
                .truthy()
                    == MaybeUndefined::Defined(true)
            }),
            Self::Map {
                target,
                mapper_function,
                knowledge,
            } => target.next().map(|object| {
                compute::evaluate(
                    &Node::Call {
                        callee: mapper_function.clone().into(),
                        with: Node::Literal(object).into(),
                    },
                    knowledge,
                )
                .into_object(knowledge)
            }),
        }
    }
}

/// Iterates over the values of a lazy composite, yielding every value at most once.
///
/// The values seen so far are remembered, so this does not materialize the composite
/// in advance but uses memory proportional to the number of yielded values.
pub struct LazyCompositeIterator {
    values: LazyObjectIter,
    yielded: BTreeSet<Object>,
}

impl LazyCompositeIterator {
    #[must_use]
    pub fn new(values: LazyObjectIter) -> Self {
        Self {
            values,
            yielded: BTreeSet::new(),
        }
    }
}

impl Iterator for LazyCompositeIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let object = self.values.next()?;

            if self.yielded.insert(object.clone()) {
                return Some(object);
            }
        }
    }
}
