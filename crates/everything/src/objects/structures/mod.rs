#[cfg(test)]
mod tests;

use everything_structures::{Object, Property, Structure};

use crate::{
    inference::{compute, query_values},
    objects::{self, ObjectExt, Statement},
};

/// Nice-to-have functions for [Structure]s.
pub trait StructureExt {
    fn node_query(node: Object) -> Self;
    fn node_count(node: Object) -> Self;
    fn node_parameter(node: Object) -> Self;
    fn node_exists(statement: Object) -> Self;
    fn has_exactly_one_value_on(&self, tag: &Object) -> bool;

    fn is_knowledge(&self) -> bool;

    fn is_statement(&self) -> bool;

    fn parse_statement<'a>(&'a self) -> Option<Statement<'a>>;

    fn node_function(body: Object) -> Self;

    fn node_equal<const N: usize>(nodes: [Object; N]) -> Self;

    fn node_and<const N: usize>(nodes: [Object; N]) -> Self;
}

impl StructureExt for Structure {
    fn has_exactly_one_value_on(&self, tag: &Object) -> bool {
        let mut values = self.values(tag);
        return values.next().is_some() && values.next().is_none();
    }

    fn is_knowledge(&self) -> bool {
        // First we validate that every object contained
        // in `self` is a statement.

        for contains_object in self.values(&super::CONTAINS) {
            if let Object::Structure(contains_structure) = contains_object
                && contains_structure.is_statement()
            {
            } else {
                // TODO: review this for abstracts
                return false;
            }
        }

        // Now we need to check constraints and values.

        for statement in self.values(&super::CONTAINS) {
            // TODO: better panic msgs
            let statement = statement.structure().unwrap().parse_statement().unwrap();

            // Get constraint function from tag for value:
            let constraint_query_result = query_values(self, &statement.tag, &objects::AXIOMATIC);
            let constraint_function = match constraint_query_result.iter().next() {
                Some(c) => c,
                None => return false,
            };

            let inter = compute::call(constraint_function, &statement.subject);
            let result = compute::call(&inter, &statement.value);

            if !result.is_truthy() {
                return false;
            }
        }

        true
    }

    fn is_statement(&self) -> bool {
        self.has_exactly_one_value_on(&objects::STATEMENT_SUBJECT)
            && self.has_exactly_one_value_on(&objects::STATEMENT_TAG)
            && self.has_exactly_one_value_on(&objects::STATEMENT_VALUE)
    }

    fn parse_statement<'a>(&'a self) -> Option<Statement<'a>> {
        let subject = self.values(&objects::STATEMENT_SUBJECT).next()?;
        let tag = self.values(&objects::STATEMENT_TAG).next()?;
        let value = self.values(&objects::STATEMENT_TAG).next()?;

        Some(Statement {
            subject,
            tag,
            value,
        })
    }

    fn node_function(body: Object) -> Self {
        Self::new(&mut [Property {
            tag: objects::NODE_FUNCTION_BODY,
            value: body,
        }])
    }

    fn node_equal<const N: usize>(nodes: [Object; N]) -> Self {
        let mut properties = nodes.map(|node| Property {
            tag: objects::NODE_EQUAL,
            value: node,
        });

        Self::new(&mut properties)
    }

    fn node_and<const N: usize>(nodes: [Object; N]) -> Self {
        let mut properties = nodes.map(|node| Property {
            tag: objects::NODE_AND,
            value: node,
        });

        Self::new(&mut properties)
    }

    fn node_exists(statement_node: Object) -> Self {
        Self::new(&mut [Property {
            tag: objects::NODE_EXISTS,
            value: statement_node,
        }])
    }

    fn node_parameter(node: Object) -> Self {
        Self::new(&mut [Property {
            tag: objects::NODE_PARAMETER,
            value: node,
        }])
    }

    fn node_count(node: Object) -> Self {
        Self::new(&mut [Property {
            tag: objects::NODE_COUNT,
            value: node,
        }])
    }

    fn node_query(query: Object) -> Self {
        Self::new(&mut [Property {
            tag: objects::NODE_QUERY,
            value: query,
        }])
    }
}
