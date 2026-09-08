mod abstract_iters;
mod queries;

pub use abstract_iters::*;
use imbl::{HashMap, HashSet};
pub use queries::*;

use everything_objects::{
    Abstract, Composite, CompositeProperties, CompositeTags, CompositeValues, Object, Property,
};

use crate::{
    ObjectOrSetValues,
    base::BASE,
    ctx::EvaluationContext,
    ext::{
        AbstractExt, IteratorExtNextAndLast, KnowledgeError, ObjectExt, ObjectForm, PropertyExt,
        SimpleStatement, StatementForm,
    },
};

pub struct Statement {
    pub subject: Abstract,
    pub property: StatementProperty,
}

impl Statement {
    /// Creates a new statement with no additional properties.
    #[must_use]
    pub fn new(subject: Abstract, tag: Object, value: Object) -> Self {
        Self {
            subject,
            property: StatementProperty {
                tag,
                value,
                additional_properties: Composite::Empty,
            },
        }
    }

    #[must_use]
    pub fn to_composite(&self) -> Composite {
        self.property.additional_properties.add(&mut [
            Property::new_statement_subject(Object::Abstract(self.subject)),
            Property::new_statement_tag(self.property.tag.clone()),
            Property::new_statement_value(self.property.value.clone()),
        ])
    }
}

impl From<Statement> for SimpleStatement {
    fn from(value: Statement) -> Self {
        Self {
            subject: value.subject.into(),
            tag: value.property.tag,
            value: value.property.value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct StatementProperty {
    pub tag: Object,
    pub value: Object,
    pub additional_properties: Composite,
}

impl From<StatementProperty> for Property {
    fn from(statement_property: StatementProperty) -> Self {
        Self {
            tag: statement_property.tag,
            value: statement_property.value,
        }
    }
}

pub(crate) type IndexedStatements =
    <HashMap<Abstract, HashSet<StatementProperty>> as IntoIterator>::IntoIter;

#[derive(Default, Clone)]
pub struct Statements {
    /// Statements indexed by subject. Every set is non empty.
    indexed_statements: HashMap<Abstract, HashSet<StatementProperty>>,
}

impl Statements {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn exists(&self, simple_statement: SimpleStatement) -> bool {
        let SimpleStatement {
            subject,
            tag,
            value,
        } = simple_statement;

        match subject {
            Object::Abstract(subject) => {
                // Search the knowledge:

                let Some(properties_of_subject) = self.indexed_statements.get(&subject) else {
                    // Subject has no properties.

                    return false;
                };

                // We use a set but iterate over entries... maybe this can be improved?
                properties_of_subject
                    .iter()
                    .any(|property| property.tag == tag && property.value == value)
            }
            Object::Composite(subject) => subject.has(&tag, &value),
        }
    }

    #[must_use]
    pub fn query_tags_and_values(&self, subject: Object) -> QueryTagsAndValues {
        match subject {
            Object::Abstract(subject) => {
                if let Some(properties) = self.indexed_statements.get(&subject) {
                    QueryTagsAndValues::Abstract(AbstractProperties {
                        extended: properties.clone().into_iter(),
                    })
                } else {
                    QueryTagsAndValues::Composite(CompositeProperties::Empty)
                }
            }
            Object::Composite(subject) => QueryTagsAndValues::Composite(subject.properties()),
        }
    }

    #[must_use]
    pub fn query_subjects(&self, tag: Object, value: Object) -> QuerySubjects {
        QuerySubjects {
            indexed_statements: self.indexed_statements.clone().into_iter(),
            tag,
            value,
        }
    }

    #[must_use]
    pub fn query_tags(&self, subject: Object, value: Object) -> QueryTags {
        match subject {
            Object::Abstract(subject) => {
                if let Some(properties) = self.indexed_statements.get(&subject) {
                    QueryTags::Abstract(AbstractTags {
                        properties: AbstractProperties {
                            extended: properties.clone().into_iter(),
                        },
                        value,
                    })
                } else {
                    // No properties to iterate.
                    QueryTags::Composite(CompositeTags::None)
                }
            }
            Object::Composite(composite) => QueryTags::Composite(composite.tags(value)),
        }
    }

    #[must_use]
    pub fn query_values(&self, subject: Object, tag: Object) -> QueryValues {
        match (subject, tag) {
            (Object::Abstract(Abstract::AXIOMATIC), Object::Abstract(Abstract::AXIOMATIC)) => {
                QueryValues::AxiomaticAxiomaticConstraint
            }
            (Object::Abstract(subject), tag) => {
                if let Some(properties) = self.indexed_statements.get(&subject) {
                    QueryValues::Abstract(AbstractValues {
                        properties: AbstractProperties {
                            extended: properties.clone().into_iter(),
                        },
                        tag,
                    })
                } else {
                    // No properties to iterate.
                    QueryValues::Composite(CompositeValues::None)
                }
            }
            (Object::Composite(subject), tag) => QueryValues::Composite(subject.values(tag)),
        }
    }

    #[must_use]
    pub fn query_subjects_and_values(&self, tag: Object) -> QuerySubjectsAndValues {
        QuerySubjectsAndValues {
            indexed_statements: self.indexed_statements.clone().into_iter(),
            tag,
            current_subject_with_properties: None,
        }
    }

    #[must_use]
    pub fn query_subjects_and_tags(&self, value: Object) -> QuerySubjectsAndTags {
        QuerySubjectsAndTags {
            indexed_statements: self.indexed_statements.clone().into_iter(),
            value,
            current_subject_with_properties: None,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn change_mut<
        'a,
        Remove: Iterator<Item = &'a Statement>,
        Add: Iterator<Item = Statement>,
    >(
        &mut self,
        remove_statements: Remove,
        add_statements: Add,
    ) {
        for statement in remove_statements {
            let Some(properties_of_abstract) = self.indexed_statements.get_mut(&statement.subject)
            else {
                // Nothing to remove.

                continue;
            };

            properties_of_abstract.remove(&statement.property);

            if properties_of_abstract.is_empty() {
                // Set is empty -> remove map entry.

                self.indexed_statements.remove(&statement.subject);
            }
        }

        for statement in add_statements {
            if !self.indexed_statements.contains_key(&statement.subject) {
                self.indexed_statements
                    .insert(statement.subject, HashSet::default());
            }

            let properties_of_abstract =
                self.indexed_statements.get_mut(&statement.subject).unwrap();
            properties_of_abstract.insert(statement.property);
        }
    }

    #[must_use]
    pub fn change<'a, Remove: Iterator<Item = &'a Statement>, Add: Iterator<Item = Statement>>(
        &self,
        remove_statements: Remove,
        add_statements: Add,
    ) -> Self {
        let mut this = self.clone();
        this.change_mut(remove_statements, add_statements);
        this
    }

    /// Checks whether `self` is valid knowledge.
    ///
    /// # Errors
    ///
    /// This function will return an error if `self` is not valid knowledge.
    pub fn is_knowledge(&self) -> Result<(), KnowledgeError> {
        // BASE needs to be included
        if !BASE
            .iter_owned()
            .all(|statement| self.exists(statement.into()))
        {
            return Err(KnowledgeError::IsNotSupersetOfBase);
        }

        for statement in self.iter_owned() {
            let Some(constraint_function) = self
                .query_values(statement.property.tag.clone(), Abstract::AXIOMATIC.into())
                .next_and_last()
            else {
                // Tag must be axiomatic (!)

                return Err(KnowledgeError::NeedsToBeTrueButIsFalse(StatementForm {
                    subject: ObjectForm::Specific(statement.property.tag.clone()),
                    tag: ObjectForm::Specific(Abstract::AXIOMATIC.into()),
                    value: ObjectForm::Any,
                }));
            };

            let mut result = constraint_function.call(
                self,
                &[
                    Object::Abstract(statement.subject),
                    statement.property.value.clone(),
                ]
                .map(ObjectOrSetValues::Object),
                &mut EvaluationContext::default(),
            );

            // Check that subject and value are matching the tag's constraint.
            if !result.is_truthy(self) {
                return Err(KnowledgeError::ValueOnSubjectDoesNotMatchTagsConstraint {
                    subject: statement.subject.into(),
                    tag: statement.property.tag,
                    value: statement.property.value,
                });
            }
        }

        // TODO: check all composites.

        Ok(())
    }

    #[must_use]
    pub fn iter_owned(&self) -> StatementsIter {
        StatementsIter {
            current_subject_with_properties: None,
            indexed_statements: self.indexed_statements.clone().into_iter(),
        }
    }
}

impl<T: IntoIterator<Item = Statement>> From<T> for Statements {
    fn from(value: T) -> Self {
        Self::new().change([].into_iter(), value.into_iter())
    }
}
