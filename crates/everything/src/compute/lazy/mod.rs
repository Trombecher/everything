#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use crate::{
    compute::{self, MaybeUndefined, Node},
    knowledge::{
        AtomValuePair, Knowledge, ObjectItems, ObjectProperties, Properties, PropertiesIter,
        QueryAtomsWithConstructor, QueryAtomsWithProperty, QueryConstructions,
    },
    model::{Atom, AtomOrComposite, Composite, Construction, Object},
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

    /// The set whose items are the properties of the target set value.
    ///
    /// For an atom these are its statements in the knowledge, for a composite
    /// its members, and for a deferred set the `contains` constructions that
    /// encode its items.
    PropertiesOf { target: Box<SetValue> },

    /// The set of atoms with the given property.
    AtomsWithProperty { property: Object },

    /// The set of atoms that have a property that is a construction with the
    /// given constructor.
    AtomsWithConstructor { constructor: Object },

    /// The values of constructions with the given constructor among the
    /// properties of a given set value.
    ConstructionValues {
        target: Box<SetValue>,
        property_constructor: Object,
    },

    /// The pairs of an atom and a value such that the atom has a construction
    /// with the given constructor in the knowledge.
    Constructions { constructor: Object },

    /// The union of the items of two sets.
    Union {
        first: Box<SetValue>,
        second: Box<SetValue>,
    },

    /// The items of the first set that are also items of the second set.
    Intersection {
        first: Box<SetValue>,
        second: Box<SetValue>,
    },

    /// The items of the source set that are not items of the other set.
    Difference {
        source: Box<SetValue>,
        without: Box<SetValue>,
    },

    /// The results of mapping a function over the items of a set.
    Map {
        target: Box<SetValue>,
        /// This function is captured.
        mapper_function: Node,
    },

    /// The items of a set for which the filter function returns exactly `true`.
    Filter {
        target: Box<SetValue>,
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
        match self {
            Self::Eager(object) => !matches!(object, Object::Construction(_)),
            Self::Node(node) => AtomOrComposite::try_from(node.into_object()).is_ok(),
            Self::Set(_) => true,
        }
    }

    /// Consumes `self` and returns it as a [`SetValue`].
    ///
    /// Returns `None` iff `self` is a construction, which is not a set.
    #[must_use]
    pub fn into_set(self) -> Option<SetValue> {
        match self {
            Self::Eager(object) => AtomOrComposite::try_from(object).ok().map(SetValue::Eager),
            Self::Node(node) => AtomOrComposite::try_from(node.into_object())
                .ok()
                .map(SetValue::Eager),
            Self::Set(set) => Some(SetValue::Set(set)),
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
            Self::Set(set) => SetValue::Set(set).into_object(knowledge),
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
}

/// A value that is a set in the open set model.
///
/// Unlike [`LazyObject`], this excludes constructions, so all property and item
/// queries on it are total.
#[derive(Clone, Debug)]
pub enum SetValue {
    Eager(AtomOrComposite),
    Set(LazySet),
}

impl SetValue {
    /// The number of items of the set view of `self`.
    #[must_use]
    pub fn item_count(&self, knowledge: &Knowledge) -> u64 {
        match self {
            Self::Eager(object) => knowledge.item_count_of(object),
            Self::Set(set) => set.item_count(knowledge),
        }
    }

    /// Returns `true` iff the set view of `self` has at least one item.
    #[must_use]
    pub fn has_at_least_one_item(&self, knowledge: &Knowledge) -> bool {
        match self {
            Self::Eager(object) => knowledge.items_of(object).next().is_some(),
            Self::Set(set) => set.has_at_least_one_item(knowledge),
        }
    }

    /// Returns `true` iff `item` is an item of the set view of `self`.
    #[must_use]
    pub fn has_item(&self, item: &Object, knowledge: &Knowledge) -> bool {
        match self {
            Self::Eager(object) => knowledge.object_has_item(object, item),
            Self::Set(set) => set.has_item(item, knowledge),
        }
    }

    /// Returns `true` iff `property` is a property of the object represented
    /// by `self`.
    ///
    /// Deferred sets are represented by composites whose members are
    /// `contains` constructions, so a property of such a set is a
    /// `contains ·· %item%` construction.
    #[must_use]
    pub fn has_property(&self, property: &Object, knowledge: &Knowledge) -> bool {
        match self {
            Self::Eager(object) => knowledge.object_has_property(object, property),
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

    /// Iterates over the items of the set view of `self`.
    ///
    /// The returned iterator owns a snapshot of everything it needs, so it
    /// stays valid if the knowledge changes after this call.
    #[must_use]
    pub fn iter(&self, knowledge: &Knowledge) -> LazySetItems {
        self.clone().into_items(knowledge)
    }

    /// Consumes the set value and returns an iterator over the items of its
    /// set view.
    #[must_use]
    pub fn into_items(self, knowledge: &Knowledge) -> LazySetItems {
        match self {
            Self::Eager(object) => LazySetItems::ObjectItems(knowledge.items_of(&object)),
            Self::Set(set) => set.into_items(knowledge),
        }
    }

    /// Consumes the set value and returns an iterator over its properties.
    ///
    /// For atoms these are the statements in the knowledge, for composites the
    /// members. A deferred set is a composite whose members are `contains`
    /// constructions.
    #[must_use]
    pub fn into_properties(self, knowledge: &Knowledge) -> LazyObjectProperties {
        match self {
            Self::Eager(object) => LazyObjectProperties::Object(knowledge.properties_of(&object)),
            Self::Set(set) => {
                LazyObjectProperties::ContainsItems(Box::new(set.into_items(knowledge)))
            }
        }
    }

    /// Consumes the set value and materializes it as a canonical set composite.
    #[must_use]
    pub fn into_object(self, knowledge: &Knowledge) -> Object {
        match self {
            Self::Eager(object) => object.into(),
            Self::Set(set) => {
                let items = set.into_items(knowledge).collect::<Vec<_>>();

                Composite::from_set_items(&items).into()
            }
        }
    }
}

impl From<SetValue> for LazyObject {
    fn from(value: SetValue) -> Self {
        match value {
            SetValue::Eager(object) => Self::Eager(object.into()),
            SetValue::Set(set) => Self::Set(set),
        }
    }
}

impl LazySet {
    /// The number of distinct items of this set.
    #[must_use]
    pub fn item_count(&self, knowledge: &Knowledge) -> u64 {
        match self {
            Self::AtomProperties(properties) => properties.size(),
            Self::PropertiesOf { target } => match target.as_ref() {
                SetValue::Eager(object) => knowledge.size_of_object(object),
                SetValue::Set(set) => set.item_count(knowledge),
            },
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
            Self::PropertiesOf { target } => match target.as_ref() {
                SetValue::Eager(object) => knowledge.size_of_object(object) != 0,
                SetValue::Set(set) => set.has_at_least_one_item(knowledge),
            },
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
            Self::PropertiesOf { target } => target.has_property(item, knowledge),
            Self::AtomsWithProperty { property } => match item {
                Object::Atom(atom) => {
                    knowledge.object_has_property(&AtomOrComposite::Atom(*atom), property)
                }
                _ => false,
            },
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
    #[must_use]
    pub fn into_items(self, knowledge: &Knowledge) -> LazySetItems {
        match self {
            Self::AtomProperties(properties) => {
                LazySetItems::AtomProperties(properties.into_iter())
            }
            Self::PropertiesOf { target } => {
                LazySetItems::Properties(target.into_properties(knowledge))
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
                properties: target.into_properties(knowledge),
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
    Properties(LazyObjectProperties),
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
        second: Box<SetValue>,
        knowledge: Knowledge,
    },
    /// Yields the items of `source` that are not items of `without`.
    Difference {
        source: Box<Self>,
        without: Box<SetValue>,
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
            Self::Properties(properties) => properties.next(),
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
