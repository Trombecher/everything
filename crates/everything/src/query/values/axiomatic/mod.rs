use everything_structures::{Abstract, Object, Property, Structure, StructureValues};

use crate::{
    base,
    ext::{AbstractExt, ObjectExt, PropertyExt},
    query::StructureSetValues,
};

#[cfg(test)]
mod tests;

/// An iterator over all axiomatic values the specified tag has on
/// an object. You can obtain an instance of this iterator by
/// calling [`values_axiomatically`].
#[derive(Clone)]
pub enum QueryValuesAxiomatically {
    /// The variant that yields nothing.
    None,

    /// The variant that only yields [base::AXIOMATIC_AXIOMATIC_CONSTRAINT]
    /// and then nothing else.
    AxiomaticAxiomaticConstraint,

    /// The variant that only yields [Structure::Empty] and then nothing else.
    EmptyStructure,

    /// The variant that yields values from [AxiomaticBorrowedQueryValues].
    Borrowed(AxiomaticBorrowedQueryValues),
}

impl QueryValuesAxiomatically {
    /// Query the knowledge for all values of the given subject with the
    /// given tag, ignoring all computations that have to be made.
    ///
    /// # Assumptions
    ///
    /// * `tag` and all downstream tags are assumed to be axiomatic.
    /// * `knowledge` is a superset of [crate::base::BASE].
    pub fn new(knowledge: &Structure, subject: Object, tag: Object) -> Self {
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
                    Object::Abstract(_) => StructureValues::None,
                    Object::Structure(structure) => structure.values(tag.clone()),
                };

                let statements_from_knowledge = StructureSetValues::new(knowledge);

                Self::Borrowed(AxiomaticBorrowedQueryValues {
                    values_from_subject,
                    statements_from_knowledge,
                    subject,
                    tag,
                })
            }
        }
    }

    /// Creates a structure with all set values from `self`.
    #[deprecated]
    pub fn collect_to_set(self) -> Structure {
        let mut properties: Vec<_> = self.map(Property::new_contains).collect();
        Structure::new(&mut properties)
    }
}

impl Iterator for QueryValuesAxiomatically {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::AxiomaticAxiomaticConstraint => {
                *self = Self::None;
                Some(base::AXIOMATIC_AXIOMATIC_CONSTRAINT.clone())
            }
            Self::EmptyStructure => {
                *self = Self::None;
                Some(Structure::Empty.into())
            }
            Self::Borrowed(values) => values.next(),
        }
    }
}

impl std::fmt::Debug for QueryValuesAxiomatically {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut this = self.clone();
        f.debug_list().entries(&mut this).finish()
    }
}

#[derive(Clone)]
pub struct AxiomaticBorrowedQueryValues {
    values_from_subject: StructureValues,
    statements_from_knowledge: StructureSetValues,
    subject: Object,
    tag: Object,
}

impl Iterator for AxiomaticBorrowedQueryValues {
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
            // the subject if it is a structure. So we
            // need to dedup here.

            if let Object::Structure(structure) = &self.subject
                && structure.has(&tag, &value)
            {
                return None;
            }

            Some(value)
        })
    }
}
