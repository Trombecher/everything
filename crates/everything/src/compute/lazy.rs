use crate::{
    compute::{self, MaybeUndefined, Node},
    knowledge::{Knowledge, ObjectValues, QueryAtomsWithProperty, Values},
    model::{Atom, Construction, Object, Set},
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
            | Self::ConstructionValues { .. }
            | Self::Map { .. }
            | Self::Filter { .. } => true,
            Self::Eager(Object::Atom(_) | Object::Construction(_)) => false,
        }
    }

    /// Tries to convert `self` into a node, returning `None` if the value
    /// cannot be materialised as an object.
    #[must_use]
    pub fn try_into_node(self, knowledge: &Knowledge) -> Option<Node> {
        if self.is_undefined() {
            return None;
        }

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
            | Self::ConstructionValues { .. }
            | Self::Map { .. }
            | Self::Filter { .. } => None,
        }
    }

    #[must_use]
    pub fn into_object(self) -> Object {
        match self {
            Self::Eager(object) => object,
            Self::Node(node) => node.into_object(),
            Self::AtomValues(values) => atom_values_into_object(values),
            object => todo!("{object:?} from lazy"),
        }
    }

    #[must_use]
    pub fn size(&self, knowledge: &Knowledge) -> u64 {
        match self {
            Self::Node(node) => node.set_size(),
            Self::Eager(object) => knowledge.size_of_object(object),
            Self::AtomValues(values) => values.size(),
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
            Self::ConstructionValues { .. } => self.iter(knowledge).count() as u64,
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
            Self::Intersection { .. }
            | Self::Difference { .. }
            | Self::ConstructionValues { .. }
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
            LazyObject::Map { .. } => self.iter(knowledge).any(|object| element == &object),
        }
    }

    /// Iterate over the lazy object.
    #[must_use]
    pub fn iter(&self, knowledge: &Knowledge) -> LazyObjectIter {
        match self {
            Self::Node(node) => {
                let object = node.into_object();

                LazyObjectIter::ObjectValues(knowledge.values_of(&object))
            }
            Self::Eager(object) => LazyObjectIter::ObjectValues(knowledge.values_of(object)),
            Self::AtomValues(values) => {
                LazyObjectIter::ObjectValues(ObjectValues::properties(values.clone()))
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
            Self::AtomsWithProperty { property } => LazyObjectIter::AtomsWithProperty(
                knowledge.query_atoms_with_property(property.clone()),
            ),
            Self::ConstructionValues {
                target,
                property_constructor,
            } => LazyObjectIter::ConstructionValues {
                target: Box::new(target.iter(knowledge)),
                property_constructor: property_constructor.clone(),
            },
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

#[must_use]
fn atom_values_into_object(values: Values) -> Object {
    let mut values = values.into_iter().collect::<Vec<_>>();

    Set::from(values.as_mut_slice()).into()
}

pub enum LazyObjectIter {
    ObjectValues(ObjectValues),
    AtomsWithProperty(QueryAtomsWithProperty),
    ConstructionValues {
        target: Box<Self>,
        property_constructor: Object,
    },
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
            Self::AtomsWithProperty(atoms) => atoms.next().map(Object::Atom),
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
