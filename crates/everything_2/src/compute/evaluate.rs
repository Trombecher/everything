use crate::{
    compute::{Node, ObjectOrExtendedSetValues},
    knowledge::Knowledge,
    model::{Object, Set},
};

#[derive(Debug)]
pub enum Task {
    Evaluate(Object),
    PushLiteralTrue,
    PushLiteralFalse,
    PartialAnd { right: Object },
    ToBoolean,
    Count,
    QueryValues,
    QuerySubjects,
    QuerySubjectsAndValues,
    QueryTagsAndValues,
    QuerySubjectsAndTags,
    QueryTags,
    QueryExists,
    Equal,
    PartialOr { right: Object },
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
    IsAbstract,
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
    pub fn parameter_value(&self, relative_depth: usize) -> ObjectOrExtendedSetValues {
        self.function_context(relative_depth)
            .map_or(Object::Set(Set::empty()).into(), |context| {
                context.parameter.clone()
            })
    }

    /// Gets the function itself of the function context at the specified relative
    /// depth, or returns the empty set if the index is out of bounds.
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
    pub parameter: ObjectOrExtendedSetValues,
}

pub fn evaluate(
    object: Object,
    knowledge: &Knowledge,
    context: &mut EvaluationContext,
) -> ObjectOrExtendedSetValues {
    let mut tasks = vec![Task::Evaluate(object.clone())];
    let mut evaluated = Vec::<ObjectOrExtendedSetValues>::new();

    while let Some(task) = tasks.pop() {
        match task {
            Task::Evaluate(object) => {
                match object.node(statements) {
                    Some(Node::IsAbstract(inner)) => {
                        tasks.push(Task::IsAbstract);
                        tasks.push(Task::Evaluate(inner));
                    }
                    Some(Node::Call(CallNode { callee, with })) => {
                        tasks.push(Task::Call);
                        tasks.push(Task::Evaluate(callee));
                        tasks.push(Task::Evaluate(with));
                    }
                    Some(Node::Multiply(BinaryNode { left, right })) => {
                        tasks.push(Task::Multiply);
                        tasks.push(Task::Evaluate(right));
                        tasks.push(Task::Evaluate(left));
                    }
                    Some(Node::UnwrapOr(UnwrapOrNode { set, default })) => {
                        tasks.push(Task::PartialUnwrapOr { default });
                        tasks.push(Task::Evaluate(set));
                    }
                    Some(Node::Function(_)) => {
                        evaluated.push(object.capture(statements, 0, context));
                    }
                    Some(Node::Literal(object)) => {
                        evaluated.push(object.into());
                    }
                    Some(Node::And(BinaryNode { left, right })) => {
                        tasks.push(Task::PartialAnd { right });
                        tasks.push(Task::Evaluate(left));
                    }
                    Some(Node::FunctionSelf(depth)) => evaluated.push(ObjectOrSetValues::Object(
                        context.function_self(depth as usize),
                    )),
                    Some(Node::Parameter(depth)) => {
                        evaluated.push(context.parameter_value(depth as usize));
                    }
                    Some(Node::Count(object)) => {
                        tasks.push(Task::Count);
                        tasks.push(Task::Evaluate(object));
                    }
                    Some(Node::QueryExists(query)) => {
                        tasks.push(Task::QueryExists);
                        tasks.push(Task::Evaluate(query.value));
                        tasks.push(Task::Evaluate(query.tag));
                        tasks.push(Task::Evaluate(query.subject));
                    }
                    Some(Node::QueryValues(query)) => {
                        tasks.push(Task::QueryValues);
                        tasks.push(Task::Evaluate(query.tag));
                        tasks.push(Task::Evaluate(query.subject));
                    }
                    Some(Node::QuerySubjects(query)) => {
                        tasks.push(Task::QuerySubjects);
                        tasks.push(Task::Evaluate(query.value));
                        tasks.push(Task::Evaluate(query.tag));
                    }
                    Some(Node::QueryTags(query)) => {
                        tasks.push(Task::QueryTags);
                        tasks.push(Task::Evaluate(query.value));
                        tasks.push(Task::Evaluate(query.subject));
                    }
                    Some(Node::QuerySubjectsAndTags(query)) => {
                        tasks.push(Task::QuerySubjectsAndTags);
                        tasks.push(Task::Evaluate(query.value));
                    }
                    Some(Node::QuerySubjectsAndValues(query)) => {
                        tasks.push(Task::QuerySubjectsAndValues);
                        tasks.push(Task::Evaluate(query.tag));
                    }
                    Some(Node::QueryTagsAndValues(query)) => {
                        tasks.push(Task::QueryTagsAndValues);
                        tasks.push(Task::Evaluate(query.subject));
                    }
                    Some(Node::Statements) => {
                        // Evaluate straight to an iterator over
                        // all statements:

                        evaluated.push(ObjectOrSetValues::SetValues(SetValues::Statements(
                            statements.iter_owned(),
                        )));
                    }
                    Some(Node::Equal(BinaryNode { left, right })) => {
                        tasks.push(Task::Equal);
                        tasks.push(Task::Evaluate(right));
                        tasks.push(Task::Evaluate(left));
                    }
                    Some(Node::Or(BinaryNode { left, right })) => {
                        tasks.push(Task::PartialOr { right });
                        tasks.push(Task::Evaluate(left));
                    }
                    Some(Node::Xor(BinaryNode { left, right })) => {
                        tasks.push(Task::Xor);
                        tasks.push(Task::Evaluate(right));
                        tasks.push(Task::Evaluate(left));
                    }
                    Some(Node::Not(object)) => {
                        tasks.push(Task::Not);
                        tasks.push(Task::Evaluate(object));
                    }
                    Some(Node::Add(BinaryNode { left, right })) => {
                        tasks.push(Task::Add);
                        tasks.push(Task::Evaluate(right));
                        tasks.push(Task::Evaluate(left));
                    }
                    Some(Node::Union(BinaryNode { left, right })) => {
                        tasks.push(Task::Union);
                        tasks.push(Task::Evaluate(right));
                        tasks.push(Task::Evaluate(left));
                    }
                    Some(Node::Map(MapNode {
                        set,
                        mapper_function,
                    })) => {
                        tasks.push(Task::Map);
                        tasks.push(Task::Evaluate(mapper_function));
                        tasks.push(Task::Evaluate(set));
                    }
                    Some(Node::Every(PredicateNode { set, predicate })) => {
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
                    Some(Node::Any(PredicateNode { set, predicate })) => {
                        tasks.push(Task::Any);

                        // Same as `Task::Every` but with false.
                        tasks.push(Task::PushLiteralFalse);

                        tasks.push(Task::Evaluate(predicate));
                        tasks.push(Task::Evaluate(set));
                    }
                    Some(Node::Filter(FilterNode {
                        set,
                        filter_function,
                    })) => {
                        tasks.push(Task::Filter);
                        tasks.push(Task::Evaluate(filter_function));
                        tasks.push(Task::Evaluate(set));
                    }
                    Some(Node::Less(BinaryNode { left, right })) => {
                        tasks.push(Task::Less);
                        tasks.push(Task::Evaluate(right));
                        tasks.push(Task::Evaluate(left));
                    }
                    Some(Node::If(IfNode {
                        condition,
                        then,
                        otherwise,
                    })) => {
                        tasks.push(Task::PartialIf { then, otherwise });
                        tasks.push(Task::Evaluate(condition));
                    }
                    None if let Object::Composite(Composite::Any(any_composite)) = object => {
                        let result = any_composite
                        .properties()
                        .map(|property| {
                            // TODO: DEBATE THIS BS
                            debug!("eval on property {property:?}");

                            let value =
                                property.value.evaluate(statements, context).into_object();

                            let result = if property.value == value {
                                Ok(())
                            } else {
                                value.is_valid(statements, false)
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

                            ObjectOrSetValues::Object(Object::Composite(Composite::Empty))
                        }, |mut properties| {
                            Object::Composite(Composite::new(&mut properties)).into()
                        });

                        evaluated.push(result);
                    }
                    None => evaluated.push(object.clone().into()),
                }
            }
            Task::PopContext => {
                context.pop();
            }
            Task::PushLiteralTrue => {
                evaluated.push(ObjectOrSetValues::Object(Composite::new_bool(true).into()));
            }
            Task::PushLiteralFalse => {
                evaluated.push(ObjectOrSetValues::Object(Composite::new_bool(false).into()));
            }
            Task::Call => {
                let callee = evaluated.pop().unwrap().into_object();
                let parameter_value = evaluated.pop().unwrap();

                tasks.push(Task::Evaluate(
                    callee.node_function_body(statements).unwrap(),
                ));

                context.push(FunctionContext {
                    function: callee,
                    parameter: parameter_value,
                });
            }
            Task::PartialAnd { right } => {
                let mut left = evaluated.pop().unwrap();

                if left.is_truthy(statements) {
                    tasks.push(Task::ToBoolean);
                    tasks.push(Task::Evaluate(right));
                } else {
                    evaluated.push(ObjectOrSetValues::Object(Composite::new_bool(false).into()));
                }
            }
            Task::Count => {
                let target = evaluated.pop().unwrap();

                evaluated.push(
                    Self::new_integer(target.set_values(statements).correct_count() as i128).into(),
                );
            }
            Task::QuerySubjectsAndTags => {
                let value = evaluated.pop().unwrap().into_object();

                evaluated.push(
                    SetValues::QuerySubjectsAndTags(statements.query_subjects_and_tags(value))
                        .into(),
                );
            }
            Task::QueryTagsAndValues => {
                // TODO: make this lazy
                let subject = evaluated.pop().unwrap().into_object();

                evaluated.push(
                    SetValues::QueryTagsAndValues(statements.query_tags_and_values(subject)).into(),
                );
            }
            Task::QueryTags => {
                let value = evaluated.pop().unwrap().into_object();
                // TODO: make this lazy
                let subject = evaluated.pop().unwrap().into_object();

                evaluated.push(SetValues::QueryTags(statements.query_tags(subject, value)).into());
            }
            Task::QueryValues => {
                let tag = evaluated.pop().unwrap().into_object();
                // TODO: make this lazy
                let subject = evaluated.pop().unwrap().into_object();

                evaluated.push(ObjectOrSetValues::SetValues(SetValues::QueryValues(
                    statements.query_values(subject, tag.clone()),
                )));
            }
            Task::QuerySubjects => {
                let value = evaluated.pop().unwrap().into_object();
                let tag = evaluated.pop().unwrap().into_object();

                evaluated
                    .push(SetValues::QuerySubjects(statements.query_subjects(tag, value)).into());
            }
            Task::QuerySubjectsAndValues => {
                let tag = evaluated.pop().unwrap().into_object();

                evaluated.push(
                    SetValues::QuerySubjectsAndValues(statements.query_subjects_and_values(tag))
                        .into(),
                );
            }
            Task::QueryExists => {
                let value = evaluated.pop().unwrap().into_object();
                let tag = evaluated.pop().unwrap().into_object();
                let subject = evaluated.pop().unwrap().into_object();

                evaluated.push(ObjectOrSetValues::Object(
                    Composite::new_bool(statements.exists(SimpleStatement {
                        subject,
                        tag,
                        value,
                    }))
                    .into(),
                ));
            }
            Task::ToBoolean => {
                let mut object = evaluated.pop().unwrap();

                evaluated.push(ObjectOrSetValues::Object(
                    Composite::new_bool(object.is_truthy(statements)).into(),
                ));
            }
            Task::Equal => {
                let right = evaluated.pop().unwrap().into_object();
                let left = evaluated.pop().unwrap().into_object();

                evaluated.push(Object::Composite(Composite::new_bool(left == right)).into());
            }
            Task::PartialOr { right } => {
                if evaluated.pop().unwrap().is_truthy(statements) {
                    evaluated.push(ObjectOrSetValues::Object(Composite::new_bool(true).into()));
                } else {
                    tasks.push(Task::ToBoolean);
                    tasks.push(Task::Evaluate(right));
                }
            }
            Task::Xor => {
                let right = evaluated.pop().unwrap().is_truthy(statements);
                let left = evaluated.pop().unwrap().is_truthy(statements);

                evaluated.push(ObjectOrSetValues::Object(
                    Composite::new_bool((left || right) && !(left && right)).into(),
                ));
            }
            Task::IsAbstract => {
                let object = evaluated.pop().unwrap();

                evaluated.push(
                    Object::Composite(Composite::new_bool(matches!(
                        object,
                        ObjectOrSetValues::Object(Object::Abstract(_))
                    )))
                    .into(),
                );
            }
            Task::Not => {
                let mut object = evaluated.pop().unwrap();

                evaluated.push(ObjectOrSetValues::Object(
                    Composite::new_bool(!object.is_truthy(statements)).into(),
                ));
            }
            Task::Add => {
                let right = evaluated.pop().unwrap().into_object();
                // TODO: (perf) maybe short circuit sets into UNDEFINED.
                let left = evaluated.pop().unwrap().into_object();

                evaluated.push(left.add(&right).into());
            }
            Task::Multiply => {
                // TODO: (perf) maybe short circuit sets into UNDEFINED.
                let right = evaluated.pop().unwrap().into_object();
                let left = evaluated.pop().unwrap().into_object();

                evaluated.push(left.multiply(&right).into());
            }
            Task::Union => {
                let right = evaluated.pop().unwrap();
                let left = evaluated.pop().unwrap();

                evaluated.push(ObjectOrSetValues::SetValues(SetValues::Union {
                    left: Box::new(left.set_values(statements)),
                    right: Box::new(right.set_values(statements)),
                }));
            }
            Task::Map => {
                let mapper = evaluated.pop().unwrap().into_object();
                let set = evaluated.pop().unwrap().set_values(statements);

                evaluated.push(ObjectOrSetValues::SetValues(SetValues::Map {
                    statements: statements.clone(),
                    set: Box::new(set),
                    mapper_function: mapper,
                }));
            }
            Task::Filter => {
                let filter = evaluated.pop().unwrap().into_object();
                let set = evaluated.pop().unwrap().set_values(statements);

                evaluated.push(ObjectOrSetValues::SetValues(SetValues::Filter {
                    statements: statements.clone(),
                    set: Box::new(set),
                    filter_function: filter,
                }));
            }
            Task::Every => {
                let mut previous_result = evaluated.pop().unwrap();
                let predicate = evaluated.pop().unwrap().into_object();
                let mut set = evaluated.pop().unwrap().set_values(statements);

                if !previous_result.is_truthy(statements) {
                    // Previous run did not succeed.

                    evaluated.push(ObjectOrSetValues::Object(Composite::new_bool(false).into()));
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

                    evaluated.push(ObjectOrSetValues::Object(Composite::new_bool(true).into()));
                }
            }
            Task::Any => {
                let mut previous_result = evaluated.pop().unwrap();
                let predicate = evaluated.pop().unwrap().into_object();
                let mut set = evaluated.pop().unwrap().set_values(statements);

                if previous_result.is_truthy(statements) {
                    // Previous run succeeded. Yay.

                    evaluated.push(ObjectOrSetValues::Object(Composite::new_bool(true).into()));
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

                    evaluated.push(ObjectOrSetValues::Object(Composite::new_bool(false).into()));
                }
            }
            Task::Less => {
                let right = evaluated.pop().unwrap();
                let left = evaluated.pop().unwrap();

                println!("is {left:?} < {right:?}?");

                evaluated.push(
                    Object::Composite(Composite::new_bool(match (left, right) {
                        (ObjectOrSetValues::Object(left), ObjectOrSetValues::Object(right))
                            if let Some(left) = left.integer()
                                && let Some(right) = right.integer() =>
                        {
                            left < right
                        }
                        // TODO: more things here
                        (left, right) => {
                            left.set_values(statements).correct_count()
                                < right.set_values(statements).correct_count()
                        }
                    }))
                    .into(),
                );
            }
            Task::PartialIf { then, otherwise } => {
                if evaluated.pop().unwrap().is_truthy(statements) {
                    tasks.push(Task::Evaluate(then));
                } else {
                    tasks.push(Task::Evaluate(otherwise));
                }
            }
            Task::PartialUnwrapOr { default } => {
                let mut set_values = evaluated.pop().unwrap().set_values(statements);

                if let Some(inner) = set_values.next_and_last() {
                    evaluated.push(ObjectOrSetValues::Object(inner));
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
