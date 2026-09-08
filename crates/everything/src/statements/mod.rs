mod abstract_iters;
mod queries;

use std::hash::Hash;

pub use abstract_iters::*;
use equivalent::Equivalent;
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
        AbstractExt, CompositeExt, IteratorExtNextAndLast, KnowledgeError, ObjectExt, ObjectForm,
        PropertyExt, StatementForm,
    },
};

/// A statement with additional data being omitted.
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleStatement {
    pub subject: Object,
    pub tag: Object,
    pub value: Object,
}

impl From<Statement> for SimpleStatement {
    fn from(value: Statement) -> Self {
        Self {
            subject: value.subject.into(),
            tag: value.tag,
            value: value.value,
        }
    }
}

/// Indicates that a statement is to be removed.
pub struct RemoveStatement(pub Statement);

impl Equivalent<IndexedStatementProperty> for RemoveStatement {
    fn equivalent(&self, key: &IndexedStatementProperty) -> bool {
        self.0.tag == key.tag
            && self.0.value == key.value
            && self.0.additional_properties == key.additional_properties
    }
}

impl Hash for RemoveStatement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Same hash as IndexedStatementProperty
        self.0.tag.hash(state);
        self.0.value.hash(state);
        self.0.additional_properties.hash(state);
    }
}

/// A statement.
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub subject: Abstract,
    pub tag: Object,
    pub value: Object,
    pub additional_properties: Composite,
}

impl Statement {
    /// Creates a new statement with no additional properties.
    #[must_use]
    pub fn new(subject: Abstract, tag: Object, value: Object) -> Self {
        Self {
            subject,
            tag,
            value,
            additional_properties: Composite::Empty,
        }
    }

    #[must_use]
    pub fn into_composite(self) -> Composite {
        self.additional_properties.add(&mut [
            Property::new_statement_subject(Object::Abstract(self.subject)),
            Property::new_statement_tag(self.tag),
            Property::new_statement_value(self.value),
        ])
    }

    #[must_use]
    pub fn to_composite(&self) -> Composite {
        self.additional_properties.add(&mut [
            Property::new_statement_subject(Object::Abstract(self.subject)),
            Property::new_statement_tag(self.tag.clone()),
            Property::new_statement_value(self.value.clone()),
        ])
    }

    fn split(self) -> (Abstract, IndexedStatementProperty) {
        (
            self.subject,
            IndexedStatementProperty {
                additional_properties: self.additional_properties,
                tag: self.tag,
                value: self.value,
            },
        )
    }
}

impl From<Statement> for Composite {
    fn from(value: Statement) -> Self {
        value.to_composite()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct IndexedStatementProperty {
    tag: Object,
    value: Object,
    additional_properties: Composite,
}

impl From<IndexedStatementProperty> for Property {
    fn from(property: IndexedStatementProperty) -> Self {
        Property {
            tag: property.tag,
            value: property.value,
        }
    }
}

type IndexedStatements =
    <HashMap<Abstract, HashSet<IndexedStatementProperty>> as IntoIterator>::IntoIter;

/// A set of statements. This may contain invalid knowledge. Use [`Self::is_knowledge`] to validate.
#[derive(Default, Clone)]
pub struct Statements {
    /// Statements indexed by subject. Every set is non empty.
    indexed_statements: HashMap<Abstract, HashSet<IndexedStatementProperty>>,
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
    pub fn add_mut(&mut self, statements: impl Iterator<Item = Statement>) {
        for (subject, property) in statements.map(Statement::split) {
            if !self.indexed_statements.contains_key(&subject) {
                self.indexed_statements.insert(subject, HashSet::default());
            }

            // This will not panic.
            let properties_of_abstract = self.indexed_statements.get_mut(&subject).unwrap();

            properties_of_abstract.insert(property);
        }
    }

    /// Create a new revision with these statements added.
    #[must_use]
    pub fn add(&self, statements: impl Iterator<Item = Statement>) -> Self {
        let mut this = self.clone();
        this.add_mut(statements);
        this
    }

    pub fn remove_mut<'a>(&mut self, statements: impl Iterator<Item = &'a RemoveStatement>) {
        for statement in statements {
            let Some(properties_of_abstract) =
                self.indexed_statements.get_mut(&statement.0.subject)
            else {
                // Nothing to remove.

                continue;
            };

            properties_of_abstract.remove(statement);

            if properties_of_abstract.is_empty() {
                // Set is empty -> remove map entry.

                self.indexed_statements.remove(&statement.0.subject);
            }
        }
    }

    #[must_use]
    pub fn remove<'a>(&self, statements: impl Iterator<Item = &'a RemoveStatement>) -> Self {
        let mut this = self.clone();
        this.remove_mut(statements);
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
                .query_values(statement.tag.clone(), Abstract::AXIOMATIC.into())
                .next_and_last()
            else {
                // Tag must be axiomatic (!)

                return Err(KnowledgeError::NeedsToBeTrueButIsFalse(StatementForm {
                    subject: ObjectForm::Specific(statement.tag.clone()),
                    tag: ObjectForm::Specific(Abstract::AXIOMATIC.into()),
                    value: ObjectForm::Any,
                }));
            };

            let mut result = constraint_function.call(
                self,
                &[Object::Abstract(statement.subject), statement.value.clone()]
                    .map(ObjectOrSetValues::Object),
                &mut EvaluationContext::default(),
            );

            // Check that subject and value are matching the tag's constraint.
            if !result.is_truthy(self) {
                return Err(KnowledgeError::ValueOnSubjectDoesNotMatchTagsConstraint {
                    subject: statement.subject.into(),
                    tag: statement.tag,
                    value: statement.value,
                });
            }
        }

        // Maybe this does not have to be owned.
        for statement in self.iter_owned() {
            statement.into_composite().is_valid(self, true)?;
        }

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
        let mut this = Self::new();
        this.add_mut(value.into_iter());
        this
    }
}
