use super::RawNode;
use crate::model::Object;

#[cfg(test)]
mod tests;

/// Converts a nested raw node into the object form used for property values.
fn child(node: impl Into<RawNode>) -> Object {
    node.into().into_object()
}

impl RawNode {
    /// Creates a function node with the given body.
    #[must_use]
    pub fn function(body: impl Into<Self>) -> Self {
        Self::Function { body: child(body) }
    }

    /// Creates a literal node.
    #[must_use]
    pub fn literal(value: impl Into<Object>) -> Self {
        Self::Literal(value.into())
    }

    /// Creates a conjunction node.
    #[must_use]
    pub fn and(first: impl Into<Self>, second: impl Into<Self>) -> Self {
        Self::And {
            first: child(first),
            second: child(second),
        }
    }

    /// Creates a function self-reference node.
    #[must_use]
    pub fn function_self(depth: u32) -> Self {
        Self::FunctionSelf { depth }
    }

    /// Creates a parameter reference node.
    #[must_use]
    pub fn parameter(depth: u32) -> Self {
        Self::Parameter { depth }
    }

    /// Creates a set-count node.
    #[must_use]
    pub fn set_count(object: impl Into<Self>) -> Self {
        Self::SetCount(child(object))
    }

    /// Creates a properties-of node.
    #[must_use]
    pub fn properties_of(object: impl Into<Self>) -> Self {
        Self::PropertiesOf(child(object))
    }

    /// Creates a composite-from-set node.
    #[must_use]
    pub fn composite_from_set(set: impl Into<Self>) -> Self {
        Self::CompositeFromSet(child(set))
    }

    /// Creates a has-a-set-item node.
    #[must_use]
    pub fn has_a_set_item(object: impl Into<Self>) -> Self {
        Self::HasASetItem(child(object))
    }

    /// Creates a has-property node.
    #[must_use]
    pub fn has_property(subject: impl Into<Self>, property: impl Into<Self>) -> Self {
        Self::HasProperty {
            subject: child(subject),
            property: child(property),
        }
    }

    /// Creates an atoms-with-property query node.
    #[must_use]
    pub fn query_atoms_with_property(property: impl Into<Self>) -> Self {
        Self::QueryAtomsWithProperty {
            property: child(property),
        }
    }

    /// Creates an atoms-with-constructor query node.
    #[must_use]
    pub fn query_atoms_with_constructor(constructor: impl Into<Self>) -> Self {
        Self::QueryAtomsWithConstructor {
            constructor: child(constructor),
        }
    }

    /// Creates a construction-values query node.
    #[must_use]
    pub fn query_construction_values(
        object: impl Into<Self>,
        property_constructor: impl Into<Self>,
    ) -> Self {
        Self::QueryConstructionValues {
            object: child(object),
            property_constructor: child(property_constructor),
        }
    }

    /// Creates a constructions query node.
    #[must_use]
    pub fn query_constructions(constructor: impl Into<Self>) -> Self {
        Self::QueryConstructions {
            constructor: child(constructor),
        }
    }

    /// Creates a set-contains node.
    #[must_use]
    pub fn set_contains(element: impl Into<Self>, set: impl Into<Self>) -> Self {
        Self::SetContains {
            element: child(element),
            set: child(set),
        }
    }

    /// Creates an equality node.
    #[must_use]
    pub fn equal(first: impl Into<Self>, second: impl Into<Self>) -> Self {
        Self::Equal {
            first: child(first),
            second: child(second),
        }
    }

    /// Creates a disjunction node.
    #[must_use]
    pub fn or(first: impl Into<Self>, second: impl Into<Self>) -> Self {
        Self::Or {
            first: child(first),
            second: child(second),
        }
    }

    /// Creates an exclusive disjunction node.
    #[must_use]
    pub fn xor(first: impl Into<Self>, second: impl Into<Self>) -> Self {
        Self::Xor {
            first: child(first),
            second: child(second),
        }
    }

    /// Creates a negation node.
    #[must_use]
    pub fn not(object: impl Into<Self>) -> Self {
        Self::Not(child(object))
    }

    /// Creates an addition node.
    #[must_use]
    pub fn add(first: impl Into<Self>, second: impl Into<Self>) -> Self {
        Self::Add {
            first: child(first),
            second: child(second),
        }
    }

    /// Creates a set-union node.
    #[must_use]
    pub fn set_union(first: impl Into<Self>, second: impl Into<Self>) -> Self {
        Self::SetUnion {
            first: child(first),
            second: child(second),
        }
    }

    /// Creates a set-intersection node.
    #[must_use]
    pub fn set_intersection(first: impl Into<Self>, second: impl Into<Self>) -> Self {
        Self::SetIntersection {
            first: child(first),
            second: child(second),
        }
    }

    /// Creates a set-difference node.
    #[must_use]
    pub fn set_difference(first: impl Into<Self>, second: impl Into<Self>) -> Self {
        Self::SetDifference {
            first: child(first),
            second: child(second),
        }
    }

    /// Creates a set-map node.
    #[must_use]
    pub fn set_map(set: impl Into<Self>, mapper_function: impl Into<Self>) -> Self {
        Self::SetMap {
            set: child(set),
            mapper_function: child(mapper_function),
        }
    }

    /// Creates a set-filter node.
    #[must_use]
    pub fn set_filter(set: impl Into<Self>, filter_function: impl Into<Self>) -> Self {
        Self::SetFilter {
            set: child(set),
            filter_function: child(filter_function),
        }
    }

    /// Creates a less-than node.
    #[must_use]
    pub fn less(first: impl Into<Self>, second: impl Into<Self>) -> Self {
        Self::Less {
            first: child(first),
            second: child(second),
        }
    }

    /// Creates a conditional node.
    #[must_use]
    pub fn if_then_else(
        condition: impl Into<Self>,
        then: impl Into<Self>,
        otherwise: impl Into<Self>,
    ) -> Self {
        Self::If {
            condition: child(condition),
            then: child(then),
            otherwise: child(otherwise),
        }
    }

    /// Creates a set-unwrap node.
    #[must_use]
    pub fn set_unwrap(object: impl Into<Self>) -> Self {
        Self::SetUnwrap(child(object))
    }

    /// Creates a multiplication node.
    #[must_use]
    pub fn multiply(first: impl Into<Self>, second: impl Into<Self>) -> Self {
        Self::Multiply {
            first: child(first),
            second: child(second),
        }
    }

    /// Creates a call node.
    #[must_use]
    pub fn call(callee: impl Into<Self>, with: impl Into<Self>) -> Self {
        Self::Call {
            callee: child(callee),
            with: child(with),
        }
    }

    /// Creates an is-atom node.
    #[must_use]
    pub fn is_atom(object: impl Into<Self>) -> Self {
        Self::IsAtom(child(object))
    }

    /// Creates an is-construction node.
    #[must_use]
    pub fn is_construction(object: impl Into<Self>) -> Self {
        Self::IsConstruction(child(object))
    }

    /// Creates an is-composite node.
    #[must_use]
    pub fn is_composite(object: impl Into<Self>) -> Self {
        Self::IsComposite(child(object))
    }

    /// Creates an is-node node.
    #[must_use]
    pub fn is_node(object: impl Into<Self>) -> Self {
        Self::IsNode(child(object))
    }

    /// Creates a set-every node.
    #[must_use]
    pub fn set_every(set: impl Into<Self>, predicate: impl Into<Self>) -> Self {
        Self::SetEvery {
            set: child(set),
            predicate: child(predicate),
        }
    }

    /// Creates a set-any node.
    #[must_use]
    pub fn set_any(set: impl Into<Self>, predicate: impl Into<Self>) -> Self {
        Self::SetAny {
            set: child(set),
            predicate: child(predicate),
        }
    }

    /// Creates a set-fold node.
    #[must_use]
    pub fn set_fold(
        set: impl Into<Self>,
        folder: impl Into<Self>,
        initial: impl Into<Self>,
    ) -> Self {
        Self::SetFold {
            set: child(set),
            folder: child(folder),
            initial: child(initial),
        }
    }

    /// Creates a new-construction node.
    #[must_use]
    pub fn new_construction(constructor: impl Into<Self>, value: impl Into<Self>) -> Self {
        Self::NewConstruction {
            constructor: child(constructor),
            value: child(value),
        }
    }

    /// Creates a new-set node with the given items.
    #[must_use]
    pub fn new_set<I, N>(items: I) -> Self
    where
        I: IntoIterator<Item = N>,
        N: Into<Self>,
    {
        Self::NewSet {
            items: items.into_iter().map(|item| child(item)).collect(),
        }
    }

    /// Creates a new-composite node with the given properties.
    #[must_use]
    pub fn new_composite<I, N>(properties: I) -> Self
    where
        I: IntoIterator<Item = N>,
        N: Into<Self>,
    {
        Self::NewComposite {
            properties: properties
                .into_iter()
                .map(|property| child(property))
                .collect(),
        }
    }

    /// Creates a construction-constructor node.
    #[must_use]
    pub fn construction_constructor(construction: impl Into<Self>) -> Self {
        Self::ConstructionConstructor {
            construction: child(construction),
        }
    }

    /// Creates a construction-value node.
    #[must_use]
    pub fn construction_value(construction: impl Into<Self>) -> Self {
        Self::ConstructionValue {
            construction: child(construction),
        }
    }
}
