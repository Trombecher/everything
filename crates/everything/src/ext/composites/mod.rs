#[cfg(test)]
mod tests;

use everything_objects::{Abstract, Composite, Object, Property};
use tracing::instrument;

use crate::{
    ObjectOrSetValues,
    ctx::EvaluationContext,
    ext::{AbstractExt, ObjectExt, PropertyExt},
    statements::Statements,
};

#[derive(PartialEq, Clone, Debug)]
pub enum ObjectForm {
    Any,
    Specific(Object),
}

impl From<Option<Object>> for ObjectForm {
    fn from(value: Option<Object>) -> Self {
        match value {
            None => Self::Any,
            Some(object) => Self::Specific(object),
        }
    }
}

impl From<ObjectForm> for Option<Object> {
    fn from(value: ObjectForm) -> Self {
        match value {
            ObjectForm::Any => None,
            ObjectForm::Specific(object) => Some(object),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct StatementForm {
    pub subject: ObjectForm,
    pub tag: ObjectForm,
    pub value: ObjectForm,
}

#[derive(PartialEq, Clone, Debug)]
pub enum KnowledgeError {
    IsNotSupersetOfBase,
    SubjectIsNotStatementComposite(Object),
    NeedsToBeTrueButIsFalse(StatementForm),
    NeedsToBeFalseButIsTrue(StatementForm),
    ValueOnSubjectDoesNotMatchTagsConstraint {
        subject: Object,
        tag: Object,
        value: Object,
    },
}

/// Nice-to-have functions for [Composite]s.
pub trait CompositeExt {
    /// Creates a new set.
    ///
    /// ```plain
    /// {(CONTAINS, ...) (CONTAINS, ...) ...}
    /// ```
    fn new_set<const N: usize>(items: [Object; N]) -> Self;

    fn is_valid(&self, statemets: &Statements, recursive: bool) -> Result<(), KnowledgeError>;

    fn new_statement(subject: Object, tag: Object, value: Object) -> Self;

    fn new_bool(b: bool) -> Self;
}

impl CompositeExt for Composite {
    fn new_bool(b: bool) -> Self {
        if b {
            Self::new_set([Composite::Empty.into()])
        } else {
            Composite::Empty
        }
    }

    #[instrument(skip(knowledge), ret)]
    fn is_valid(&self, knowledge: &Statements, recursive: bool) -> Result<(), KnowledgeError> {
        if self.any().is_none() {
            // All specializations are valid
            return Ok(());
        }

        for property in self.properties() {
            let Some(constraint_function) = knowledge
                .query_values(property.tag.clone(), Abstract::AXIOMATIC.into())
                .next()
            else {
                return Err(KnowledgeError::NeedsToBeTrueButIsFalse(StatementForm {
                    subject: ObjectForm::Specific(property.tag.clone()),
                    tag: ObjectForm::Specific(Abstract::AXIOMATIC.into()),
                    value: ObjectForm::Any,
                }));
            };

            let mut result = constraint_function.call(
                knowledge,
                &[self.clone().into(), property.value.clone()].map(ObjectOrSetValues::Object),
                &mut EvaluationContext::default(),
            );

            if !result.is_truthy(knowledge) {
                return Err(KnowledgeError::ValueOnSubjectDoesNotMatchTagsConstraint {
                    subject: self.clone().into(),
                    tag: property.tag.clone(),
                    value: property.value.clone(),
                });
            }

            if recursive {
                property.tag.is_valid(knowledge, true)?;
                property.value.is_valid(knowledge, true)?;
            }
        }

        Ok(())
    }

    fn new_set<const N: usize>(items: [Object; N]) -> Self {
        let mut properties = items.map(Property::new_contains);
        Self::new(&mut properties)
    }

    fn new_statement(subject: Object, tag: Object, value: Object) -> Self {
        Self::new(&mut [
            Property::new_statement_subject(subject),
            Property::new_statement_tag(tag),
            Property::new_statement_value(value),
        ])
    }
}
