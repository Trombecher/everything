use std::collections::BTreeSet;

use crate::{
    compute::{self, EvaluationContext, MaybeUndefined, Node, evaluate},
    knowledge::Knowledge,
    model::{Object, Set, SetApi, SetValues},
};

#[derive(Clone, Debug)]
pub enum LazyObject {
    Eager(Object),

    /// Represents the set that is the union of two [`LazyObject`]s.
    Union {
        first: Box<Self>,
        second: Box<Self>,
    },

    /// Represents the set that is the intersection of two [`LazyObject`]s.
    Intersection {
        first: Box<Self>,
        second: Box<Self>,
    },

    /// Represents the set that is the difference of two [`LazyObject`]s.
    Difference {
        source: Box<Self>,
        without: Box<Self>,
    },

    /// The set of atoms with the given property.
    AtomsWithProperty {
        property: Object,
    },

    /// Maps every value of a given set with a mapper function.
    Map {
        target: Box<Self>,
        /// This function is captured.
        mapper_function: Object,
    },

    /// Retains all values for that the filter function returns a truthy value.
    Filter {
        target: Box<Self>,
        /// This function is captured.
        filter_function: Object,
    },
}

impl From<Object> for LazyObject {
    fn from(object: Object) -> Self {
        Self::Eager(object)
    }
}

impl LazyObject {
    #[must_use]
    pub fn into_object(self) -> Object {
        match self {
            Self::Eager(object) => object,
            _ => todo!("object from lazy"),
        }
    }

    /// Returns `true` iff `self` has at least one element; `false` otherwise.
    #[must_use]
    pub fn has_at_least_one_element(&self, knowledge: &Knowledge) -> bool {
        match self {
            Self::Eager(object) => knowledge
                .values_of_by_ref(object)
                .is_some_and(|set| set.size() != 0),
            Self::Union { first, second } => {
                first.has_at_least_one_element(knowledge)
                    || second.has_at_least_one_element(knowledge)
            }
            Self::Map { target, .. } => target.has_at_least_one_element(knowledge),
            Self::Intersection { .. }
            | Self::Difference { .. }
            | Self::AtomsWithProperty { .. }
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
            Self::Eager(object) => knowledge.object_has(object, element),
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
            Self::Filter {
                target,
                filter_function,
            } => {
                // `element` is member of the set which all elements satisfy the constraint
                // iff it is in that set and matches the constraint.

                target.has(element, knowledge)
                    && (compute::evaluate(
                        &Object::new_node(Node::Call {
                            callee: filter_function.clone(),
                            with: element.clone(),
                        }),
                        knowledge,
                        &mut EvaluationContext::default(),
                    )
                    .truthy()
                        == MaybeUndefined::Defined(true))
            }
            LazyObject::Map { .. } => self.iter(knowledge).any(|object| element == &object),
        }
    }

    /// Iterate over the lazy object.
    #[must_use]
    pub fn iter(&self, knowledge: &Knowledge) -> LazyObjectIter {
        match self {
            LazyObject::Eager(object) => {
                LazyObjectIter::ObjectValues(knowledge.values_of(object).into_iter())
            }
            LazyObject::Union { first, second } => LazyObjectIter::Union {
                first: Box::new(first.iter(knowledge)),
                second: Box::new(second.iter(knowledge)),
            },
            LazyObject::Intersection {
                first: _,
                second: _,
            } => todo!(),
            LazyObject::Difference {
                source: _,
                without: _,
            } => todo!(),
            LazyObject::AtomsWithProperty { property: _ } => todo!(),
            LazyObject::Map {
                target: _,
                mapper_function: _,
            } => todo!(),
            LazyObject::Filter {
                target,
                filter_function,
            } => LazyObjectIter::Filter {
                target: Box::new(target.iter(knowledge)),
                filter_function: filter_function.clone(),
                knowledge: knowledge.clone(),
            },
        }
    }
}

pub enum LazyObjectIter {
    ObjectValues(SetValues),
    Union {
        first: Box<Self>,
        second: Box<Self>,
    },
    Filter {
        target: Box<Self>,
        filter_function: Object,
        knowledge: Knowledge,
    },
}

impl Iterator for LazyObjectIter {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::ObjectValues(set_values) => set_values.next(),
            Self::Union { first, second } => first.next().or_else(|| second.next()),
            Self::Filter {
                target,
                filter_function,
                knowledge,
            } => target.find(|object| {
                evaluate(
                    &Object::new_node(Node::Call {
                        callee: filter_function.clone(),
                        with: object.clone(),
                    }),
                    knowledge,
                    &mut EvaluationContext::default(),
                )
                .truthy()
                    == MaybeUndefined::Defined(true)
            }),
        }
    }
}

/// A collected set.
pub enum CollectedSet {
    Set(Set),
    HashSet(BTreeSet<Object>),
}

impl CollectedSet {
    #[must_use]
    pub fn has(&self, element: &Object) -> bool {
        match self {
            CollectedSet::Set(set) => set.has(element),
            CollectedSet::HashSet(btree_set) => btree_set.contains(element),
        }
    }
}
