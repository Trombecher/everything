use crate::{
    compute::{self, CallNode, EvaluationContext, Node},
    knowledge::Knowledge,
    model::Object,
};

pub enum LazyObject {
    Eager(Object),

    /// A union of two [`LazyObject`]s.
    Union {
        first: Box<Self>,
        second: Box<Self>,
    },

    Intersection {
        first: Box<Self>,
        second: Box<Self>,
    },

    Difference {
        source: Box<Self>,
        without: Box<Self>,
    },

    /// The set of atom with the given property.
    AtomsWithProperty {
        property: Object,
    },

    /// Maps every value of a given set with a mapper function.
    Map {
        target: Box<Self>,
        /// This function is captured.
        mapper_function: Object,
    },

    /// Retains all values for that the filter function returns a truthy value.
    Filter {
        target: Box<Self>,
        /// This function is captured.
        filter_function: Object,
    },
}

impl LazyObject {
    pub fn has(&self, element: &Object, knowledge: &Knowledge) -> bool {
        match self {
            Self::Eager(object) => knowledge.object_has(object, element),
            Self::Union { first, second } => {
                first.has(element, knowledge) || second.has(element, knowledge)
            }
            Self::Difference { source, without } => {
                source.has(element, knowledge) && !without.has(element, knowledge)
            }
            Self::Intersection { first, second } => {
                first.has(element, knowledge) && second.has(element, knowledge)
            }
            Self::AtomsWithProperty { property } => {
                if let Object::Atom(_) = element {
                    knowledge.object_has(element, property)
                } else {
                    false
                }
            }
            Self::Map {
                target,
                mapper_function,
            } => todo!(),
            Self::Filter {
                target,
                filter_function,
            } => {
                // `element` is member of the set which all elements satisfy the constraint
                // iff it is in that set and matches the constraint.

                target.has(element, knowledge)
                    && compute::evaluate(
                        Object::new_node(Node::Call(CallNode {
                            callee: filter_function.clone(),
                            with: element.clone(),
                        })),
                        knowledge,
                        &mut EvaluationContext::default(),
                    )
            }
        }
    }
}
