#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use crate::{
    compute::{self, MaybeUndefined, Node},
    knowledge::{
        AtomValuePair, Knowledge, ObjectItems, ObjectProperties, Properties, PropertiesIter,
        QueryAtomsWithConstructor, QueryAtomsWithProperty, QueryConstructions,
    },
    model::{Atom, Composite, Construction, Object},
};

/// A result of evaluation.
///
/// A deferred [`LazySet`] describes a set without iterating its items. Every
/// atom and every composite is a set in the open set model; its items are the
/// values of its `contains` properties.
#[derive(Clone, Debug)]
pub enum LazyObject {
    /// An evaluated object.
    Eager(Object),

    /// A function value that is kept as code.
    Node(Node),

    /// A deferred set item sequence.
    Set(LazySet),
}

/// A deferred sequence of set items.
///
/// Iteration yields items, never the `contains` constructions that encode them
/// in an object. Materialising the set wraps every item with `contains`.
#[derive(Clone, Debug)]
pub enum LazySet {
    /// The set whose items are the properties of an atom in the knowledge.
    AtomProperties(Properties),

    /// The set of atoms with the given property.
    AtomsWithProperty { property: Object },

    /// The set of atoms that have a property that is a construction with the
    /// given constructor.
    AtomsWithConstructor { constructor: Object },

    /// The values of constructions with the given constructor among the
    /// properties of a given object.
    ConstructionValues {
        target: Box<LazyObject>,
        property_constructor: Object,
    },

    /// The pairs of an atom and a value such that the atom has a construction
    /// with the given constructor in the knowledge.
    Constructions { constructor: Object },

    /// The union of the items of two sets.
    Union {
        first: Box<LazyObject>,
        second: Box<LazyObject>,
    },

    /// The items of the first set that are also items of the second set.
    Intersection {
        first: Box<LazyObject>,
        second: Box<LazyObject>,
    },

    /// The items of the source set that are not items of the other set.
    Difference {
        source: Box<LazyObject>,
        without: Box<LazyObject>,
    },

    /// The results of mapping a function over the items of a set.
    Map {
        target: Box<LazyObject>,
        /// This function is captured.
        mapper_function: Node,
    },

    /// The items of a set for which the filter function returns exactly `true`.
    Filter {
        target: Box<LazyObject>,
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
    fn from(node: Node) -> Self {
        Self::Node(node)
    }
}

impl From<LazySet> for LazyObject {
    fn from(set: LazySet) -> Self {
        Self::Set(set)
    }
}

impl LazyObject {
    /// Returns `true` iff `self` is a composite: an evaluated composite, a
    /// function value, or a deferred set.
    #[must_use]
    pub fn is_composite(&self) -> bool {
        matches!(
            self,
            Self::Eager(Object::Composite(_)) | Self::Node(_) | Self::Set(_)
        )
    }

    /// Returns `true` iff `self` is a set in the open set model.
    ///
    /// Every atom and every composite is a set; constructions are not.
    #[must_use]
    pub fn is_set(&self) -> bool {
        !matches!(self, Self::Eager(Object::Construction(_)))
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
    /// for deferred sets that cannot be materialised yet.
    #[must_use]
    pub fn try_into_object(self) -> Option<Object> {
        match self {
            Self::Eager(object) => Some(object),
            Self::Node(node) => Some(node.into_object()),
            Self::Set(_) => None,
        }
    }

    /// Converts the lazy object into an evaluated object, materializing a
    /// deferred set by iterating its items.
    #[must_use]
    pub fn into_object(self, knowledge: &Knowledge) -> Object {
        match self {
            Self::Eager(object) => object,
            Self::Node(node) => node.into_object(),
            Self::Set(set) => {
                let items = set.into_items(knowledge).collect::<Vec<_>>();

                Composite::from_set_items(&items).into()
            }
        }
    }

    /// The number of items of the set view of `self`.
    ///
    /// # Panics
    ///
    /// Panics if `self` is a construction, which is not a set. Check
    /// [`Self::is_set`] first.
    #[must_use]
    pub fn item_count(&self, knowledge: &Knowledge) -> u64 {
        match self {
            Self::Eager(object) => knowledge
                .item_count_of(object)
                .expect("a construction has no items"),
            Self::Node(node) => knowledge
                .item_count_of(&node.into_object())
                .expect("a construction has no items"),
            Self::Set(set) => set.item_count(knowledge),
        }
    }

    /// Returns `true` iff the set view of `self` has at least one item.
    ///
    /// # Panics
    ///
    /// Panics if `self` is a construction, which is not a set. Check
    /// [`Self::is_set`] first.
    #[must_use]
    pub fn has_at_least_one_item(&self, knowledge: &Knowledge) -> bool {
        match self {
            Self::Eager(object) => knowledge
                .items_of(object)
                .expect("a construction has no items")
                .next()
                .is_some(),
            Self::Node(node) => knowledge
                .items_of(&node.into_object())
                .expect("a construction has no items")
                .next()
                .is_some(),
            Self::Set(set) => set.has_at_least_one_item(knowledge),
        }
    }

    /// Returns `true` iff `item` is an item of the set view of `self`.
    ///
    /// # Panics
    ///
    /// Panics if `self` is a construction, which is not a set. Check
    /// [`Self::is_set`] first.
    #[must_use]
    pub fn has_item(&self, item: &Object, knowledge: &Knowledge) -> bool {
        match self {
            Self::Eager(object) => knowledge
                .object_has_item(object, item)
                .expect("a construction has no items"),
            Self::Node(node) => knowledge
                .object_has_item(&node.into_object(), item)
                .expect("a construction has no items"),
            Self::Set(set) => set.has_item(item, knowledge),
        }
    }

    /// Returns `true` iff `property` is a property of the object represented
    /// by `self`.
    ///
    /// Deferred sets are represented by composites whose members are
    /// `contains` constructions, so a property of such a set is a
    /// `contains ·· %item%` construction.
    ///
    /// # Panics
    ///
    /// Panics if `self` is a construction, which has no properties.
    #[must_use]
    pub fn has_property(&self, property: &Object, knowledge: &Knowledge) -> bool {
        match self {
            Self::Eager(object) => knowledge
                .object_has_property(object, property)
                .expect("a construction has no properties"),
            Self::Node(node) => knowledge
                .object_has_property(&node.into_object(), property)
                .expect("a construction has no properties"),
            Self::Set(set) => match property {
                Object::Construction(Construction { constructor, value })
                    if matches!(&**constructor, Object::Atom(Atom::CONTAINS)) =>
                {
                    set.has_item(value, knowledge)
                }
                _ => false,
            },
        }
    }

    #[must_use]
    pub fn truthy(&self) -> MaybeUndefined<bool> {
        match self {
            Self::Eager(object) => object.truthy().into(),
            _ => MaybeUndefined::Undefined,
        }
    }

    /// Returns `true` iff `self` is a set that is computed lazily.
    #[must_use]
    pub fn is_lazy(&self) -> bool {
        matches!(self, Self::Set(_))
    }

    /// Iterate over the items of the set view of `self`.
    ///
    /// The returned iterator owns a snapshot of everything it needs, so it
    /// stays valid if the knowledge changes after this call.
    ///
    /// # Panics
    ///
    /// Panics if `self` is a construction, which is not a set. Check
    /// [`Self::is_set`] first.
    #[must_use]
    pub fn iter(&self, knowledge: &Knowledge) -> LazySetItems {
        self.clone().into_items(knowledge)
    }

    /// Consumes the lazy object and returns an iterator over the items of its
    /// set view.
    ///
    /// # Panics
    ///
    /// Panics if `self` is a construction, which is not a set. Check
    /// [`Self::is_set`] first.
    #[must_use]
    pub fn into_items(self, knowledge: &Knowledge) -> LazySetItems {
        match self {
            Self::Eager(object) => LazySetItems::ObjectItems(
                knowledge
                    .items_of(&object)
                    .expect("a construction has no items"),
            ),
            Self::Node(node) => LazySetItems::ObjectItems(
                knowledge
                    .items_of(&node.into_object())
                    .expect("a construction has no items"),
            ),
            Self::Set(set) => set.into_items(knowledge),
        }
    }

    /// Consumes the lazy object and returns an iterator over its properties.
    ///
    /// For atoms these are the statements in the knowledge, for composites the
    /// members. A deferred set is a composite whose members are `contains`
    /// constructions.
    ///
    /// Returns `None` iff `self` is a construction: constructions have no
    /// properties, so asking for them is rejected instead of yielding an empty
    /// iterator.
    #[must_use]
    pub fn into_properties(self, knowledge: &Knowledge) -> Option<LazyObjectProperties> {
        match self {
            Self::Eager(object) => knowledge
                .properties_of(&object)
                .map(LazyObjectProperties::Object),
            Self::Node(node) => knowledge
                .properties_of(&node.into_object())
                .map(LazyObjectProperties::Object),
            Self::Set(set) => Some(LazyObjectProperties::ContainsItems(Box::new(
                set.into_items(knowledge),
            ))),
        }
    }
}

impl LazySet {
    /// The number of distinct items of this set.
    #[must_use]
    pub fn item_count(&self, knowledge: &Knowledge) -> u64 {
        match self {
            Self::AtomProperties(properties) => properties.size(),
            Self::AtomsWithProperty { property } => u64::try_from(
                knowledge
                    .query_atoms_with_property(property.clone())
                    .count(),
            )
            .unwrap_or(u64::MAX),
            Self::AtomsWithConstructor { constructor } => u64::try_from(
                knowledge
                    .query_atoms_with_constructor(constructor.clone())
                    .count(),
            )
            .unwrap_or(u64::MAX),
            _ => u64::try_from(self.clone().into_items(knowledge).count()).unwrap_or(u64::MAX),
        }
    }

    /// Returns `true` iff this set has at least one item.
    #[must_use]
    pub fn has_at_least_one_item(&self, knowledge: &Knowledge) -> bool {
        match self {
            Self::AtomProperties(properties) => !properties.is_empty(),
            Self::AtomsWithProperty { property } => knowledge
                .query_atoms_with_property(property.clone())
                .next()
                .is_some(),
            Self::AtomsWithConstructor { constructor } => knowledge
                .query_atoms_with_constructor(constructor.clone())
                .next()
                .is_some(),
            Self::Union { first, second } => {
                first.has_at_least_one_item(knowledge) || second.has_at_least_one_item(knowledge)
            }
            Self::Map { target, .. } => target.has_at_least_one_item(knowledge),
            _ => self.clone().into_items(knowledge).next().is_some(),
        }
    }

    /// Returns `true` iff `item` is an item of this set.
    #[must_use]
    pub fn has_item(&self, item: &Object, knowledge: &Knowledge) -> bool {
        match self {
            Self::AtomProperties(properties) => properties.has(item),
            Self::AtomsWithProperty { property } => {
                matches!(item, Object::Atom(_))
                    && knowledge.object_has_property(item, property) == Some(true)
            }
            Self::AtomsWithConstructor { .. } | Self::Constructions { .. } | Self::Map { .. } => {
                self.clone()
                    .into_items(knowledge)
                    .any(|object| item == &object)
            }
            Self::ConstructionValues {
                target,
                property_constructor,
            } => target.has_property(
                &Construction::new(property_constructor.clone(), item.clone()).into(),
                knowledge,
            ),
            Self::Union { first, second } => {
                first.has_item(item, knowledge) || second.has_item(item, knowledge)
            }
            Self::Intersection { first, second } => {
                first.has_item(item, knowledge) && second.has_item(item, knowledge)
            }
            Self::Difference { source, without } => {
                source.has_item(item, knowledge) && !without.has_item(item, knowledge)
            }
            Self::Filter {
                target,
                filter_function,
            } => {
                if !target.has_item(item, knowledge) {
                    return false;
                }

                compute::evaluate(
                    &Node::Call {
                        callee: filter_function.clone().into(),
                        with: Node::Literal(item.clone()).into(),
                    },
                    knowledge,
                )
                .truthy()
                    == MaybeUndefined::Defined(true)
            }
        }
    }

    /// Consumes the set and returns an iterator over its distinct items.
    ///
    /// # Panics
    ///
    /// Panics if a `ConstructionValues` set targets a construction. The
    /// evaluator rejects constructions before building that set, since asking
    /// for the properties of a construction is not allowed.
    #[must_use]
    pub fn into_items(self, knowledge: &Knowledge) -> LazySetItems {
        match self {
            Self::AtomProperties(properties) => {
                LazySetItems::AtomProperties(properties.into_iter())
            }
            Self::AtomsWithProperty { property } => {
                LazySetItems::AtomsWithProperty(knowledge.query_atoms_with_property(property))
            }
            Self::AtomsWithConstructor { constructor } => LazySetItems::AtomsWithConstructor(
                knowledge.query_atoms_with_constructor(constructor),
            ),
            Self::ConstructionValues {
                target,
                property_constructor,
            } => LazySetItems::ConstructionValues {
                properties: target.into_properties(knowledge).expect(
                    "constructions have no properties; querying them is rejected in the evaluator",
                ),
                property_constructor,
            },
            Self::Constructions { constructor } => {
                LazySetItems::Constructions(knowledge.query_constructions(constructor))
            }
            Self::Union { first, second } => LazySetItems::Deduplicated {
                items: Box::new(LazySetItems::Union {
                    first: Box::new(first.into_items(knowledge)),
                    second: Box::new(second.into_items(knowledge)),
                }),
                yielded: BTreeSet::new(),
            },
            Self::Intersection { first, second } => LazySetItems::Deduplicated {
                items: Box::new(LazySetItems::Intersection {
                    first: Box::new(first.into_items(knowledge)),
                    second,
                    knowledge: knowledge.clone(),
                }),
                yielded: BTreeSet::new(),
            },
            Self::Difference { source, without } => LazySetItems::Deduplicated {
                items: Box::new(LazySetItems::Difference {
                    source: Box::new(source.into_items(knowledge)),
                    without,
                    knowledge: knowledge.clone(),
                }),
                yielded: BTreeSet::new(),
            },
            Self::Map {
                target,
                mapper_function,
            } => LazySetItems::Deduplicated {
                items: Box::new(LazySetItems::Map {
                    target: Box::new(target.into_items(knowledge)),
                    mapper_function,
                    knowledge: knowledge.clone(),
                }),
                yielded: BTreeSet::new(),
            },
            Self::Filter {
                target,
                filter_function,
            } => LazySetItems::Filter {
                target: Box::new(target.into_items(knowledge)),
                filter_function,
                knowledge: knowledge.clone(),
            },
        }
    }
}

/// Iterates over the properties of a lazy object.
#[derive(Clone)]
pub enum LazyObjectProperties {
    Object(ObjectProperties),
    ContainsItems(Box<LazySetItems>),
}

impl Iterator for LazyObjectProperties {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Object(properties) => properties.next(),
            Self::ContainsItems(items) => items
                .next()
                .map(|item| Construction::new(Atom::CONTAINS.into(), item).into()),
        }
    }
}

/// Iterates over the distinct items of a set.
#[derive(Clone)]
pub enum LazySetItems {
    ObjectItems(ObjectItems),
    AtomProperties(PropertiesIter),
    AtomsWithProperty(QueryAtomsWithProperty),
    AtomsWithConstructor(QueryAtomsWithConstructor),
    ConstructionValues {
        properties: LazyObjectProperties,
        property_constructor: Object,
    },
    Constructions(QueryConstructions),
    Union {
        first: Box<Self>,
        second: Box<Self>,
    },
    /// Yields the items of `items` that have not been yielded before.
    Deduplicated {
        items: Box<Self>,
        yielded: BTreeSet<Object>,
    },
    /// Yields the items of `first` that are items of `second`.
    Intersection {
        first: Box<Self>,
        second: Box<LazyObject>,
        knowledge: Knowledge,
    },
    /// Yields the items of `source` that are not items of `without`.
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

impl std::fmt::Debug for LazySetItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.clone()).finish()
    }
}

impl Iterator for LazySetItems {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::ObjectItems(items) => items.next(),
            Self::AtomProperties(properties) => properties.next(),
            Self::AtomsWithProperty(atoms) => atoms.next().map(Object::Atom),
            Self::AtomsWithConstructor(atoms) => atoms.next().map(Object::Atom),
            Self::ConstructionValues {
                properties,
                property_constructor,
            } => properties.find_map(|object| match object {
                Object::Construction(Construction { constructor, value })
                    if constructor.as_ref() == property_constructor =>
                {
                    Some(*value)
                }
                _ => None,
            }),
            Self::Constructions(pairs) => pairs.next().map(AtomValuePair::into_object),
            Self::Union { first, second } => first.next().or_else(|| second.next()),
            Self::Deduplicated { items, yielded } => loop {
                let object = items.next()?;

                if yielded.insert(object.clone()) {
                    return Some(object);
                }
            },
            Self::Intersection {
                first,
                second,
                knowledge,
            } => first.find(|object| second.has_item(object, knowledge)),
            Self::Difference {
                source,
                without,
                knowledge,
            } => source.find(|object| !without.has_item(object, knowledge)),
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
