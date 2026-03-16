#[cfg(test)]
mod tests;

use everything_structures::{Object, Property, Structure};

use crate::{
    base::BASE,
    ext::{ObjectExt, Statement},
    query::query_values,
};

/// Nice-to-have functions for [Structure]s.
pub trait StructureExt {
    fn new_node_not(node: Object) -> Self;
    /// Constructs a query node.
    fn new_node_query(node: Object) -> Self;

    /// Constructs a count node.
    fn new_node_count(node: Object) -> Self;

    /// Constructs a parameter node.
    fn new_node_parameter(node: Object) -> Self;

    fn new_node_exists(statement: Object) -> Self;

    fn has_exactly_one_value_on(&self, tag: Object) -> bool;

    fn is_knowledge(&self) -> bool;

    fn is_statement(&self) -> bool;

    fn parse_statement<'a>(&'a self) -> Option<Statement<'a>>;

    fn new_computed(body: Object) -> Self;

    fn new_node_equal<const N: usize>(nodes: [Object; N]) -> Self;

    fn new_node_and<const N: usize>(nodes: [Object; N]) -> Self;

    fn new_node_or<const N: usize>(nodes: [Object; N]) -> Self;

    fn new_node_xor<const N: usize>(nodes: [Object; N]) -> Self;
}

impl StructureExt for Structure {
    fn has_exactly_one_value_on(&self, tag: Object) -> bool {
        let mut values = self.values(tag);
        return values.next().is_some() && values.next().is_none();
    }

    fn is_knowledge(&self) -> bool {
        // BASE needs to be included
        if !BASE.is_subset_of(self) {
            return false;
        }

        // We validate that every object contained
        // in `self` is a statement.

        for contains_object in self.values(Object::CONTAINS) {
            if let Object::Structure(contains_structure) = contains_object
                && contains_structure.is_statement()
            {
            } else {
                // TODO: review this for abstracts
                return false;
            }
        }

        // Now we need to check constraints and values.

        for statement in self.values(Object::CONTAINS) {
            let statement = statement
                .structure()
                .expect("expected structure because it was validated earlier")
                .parse_statement()
                .expect("found a structure which is not a statement");

            // Get constraint function from tag for value:
            let constraint_qr = query_values(
                self,
                &statement.tag,
                Object::AXIOMATIC,
                &mut Default::default(),
            );
            let constraint_function = match constraint_qr.iter().next() {
                Some(c) => c,
                None => return false,
            };

            let result = constraint_function
                .call(self, &statement.subject, &mut Default::default())
                .call(self, &statement.value, &mut Default::default());

            if !result.is_truthy() {
                return false;
            }
        }

        true
    }

    fn is_statement(&self) -> bool {
        self.has_exactly_one_value_on(Object::STATEMENT_SUBJECT)
            && self.has_exactly_one_value_on(Object::STATEMENT_TAG)
            && self.has_exactly_one_value_on(Object::STATEMENT_VALUE)
    }

    fn parse_statement<'a>(&'a self) -> Option<Statement<'a>> {
        let subject = self.values(Object::STATEMENT_SUBJECT).next()?;
        let tag = self.values(Object::STATEMENT_TAG).next()?;
        let value = self.values(Object::STATEMENT_VALUE).next()?;

        Some(Statement {
            subject,
            tag,
            value,
        })
    }

    fn new_computed(body: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::COMPUTED,
            value: body,
        }])
    }

    fn new_node_equal<const N: usize>(nodes: [Object; N]) -> Self {
        let mut properties = nodes.map(|node| Property {
            tag: Object::NODE_EQUAL,
            value: node,
        });

        Self::new(&mut properties)
    }

    fn new_node_and<const N: usize>(nodes: [Object; N]) -> Self {
        let mut properties = nodes.map(|node| Property {
            tag: Object::NODE_AND,
            value: node,
        });

        Self::new(&mut properties)
    }

    fn new_node_exists(statement_node: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::NODE_EXISTS,
            value: statement_node,
        }])
    }

    fn new_node_parameter(node: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::NODE_PARAMETER,
            value: node,
        }])
    }

    fn new_node_count(node: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::NODE_COUNT,
            value: node,
        }])
    }

    fn new_node_query(query: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::NODE_QUERY,
            value: query,
        }])
    }

    fn new_node_or<const N: usize>(nodes: [Object; N]) -> Self {
        let mut properties = nodes.map(|node| Property {
            tag: Object::NODE_OR,
            value: node,
        });

        Self::new(&mut properties)
    }

    fn new_node_xor<const N: usize>(nodes: [Object; N]) -> Self {
        let mut properties = nodes.map(|node| Property {
            tag: Object::NODE_XOR,
            value: node,
        });

        Self::new(&mut properties)
    }

    fn new_node_not(node: Object) -> Self {
        Self::new(&mut [Property {
            tag: Object::NODE_NOT,
            value: node,
        }])
    }
}
