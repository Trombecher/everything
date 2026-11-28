mod error;
mod properties;
mod query;
#[cfg(test)]
mod tests;

pub use error::*;
pub use properties::*;
pub use query::*;

use imbl::OrdMap;

use crate::{
    compute::{MaybeUndefined, Node, evaluate},
    model::{Atom, AtomOrComposite, CompositeApi, CompositeItems, Construction, Object},
};

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
    ///   of that composite.
    #[must_use]
    pub fn properties_of(&self, object: &AtomOrComposite) -> ObjectProperties {
        match object {
            AtomOrComposite::Atom(atom) => {
                ObjectProperties::properties(self.properties_of_atom(atom))
            }
            AtomOrComposite::Composite(composite) => ObjectProperties::members(composite.clone()),
        }
    }

    /// Returns `true` iff `object` has `property` as one of its properties.
    #[must_use]
    pub fn object_has_property(&self, object: &AtomOrComposite, property: &Object) -> bool {
        match object {
            AtomOrComposite::Atom(atom) => self
                .map
                .get(atom)
                .is_some_and(|properties| properties.has(property)),
            AtomOrComposite::Composite(composite) => composite.has(property),
        }
    }

    /// Returns `true` iff `item` is an item of `object`'s set view: iff
    /// `object` has the property `contains ·· item`.
    #[must_use]
    pub fn object_has_item(&self, object: &AtomOrComposite, item: &Object) -> bool {
        match object {
            AtomOrComposite::Atom(_) => self.object_has_property(
                object,
                &Construction::new(Atom::CONTAINS.into(), item.clone()).into(),
            ),
            AtomOrComposite::Composite(composite) => composite.has_item(item),
        }
    }

    /// Iterates over the items of `object`'s set view.
    ///
    /// Atoms contribute the values of their `contains` statements, composites
    /// the values of their `contains` members.
    #[must_use]
    pub fn items_of(&self, object: &AtomOrComposite) -> ObjectItems {
        match object {
            AtomOrComposite::Atom(atom) => ObjectItems::Atom(
                self.properties_of_atom(atom)
                    .into_iter()
                    .filter_map(item_of_property),
            ),
            AtomOrComposite::Composite(composite) => {
                ObjectItems::Composite(composite.clone().into_items())
            }
        }
    }

    /// The number of items of `object`'s set view.
    #[must_use]
    pub fn item_count_of(&self, object: &AtomOrComposite) -> u64 {
        match object {
            AtomOrComposite::Atom(_) => {
                u64::try_from(self.items_of(object).count()).unwrap_or(u64::MAX)
            }
            AtomOrComposite::Composite(composite) => composite.item_count(),
        }
    }

    /// Queries the values of the constructions of `target` with the given
    /// `property_constructor`.
    ///
    /// Returns `None` iff `target` is a construction, which has no properties.
    #[must_use]
    pub fn query_construction_values(
        &self,
        target: &Object,
        property_constructor: Object,
    ) -> Option<QueryConstructionValues> {
        let target = AtomOrComposite::try_from(target).ok()?;

        Some(QueryConstructionValues {
            properties: self.properties_of(&target),
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
    #[must_use]
    pub fn size_of_object(&self, object: &AtomOrComposite) -> u64 {
        match object {
            AtomOrComposite::Atom(atom) => self.map.get(atom).map_or(0, Properties::size),
            AtomOrComposite::Composite(composite) => composite.size(),
        }
    }

    /// Checks that every statement in this knowledge is valid.
    ///
    /// A statement `a is p` is valid iff `p` is a valid property of `a` in the
    /// complete knowledge.
    ///
    /// The constraints of `property_constructor` and
    /// `non_constructed_property` must be the ones from the [base
    /// knowledge](crate::base::BASE). This pins them to constraints that
    /// enforce that a subject has at most one declaration of each kind, which
    /// implements conditions 1 and 2 of knowledge validation.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidKnowledgeError::BaseConstraintNotPinned`] if a base
    /// constraint is not the one from the base knowledge, and
    /// [`InvalidKnowledgeError::InvalidStatement`] with the offending statement
    /// and the reason why its property is invalid if a statement is not valid.
    pub fn is_valid(&self) -> Result<(), InvalidKnowledgeError> {
        self.base_constraints_are_pinned()?;

        for (atom, properties) in &self.map {
            for property in properties.clone() {
                self.object_has_valid_property(&AtomOrComposite::Atom(*atom), &property)
                    .map_err(|source| InvalidKnowledgeError::InvalidStatement {
                        atom: *atom,
                        property,
                        source: Box::new(source),
                    })?;
            }
        }

        Ok(())
    }

    /// Checks that the constraints of the base atoms that occur in this
    /// knowledge are the ones from the base knowledge.
    ///
    /// A base atom that does not occur may omit its constraint: the empty
    /// knowledge is valid, and a non-empty knowledge cannot use these atoms
    /// without declaring their constraints.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidKnowledgeError::BaseConstraintNotPinned`] if an
    /// occurring base atom has a different constraint.
    fn base_constraints_are_pinned(&self) -> Result<(), InvalidKnowledgeError> {
        let base = &*crate::base::BASE;

        for atom in [Atom::PROPERTY_CONSTRUCTOR, Atom::NON_CONSTRUCTED_PROPERTY] {
            if !self.map.contains_key(&atom) {
                continue;
            }

            let object = Object::from(atom);

            if self.property_constructor_constraint(&object)
                != base.property_constructor_constraint(&object)
            {
                return Err(InvalidKnowledgeError::BaseConstraintNotPinned { atom });
            }
        }

        Ok(())
    }

    /// Checks that `object` is valid in this knowledge.
    ///
    /// Atoms are always valid. A construction is valid iff its constructor and
    /// value are valid objects and its constructor is a property constructor.
    /// A composite is valid iff every one of its properties is a valid property
    /// of the composite.
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidObjectError`] that names the offending part of the
    /// object and the reason why it is invalid.
    pub fn object_is_valid(&self, object: &Object) -> Result<(), InvalidObjectError> {
        match object {
            Object::Atom(_) => Ok(()),
            Object::Construction(construction) => {
                self.object_is_valid(&construction.constructor)
                    .map_err(|source| InvalidObjectError::InvalidConstructor {
                        construction: construction.clone(),
                        source: Box::new(source),
                    })?;

                self.object_is_valid(&construction.value)
                    .map_err(|source| InvalidObjectError::InvalidValue {
                        construction: construction.clone(),
                        source: Box::new(source),
                    })?;

                if self
                    .property_constructor_constraint(&construction.constructor)
                    .is_none()
                {
                    return Err(InvalidObjectError::NotAPropertyConstructor {
                        constructor: (*construction.constructor).clone(),
                    });
                }

                Ok(())
            }
            Object::Composite(composite) => {
                for property in composite.clone() {
                    self.object_has_valid_property(
                        &AtomOrComposite::Composite(composite.clone()),
                        &property,
                    )
                    .map_err(|source| {
                        InvalidObjectError::InvalidCompositeProperty {
                            composite: composite.clone(),
                            property,
                            source: Box::new(source),
                        }
                    })?;
                }

                Ok(())
            }
        }
    }

    /// Checks that `property` is a valid property of `object`.
    ///
    /// The property must be a valid object, must belong to the object, and the
    /// applicable constraint must evaluate to exactly `true` in this complete
    /// knowledge.
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidPropertyError`] that names the offending property
    /// and the reason why it is not a valid property of `object`.
    pub fn object_has_valid_property(
        &self,
        object: &AtomOrComposite,
        property: &Object,
    ) -> Result<(), InvalidPropertyError> {
        self.object_is_valid(property)
            .map_err(|source| InvalidPropertyError::InvalidObject {
                property: property.clone(),
                source: Box::new(source),
            })?;

        if !self.object_has_property(object, property) {
            return Err(InvalidPropertyError::DoesNotBelong {
                object: object.clone(),
                property: property.clone(),
            });
        }

        self.property_constraint_accepts(object, property)
    }

    /// Returns the unique constraint of a property constructor, if `object` is
    /// one.
    fn property_constructor_constraint(&self, object: &Object) -> Option<Object> {
        let object = AtomOrComposite::try_from(object).ok()?;

        self.unique_property_value(&object, &Atom::PROPERTY_CONSTRUCTOR.into())
    }

    /// Returns the only value of the property `constructor ·· v` of `object`, if
    /// there is exactly one such property.
    fn unique_property_value(
        &self,
        object: &AtomOrComposite,
        constructor: &Object,
    ) -> Option<Object> {
        let mut values = self
            .properties_of(object)
            .filter_map(|property| match property {
                Object::Construction(Construction {
                    constructor: property_constructor,
                    value,
                }) if property_constructor.as_ref() == constructor => Some(*value),
                _ => None,
            });

        let value = values.next()?;

        values.next().is_none().then_some(value)
    }

    /// Checks that the constraint applicable to the use of `property` on
    /// `object` evaluates to exactly `true`.
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidPropertyError`] if no constraint applies or if the
    /// applicable constraint does not evaluate to exactly `true`.
    fn property_constraint_accepts(
        &self,
        object: &AtomOrComposite,
        property: &Object,
    ) -> Result<(), InvalidPropertyError> {
        match property {
            Object::Construction(Construction { constructor, value }) => {
                let Some(constraint) = self.property_constructor_constraint(constructor) else {
                    return Err(InvalidPropertyError::InvalidObject {
                        property: property.clone(),
                        source: Box::new(InvalidObjectError::NotAPropertyConstructor {
                            constructor: (**constructor).clone(),
                        }),
                    });
                };

                if self.constraint_accepts_subject_and_value(&constraint, object, value) {
                    Ok(())
                } else {
                    Err(InvalidPropertyError::RejectedByConstraint {
                        object: object.clone(),
                        property: property.clone(),
                        constraint,
                    })
                }
            }
            Object::Atom(_) | Object::Composite(_) => {
                let non_constructed_property = AtomOrComposite::try_from(property)
                    .expect("an atom or composite has properties");

                let Some(constraint) = self.unique_property_value(
                    &non_constructed_property,
                    &Atom::NON_CONSTRUCTED_PROPERTY.into(),
                ) else {
                    return Err(
                        InvalidPropertyError::MissingNonConstructedPropertyConstraint {
                            property: non_constructed_property,
                        },
                    );
                };

                if self.constraint_accepts_subject(&constraint, object) {
                    Ok(())
                } else {
                    Err(InvalidPropertyError::RejectedByConstraint {
                        object: object.clone(),
                        property: property.clone(),
                        constraint,
                    })
                }
            }
        }
    }

    /// Calls a unary constraint with the subject passed as a literal.
    fn constraint_accepts_subject(&self, constraint: &Object, subject: &AtomOrComposite) -> bool {
        let call = Node::Call {
            callee: Node::Literal(constraint.clone()).into(),
            with: Node::Literal(subject.clone().into()).into(),
        };

        evaluate(&call, self).truthy() == MaybeUndefined::Defined(true)
    }

    /// Calls a curried binary constraint with the subject and value passed as
    /// literals.
    fn constraint_accepts_subject_and_value(
        &self,
        constraint: &Object,
        subject: &AtomOrComposite,
        value: &Object,
    ) -> bool {
        let callee = Node::Call {
            callee: Node::Literal(constraint.clone()).into(),
            with: Node::Literal(subject.clone().into()).into(),
        };
        let call = Node::Call {
            callee: callee.into(),
            with: Node::Literal(value.clone()).into(),
        };

        evaluate(&call, self).truthy() == MaybeUndefined::Defined(true)
    }
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
