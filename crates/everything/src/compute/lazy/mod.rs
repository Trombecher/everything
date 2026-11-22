#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use crate::{
    compute::{self, MaybeUndefined, Node},
    knowledge::{
        AtomValuePair, Knowledge, ObjectValues, QueryAtomsWithConstructor, QueryAtomsWithProperty,
        QueryConstructions, Values,
    },
    model::{Construction, Object, Set},
};

#[derive(Clone, Debug)]
pub enum LazyObject {
    /// Just an object.
    Eager(Object),

    /// A node.
    Node(Node),

    /// The set of values of an atom in the knowledge.
    ///
    /// The values are kept in their knowledge container, so iterating
    /// them does not require materialising a model [`Set`].
    AtomValues(Values),

    /// Represents the set that is the union of two [`LazyObject`]s.
    Union { first: Box<Self>, second: Box<Self> },

    /// Represents the set that is the intersection of two [`LazyObject`]s.
    Intersection { first: Box<Self>, second: Box<Self> },

    /// Represents the set that is the difference of two [`LazyObject`]s.
    Difference {
        source: Box<Self>,
        without: Box<Self>,
    },

    /// The set of atoms with the given property.
    AtomsWithProperty { property: Object },

    /// The set of atoms that have a property that is a construction with the
    /// given constructor.
    AtomsWithConstructor { constructor: Object },

    /// The values of constructions with the given constructor among the values
    /// of a given object.
    ///
    /// The target is iterated lazily and only constructions whose constructor
    /// equals `property_constructor` contribute their value. Atoms are resolved
    /// through the knowledge and sets through their members.
    ConstructionValues {
        target: Box<Self>,
        property_constructor: Object,
    },

    /// The pairs of an atom and a value such that the atom has a construction
    /// with the given constructor in the knowledge.
    ///
    /// Each pair is yielded as the object `{atom <atom>, value <value>}`.
    Constructions { constructor: Object },

    /// Maps every value of a given set with a mapper function.
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
    /// Returns `true` iff `self` is a set result: an evaluated set, atom
    /// values, a function value, or a deferred set operation.
    #[must_use]
    pub fn is_set(&self) -> bool {
        match self {
            Self::Eager(Object::Set(_))
            | Self::AtomValues(_)
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
            object => Node::new(&object.try_into_object()?, knowledge).ok(),
        }
    }

    /// Tries to convert `self` into an evaluated object, returning `None`
    /// for deferred values that cannot be materialised yet.
    #[must_use]
    pub fn try_into_object(self) -> Option<Object> {
        match self {
            Self::Eager(object) => Some(object),
            Self::Node(node) => Some(node.into_object()),
            Self::AtomValues(values) => Some(atom_values_into_object(values)),
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
    /// deferred set by iterating it.
    #[must_use]
    pub fn into_object(self, knowledge: &Knowledge) -> Object {
        match self {
            Self::Eager(object) => object,
            Self::Node(node) => node.into_object(),
            Self::AtomValues(values) => atom_values_into_object(values),
            lazy_set => {
                let mut values = lazy_set.into_set_iter(knowledge).collect::<Vec<_>>();

                Set::from(values.as_mut_slice()).into()
            }
        }
    }

    #[must_use]
    pub fn size(&self, knowledge: &Knowledge) -> u64 {
        match self {
            Self::Node(node) => node.set_size(),
            Self::Eager(object) => knowledge.size_of_object(object),
            Self::AtomValues(values) => values.size(),
            Self::Union { .. }
            | Self::Intersection { .. }
            | Self::Difference { .. }
            | Self::Map { .. }
            | Self::Filter { .. } => {
                // Deferred sets may yield an element more than once, so the
                // values are deduplicated while counting.
                self.clone().into_set_iter(knowledge).count() as u64
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
            Self::Node(node) => node.set_size() != 0,
            Self::Eager(object) => knowledge.size_of_object(object) != 0,
            Self::AtomValues(values) => !values.is_empty(),
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
            Self::AtomValues(values) => values.has(element),
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
                // `element` is member of the set which all elements satisfy the constraint
                // iff it is in that set and matches the constraint.

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

    /// Returns `true` iff `self` is a set that is computed lazily.
    ///
    /// Eagerly represented objects and nodes that can be converted into a model
    /// object without further computation are not lazy. Lazy set results should
    /// be iterated with [`Self::into_set_iter`] instead of being materialized
    /// into a [`Set`].
    #[must_use]
    pub fn is_lazy(&self) -> bool {
        self.is_set() && !matches!(self, Self::Eager(_) | Self::Node(_))
    }

    /// Iterate over the lazy object.
    ///
    /// The returned iterator owns a snapshot of everything it needs, so it
    /// stays valid if the knowledge changes after this call. Use
    /// [`Self::into_iter`] to avoid cloning the lazy object.
    #[must_use]
    pub fn iter(&self, knowledge: &Knowledge) -> LazyObjectIter {
        match self {
            Self::Eager(object) => LazyObjectIter::ObjectValues(knowledge.values_of(object)),
            Self::Node(node) => {
                let object = node.into_object();

                LazyObjectIter::ObjectValues(knowledge.values_of(&object))
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

                LazyObjectIter::ObjectValues(knowledge.values_of(&object))
            }
            Self::Eager(object) => LazyObjectIter::ObjectValues(knowledge.values_of(&object)),
            Self::AtomValues(values) => {
                LazyObjectIter::ObjectValues(ObjectValues::properties(values))
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
    /// respects set semantics, i.e. yields every value at most once.
    ///
    /// Like [`Self::into_iter`], the returned iterator captures the parts of the
    /// knowledge it needs.
    #[must_use]
    pub fn into_set_iter(self, knowledge: &Knowledge) -> LazySetIterator {
        LazySetIterator::new(self.into_iter(knowledge))
    }
}

#[must_use]
fn atom_values_into_object(values: Values) -> Object {
    let mut values = values.into_iter().collect::<Vec<_>>();

    Set::from(values.as_mut_slice()).into()
}

#[derive(Clone)]
pub enum LazyObjectIter {
    ObjectValues(ObjectValues),
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
            Self::ObjectValues(set_values) => set_values.next(),
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

/// Iterates over the values of a lazy set, yielding every value at most once.
///
/// The values seen so far are remembered, so this does not materialize the set
/// in advance but uses memory proportional to the number of yielded values.
pub struct LazySetIterator {
    values: LazyObjectIter,
    yielded: BTreeSet<Object>,
}

impl LazySetIterator {
    #[must_use]
    pub fn new(values: LazyObjectIter) -> Self {
        Self {
            values,
            yielded: BTreeSet::new(),
        }
    }
}

impl Iterator for LazySetIterator {
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
