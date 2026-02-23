use crate::{AxiomaticProperty, Error, FactPattern, Object, ObjectOrAny, Structure, axioms::Fact};

#[derive(Clone, Debug, Copy)]
pub struct Knowledge<'a>(UncheckedKnowlege<'a>);

impl<'a> Knowledge<'a> {
    #[must_use]
    pub fn new(uk: UncheckedKnowlege<'a>) -> Result<Self, Error> {
        uk.check().map(|()| Self(uk))
    }
}

#[derive(Clone, Debug, Copy)]
pub struct UncheckedKnowlege<'a> {
    /// A sorted array of facts.
    axioms: &'a [Fact],
}

impl<'a> UncheckedKnowlege<'a> {
    pub fn new(axioms: &'a mut [Fact]) -> Self {
        axioms.sort();
        Self { axioms }
    }

    fn check(self) -> Result<(), Error> {
        // Check constraints:
        for fact in self.axioms {
            if !self.is_tag(fact.tag.clone()) {
                return Err(Error::CouldNotProveTheorem(FactPattern {
                    target: ObjectOrAny::Object(fact.tag.clone()),
                    tag: ObjectOrAny::Object(Object::AXIOMATIC),
                    value: ObjectOrAny::Any,
                }));
            }

            if let TagKind::Computed = self.tag_kind(fact.tag.clone()) {
                // Computed tags cannot be used axiomatically.

                return Err(Error::CouldNotProveTheorem(fact.clone().into()));
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
        self.exists_axiomatic_target_tag(object, Object::TAG)
    }

    #[inline(always)]
    #[must_use]
    pub fn exists_axiomatic(self, axiom_pattern: FactPattern) -> bool {
        match axiom_pattern {
            FactPattern {
                target: ObjectOrAny::Any,
                tag: ObjectOrAny::Any,
                value: ObjectOrAny::Any,
            } => true,
            FactPattern {
                target: ObjectOrAny::Object(target),
                tag: ObjectOrAny::Any,
                value: ObjectOrAny::Any,
            } => self.exists_axiomatic_target(target),
            FactPattern {
                target: ObjectOrAny::Any,
                tag: ObjectOrAny::Object(tag),
                value: ObjectOrAny::Any,
            } => self.exists_axiomatic_tag(tag),
            FactPattern {
                target: ObjectOrAny::Any,
                tag: ObjectOrAny::Any,
                value: ObjectOrAny::Object(value),
            } => self.exists_axiomatic_value(value),
            FactPattern {
                target: ObjectOrAny::Object(target),
                tag: ObjectOrAny::Object(tag),
                value: ObjectOrAny::Any,
            } => self.exists_axiomatic_target_tag(target, tag),
            FactPattern {
                target: ObjectOrAny::Any,
                tag: ObjectOrAny::Object(tag),
                value: ObjectOrAny::Object(value),
            } => self.exists_axiomatic_tag_value(tag, value),
            FactPattern {
                target: ObjectOrAny::Object(target),
                tag: ObjectOrAny::Any,
                value: ObjectOrAny::Object(value),
            } => self.exists_axiomatic_target_value(target, value),
            FactPattern {
                target: ObjectOrAny::Object(target),
                tag: ObjectOrAny::Object(tag),
                value: ObjectOrAny::Object(value),
            } => self.exists_axiomatic_fact(&Fact { target, tag, value }),
        }
    }

    #[must_use]
    fn exists_axiomatic_fact(self, fact: &Fact) -> bool {
        if let Object::Structure(s) = fact.target.clone() {
            if s.properties()
                .binary_search(&AxiomaticProperty {
                    tag: fact.tag.clone(),
                    value: fact.value.clone(),
                })
                .is_ok()
            {
                return true;
            }
        }

        self.axioms.binary_search(fact).is_ok()
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

        self.axioms
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

        self.axioms
            .binary_search_by_key(&(target.clone(), value.clone()), |a| {
                (a.target.clone(), a.value.clone())
            })
            .is_ok()
    }

    #[must_use]
    fn exists_axiomatic_tag_value(self, tag: Object, value: Object) -> bool {
        self.axioms
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
    fn exists_target_tag(self, target: Object, tag: Object) -> bool {
        match self.tag_kind(tag.clone()) {
            TagKind::Axiom => self.exists_axiomatic_target_tag(target, tag),
            TagKind::Computed => self.compute(target, tag).properties().len() > 0,
        }
    }

    fn compute(self, target: Object, tag: Object) -> Structure {
        todo!("compute")
    }
}

pub struct Ignore;

/// Convenience trait to query the knowledge for existance.
pub trait ExistsAxiomatic {
    fn exists_axiomatic(self, uk: UncheckedKnowlege) -> bool;
}

impl ExistsAxiomatic for (Object, Object, Object) {
    fn exists_axiomatic(self, uk: UncheckedKnowlege) -> bool {
        let (target, tag, value) = self;
        uk.exists_axiomatic_fact(&Fact { target, tag, value })
    }
}

impl ExistsAxiomatic for (Object, Object, Ignore) {
    fn exists_axiomatic(self, uk: UncheckedKnowlege) -> bool {
        let (target, tag, Ignore) = self;
        uk.exists_axiomatic_target_tag(target, tag)
    }
}

impl ExistsAxiomatic for (Object, Ignore, Object) {
    fn exists_axiomatic(self, uk: UncheckedKnowlege) -> bool {
        let (target, Ignore, value) = self;
        uk.exists_axiomatic_target_value(target, value)
    }
}

impl ExistsAxiomatic for (Ignore, Object, Object) {
    fn exists_axiomatic(self, uk: UncheckedKnowlege) -> bool {
        let (Ignore, tag, value) = self;
        uk.exists_axiomatic_tag_value(tag, value)
    }
}

impl ExistsAxiomatic for (Object, Ignore, Ignore) {
    fn exists_axiomatic(self, uk: UncheckedKnowlege) -> bool {
        let (target, Ignore, Ignore) = self;
        uk.exists_axiomatic_target(target)
    }
}

impl ExistsAxiomatic for (Ignore, Object, Ignore) {
    fn exists_axiomatic(self, uk: UncheckedKnowlege) -> bool {
        let (Ignore, tag, Ignore) = self;
        uk.exists_axiomatic_tag(tag)
    }
}

impl ExistsAxiomatic for (Ignore, Ignore, Object) {
    fn exists_axiomatic(self, uk: UncheckedKnowlege) -> bool {
        let (Ignore, Ignore, value) = self;
        uk.exists_axiomatic_value(value)
    }
}

pub enum TagKind {
    Axiom,
    Computed,
}
