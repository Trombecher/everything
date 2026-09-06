use everything_objects::{Abstract, Composite, CompositeValues, Object, Property};

use crate::{
    CompositeSetValues, base,
    ext::{AbstractExt, ObjectExt, PropertyExt},
};

#[cfg(test)]
mod tests;

/// An iterator over all axiomatic values the specified tag has on
/// an object.
#[derive(Clone)]
pub enum QueryValues {
    /// The variant that yields nothing.
    None,

    /// The variant that only yields [`base::AXIOMATIC_AXIOMATIC_CONSTRAINT`]
    /// and then nothing else.
    AxiomaticAxiomaticConstraint,

    /// The variant that only yields [`Composite::Empty`] and then nothing else.
    EmptyComposite,

    /// The variant that yields values from [`AxiomaticBorrowedQueryValues`].
    Borrowed(BorrowedQueryValues),
}

impl QueryValues {
    /// Query the knowledge for all values of the given subject with the
    /// given tag, ignoring all computations that have to be made.
    ///
    /// # Assumptions
    ///
    /// * `tag` and all downstream tags are assumed to be axiomatic.
    /// * `knowledge` is a superset of [crate::base::BASE].
    pub fn new(knowledge: &Composite, subject: Object, tag: Object) -> Self {
        match (&subject, &tag) {
            (Object::Abstract(Abstract::AXIOMATIC), Object::Abstract(Abstract::AXIOMATIC)) => {
                Self::AxiomaticAxiomaticConstraint
            }
            (
                Object::Abstract(Abstract::AXIOMATIC | Abstract::FUNCTION),
                Object::Abstract(Abstract::FUNCTION),
            ) => Self::None,
            _ => {
                let values_from_subject = match &subject {
                    Object::Abstract(_) => CompositeValues::None,
                    Object::Composite(composite) => composite.values(tag.clone()),
                };

                let statements_from_knowledge = CompositeSetValues::new(knowledge);

                Self::Borrowed(BorrowedQueryValues {
                    values_from_subject,
                    statements_from_knowledge,
                    subject,
                    tag,
                })
            }
        }
    }

    /// Creates a Composite with all set values from `self`.
    #[deprecated]
    pub fn collect_to_set(self) -> Composite {
        let mut properties: Vec<_> = self.map(Property::new_contains).collect();
        Composite::new(&mut properties)
    }
}

impl Iterator for QueryValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::AxiomaticAxiomaticConstraint => {
                *self = Self::None;
                Some(base::AXIOMATIC_AXIOMATIC_CONSTRAINT.clone())
            }
            Self::EmptyComposite => {
                *self = Self::None;
                Some(Composite::Empty.into())
            }
            Self::Borrowed(values) => values.next(),
        }
    }
}

impl std::fmt::Debug for QueryValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut this = self.clone();
        f.debug_list().entries(&mut this).finish()
    }
}

#[derive(Clone)]
pub struct BorrowedQueryValues {
    values_from_subject: CompositeValues,
    statements_from_knowledge: CompositeSetValues,
    subject: Object,
    tag: Object,
}

impl Iterator for BorrowedQueryValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.values_from_subject.next() {
            return Some(value);
        }

        self.statements_from_knowledge.find_map(|statement| {
            if statement.intrinsic_statement_subject().unwrap() != self.subject {
                return None;
            }

            let tag = statement.intrinsic_statement_tag().unwrap();
            if tag != self.tag {
                return None;
            }

            let value = statement.intrinsic_statement_value().unwrap();

            // Now this value may be already been in
            // the subject if it is a Composite. So we
            // need to dedup here.

            if let Object::Composite(composite) = &self.subject
                && composite.has(&tag, &value)
            {
                return None;
            }

            Some(value)
        })
    }
}
