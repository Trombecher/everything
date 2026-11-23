mod context;
#[cfg(test)]
mod tests;

pub use context::*;

use std::sync::Arc;

use num_bigint::BigInt;

use crate::{
    compute::{LazyObject, LazyObjectIter, MaybeUndefined, Node, capture, operations},
    iter::IteratorExtNextAndLast,
    knowledge::Knowledge,
    model::{Atom, Construction, Object, Set},
};

const UNDEFINED: LazyObject = LazyObject::Eager(Object::Atom(Atom::UNDEFINED));

#[derive(Debug)]
enum Task {
    Evaluate(Arc<Node>),
    PartialAnd {
        second: Arc<Node>,
    },
    /// `1 -> 1`
    ///
    /// Pops the top value from the evaluated stack and if that value is not a boolean,
    /// it transforms it to undefined.
    ForceToBoolean,
    Count,
    ValuesOf,
    HasValue,
    QueryAtomsWithProperty,
    QueryAtomsWithConstructor,
    QueryConstructionValues,
    QueryConstructions,
    Contains,
    NewConstruction,
    Equal,
    PartialOr {
        second: Arc<Node>,
    },
    Xor,
    Not,
    Add,
    Multiply,
    Union,
    Intersection,
    /// Evaluates the first child and short-circuits the second one if the
    /// first result is empty.
    PartialDifference {
        second: Arc<Node>,
    },
    Difference,
    Map,
    Filter,
    Every,
    /// One iteration of an `every` evaluation. Calls the predicate for the
    /// next element, if there is one.
    EveryStep {
        values: Box<LazyObjectIter>,
        predicate: Node,
    },
    /// Checks the predicate result produced by the preceding `EveryStep`.
    EveryCheck {
        values: Box<LazyObjectIter>,
        predicate: Node,
    },
    Any,
    /// One iteration of an `any` evaluation. Calls the predicate for the
    /// next element, if there is one.
    AnyStep {
        values: Box<LazyObjectIter>,
        predicate: Node,
    },
    /// Checks the predicate result produced by the preceding `AnyStep`.
    AnyCheck {
        values: Box<LazyObjectIter>,
        predicate: Node,
    },
    Fold,
    /// One iteration of a fold. The accumulator is taken from the evaluated
    /// stack.
    FoldStep {
        values: Box<LazyObjectIter>,
        folder: Node,
    },
    /// Applies the result of the first folder call to the current element.
    FoldApply {
        element: Object,
    },
    Less,
    PartialIf {
        then: Arc<Node>,
        otherwise: Arc<Node>,
    },
    Call,
    PopContext,
    Unwrap,
    ConstructorOfConstruction,
    ValueOfConstruction,
    IsAtom,
    IsConstruction,
    IsSet,
}

#[allow(clippy::too_many_lines)]
fn resolve_evaluate_node(
    node: &Node,
    knowledge: &Knowledge,
    evaluation_context: &mut EvaluationContext,
    tasks: &mut Vec<Task>,
    evaluated: &mut Vec<LazyObject>,
) {
    let node = node.resolve();

    match node {
        Node::WithSource { .. } => unreachable!("resolve removes source wrappers"),
        Node::IsSet(inner) => {
            tasks.push(Task::IsSet);
            tasks.push(Task::Evaluate(Arc::clone(inner)));
        }
        Node::IsConstruction(inner) => {
            tasks.push(Task::IsConstruction);
            tasks.push(Task::Evaluate(Arc::clone(inner)));
        }
        Node::ConstructionConstructor { construction } => {
            tasks.push(Task::ConstructorOfConstruction);
            tasks.push(Task::Evaluate(Arc::clone(construction)));
        }
        Node::ConstructionValue { construction } => {
            tasks.push(Task::ValueOfConstruction);
            tasks.push(Task::Evaluate(Arc::clone(construction)));
        }
        Node::QueryAtomsWithProperty { property } => {
            tasks.push(Task::QueryAtomsWithProperty);
            tasks.push(Task::Evaluate(Arc::clone(property)));
        }
        Node::QueryAtomsWithConstructor { constructor } => {
            tasks.push(Task::QueryAtomsWithConstructor);
            tasks.push(Task::Evaluate(Arc::clone(constructor)));
        }
        Node::QueryConstructionValues {
            object,
            property_constructor,
        } => {
            tasks.push(Task::QueryConstructionValues);
            tasks.push(Task::Evaluate(Arc::clone(property_constructor)));
            tasks.push(Task::Evaluate(Arc::clone(object)));
        }
        Node::QueryConstructions { constructor } => {
            tasks.push(Task::QueryConstructions);
            tasks.push(Task::Evaluate(Arc::clone(constructor)));
        }
        Node::Fold {
            set,
            folder,
            initial,
        } => {
            tasks.push(Task::Fold);
            tasks.push(Task::Evaluate(Arc::clone(initial)));
            tasks.push(Task::Evaluate(Arc::clone(folder)));
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::Contains { element, set } => {
            tasks.push(Task::Contains);
            tasks.push(Task::Evaluate(Arc::clone(set)));
            tasks.push(Task::Evaluate(Arc::clone(element)));
        }
        Node::NewConstruction { constructor, value } => {
            tasks.push(Task::NewConstruction);
            tasks.push(Task::Evaluate(Arc::clone(value)));
            tasks.push(Task::Evaluate(Arc::clone(constructor)));
        }
        Node::IsAtom(inner) => {
            tasks.push(Task::IsAtom);
            tasks.push(Task::Evaluate(Arc::clone(inner)));
        }
        Node::Call { callee, with } => {
            tasks.push(Task::Call);
            tasks.push(Task::Evaluate(Arc::clone(callee)));
            tasks.push(Task::Evaluate(Arc::clone(with)));
        }
        Node::Multiply {
            first: left,
            second: right,
        } => {
            tasks.push(Task::Multiply);
            tasks.push(Task::Evaluate(Arc::clone(right)));
            tasks.push(Task::Evaluate(Arc::clone(left)));
        }
        Node::Unwrap(set) => {
            tasks.push(Task::Unwrap);
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::Function { .. } => {
            evaluated.push(capture(node, 0, evaluation_context, knowledge).into());
        }
        Node::Literal(object) => {
            evaluated.push(object.clone().into());
        }
        Node::And { first, second } => {
            tasks.push(Task::PartialAnd {
                second: Arc::clone(second),
            });
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::FunctionSelf { depth } => {
            evaluated.push(match evaluation_context.function_self(*depth as usize) {
                MaybeUndefined::Defined(node) => LazyObject::Node(node),
                MaybeUndefined::Undefined => UNDEFINED,
            });
        }
        Node::Parameter { depth } => {
            evaluated.push(match evaluation_context.parameter_value(*depth as usize) {
                MaybeUndefined::Defined(object) => object,
                MaybeUndefined::Undefined => UNDEFINED,
            });
        }
        Node::Count(object) => {
            tasks.push(Task::Count);
            tasks.push(Task::Evaluate(Arc::clone(object)));
        }
        Node::ValuesOf(object) => {
            tasks.push(Task::ValuesOf);
            tasks.push(Task::Evaluate(Arc::clone(object)));
        }
        Node::HasValue(object) => {
            tasks.push(Task::HasValue);
            tasks.push(Task::Evaluate(Arc::clone(object)));
        }
        Node::Equal { first, second } => {
            tasks.push(Task::Equal);
            tasks.push(Task::Evaluate(Arc::clone(second)));
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::Or { first, second } => {
            tasks.push(Task::PartialOr {
                second: Arc::clone(second),
            });
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::Xor { first, second } => {
            tasks.push(Task::Xor);
            tasks.push(Task::Evaluate(Arc::clone(second)));
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::Not(object) => {
            tasks.push(Task::Not);
            tasks.push(Task::Evaluate(Arc::clone(object)));
        }
        Node::Add { first, second } => {
            tasks.push(Task::Add);
            tasks.push(Task::Evaluate(Arc::clone(second)));
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::Union { first, second } => {
            tasks.push(Task::Union);
            tasks.push(Task::Evaluate(Arc::clone(second)));
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::Intersection { first, second } => {
            tasks.push(Task::Intersection);
            tasks.push(Task::Evaluate(Arc::clone(second)));
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::Difference { first, second } => {
            tasks.push(Task::PartialDifference {
                second: Arc::clone(second),
            });
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::Map {
            set,
            mapper_function,
        } => {
            tasks.push(Task::Map);
            tasks.push(Task::Evaluate(Arc::clone(mapper_function)));
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::Every { set, predicate } => {
            tasks.push(Task::Every);
            tasks.push(Task::Evaluate(Arc::clone(predicate)));
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::Any { set, predicate } => {
            tasks.push(Task::Any);
            tasks.push(Task::Evaluate(Arc::clone(predicate)));
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::Filter {
            set,
            filter_function,
        } => {
            tasks.push(Task::Filter);
            tasks.push(Task::Evaluate(Arc::clone(filter_function)));
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::Less { first, second } => {
            tasks.push(Task::Less);
            tasks.push(Task::Evaluate(Arc::clone(second)));
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::If {
            condition,
            then,
            otherwise,
        } => {
            tasks.push(Task::PartialIf {
                then: Arc::clone(then),
                otherwise: Arc::clone(otherwise),
            });
            tasks.push(Task::Evaluate(Arc::clone(condition)));
        }
    }
}

/// Evaluates an object in the given knowledge and context.
///
/// # Panics
///
/// On implementation bugs.
#[allow(clippy::too_many_lines)]
#[must_use]
pub fn evaluate(node: &Node, knowledge: &Knowledge) -> LazyObject {
    let mut evaluation_context = EvaluationContext::default();
    let mut tasks = vec![Task::Evaluate(Arc::new(node.clone()))];
    let mut evaluated = Vec::<LazyObject>::new();

    while let Some(task) = tasks.pop() {
        match task {
            Task::Evaluate(node) => {
                resolve_evaluate_node(
                    &node,
                    knowledge,
                    &mut evaluation_context,
                    &mut tasks,
                    &mut evaluated,
                );
            }
            Task::PopContext => {
                evaluation_context.pop();
            }
            Task::ConstructorOfConstruction => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(
                    operations::constructor_of_construction(object).into(),
                ));
            }
            Task::ValueOfConstruction => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(
                    operations::value_of_construction(object).into(),
                ));
            }
            Task::Call => {
                let callee_lazy_object = evaluated.pop().unwrap();
                let parameter_value = evaluated.pop().unwrap();

                if let Some(Node::Function { body, object }) = callee_lazy_object
                    .try_into_node(knowledge)
                    .map(Node::into_plain)
                {
                    tasks.push(Task::PopContext);
                    tasks.push(Task::Evaluate(Arc::clone(&body)));

                    evaluation_context.push(FunctionContext {
                        function: Node::Function { body, object },
                        parameter: parameter_value,
                    });
                } else {
                    evaluated.push(UNDEFINED);
                }
            }
            Task::PartialAnd { second } => {
                let first = evaluated.pop().unwrap();

                match first.truthy() {
                    MaybeUndefined::Defined(true) => {
                        tasks.push(Task::ForceToBoolean);
                        tasks.push(Task::Evaluate(second));
                    }
                    other @ (MaybeUndefined::Defined(false) | MaybeUndefined::Undefined) => {
                        evaluated.push(LazyObject::Eager(other.into()));
                    }
                }
            }
            Task::Count => {
                let target = evaluated.pop().unwrap();

                evaluated.push(if target.is_set() {
                    Object::new_integer(BigInt::from(target.size(knowledge))).into()
                } else {
                    UNDEFINED
                });
            }
            Task::ValuesOf => {
                let object = evaluated.pop().unwrap();

                evaluated.push(match object {
                    LazyObject::Eager(Object::Atom(atom)) => {
                        LazyObject::AtomValues(knowledge.values_of_atom(&atom))
                    }
                    LazyObject::Eager(Object::Construction(_)) => UNDEFINED,
                    object => object,
                });
            }
            Task::HasValue => {
                let object = evaluated.pop().unwrap();

                evaluated.push(
                    if matches!(object, LazyObject::Eager(Object::Construction(_))) {
                        UNDEFINED
                    } else {
                        LazyObject::Eager(object.has_at_least_one_element(knowledge).into())
                    },
                );
            }
            Task::QueryAtomsWithProperty => {
                let property = evaluated.pop().unwrap().into_object(knowledge);

                evaluated.push(LazyObject::AtomsWithProperty { property });
            }
            Task::QueryConstructionValues => {
                let property_constructor = evaluated.pop().unwrap().into_object(knowledge);
                let object = evaluated.pop().unwrap();

                evaluated.push(
                    if matches!(&object, LazyObject::Eager(Object::Construction(_))) {
                        UNDEFINED
                    } else {
                        LazyObject::ConstructionValues {
                            target: Box::new(object),
                            property_constructor,
                        }
                    },
                );
            }
            Task::QueryAtomsWithConstructor => {
                let constructor = evaluated.pop().unwrap().into_object(knowledge);

                evaluated.push(LazyObject::AtomsWithConstructor { constructor });
            }
            Task::QueryConstructions => {
                let constructor = evaluated.pop().unwrap().into_object(knowledge);

                evaluated.push(LazyObject::Constructions { constructor });
            }
            Task::Contains => {
                let set = evaluated.pop().unwrap();
                let element = evaluated.pop().unwrap();

                evaluated.push(if set.is_set() {
                    LazyObject::Eager(set.has(&element.into_object(knowledge), knowledge).into())
                } else {
                    UNDEFINED
                });
            }
            Task::NewConstruction => {
                let value = evaluated.pop().unwrap().into_object(knowledge);
                let constructor = evaluated.pop().unwrap().into_object(knowledge);

                evaluated.push(LazyObject::Eager(
                    Construction::new(constructor, value).into(),
                ));
            }
            Task::ForceToBoolean => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(object.truthy().into()));
            }
            Task::Equal => {
                let right = evaluated.pop().unwrap().into_object(knowledge);
                let left = evaluated.pop().unwrap().into_object(knowledge);

                // TODO: maybe optimize this comparison (?)
                evaluated.push(LazyObject::Eager((left == right).into()));
            }
            Task::PartialOr { second } => match evaluated.pop().unwrap().truthy() {
                MaybeUndefined::Defined(false) => {
                    tasks.push(Task::ForceToBoolean);
                    tasks.push(Task::Evaluate(second));
                }
                other @ (MaybeUndefined::Defined(true) | MaybeUndefined::Undefined) => {
                    evaluated.push(LazyObject::Eager(other.into()));
                }
            },
            Task::Xor => {
                let second = evaluated.pop().unwrap().truthy();
                let first = evaluated.pop().unwrap().truthy();

                evaluated.push(match (first, second) {
                    (MaybeUndefined::Defined(true), MaybeUndefined::Defined(false))
                    | (MaybeUndefined::Defined(false), MaybeUndefined::Defined(true)) => {
                        LazyObject::Eager(true.into())
                    }
                    (MaybeUndefined::Defined(false), MaybeUndefined::Defined(false))
                    | (MaybeUndefined::Defined(true), MaybeUndefined::Defined(true)) => {
                        LazyObject::Eager(false.into())
                    }
                    _ => UNDEFINED,
                });
            }
            Task::IsAtom => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(
                    matches!(object, LazyObject::Eager(Object::Atom(_))).into(),
                ));
            }
            Task::IsConstruction => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(
                    matches!(object, LazyObject::Eager(Object::Construction(_))).into(),
                ));
            }
            Task::IsSet => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(object.is_set().into()));
            }
            Task::Not => match evaluated.pop().unwrap().truthy() {
                MaybeUndefined::Defined(truth) => {
                    evaluated.push(LazyObject::Eager((!truth).into()));
                }
                MaybeUndefined::Undefined => {
                    evaluated.push(UNDEFINED);
                }
            },
            Task::Add => {
                let right = evaluated.pop().unwrap().try_into_object();
                let left = evaluated.pop().unwrap().try_into_object();

                evaluated.push(match (left, right) {
                    (Some(left), Some(right)) => {
                        LazyObject::Eager(operations::add(&left, &right).into())
                    }
                    _ => UNDEFINED,
                });
            }
            Task::Multiply => {
                let right = evaluated.pop().unwrap().try_into_object();
                let left = evaluated.pop().unwrap().try_into_object();

                evaluated.push(match (left, right) {
                    (Some(left), Some(right)) => {
                        LazyObject::Eager(operations::multiply(&left, &right).into())
                    }
                    _ => UNDEFINED,
                });
            }
            Task::Union => {
                let first = evaluated.pop().unwrap();
                let second = evaluated.pop().unwrap();

                evaluated.push(if first.is_set() && second.is_set() {
                    LazyObject::Union {
                        first: Box::new(second),
                        second: Box::new(first),
                    }
                } else {
                    UNDEFINED
                });
            }
            Task::Intersection => {
                let first = evaluated.pop().unwrap();
                let second = evaluated.pop().unwrap();

                evaluated.push(if first.is_set() && second.is_set() {
                    LazyObject::Intersection {
                        first: Box::new(second),
                        second: Box::new(first),
                    }
                } else {
                    UNDEFINED
                });
            }
            Task::PartialDifference { second } => {
                let first = evaluated.pop().unwrap();

                if !first.is_set() {
                    evaluated.push(UNDEFINED);
                } else if !first.has_at_least_one_element(knowledge) {
                    // Nothing can be removed from an empty set, so the second
                    // child is never evaluated.
                    evaluated.push(LazyObject::Eager(Object::Set(Set::empty())));
                } else {
                    evaluated.push(first);
                    tasks.push(Task::Difference);
                    tasks.push(Task::Evaluate(second));
                }
            }
            Task::Difference => {
                let without = evaluated.pop().unwrap();
                let source = evaluated.pop().unwrap();

                evaluated.push(if without.is_set() {
                    LazyObject::Difference {
                        source: Box::new(source),
                        without: Box::new(without),
                    }
                } else {
                    UNDEFINED
                });
            }
            Task::Map => {
                let mapper_function = evaluated.pop().unwrap();
                let set = evaluated.pop().unwrap();

                if set.is_set()
                    && let Some(mapper_function @ Node::Function { .. }) = mapper_function
                        .try_into_node(knowledge)
                        .map(Node::into_plain)
                {
                    evaluated.push(LazyObject::Map {
                        target: Box::new(set),
                        mapper_function,
                    });
                } else {
                    evaluated.push(UNDEFINED);
                }
            }
            Task::Filter => {
                let filter_function = evaluated.pop().unwrap();
                let set = evaluated.pop().unwrap();

                if set.is_set()
                    && let Some(filter_function @ Node::Function { .. }) = filter_function
                        .try_into_node(knowledge)
                        .map(Node::into_plain)
                {
                    evaluated.push(LazyObject::Filter {
                        target: Box::new(set),
                        filter_function,
                    });
                } else {
                    evaluated.push(UNDEFINED);
                }
            }
            Task::Every => {
                let predicate = evaluated.pop().unwrap();
                let set = evaluated.pop().unwrap();

                if !set.is_set() {
                    evaluated.push(UNDEFINED);
                } else if !set.has_at_least_one_element(knowledge) {
                    // The predicate is never called and may therefore be invalid.
                    evaluated.push(LazyObject::Eager(true.into()));
                } else if let Some(predicate @ Node::Function { .. }) =
                    predicate.try_into_node(knowledge).map(Node::into_plain)
                {
                    tasks.push(Task::EveryStep {
                        values: Box::new(set.into_iter(knowledge)),
                        predicate,
                    });
                } else {
                    evaluated.push(UNDEFINED);
                }
            }
            Task::EveryStep {
                mut values,
                predicate,
            } => {
                if let Some(element) = values.next() {
                    tasks.push(Task::EveryCheck {
                        values,
                        predicate: predicate.clone(),
                    });
                    tasks.push(Task::Call);

                    // `Task::Call` pops the callee first and the parameter
                    // second, so the element has to be pushed before the
                    // callee.
                    evaluated.push(LazyObject::Eager(element));
                    evaluated.push(LazyObject::Node(predicate));
                } else {
                    evaluated.push(LazyObject::Eager(true.into()));
                }
            }
            Task::EveryCheck { values, predicate } => match evaluated.pop().unwrap().truthy() {
                MaybeUndefined::Defined(true) => {
                    tasks.push(Task::EveryStep { values, predicate });
                }
                MaybeUndefined::Defined(false) => {
                    evaluated.push(LazyObject::Eager(false.into()));
                }
                MaybeUndefined::Undefined => {
                    evaluated.push(UNDEFINED);
                }
            },
            Task::Any => {
                let predicate = evaluated.pop().unwrap();
                let set = evaluated.pop().unwrap();

                if !set.is_set() {
                    evaluated.push(UNDEFINED);
                } else if !set.has_at_least_one_element(knowledge) {
                    // The predicate is never called and may therefore be invalid.
                    evaluated.push(LazyObject::Eager(false.into()));
                } else if let Some(predicate @ Node::Function { .. }) =
                    predicate.try_into_node(knowledge).map(Node::into_plain)
                {
                    tasks.push(Task::AnyStep {
                        values: Box::new(set.into_iter(knowledge)),
                        predicate,
                    });
                } else {
                    evaluated.push(UNDEFINED);
                }
            }
            Task::AnyStep {
                mut values,
                predicate,
            } => {
                if let Some(element) = values.next() {
                    tasks.push(Task::AnyCheck {
                        values,
                        predicate: predicate.clone(),
                    });
                    tasks.push(Task::Call);

                    // `Task::Call` pops the callee first and the parameter
                    // second, so the element has to be pushed before the
                    // callee.
                    evaluated.push(LazyObject::Eager(element));
                    evaluated.push(LazyObject::Node(predicate));
                } else {
                    evaluated.push(LazyObject::Eager(false.into()));
                }
            }
            Task::AnyCheck { values, predicate } => match evaluated.pop().unwrap().truthy() {
                MaybeUndefined::Defined(false) => {
                    tasks.push(Task::AnyStep { values, predicate });
                }
                MaybeUndefined::Defined(true) => {
                    evaluated.push(LazyObject::Eager(true.into()));
                }
                MaybeUndefined::Undefined => {
                    evaluated.push(UNDEFINED);
                }
            },
            Task::Fold => {
                let initial = evaluated.pop().unwrap();
                let folder = evaluated.pop().unwrap();
                let set = evaluated.pop().unwrap();

                if !set.is_set() {
                    evaluated.push(UNDEFINED);
                } else if !set.has_at_least_one_element(knowledge) {
                    // The folder is never called and may therefore be invalid.
                    evaluated.push(initial);
                } else if let Some(folder @ Node::Function { .. }) =
                    folder.try_into_node(knowledge).map(Node::into_plain)
                {
                    tasks.push(Task::FoldStep {
                        values: Box::new(set.into_iter(knowledge)),
                        folder,
                    });
                    evaluated.push(initial);
                } else {
                    evaluated.push(UNDEFINED);
                }
            }
            Task::FoldStep { mut values, folder } => {
                let accumulator = evaluated.pop().unwrap();

                if let Some(element) = values.next() {
                    // Schedule `(folder(accumulator))(element)`.
                    tasks.push(Task::FoldStep {
                        values,
                        folder: folder.clone(),
                    });
                    tasks.push(Task::FoldApply { element });

                    tasks.push(Task::Call);
                    evaluated.push(accumulator);
                    evaluated.push(LazyObject::Node(folder));
                } else {
                    evaluated.push(accumulator);
                }
            }
            Task::FoldApply { element } => {
                let callee = evaluated.pop().unwrap();

                // `Task::Call` pops the callee first and the parameter second,
                // so the element has to be pushed before the callee.
                evaluated.push(LazyObject::Eager(element));
                evaluated.push(callee);

                tasks.push(Task::Call);
            }
            Task::Less => {
                let second = evaluated.pop().unwrap().try_into_object();
                let first = evaluated.pop().unwrap().try_into_object();

                evaluated.push(match (first, second) {
                    (Some(first), Some(second)) => {
                        LazyObject::Eager(operations::less(&first, &second).into())
                    }
                    _ => UNDEFINED,
                });
            }
            Task::PartialIf { then, otherwise } => match evaluated.pop().unwrap().truthy() {
                MaybeUndefined::Defined(true) => {
                    tasks.push(Task::Evaluate(then));
                }
                MaybeUndefined::Defined(false) => tasks.push(Task::Evaluate(otherwise)),
                MaybeUndefined::Undefined => {
                    evaluated.push(UNDEFINED);
                }
            },
            Task::Unwrap => {
                let inner = evaluated.pop().unwrap();

                evaluated.push(
                    if inner.is_set()
                        && let Some(inner) = inner.iter(knowledge).next_and_last()
                    {
                        LazyObject::Eager(inner)
                    } else {
                        UNDEFINED
                    },
                );
            }
        }
    }

    let last = evaluated.pop().unwrap();

    if let Some(top) = evaluated.last() {
        unreachable!("internal bug: there is still {top:?} on the stack")
    }

    if let Some(top) = evaluation_context.last() {
        unreachable!("internal bug: there is still {top:?} on the evaluation context stack")
    }

    last
}
