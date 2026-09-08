mod abstract_iters;
mod queries;

pub use abstract_iters::*;
use imbl::{HashMap, HashSet};
pub use queries::*;

use everything_objects::{
    Abstract, Composite, CompositeProperties, CompositeTags, CompositeValues, Object, Property,
};

use crate::ext::{AbstractExt, PropertyExt, SimpleStatement};

pub struct Statement {
    pub subject: Abstract,
    pub property: StatementProperty,
}

impl Statement {
    pub fn to_composite(&self) -> Composite {
        self.property.additional_properties.add(&mut [
            Property::new_statement_subject(Object::Abstract(self.subject)),
            Property::new_statement_tag(self.property.tag.clone()),
            Property::new_statement_value(self.property.value.clone()),
        ])
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

#[derive(Clone, Default)]
pub struct Statements {
    /// Statements indexed by subject. Every set is non empty.
    indexed_statements: HashMap<Abstract, HashSet<StatementProperty>>,
}

impl Statements {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn query_tags_and_values(&self, subject: Object) -> QueryTagsAndValues2 {
        match subject {
            Object::Abstract(subject) => {
                if let Some(properties) = self.indexed_statements.get(&subject) {
                    QueryTagsAndValues2::Abstract(AbstractProperties {
                        extended: properties.clone().into_iter(),
                    })
                } else {
                    QueryTagsAndValues2::Composite(CompositeProperties::Empty)
                }
            }
            Object::Composite(subject) => QueryTagsAndValues2::Composite(subject.properties()),
        }
    }

    pub fn query_subjects(&self, tag: Object, value: Object) -> QuerySubjects2 {
        QuerySubjects2 {
            indexed_statements: self.indexed_statements.clone().into_iter(),
            tag,
            value,
        }
    }

    pub fn query_tags(&self, subject: Object, value: Object) -> QueryTags2 {
        match subject {
            Object::Abstract(subject) => {
                if let Some(properties) = self.indexed_statements.get(&subject) {
                    QueryTags2::Abstract(AbstractTags {
                        properties: AbstractProperties {
                            extended: properties.clone().into_iter(),
                        },
                        value,
                    })
                } else {
                    // No properties to iterate.
                    QueryTags2::Composite(CompositeTags::None)
                }
            }
            Object::Composite(composite) => QueryTags2::Composite(composite.tags(value)),
        }
    }

    pub fn query_values(&self, subject: Object, tag: Object) -> QueryValues2 {
        match (subject, tag) {
            (Object::Abstract(Abstract::AXIOMATIC), Object::Abstract(Abstract::AXIOMATIC)) => {
                QueryValues2::AxiomaticAxiomaticConstraint
            }
            (Object::Abstract(subject), tag) => {
                if let Some(properties) = self.indexed_statements.get(&subject) {
                    QueryValues2::Abstract(AbstractValues {
                        properties: AbstractProperties {
                            extended: properties.clone().into_iter(),
                        },
                        tag,
                    })
                } else {
                    // No properties to iterate.
                    QueryValues2::Composite(CompositeValues::None)
                }
            }
            (Object::Composite(subject), tag) => QueryValues2::Composite(subject.values(tag)),
        }
    }

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
                    .insert(statement.subject, Default::default());
            }

            let properties_of_abstract =
                self.indexed_statements.get_mut(&statement.subject).unwrap();
            properties_of_abstract.insert(statement.property);
        }
    }

    pub fn change<'a, Remove: Iterator<Item = &'a Statement>, Add: Iterator<Item = Statement>>(
        &self,
        remove_statements: Remove,
        add_statements: Add,
    ) -> Self {
        let mut this = self.clone();
        this.change_mut(remove_statements, add_statements);
        this
    }
}
