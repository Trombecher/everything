#[cfg(test)]
mod tests;

use crate::{
    compute::{
        LazyObject, MaybeUndefined, Node, add, capture, multiply, node_function_body, operations,
    },
    iter::IteratorExtNextAndLast,
    knowledge::Knowledge,
    model::{Object, Set},
};

#[derive(Debug)]
pub enum Task {
    Evaluate(Object),
    PushLiteralTrue,
    PushLiteralFalse,
    PartialAnd { second: Object },
    ToBoolean,
    Count,
    Equal,
    PartialOr { second: Object },
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
    PartialIf { then: Object, otherwise: Object },
    Call,
    PopContext,
    PartialUnwrapOr { default: Object },
    IsAtom,
}

/// An evaluation context is a stack of function contexts,
/// each holding a function and the parameter value from
/// call.
///
/// Instead of capturing every variable on each call,
/// the evaluation engine can just lookup parameter values
/// from this Composite.
#[derive(Debug, Clone, Default)]
pub struct EvaluationContext {
    stack: Vec<FunctionContext>,
}

impl EvaluationContext {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn function_context(&self, relative_depth: usize) -> Option<&FunctionContext> {
        self.stack
            .len()
            .checked_sub(1 + relative_depth)
            .map(|index| self.stack.get(index).unwrap())
    }

    /// Gets the parameter value of the function context at the specified relative
    /// depth, or returns the empty set if the index is out of bounds.
    #[must_use]
    pub fn parameter_value(&self, relative_depth: usize) -> LazyObject {
        self.function_context(relative_depth)
            .map_or(Object::Set(Set::empty()).into(), |context| {
                context.parameter.clone()
            })
    }

    /// Gets the function itself of the function context at the specified relative
    /// depth, or returns the empty set if the index is out of bounds.
    #[must_use]
    pub fn function_self(&self, relative_depth: usize) -> Object {
        self.function_context(relative_depth)
            .map_or(Object::Set(Set::empty()), |context| {
                context.function.clone()
            })
    }

    /// Pushes a new function context onto the stack.
    pub fn push(&mut self, fc: FunctionContext) {
        self.stack.push(fc);
    }

    /// Pops the top-most function context off the stack.
    pub fn pop(&mut self) {
        self.stack.pop();
    }
}

#[derive(Debug, Clone)]
pub struct FunctionContext {
    pub function: Object,
    pub parameter: LazyObject,
}

/// Evaluates an object in the given knowledge and context.
///
/// # Panics
///
/// TODO
#[allow(clippy::too_many_lines)]
pub fn evaluate(
    object: &Object,
    knowledge: &Knowledge,
    context: &mut EvaluationContext,
) -> LazyObject {
    let mut tasks = vec![Task::Evaluate(object.clone())];
    let mut evaluated = Vec::<LazyObject>::new();

    while let Some(task) = tasks.pop() {
        match task {
            Task::Evaluate(object) => {
                match Node::new(&object, knowledge) {
                    Some(
                        Node::QueryAtomsWithProperty { .. }
                        | Node::QueryConstructionValues { .. }
                        | Node::Contains { .. }
                        | Node::IsConstruction(_)
                        | Node::IsSet(_)
                        | Node::Fold { .. }
                        | Node::NewConstruction { .. }
                        | Node::ConstructionConstructor { .. }
                        | Node::ConstructionValue { .. },
                    ) => todo!(),
                    Some(Node::IsAtom(inner)) => {
                        tasks.push(Task::IsAtom);
                        tasks.push(Task::Evaluate(inner));
                    }
                    Some(Node::Call { callee, with }) => {
                        tasks.push(Task::Call);
                        tasks.push(Task::Evaluate(callee));
                        tasks.push(Task::Evaluate(with));
                    }
                    Some(Node::Multiply {
                        first: left,
                        second: right,
                    }) => {
                        tasks.push(Task::Multiply);
                        tasks.push(Task::Evaluate(right));
                        tasks.push(Task::Evaluate(left));
                    }
                    Some(Node::UnwrapOr { set, default }) => {
                        tasks.push(Task::PartialUnwrapOr { default });
                        tasks.push(Task::Evaluate(set));
                    }
                    Some(Node::Function { .. }) => {
                        evaluated.push(capture(&object, knowledge, 0, context));
                    }
                    Some(Node::Literal(object)) => {
                        evaluated.push(object.into());
                    }
                    Some(Node::And { first, second }) => {
                        tasks.push(Task::PartialAnd { second });
                        tasks.push(Task::Evaluate(first));
                    }
                    Some(Node::FunctionSelf { depth }) => {
                        evaluated.push(LazyObject::Eager(context.function_self(depth as usize)));
                    }
                    Some(Node::Parameter { depth }) => {
                        evaluated.push(context.parameter_value(depth as usize));
                    }
                    Some(Node::Count(object)) => {
                        tasks.push(Task::Count);
                        tasks.push(Task::Evaluate(object));
                    }
                    Some(Node::Equal { first, second }) => {
                        tasks.push(Task::Equal);
                        tasks.push(Task::Evaluate(second));
                        tasks.push(Task::Evaluate(first));
                    }
                    Some(Node::Or { first, second }) => {
                        tasks.push(Task::PartialOr { second });
                        tasks.push(Task::Evaluate(first));
                    }
                    Some(Node::Xor { first, second }) => {
                        tasks.push(Task::Xor);
                        tasks.push(Task::Evaluate(second));
                        tasks.push(Task::Evaluate(first));
                    }
                    Some(Node::Not(object)) => {
                        tasks.push(Task::Not);
                        tasks.push(Task::Evaluate(object));
                    }
                    Some(Node::Add { first, second }) => {
                        tasks.push(Task::Add);
                        tasks.push(Task::Evaluate(second));
                        tasks.push(Task::Evaluate(first));
                    }
                    Some(Node::Union { first, second }) => {
                        tasks.push(Task::Union);
                        tasks.push(Task::Evaluate(second));
                        tasks.push(Task::Evaluate(first));
                    }
                    Some(Node::Map {
                        set,
                        mapper_function,
                    }) => {
                        tasks.push(Task::Map);
                        tasks.push(Task::Evaluate(mapper_function));
                        tasks.push(Task::Evaluate(set));
                    }
                    Some(Node::Every { set, predicate }) => {
                        tasks.push(Task::Every);

                        // Push the result of a previous, virtual predicate call.
                        // This literal acts like the virtual call completed
                        // last which aligns with the `Task::Every` case where
                        // subsequent predicate calls for set items are scheduled
                        // and pushed onto the evaluated objects when the evaluated
                        // set and predicate already sit there (late).
                        tasks.push(Task::PushLiteralTrue);

                        tasks.push(Task::Evaluate(predicate));
                        tasks.push(Task::Evaluate(set));
                    }
                    Some(Node::Any { set, predicate }) => {
                        tasks.push(Task::Any);

                        // Same as `Task::Every` but with false.
                        tasks.push(Task::PushLiteralFalse);

                        tasks.push(Task::Evaluate(predicate));
                        tasks.push(Task::Evaluate(set));
                    }
                    Some(Node::Filter {
                        set,
                        filter_function,
                    }) => {
                        tasks.push(Task::Filter);
                        tasks.push(Task::Evaluate(filter_function));
                        tasks.push(Task::Evaluate(set));
                    }
                    Some(Node::Less { first, second }) => {
                        tasks.push(Task::Less);
                        tasks.push(Task::Evaluate(second));
                        tasks.push(Task::Evaluate(first));
                    }
                    Some(Node::If {
                        condition,
                        then,
                        otherwise,
                    }) => {
                        tasks.push(Task::PartialIf { then, otherwise });
                        tasks.push(Task::Evaluate(condition));
                    }
                    /*
                    TODO: validate dynamic object creation
                    None if let Object::Composite(Composite::Any(any_composite)) = object => {
                        let result = any_composite
                        .properties()
                        .map(|property| {
                            // TODO: DEBATE THIS BS
                            debug!("eval on property {property:?}");

                            let value =
                                property.value.evaluate(knowledge, context).into_object();

                            let result = if property.value == value {
                                Ok(())
                            } else {
                                value.is_valid(knowledge, false)
                            };

                            match result {
                                Ok(()) => Ok(Property {
                                    tag: property.tag.clone(),
                                    value,
                                }),
                                // TODO: debate box
                                Err(error) => Err((value, Box::new(error))),
                            }
                        })
                        .transpose_into_fallible()
                        .collect::<Vec<_>>()
                        .map_or_else(|(o, error)| {
                            warn!(
                                "invalid object {o:?} with error {error:?}; replacing with {{}}"
                            );

                            LazyObject::Object(Object::Composite(Composite::Empty))
                        }, |mut properties| {
                            Object::Composite(Composite::new(&mut properties)).into()
                        });

                        evaluated.push(result);
                    }
                     */
                    None => evaluated.push(object.clone().into()),
                }
            }
            Task::PopContext => {
                context.pop();
            }
            Task::PushLiteralTrue => {
                evaluated.push(LazyObject::Eager(true.into()));
            }
            Task::PushLiteralFalse => {
                evaluated.push(LazyObject::Eager(false.into()));
            }
            Task::Call => {
                let callee = evaluated.pop().unwrap().into_object();
                let parameter_value = evaluated.pop().unwrap();

                tasks.push(Task::Evaluate(
                    node_function_body(&callee, knowledge).unwrap(),
                ));

                context.push(FunctionContext {
                    function: callee,
                    parameter: parameter_value,
                });
            }
            Task::PartialAnd { second } => {
                let first = evaluated.pop().unwrap();

                match first.truthy() {
                    MaybeUndefined::Defined(true) => {
                        tasks.push(Task::ToBoolean);
                        tasks.push(Task::Evaluate(second));
                    }
                    MaybeUndefined::Defined(false) => {
                        evaluated.push(LazyObject::Eager(Set::empty().into()));
                    }
                    MaybeUndefined::Undefined => evaluated.push(LazyObject::Eager(
                        MaybeUndefined::<Object>::Undefined.into(),
                    )),
                }
            }
            Task::Count => todo!("task count"),
            /*
            Task::Count => {
                let target = evaluated.pop().unwrap();

                evaluated.push(
                    Object::new_integer(target.set_values(knowledge).correct_count() as i128)
                        .into(),
                );
            }
             */
            Task::ToBoolean => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(
                    operations::to_boolean(&object, knowledge).into(),
                ));
            }
            Task::Equal => {
                let right = evaluated.pop().unwrap().into_object();
                let left = evaluated.pop().unwrap().into_object();

                evaluated.push(LazyObject::Eager((left == right).into()));
            }
            Task::PartialOr { second } => match evaluated.pop().unwrap().truthy() {
                MaybeUndefined::Defined(true) => evaluated.push(LazyObject::Eager(true.into())),
                MaybeUndefined::Defined(false) => {
                    tasks.push(Task::ToBoolean);
                    tasks.push(Task::Evaluate(second));
                }
                MaybeUndefined::Undefined => evaluated.push(LazyObject::Eager(
                    MaybeUndefined::<Object>::Undefined.into(),
                )),
            },
            Task::Xor => {
                let second = evaluated.pop().unwrap().truthy();
                let first = evaluated.pop().unwrap().truthy();

                evaluated.push(LazyObject::Eager(match (first, second) {
                    (MaybeUndefined::Defined(true), MaybeUndefined::Defined(false))
                    | (MaybeUndefined::Defined(false), MaybeUndefined::Defined(true)) => {
                        true.into()
                    }
                    (MaybeUndefined::Defined(false), MaybeUndefined::Defined(false))
                    | (MaybeUndefined::Defined(true), MaybeUndefined::Defined(true)) => {
                        false.into()
                    }
                    _ => MaybeUndefined::<Object>::Undefined.into(),
                }));
            }
            Task::IsAtom => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(
                    matches!(object, LazyObject::Eager(Object::Atom(_))).into(),
                ));
            }
            Task::Not => match evaluated.pop().unwrap().truthy() {
                MaybeUndefined::Defined(truth) => {
                    evaluated.push(LazyObject::Eager((!truth).into()));
                }
                MaybeUndefined::Undefined => {
                    evaluated.push(LazyObject::Eager(
                        MaybeUndefined::<Object>::Undefined.into(),
                    ));
                }
            },
            Task::Add => {
                let right = evaluated.pop().unwrap().into_object();
                // TODO: (perf) maybe short circuit sets into UNDEFINED.
                let left = evaluated.pop().unwrap().into_object();

                evaluated.push(LazyObject::Eager(add(&left, &right).into()));
            }
            Task::Multiply => {
                // TODO: (perf) maybe short circuit sets into UNDEFINED.
                let right = evaluated.pop().unwrap().into_object();
                let left = evaluated.pop().unwrap().into_object();

                evaluated.push(LazyObject::Eager(multiply(&left, &right).into()));
            }
            Task::Union => {
                let first = evaluated.pop().unwrap();
                let second = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Union {
                    first: Box::new(second),
                    second: Box::new(first),
                });
            }
            Task::Map => {
                let mapper = evaluated.pop().unwrap().into_object();
                let set = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Map {
                    target: Box::new(set),
                    mapper_function: mapper,
                });
            }
            Task::Filter => {
                let filter = evaluated.pop().unwrap().into_object();
                let set = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Filter {
                    target: Box::new(set),
                    filter_function: filter,
                });
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
                    evaluated.push(LazyObject::Eager(
                        MaybeUndefined::<Object>::Undefined.into(),
                    ));
                }
            },
            Task::PartialUnwrapOr { default } => {
                let mut set_values = evaluated.pop().unwrap().iter(knowledge);

                if let Some(inner) = set_values.next_and_last() {
                    evaluated.push(LazyObject::Eager(inner));
                } else {
                    tasks.push(Task::Evaluate(default));
                }
            }
        }
    }

    let last = evaluated.pop().unwrap();
    if evaluated.pop().is_some() {
        unreachable!()
    }

    last
}
