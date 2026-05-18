#[cfg(test)]
mod tests;

use everything_structures::{Abstract, Object, Property, Structure};
use tracing::instrument;

use crate::{
    base::BASE,
    ctx::EvaluationContext,
    ext::{AbstractExt, ObjectExt, PropertyExt, Statement},
    nodes::{BinaryNode, FilterNode, IfNode, MapNode, Node},
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

    fn has_exactly_one_value_on(&self, tag: Object) -> bool;

    fn is_knowledge(&self) -> Result<(), KnowledgeError>;

    fn is_statement(&self) -> bool;

    fn parse_statement(&self) -> Option<Statement>;

    /// Creates a new query node, set up for value querying.
    fn new_node_query_values(subject: Object, tag: Object) -> Self;

    fn is_valid(&self, knowledge: &Structure, recursive: bool) -> Result<(), KnowledgeError>;

    fn new_statement(subject: Object, tag: Object, value: Object) -> Self;

    fn new_bool(b: bool) -> Self;

    /// Creates a new node.
    fn new_node(node: Node) -> Self;
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
            Self::Integer(_) if tag == Object::Abstract(Abstract::SUCCESSOR_OF) => true,
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
                property.tag.clone(),
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

            let mut result =
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
                statement.tag.clone(),
                Abstract::AXIOMATIC.into(),
            )
            .next()
            .ok_or_else(|| {
                KnowledgeError::NeedsToBeTrueButIsFalse(StatementForm {
                    subject: ObjectForm::Specific(statement.tag.clone()),
                    tag: ObjectForm::Specific(Abstract::AXIOMATIC.into()),
                    value: ObjectForm::Any,
                })
            })?;

            let arguments = [statement.subject.clone(), statement.value.clone()];

            let mut result =
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
        self.has_exactly_one_value_on(Abstract::STATEMENT_SUBJECT.into())
            && self.has_exactly_one_value_on(Abstract::STATEMENT_TAG.into())
            && self.has_exactly_one_value_on(Abstract::STATEMENT_VALUE.into())
    }

    fn parse_statement(&self) -> Option<Statement> {
        let subject = self.values(Abstract::STATEMENT_SUBJECT.into()).next()?;
        let tag = self.values(Abstract::STATEMENT_TAG.into()).next()?;
        let value = self.values(Abstract::STATEMENT_VALUE.into()).next()?;

        Some(Statement {
            subject,
            tag,
            value,
        })
    }

    fn new_node_query_values(subject: Object, tag: Object) -> Self {
        Self::new_node(Node::Query(
            Self::new(&mut [
                Property::new_statement_subject(subject),
                Property::new_statement_tag(tag),
            ])
            .into(),
        ))
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

    fn new_node(node: Node) -> Self {
        match node {
            Node::Function(body) => Self::new(&mut [Property::new_function(body)]),
            Node::Literal(literal) => Self::new(&mut [Property::new_node_literal(literal)]),
            Node::And(BinaryNode { left, right }) => Self::new(&mut [
                Property::new_node_and_left(left),
                Property::new_node_and_right(right),
            ]),
            Node::FunctionSelf(depth) => Self::new(&mut [Property::new_node_function_self(depth)]),
            Node::Parameter(depth) => {
                Self::new(&mut [Property::new_node_parameter(depth as usize)])
            }
            Node::Count(object) => Self::new(&mut [Property::new_node_count(object)]),
            Node::Query(query) => Self::new(&mut [Property::new_node_query(query)]),
            Node::Equal(BinaryNode { left, right }) => Self::new(&mut [
                Property::new_node_equal_left(left),
                Property::new_node_equal_right(right),
            ]),
            Node::Or(BinaryNode { left, right }) => Self::new(&mut [
                Property::new_node_or_left(left),
                Property::new_node_or_right(right),
            ]),
            Node::Xor(BinaryNode { left, right }) => Self::new(&mut [
                Property::new_node_xor_left(left),
                Property::new_node_xor_right(right),
            ]),
            Node::Not(node) => Self::new(&mut [Property::new_node_not(node)]),
            Node::Add(BinaryNode { left, right }) => Self::new(&mut [
                Property::new_node_add_left(left),
                Property::new_node_add_right(right),
            ]),
            Node::Union(BinaryNode { left, right }) => Self::new(&mut [
                Property::new_node_union_left(left),
                Property::new_node_union_right(right),
            ]),
            Node::Map(MapNode {
                set,
                mapper_function,
            }) => Self::new(&mut [
                Property::new_node_map_set(set),
                Property::new_node_map_mapper(mapper_function),
            ]),
            Node::Filter(FilterNode {
                set,
                filter_function,
            }) => Self::new(&mut [
                Property::new_node_filter_set(set),
                Property::new_node_filter_filter(filter_function),
            ]),
            Node::Less(BinaryNode { left, right }) => Self::new(&mut [
                Property::new_node_less_left(left),
                Property::new_node_less_right(right),
            ]),
            Node::If(IfNode {
                condition,
                otherwise,
                then,
            }) => Self::new(&mut [
                Property::new_node_if_condition(condition),
                Property::new_node_if_then(then),
                Property::new_node_if_else(otherwise),
            ]),
        }
    }
}
