#[cfg(test)]
mod tests;

use everything_structures::{Abstract, Object, Property, Structure};
use tracing::instrument;

use crate::{
    base::BASE,
    ctx::EvaluationContext,
    ext::{AbstractExt, ObjectExt, Statement},
    query,
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
    SubjectIsNotStatementStructure(Object),
    NeedsToBeTrueButIsFalse(StatementForm),
    NeedsToBeFalseButIsTrue(StatementForm),
    ValueOnSubjectDoesNotMatchTagsConstraint {
        subject: Object,
        tag: Object,
        value: Object,
    },
}

/// Nice-to-have functions for [Structure]s.
pub trait StructureExt {
    /// Creates a new set.
    ///
    /// ```plain
    /// {(CONTAINS, ...) (CONTAINS, ...) ...}
    /// ```
    fn new_set<const N: usize>(items: [Object; N]) -> Self;

    /// Creates a _not_ node.
    ///
    /// ```plain
    /// {(NODE_NOT, ...)}
    /// ```
    fn new_node_not(node: Object) -> Self;

    /// Creates a query node.
    ///
    /// ```plain
    /// {(NODE_QUERY, ...)}
    /// ```
    fn new_node_query(node: Object) -> Self;

    /// Creates a count node.
    ///
    /// ```plain
    /// {(NODE_COUNT, ...)}
    /// ```
    fn new_node_count(node: Object) -> Self;

    /// Constructs a parameter node.
    fn new_node_parameter(depth: usize) -> Self;

    fn new_node_exists(statement: Object) -> Self;

    fn has_exactly_one_value_on(&self, tag: Object) -> bool;

    fn is_knowledge(&self) -> Result<(), KnowledgeError>;

    fn is_statement(&self) -> bool;

    fn parse_statement(&self) -> Option<Statement>;

    fn new_computed(body: Object) -> Self;

    fn new_node_equal<const N: usize>(nodes: [Object; N]) -> Self;

    fn new_node_and<const N: usize>(nodes: [Object; N]) -> Self;

    fn new_node_or<const N: usize>(nodes: [Object; N]) -> Self;

    fn new_node_xor<const N: usize>(nodes: [Object; N]) -> Self;

    fn new_node_literal(object: Object) -> Self;

    /// Creates a new query node, set up for value querying.
    fn new_node_query_values(subject: Object, tag: Object) -> Self;

    fn is_valid(&self, knowledge: &Structure, recursive: bool) -> Result<(), KnowledgeError>;

    fn new_statement(subject: Object, tag: Object, value: Object) -> Self;

    fn new_bool(b: bool) -> Self;
}

impl StructureExt for Structure {
    fn new_bool(b: bool) -> Self {
        if b {
            // `{(@1, {})}`
            Self::new_set([Structure::Empty.into()])
        } else {
            // `{}`
            Structure::Empty
        }
    }

    fn has_exactly_one_value_on(&self, tag: Object) -> bool {
        match self {
            Self::NaturalNumber(_) if tag == Object::Abstract(Abstract::SUCCESSOR_OF) => true,
            Self::Any(any) => {
                let mut values = any.values(tag);
                values.next().is_some() && values.next().is_none()
            }
            _ => false,
        }
    }

    #[instrument(skip(knowledge), ret)]
    fn is_valid(&self, knowledge: &Structure, recursive: bool) -> Result<(), KnowledgeError> {
        if self.any().is_none() {
            // All specializations are valid
            return Ok(());
        }

        for property in self.properties() {
            let constraint_function = query::values_axiomatically(
                knowledge,
                &property.tag,
                Object::Abstract(Abstract::AXIOMATIC),
            )
            .next()
            .ok_or_else(|| {
                KnowledgeError::NeedsToBeTrueButIsFalse(StatementForm {
                    subject: ObjectForm::Specific(property.tag.clone()),
                    tag: ObjectForm::Specific(Object::Abstract(Abstract::AXIOMATIC)),
                    value: ObjectForm::Any,
                })
            })?;

            let parameters = [self.clone().into(), property.value.clone()];

            let result =
                constraint_function.call(knowledge, &parameters, &mut EvaluationContext::default());

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

    fn is_knowledge(&self) -> Result<(), KnowledgeError> {
        // BASE needs to be included
        if !BASE.is_subset_of(self) {
            return Err(KnowledgeError::IsNotSupersetOfBase);
        }

        // We validate that every object contained
        // in `self` is a statement.

        for contains_object in self.values(Object::Abstract(Abstract::CONTAINS)) {
            if let Object::Structure(contains_structure) = &contains_object
                && contains_structure.is_statement()
            {
            } else {
                // TODO: review this for abstracts
                return Err(KnowledgeError::SubjectIsNotStatementStructure(
                    contains_object,
                ));
            }
        }

        // Now we need to check constraints and values.

        for statement in self.values(Object::Abstract(Abstract::CONTAINS)) {
            let statement = statement
                .structure()
                .expect("expected structure because it was validated earlier")
                .parse_statement()
                .expect("found a structure which is not a statement");

            let constraint_function = query::values_axiomatically(
                self,
                &statement.tag,
                Object::Abstract(Abstract::AXIOMATIC),
            )
            .next()
            .ok_or_else(|| {
                KnowledgeError::NeedsToBeTrueButIsFalse(StatementForm {
                    subject: ObjectForm::Specific(statement.tag.clone()),
                    tag: ObjectForm::Specific(Object::Abstract(Abstract::AXIOMATIC)),
                    value: ObjectForm::Any,
                })
            })?;

            let arguments = [statement.subject.clone(), statement.value.clone()];

            let result =
                constraint_function.call(self, &arguments, &mut EvaluationContext::default());

            if !result.is_truthy(self) {
                return Err(KnowledgeError::ValueOnSubjectDoesNotMatchTagsConstraint {
                    subject: statement.subject,
                    tag: statement.tag,
                    value: statement.value,
                });
            }
        }

        self.is_valid(self, true)
    }

    fn is_statement(&self) -> bool {
        self.has_exactly_one_value_on(Object::Abstract(Abstract::STATEMENT_SUBJECT))
            && self.has_exactly_one_value_on(Object::Abstract(Abstract::STATEMENT_TAG))
            && self.has_exactly_one_value_on(Object::Abstract(Abstract::STATEMENT_VALUE))
    }

    fn parse_statement(&self) -> Option<Statement> {
        let subject = self
            .values(Object::Abstract(Abstract::STATEMENT_SUBJECT))
            .next()?;
        let tag = self
            .values(Object::Abstract(Abstract::STATEMENT_TAG))
            .next()?;
        let value = self
            .values(Object::Abstract(Abstract::STATEMENT_VALUE))
            .next()?;

        Some(Statement {
            subject,
            tag,
            value,
        })
    }

    fn new_computed(body: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::Abstract(Abstract::COMPUTED),
            value: body,
        }])
    }

    fn new_node_equal<const N: usize>(nodes: [Object; N]) -> Self {
        let mut properties = nodes.map(|node| Property {
            tag: Object::Abstract(Abstract::NODE_EQUAL),
            value: node,
        });

        Self::new(&mut properties)
    }

    fn new_node_and<const N: usize>(nodes: [Object; N]) -> Self {
        let mut properties = nodes.map(|node| Property {
            tag: Object::Abstract(Abstract::NODE_AND),
            value: node,
        });

        Self::new(&mut properties)
    }

    fn new_node_exists(statement_node: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::Abstract(Abstract::NODE_EXISTS),
            value: statement_node,
        }])
    }

    fn new_node_parameter(depth: usize) -> Self {
        Self::new(&mut [Property {
            tag: Object::Abstract(Abstract::NODE_PARAMETER),
            value: Object::new_natural_number(depth as u128),
        }])
    }

    fn new_node_count(node: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::Abstract(Abstract::NODE_COUNT),
            value: node,
        }])
    }

    fn new_node_query(query: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::Abstract(Abstract::NODE_QUERY),
            value: query,
        }])
    }

    fn new_node_query_values(subject: Object, tag: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::Abstract(Abstract::NODE_QUERY),
            value: Self::new(&mut [
                Property {
                    tag: Object::Abstract(Abstract::STATEMENT_SUBJECT),
                    value: subject,
                },
                Property {
                    tag: Object::Abstract(Abstract::STATEMENT_TAG),
                    value: tag,
                },
            ])
            .into(),
        }])
    }

    fn new_set<const N: usize>(items: [Object; N]) -> Self {
        let mut properties = items.map(|node| Property {
            tag: Object::Abstract(Abstract::CONTAINS),
            value: node,
        });

        Self::new(&mut properties)
    }

    fn new_node_or<const N: usize>(nodes: [Object; N]) -> Self {
        let mut properties = nodes.map(|node| Property {
            tag: Object::Abstract(Abstract::NODE_OR),
            value: node,
        });

        Self::new(&mut properties)
    }

    fn new_node_xor<const N: usize>(nodes: [Object; N]) -> Self {
        let mut properties = nodes.map(|node| Property {
            tag: Object::Abstract(Abstract::NODE_XOR),
            value: node,
        });

        Self::new(&mut properties)
    }

    fn new_node_not(node: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::Abstract(Abstract::NODE_NOT),
            value: node,
        }])
    }

    fn new_node_literal(object: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::Abstract(Abstract::NODE_LITERAL),
            value: object,
        }])
    }

    fn new_statement(subject: Object, tag: Object, value: Object) -> Self {
        Self::new(&mut [
            Property {
                tag: Object::Abstract(Abstract::STATEMENT_SUBJECT),
                value: subject,
            },
            Property {
                tag: Object::Abstract(Abstract::STATEMENT_TAG),
                value: tag,
            },
            Property {
                tag: Object::Abstract(Abstract::STATEMENT_VALUE),
                value,
            },
        ])
    }
}
