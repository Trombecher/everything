mod properties;
mod query;
#[cfg(test)]
mod tests;

pub use properties::*;
pub use query::*;

use imbl::OrdMap;

use crate::model::{Atom, CompositeApi, CompositeItems, Construction, Object};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement {
    pub atom: Atom,
    pub object: Object,
}

#[derive(Clone)]
pub struct Knowledge {
    pub map: OrdMap<Atom, Properties>,
}

impl Default for Knowledge {
    fn default() -> Self {
        Self::empty()
    }
}

impl Knowledge {
    #[must_use]
    pub fn known_atoms(self) -> KnownAtoms {
        KnownAtoms {
            map_iter: self.map.into_iter(),
        }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        match self.map.get_mut(&statement.atom) {
            Some(properties) => properties.insert(statement.object),
            None => {
                self.map
                    .insert(statement.atom, Properties::from([statement.object]));
            }
        }
    }

    pub fn remove_statement(&mut self, statement: &Statement) {
        let is_empty = match self.map.get_mut(&statement.atom) {
            Some(properties) => {
                properties.remove(&statement.object);
                properties.is_empty()
            }
            None => return,
        };

        if is_empty {
            self.map.remove(&statement.atom);
        }
    }

    #[must_use]
    pub fn empty() -> Self {
        Self {
            map: OrdMap::default(),
        }
    }

    #[must_use]
    pub fn query_atoms_with_property(&self, property: Object) -> QueryAtomsWithProperty {
        QueryAtomsWithProperty {
            atoms: self.map.clone().into_iter(),
            property,
        }
    }

    /// Returns the properties of the given atom in the knowledge.
    #[must_use]
    pub fn properties_of_atom(&self, atom: &Atom) -> Properties {
        self.map.get(atom).cloned().unwrap_or_default()
    }

    /// Returns an iterator over the properties of the given object.
    ///
    /// * If the given object is an atom, then the properties are all properties
    ///   of that atom;
    /// * if the given object is a composite, then the properties are the members
    ///   of that composite;
    /// * a construction has no properties, so `None` is returned.
    #[must_use]
    pub fn properties_of(&self, object: &Object) -> Option<ObjectProperties> {
        match object {
            Object::Atom(atom) => Some(ObjectProperties::properties(self.properties_of_atom(atom))),
            Object::Construction(_) => None,
            Object::Composite(composite) => Some(ObjectProperties::members(composite.clone())),
        }
    }

    /// Returns `true` iff `object` has `property` as one of its properties.
    ///
    /// Returns `None` iff `object` is a construction, which has no properties.
    #[must_use]
    pub fn object_has_property(&self, object: &Object, property: &Object) -> Option<bool> {
        match object {
            Object::Atom(atom) => Some(
                self.map
                    .get(atom)
                    .is_some_and(|properties| properties.has(property)),
            ),
            Object::Construction(_) => None,
            Object::Composite(composite) => Some(composite.has(property)),
        }
    }

    /// Returns `true` iff `item` is an item of `object`'s set view: iff
    /// `object` has the property `contains ·· item`.
    ///
    /// Returns `None` iff `object` is a construction, which is not a set.
    #[must_use]
    pub fn object_has_item(&self, object: &Object, item: &Object) -> Option<bool> {
        match object {
            Object::Atom(_) => self.object_has_property(
                object,
                &Construction::new(Atom::CONTAINS.into(), item.clone()).into(),
            ),
            Object::Composite(composite) => Some(composite.has_item(item)),
            Object::Construction(_) => None,
        }
    }

    /// Iterates over the items of `object`'s set view.
    ///
    /// Atoms contribute the values of their `contains` statements, composites
    /// the values of their `contains` members.
    ///
    /// Returns `None` iff `object` is a construction, which is not a set.
    #[must_use]
    pub fn items_of(&self, object: &Object) -> Option<ObjectItems> {
        match object {
            Object::Atom(atom) => Some(ObjectItems::Atom(
                self.properties_of_atom(atom)
                    .into_iter()
                    .filter_map(item_of_property),
            )),
            Object::Composite(composite) => {
                Some(ObjectItems::Composite(composite.clone().into_items()))
            }
            Object::Construction(_) => None,
        }
    }

    /// The number of items of `object`'s set view.
    ///
    /// Returns `None` iff `object` is a construction, which is not a set.
    #[must_use]
    pub fn item_count_of(&self, object: &Object) -> Option<u64> {
        match object {
            Object::Atom(_) => {
                Some(u64::try_from(self.items_of(object)?.count()).unwrap_or(u64::MAX))
            }
            Object::Composite(composite) => Some(composite.item_count()),
            Object::Construction(_) => None,
        }
    }

    /// Queries the values of the constructions of `target` with the given
    /// `property_constructor`.
    ///
    /// Returns `None` iff `target` has no properties, i.e. iff it is a
    /// construction.
    #[must_use]
    pub fn query_construction_values(
        &self,
        target: &Object,
        property_constructor: Object,
    ) -> Option<QueryConstructionValues> {
        Some(QueryConstructionValues {
            properties: self.properties_of(target)?,
            property_constructor,
        })
    }

    /// Queries the knowledge for all atoms that have a property that is a
    /// construction with the given constructor.
    #[must_use]
    pub fn query_atoms_with_constructor(&self, constructor: Object) -> QueryAtomsWithConstructor {
        QueryAtomsWithConstructor {
            atoms: self.map.clone().into_iter(),
            constructor,
        }
    }

    /// Queries the knowledge for all pairs of an atom and a value such that the
    /// atom has the construction `constructor : value`.
    #[must_use]
    pub fn query_constructions(&self, constructor: Object) -> QueryConstructions {
        QueryConstructions::new(self.map.clone().into_iter(), constructor)
    }

    /// Determines the size of the object in this knowledge, i.e.
    /// the number of properties it has.
    ///
    /// Returns `None` iff `object` is a construction, which has no properties.
    #[must_use]
    pub fn size_of_object(&self, object: &Object) -> Option<u64> {
        match object {
            Object::Atom(atom) => Some(self.map.get(atom).map_or(0, Properties::size)),
            Object::Construction(_) => None,
            Object::Composite(composite) => Some(composite.size()),
        }
    }

    /*
    fn object_is_property_in(&self, object: &Object, containing_composite: &Composite) -> bool {
        match object {
            Object::Atom(_) | Object::Composite(_) => {
                let f = self
                    .query_construction_values(object, Atom::PROPERTY.into())
                    .and_then(|mut values| values.next_and_last());

                if let Some(constraint_function) = f {
                } else {
                    false
                }
            }
            Object::Construction(construction) => todo!(),
        }
    }

    #[must_use]
    fn composite_is_valid(&self, composite: &Composite) -> bool {
        composite.properties().all(|property| self.object_is_valid())
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.map.iter().all(|(_, composite)| {
            composite.clone()
                .properties()
                .all(|property| self.object_is_property(&property))
        })
    }
     */
}

#[derive(Clone)]
pub struct KnownAtoms {
    map_iter: <OrdMap<Atom, Properties> as IntoIterator>::IntoIter,
}

impl Iterator for KnownAtoms {
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next().map(|(atom, _)| atom)
    }
}

/// Converts a property into the item it contributes to a set, if any.
fn item_of_property(property: Object) -> Option<Object> {
    match property {
        Object::Construction(Construction { constructor, value })
            if matches!(&*constructor, Object::Atom(Atom::CONTAINS)) =>
        {
            Some(*value)
        }
        _ => None,
    }
}

/// Iterates over the items of an object's set view.
#[derive(Clone)]
pub enum ObjectItems {
    Atom(std::iter::FilterMap<PropertiesIter, fn(Object) -> Option<Object>>),
    Composite(CompositeItems),
}

impl Iterator for ObjectItems {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Atom(items) => items.next(),
            Self::Composite(items) => items.next(),
        }
    }
}
