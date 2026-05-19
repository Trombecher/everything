#[cfg(test)]
mod tests;

use everything_structures::{Abstract, Object, Property, Structure};
use tracing::instrument;

use crate::{
    base::BASE,
    ctx::EvaluationContext,
    ext::{AbstractExt, ObjectExt, PropertyExt},
    nodes::{BinaryNode, FilterNode, IfNode, MapNode, Node, UnwrapOrNode},
    query::QueryValuesAxiomatically,
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

    fn is_knowledge(&self) -> Result<(), KnowledgeError>;

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

    #[instrument(skip(knowledge), ret)]
    fn is_valid(&self, knowledge: &Structure, recursive: bool) -> Result<(), KnowledgeError> {
        if self.any().is_none() {
            // All specializations are valid
            return Ok(());
        }

        for property in self.properties() {
            let constraint_function = QueryValuesAxiomatically::new(
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
        // in `self` is an intrinsic statement.

        for set_value in self.values(Object::Abstract(Abstract::CONTAINS)) {
            if set_value.intrinsic_statement().is_none() {
                return Err(KnowledgeError::SubjectIsNotStatementStructure(set_value));
            }
        }

        // Now we need to check constraints and values.

        for statement in self.values(Object::Abstract(Abstract::CONTAINS)) {
            let statement = statement.intrinsic_statement().unwrap();

            let constraint_function = QueryValuesAxiomatically::new(
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

            // Check that the tag in the statement is not a function.
            if QueryValuesAxiomatically::new(self, statement.tag.clone(), Abstract::FUNCTION.into())
                .next()
                .is_some()
            {
                return Err(KnowledgeError::NeedsToBeFalseButIsTrue(StatementForm {
                    subject: ObjectForm::Specific(statement.tag.clone()),
                    tag: ObjectForm::Specific(Abstract::FUNCTION.into()),
                    value: ObjectForm::Any,
                }));
            }

            let mut result = constraint_function.call(
                self,
                &[statement.subject.clone(), statement.value.clone()],
                &mut Default::default(),
            );

            // Check that subject and value are matching the tag's constraint.
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
            Node::UnwrapOr(UnwrapOrNode { set, default }) => Self::new(&mut [
                Property::new_node_unwrap_or_set(set),
                Property::new_node_unwrap_or_default(default),
            ]),
        }
    }
}
