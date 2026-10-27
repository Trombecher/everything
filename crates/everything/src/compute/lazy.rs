use crate::{
    compute::{self, InvalidNodeError, MaybeUndefined, Node},
    knowledge::Knowledge,
    model::{Atom, Object, SetApi, SetValues},
};

#[derive(Clone, Debug)]
pub enum LazyObject {
    /// Just an object.
    Eager(Object),

    /// A node.
    Node(Node),

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
    #[must_use]
    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Eager(Object::Atom(Atom::UNDEFINED)))
    }

    /// Tries to convert `self` into a node.
    ///
    /// # Errors
    ///
    /// If `self` is an invalid node.
    pub fn into_node(self, knowledge: &Knowledge) -> Result<Node, InvalidNodeError> {
        match self {
            Self::Node(node) => Ok(node),
            object => Node::new(&object.into_object(), knowledge),
        }
    }

    #[must_use]
    pub fn into_object(self) -> Object {
        match self {
            Self::Eager(object) => object,
            object => todo!("{object:?} from lazy"),
        }
    }

    #[must_use]
    pub fn size(&self, knowledge: &Knowledge) -> u64 {
        match self {
            Self::Node(node) => node.set_size(),
            Self::Eager(object) => knowledge.size_of_object(object),
            Self::Union { .. } => {
                // We have to deduplicate here,
                // maybe best to just iterate (?)

                todo!("size of union")
            }
            Self::Intersection { .. } => {
                // Here too.

                todo!("size of intersection")
            }
            Self::Difference { .. } => {
                // Here too.

                todo!("size of difference")
            }
            Self::AtomsWithProperty { property } => knowledge
                .query_atoms_with_property(property.clone())
                .count() as u64,
            Self::Map { .. } => {
                // Here too.

                todo!("size of map")
            }
            Self::Filter { .. } => {
                todo!("size of filter")
            }
        }
    }

    /// Returns `true` iff `self` has at least one element; `false` otherwise.
    #[must_use]
    pub fn has_at_least_one_element(&self, knowledge: &Knowledge) -> bool {
        match self {
            Self::Node(_) => todo!(),
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
            Self::Node(_) => todo!(),
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
            LazyObject::Map { .. } => self.iter(knowledge).any(|object| element == &object),
        }
    }

    /// Iterate over the lazy object.
    #[must_use]
    pub fn iter(&self, knowledge: &Knowledge) -> LazyObjectIter {
        match self {
            Self::Node(_) => todo!("iter node"),
            Self::Eager(object) => {
                LazyObjectIter::ObjectValues(knowledge.values_of(object).into_iter())
            }
            Self::Union { first, second } => LazyObjectIter::Union {
                first: Box::new(first.iter(knowledge)),
                second: Box::new(second.iter(knowledge)),
            },
            Self::Intersection {
                first: _,
                second: _,
            } => todo!(),
            Self::Difference {
                source: _,
                without: _,
            } => todo!(),
            Self::AtomsWithProperty { property: _ } => todo!(),
            Self::Map {
                target,
                mapper_function,
            } => LazyObjectIter::Map {
                target: Box::new(target.iter(knowledge)),
                mapper_function: mapper_function.clone(),
                knowledge: knowledge.clone(),
            },
            Self::Filter {
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
        filter_function: Node,
        knowledge: Knowledge,
    },
    Map {
        target: Box<Self>,
        mapper_function: Node,
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
                .into_object()
            }),
        }
    }
}
