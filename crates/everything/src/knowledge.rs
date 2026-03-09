use std::ops::Deref;

use crate::{
    Property, Error, Object, ObjectOrAny, StatementPattern, Structure,
    statements::Statement,
};

#[derive(Clone, Debug, Copy)]
pub struct Knowledge<'a>(UnvaliatedKnowledge<'a>);

impl<'a> Knowledge<'a> {
    pub fn new(statements: &'a mut [Statement]) -> Result<Self, Error> {
        Self::from_unvalidated(UnvaliatedKnowledge::new(statements))
    }

    #[must_use]
    pub fn from_unvalidated(uk: UnvaliatedKnowledge<'a>) -> Result<Self, Error> {
        uk.validate().map(|()| Self(uk))
    }
}

impl<'a> Deref for Knowledge<'a> {
    type Target = UnvaliatedKnowledge<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A set of statements which has not been checked
/// for correctness.
#[derive(Clone, Debug, Copy)]
pub struct UnvaliatedKnowledge<'a> {
    /// A sorted array of statements.
    statements: &'a [Statement],
}

impl<'a> UnvaliatedKnowledge<'a> {
    pub fn new(axioms: &'a mut [Statement]) -> Self {
        axioms.sort();
        Self { statements: axioms }
    }

    fn validate(self) -> Result<(), Error> {
        // Check constraints:
        for statement in self.statements {
            if !self.is_tag(statement.tag.clone()) {
                return Err(Error::CouldNotProveTheorem(StatementPattern {
                    target: ObjectOrAny::Object(statement.tag.clone()),
                    tag: ObjectOrAny::Object(Object::AXIOMATIC),
                    value: ObjectOrAny::Any,
                }));
            }

            if let TagKind::Computed = self.tag_kind(statement.tag.clone()) {
                // Computed tags cannot be used axiomatically.

                return Err(Error::CouldNotProveTheorem(statement.clone().into()));
            }
        }

        Ok(())
    }

    fn eval(self) -> Object {
        todo!()
    }

    /// Returns [TagKind::Axiom] if the tag is axiomatic;
    /// and [TagKind::Computed] if the tag is computed.
    ///
    /// Assumes that there exists a value such that (tag, @Tag, value) is true.
    #[must_use]
    #[inline]
    fn tag_kind(self, tag: Object) -> TagKind {
        if self.exists_axiomatic_target_tag(tag, Object::AXIOMATIC) {
            TagKind::Axiom
        } else {
            TagKind::Computed
        }
    }

    fn is_tag(self, object: Object) -> bool {
        self.exists_target_tag(object, Object::TAG)
    }

    #[inline(always)]
    #[must_use]
    pub fn exists_axiomatic(self, sp: StatementPattern) -> bool {
        match sp {
            StatementPattern {
                target: ObjectOrAny::Any,
                tag: ObjectOrAny::Any,
                value: ObjectOrAny::Any,
            } => true,
            StatementPattern {
                target: ObjectOrAny::Object(target),
                tag: ObjectOrAny::Any,
                value: ObjectOrAny::Any,
            } => self.exists_axiomatic_target(target),
            StatementPattern {
                target: ObjectOrAny::Any,
                tag: ObjectOrAny::Object(tag),
                value: ObjectOrAny::Any,
            } => self.exists_axiomatic_tag(tag),
            StatementPattern {
                target: ObjectOrAny::Any,
                tag: ObjectOrAny::Any,
                value: ObjectOrAny::Object(value),
            } => self.exists_axiomatic_value(value),
            StatementPattern {
                target: ObjectOrAny::Object(target),
                tag: ObjectOrAny::Object(tag),
                value: ObjectOrAny::Any,
            } => self.exists_axiomatic_target_tag(target, tag),
            StatementPattern {
                target: ObjectOrAny::Any,
                tag: ObjectOrAny::Object(tag),
                value: ObjectOrAny::Object(value),
            } => self.exists_axiomatic_tag_value(tag, value),
            StatementPattern {
                target: ObjectOrAny::Object(target),
                tag: ObjectOrAny::Any,
                value: ObjectOrAny::Object(value),
            } => self.exists_axiomatic_target_value(target, value),
            StatementPattern {
                target: ObjectOrAny::Object(target),
                tag: ObjectOrAny::Object(tag),
                value: ObjectOrAny::Object(value),
            } => self.exists_axiomatic_statement(&Statement { target, tag, value }),
        }
    }

    #[must_use]
    fn exists_axiomatic_statement(self, fact: &Statement) -> bool {
        if let Object::Structure(s) = fact.target.clone() {
            if s.properties()
                .binary_search(&Property {
                    tag: fact.tag.clone(),
                    value: fact.value.clone(),
                })
                .is_ok()
            {
                return true;
            }
        }

        self.statements.binary_search(fact).is_ok()
    }

    #[must_use]
    fn exists_axiomatic_target_tag(self, target: Object, tag: Object) -> bool {
        if let Object::Structure(s) = target.clone() {
            // Objects also have properties,
            // which we should check first.

            if s.properties()
                .binary_search_by(|property| property.tag.cmp(&tag))
                .is_ok()
            {
                return true;
            }
        }

        self.statements
            .binary_search_by_key(&(target.clone(), tag.clone()), |a| {
                (a.target.clone(), a.tag.clone())
            })
            .is_ok()
    }

    #[must_use]
    fn exists_axiomatic_target_value(self, target: Object, value: Object) -> bool {
        if let Object::Structure(s) = target.clone() {
            if s.properties()
                .binary_search_by(|property| property.value.cmp(&value))
                .is_ok()
            {
                return true;
            }
        }

        self.statements
            .binary_search_by_key(&(target.clone(), value.clone()), |a| {
                (a.target.clone(), a.value.clone())
            })
            .is_ok()
    }

    #[must_use]
    fn exists_axiomatic_tag_value(self, tag: Object, value: Object) -> bool {
        self.statements
            .binary_search_by(|a| {
                (a.tag.clone(), a.value.clone()).cmp(&(tag.clone(), value.clone()))
            })
            .is_ok()
    }

    #[must_use]
    fn exists_axiomatic_target(self, _target: Object) -> bool {
        todo!()
    }

    #[must_use]
    fn exists_axiomatic_tag(self, _tag: Object) -> bool {
        todo!()
    }

    #[must_use]
    fn exists_axiomatic_value(self, _value: Object) -> bool {
        todo!()
    }

    /// Determines if there exists a value such that
    /// `(target, tag, value)` is true.
    ///
    /// * If `tag` is an axiom, it does a binary search.
    /// * If `tag` is computed, it computes the result and checks
    /// if the resulting set has one or more elements.
    #[must_use]
    pub fn exists_target_tag(self, target: Object, tag: Object) -> bool {
        match self.tag_kind(tag.clone()) {
            TagKind::Axiom => self.exists_axiomatic_target_tag(target, tag),
            TagKind::Computed => self.compute(target, tag).properties().len() > 0,
        }
    }

    fn compute(self, target: Object, tag: Object) -> Structure {
        // FIXME: Debug
        if tag == Object::TAG {
            if self.exists_axiomatic_target_tag(target.clone(), Object::AXIOMATIC)
                ^ self.exists_axiomatic_target_tag(target, Object::COMPUTED)
            {
                // Return {(@Contains, {})}; "true"
                return Structure::new(&mut [Property {
                    tag: Object::CONTAINS,
                    value: Object::Structure(Structure::new(&mut [])),
                }]);
            } else {
                // Return {}; "false"
                return Structure::new(&mut []);
            }
        }

        todo!("compute")
    }
}

pub enum TagKind {
    Axiom,
    Computed,
}
