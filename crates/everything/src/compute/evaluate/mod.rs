mod context;
#[cfg(test)]
mod tests;

pub use context::*;

use std::sync::Arc;

use num_bigint::BigInt;

use crate::{
    compute::{LazyObject, MaybeUndefined, Node, capture, operations},
    iter::IteratorExtNextAndLast,
    knowledge::Knowledge,
    model::{Atom, Object},
};

const UNDEFINED: LazyObject = LazyObject::Eager(Object::Atom(Atom::UNDEFINED));

#[allow(dead_code)]
#[derive(Debug)]
enum Task {
    Evaluate(Arc<Node>),
    PushLiteralTrue,
    PushLiteralFalse,
    PartialAnd {
        second: Arc<Node>,
    },
    ToBoolean,
    /// `1 -> 1`
    ///
    /// Pops the top value from the evaluated stack and if that value is not a boolean,
    /// it transforms it to undefined.
    ForceToBoolean,
    Count,
    Equal,
    PartialOr {
        second: Arc<Node>,
    },
    Xor,
    Not,
    Add,
    Multiply,
    Union,
    Map,
    Filter,
    Every,
    Any,
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
    evaluation_context: &mut EvaluationContext,
    tasks: &mut Vec<Task>,
    evaluated: &mut Vec<LazyObject>,
) {
    match node {
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
        Node::QueryAtomsWithProperty { .. }
        | Node::QueryConstructionValues { .. }
        | Node::Contains { .. }
        | Node::Fold { .. }
        | Node::NewConstruction { .. } => todo!(),
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
            evaluated.push(capture(node, 0, evaluation_context).into());
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

            // Push the result of a previous, virtual predicate call.
            // This literal acts like the virtual call completed
            // last which aligns with the `Task::Every` case where
            // subsequent predicate calls for set items are scheduled
            // and pushed onto the evaluated objects when the evaluated
            // set and predicate already sit there (late).
            tasks.push(Task::PushLiteralTrue);

            tasks.push(Task::Evaluate(Arc::clone(predicate)));
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::Any { set, predicate } => {
            tasks.push(Task::Any);

            // Same as `Task::Every` but with false.
            tasks.push(Task::PushLiteralFalse);

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
                resolve_evaluate_node(&node, &mut evaluation_context, &mut tasks, &mut evaluated);
            }
            Task::PopContext => {
                evaluation_context.pop();
            }
            Task::PushLiteralTrue => {
                evaluated.push(LazyObject::Eager(true.into()));
            }
            Task::PushLiteralFalse => {
                evaluated.push(LazyObject::Eager(false.into()));
            }
            Task::ConstructorOfConstruction => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(
                    // This function is undefined-aware.
                    operations::constructor_of_construction(object).into(),
                ));
            }
            Task::ValueOfConstruction => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(
                    // This function is undefined-aware.
                    operations::value_of_construction(object).into(),
                ));
            }
            Task::Call => {
                let callee_lazy_object = evaluated.pop().unwrap();
                let parameter_value = evaluated.pop().unwrap();

                if !callee_lazy_object.is_undefined()
                    && let Ok(Node::Function { body }) = callee_lazy_object.into_node(knowledge)
                {
                    tasks.push(Task::PopContext);
                    tasks.push(Task::Evaluate(Arc::clone(&body)));

                    evaluation_context.push(FunctionContext {
                        function: Node::Function { body },
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

                evaluated.push(if target.is_undefined() {
                    UNDEFINED
                } else {
                    Object::new_integer(BigInt::from(target.size(knowledge))).into()
                });
            }
            Task::ToBoolean => {
                // TODO: make this undefined-aware.
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(
                    operations::to_boolean(&object, knowledge).into(),
                ));
            }
            Task::ForceToBoolean => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(object.truthy().into()));
            }
            Task::Equal => {
                let right_lazy = evaluated.pop().unwrap();
                let left_lazy = evaluated.pop().unwrap();

                if !right_lazy.is_undefined() && !left_lazy.is_undefined() {
                    let left = left_lazy.into_object();
                    let right = right_lazy.into_object();

                    // TODO: maybe optimize this comparison (?)
                    evaluated.push(LazyObject::Eager((left == right).into()));
                } else {
                    evaluated.push(UNDEFINED);
                }
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

                evaluated.push(if object.is_undefined() {
                    UNDEFINED
                } else {
                    LazyObject::Eager(matches!(object, LazyObject::Eager(Object::Atom(_))).into())
                });
            }
            Task::IsConstruction => {
                let object = evaluated.pop().unwrap();

                evaluated.push(if object.is_undefined() {
                    UNDEFINED
                } else {
                    LazyObject::Eager(
                        matches!(object, LazyObject::Eager(Object::Construction(_))).into(),
                    )
                });
            }
            Task::IsSet => {
                let object = evaluated.pop().unwrap();

                evaluated.push(if object.is_undefined() {
                    UNDEFINED
                } else {
                    LazyObject::Eager(
                        (!matches!(
                            object,
                            LazyObject::Eager(Object::Atom(_) | Object::Construction(_))
                        ))
                        .into(),
                    )
                });
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
                let right = evaluated.pop().unwrap().into_object();
                // TODO: (perf) maybe short circuit sets into UNDEFINED.
                let left = evaluated.pop().unwrap().into_object();

                evaluated.push(LazyObject::Eager(operations::add(&left, &right).into()));
            }
            Task::Multiply => {
                // TODO: (perf) maybe short circuit sets into UNDEFINED.
                let right = evaluated.pop().unwrap().into_object();
                let left = evaluated.pop().unwrap().into_object();

                evaluated.push(LazyObject::Eager(
                    operations::multiply(&left, &right).into(),
                ));
            }
            Task::Union => {
                let first = evaluated.pop().unwrap();
                let second = evaluated.pop().unwrap();

                evaluated.push(if first.is_undefined() || second.is_undefined() {
                    UNDEFINED
                } else {
                    LazyObject::Union {
                        first: Box::new(second),
                        second: Box::new(first),
                    }
                });
            }
            Task::Map => {
                // TODO: make this undefined-aware
                let mapper_function_object = evaluated.pop().unwrap().into_object();
                let set = evaluated.pop().unwrap();

                if !set.is_undefined()
                    && let Ok(mapper_function @ Node::Function { .. }) =
                        Node::new(&mapper_function_object, knowledge)
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
                let filter_function_object = evaluated.pop().unwrap().into_object();
                let set = evaluated.pop().unwrap();

                if !set.is_undefined()
                    && let Ok(filter_function @ Node::Function { .. }) =
                        Node::new(&filter_function_object, knowledge)
                {
                    evaluated.push(LazyObject::Filter {
                        target: Box::new(set),
                        filter_function,
                    });
                } else {
                    evaluated.push(UNDEFINED);
                }
            }
            Task::Every => todo!("task every"),
            /*
            Task::Every => {
                let mut previous_result = evaluated.pop().unwrap();
                let predicate = evaluated.pop().unwrap().into_object();
                let mut set = evaluated.pop().unwrap();

                if !previous_result.is_truthy(knowledge) {
                    // Previous run did not succeed.

                    evaluated.push(LazyObject::Object(Composite::new_bool(false).into()));
                } else if let Some(value) = set.next() {
                    // Previous run did succeed, but there is another value.
                    // -> Validate predicate for this value.

                    tasks.push(Task::Every);

                    // Simulate the set and predicate being evaluated first.
                    evaluated.push(set.into());
                    evaluated.push(predicate.clone().into());

                    // Prepare for call.
                    tasks.push(Task::Call);
                    evaluated.push(value.into());
                    evaluated.push(predicate.into());
                } else {
                    // Previous run succeeded and there are no more elements. Yay.

                    evaluated.push(LazyObject::Object(Composite::new_bool(true).into()));
                }
            }
             */
            Task::Any => todo!("task any"),
            /*
            Task::Any => {
                let mut previous_result = evaluated.pop().unwrap();
                let predicate = evaluated.pop().unwrap().into_object();
                let mut set = evaluated.pop().unwrap();

                if previous_result.is_truthy(knowledge) {
                    // Previous run succeeded. Yay.

                    evaluated.push(LazyObject::Object(Composite::new_bool(true).into()));
                } else if let Some(value) = set.next() {
                    // Previous run did not succeed, but there is another value.
                    // -> Validate predicate for this value.

                    tasks.push(Task::Any);

                    // Simulate the set and predicate being evaluated first.
                    evaluated.push(set.into());
                    evaluated.push(predicate.clone().into());

                    // Prepare for call.
                    tasks.push(Task::Call);
                    evaluated.push(value.into());
                    evaluated.push(predicate.into());
                } else {
                    // Previous run did not succeed and there are no more elements.

                    evaluated.push(LazyObject::Object(Composite::new_bool(false).into()));
                }
            }
             */
            Task::Less => {
                // TODO: make this undefined-aware
                let second = evaluated.pop().unwrap().into_object();
                let first = evaluated.pop().unwrap().into_object();

                evaluated.push(LazyObject::Eager(operations::less(&first, &second).into()));
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
                // TODO: make this undefined-aware
                let inner = evaluated.pop().unwrap();

                evaluated.push(
                    if !inner.is_undefined()
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
