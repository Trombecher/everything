use std::sync::Arc;

use num_bigint::BigInt;

use super::context::{Argument, EvaluationContext, FunctionContext, Thunk};
use crate::{
    compute::{
        LazyObject, LazySet, LazySetItems, MaybeUndefined, Node, SetValue, capture,
        capture_argument, operations,
    },
    iter::IteratorExtNextAndLast,
    knowledge::Knowledge,
    model::{Atom, Composite, Construction, Object},
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
    SetCount,
    PropertiesOf,
    HasASetItem,
    /// Evaluates the subject and short-circuits the property child if the
    /// subject is a construction.
    PartialHasProperty {
        property: Arc<Node>,
    },
    /// Checks whether the evaluated property belongs to the subject.
    HasProperty {
        subject: Box<SetValue>,
    },
    QueryAtomsWithProperty,
    QueryAtomsWithConstructor,
    QueryConstructionValues,
    QueryConstructions,
    /// Evaluates the set and short-circuits the element child if the set is a
    /// construction.
    PartialSetContains {
        element: Arc<Node>,
    },
    /// Checks whether the evaluated element is an item of the set.
    SetContains {
        set: Box<SetValue>,
    },
    NewConstruction,
    /// Collects the results of the preceding child evaluations into a set.
    NewSet {
        count: usize,
    },
    /// Collects the results of the preceding child evaluations into a
    /// composite.
    NewComposite {
        count: usize,
    },
    Equal,
    PartialOr {
        second: Arc<Node>,
    },
    Xor,
    Not,
    Add,
    Multiply,
    SetUnion,
    SetIntersection,
    /// Evaluates the first child and short-circuits the second one if the
    /// first result is empty.
    PartialSetDifference {
        second: Arc<Node>,
    },
    SetDifference,
    SetMap,
    SetFilter,
    SetEvery,
    /// One iteration of an `every` evaluation. Calls the predicate for the
    /// next element, if there is one.
    EveryStep {
        values: Box<LazySetItems>,
        predicate: Node,
    },
    /// Checks the predicate result produced by the preceding `EveryStep`.
    EveryCheck {
        values: Box<LazySetItems>,
        predicate: Node,
    },
    SetAny,
    /// One iteration of an `any` evaluation. Calls the predicate for the
    /// next element, if there is one.
    AnyStep {
        values: Box<LazySetItems>,
        predicate: Node,
    },
    /// Checks the predicate result produced by the preceding `AnyStep`.
    AnyCheck {
        values: Box<LazySetItems>,
        predicate: Node,
    },
    SetFold,
    /// One iteration of a fold. The accumulator is taken from the evaluated
    /// stack.
    FoldStep {
        values: Box<LazySetItems>,
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
    Call {
        argument: Argument,
    },
    /// Caches the result of an evaluated thunk.
    CacheThunk {
        thunk: Arc<Thunk>,
    },
    PopContext,
    SetUnwrap,
    ConstructorOfConstruction,
    ValueOfConstruction,
    IsAtom,
    IsConstruction,
    IsComposite,
    IsNode,
}

/// Pushes a thunk's cached value, or schedules its expression and the task
/// that caches the result.
fn evaluate_thunk(thunk: &Arc<Thunk>, tasks: &mut Vec<Task>, evaluated: &mut Vec<LazyObject>) {
    if let Some(value) = thunk.value() {
        evaluated.push(value.clone());
    } else {
        tasks.push(Task::CacheThunk {
            thunk: Arc::clone(thunk),
        });
        tasks.push(Task::Evaluate(Arc::new(thunk.expression().clone())));
    }
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
        Node::IsComposite(inner) => {
            tasks.push(Task::IsComposite);
            tasks.push(Task::Evaluate(Arc::clone(inner)));
        }
        Node::IsConstruction(inner) => {
            tasks.push(Task::IsConstruction);
            tasks.push(Task::Evaluate(Arc::clone(inner)));
        }
        Node::IsNode(inner) => {
            tasks.push(Task::IsNode);
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
        Node::SetFold {
            set,
            folder,
            initial,
        } => {
            tasks.push(Task::SetFold);
            tasks.push(Task::Evaluate(Arc::clone(initial)));
            tasks.push(Task::Evaluate(Arc::clone(folder)));
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::SetContains { element, set } => {
            tasks.push(Task::PartialSetContains {
                element: Arc::clone(element),
            });
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::NewConstruction { constructor, value } => {
            tasks.push(Task::NewConstruction);
            tasks.push(Task::Evaluate(Arc::clone(value)));
            tasks.push(Task::Evaluate(Arc::clone(constructor)));
        }
        Node::NewSet { items } => {
            tasks.push(Task::NewSet { count: items.len() });

            // The items are evaluated in order: the stack is LIFO, so they
            // have to be scheduled in reverse.
            for item in items.iter().rev() {
                tasks.push(Task::Evaluate(Arc::clone(item)));
            }
        }
        Node::NewComposite { properties } => {
            tasks.push(Task::NewComposite {
                count: properties.len(),
            });

            // The properties are evaluated in order: the stack is LIFO, so
            // they have to be scheduled in reverse.
            for property in properties.iter().rev() {
                tasks.push(Task::Evaluate(Arc::clone(property)));
            }
        }
        Node::IsAtom(inner) => {
            tasks.push(Task::IsAtom);
            tasks.push(Task::Evaluate(Arc::clone(inner)));
        }
        Node::Call { callee, with } => {
            // The argument is not evaluated here. It is closed over the
            // caller's context and evaluated only if the parameter is used.
            tasks.push(Task::Call {
                argument: Argument::Unevaluated(Arc::new(Thunk::new(capture_argument(
                    with,
                    evaluation_context,
                    knowledge,
                )))),
            });
            tasks.push(Task::Evaluate(Arc::clone(callee)));
        }
        Node::Multiply {
            first: left,
            second: right,
        } => {
            tasks.push(Task::Multiply);
            tasks.push(Task::Evaluate(Arc::clone(right)));
            tasks.push(Task::Evaluate(Arc::clone(left)));
        }
        Node::SetUnwrap(set) => {
            tasks.push(Task::SetUnwrap);
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::Function { .. } => {
            evaluated.push(capture(node, 0, evaluation_context, knowledge).into());
        }
        Node::Literal(object) => {
            evaluated.push(object.clone().into());
        }
        Node::Thunk(thunk) => {
            evaluate_thunk(thunk, tasks, evaluated);
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
        Node::Parameter { depth } => match evaluation_context.parameter(*depth as usize) {
            MaybeUndefined::Defined(Argument::Value(value)) => evaluated.push(value),
            MaybeUndefined::Defined(Argument::Unevaluated(thunk)) => {
                evaluate_thunk(&thunk, tasks, evaluated);
            }
            MaybeUndefined::Undefined => evaluated.push(UNDEFINED),
        },
        Node::SetCount(object) => {
            tasks.push(Task::SetCount);
            tasks.push(Task::Evaluate(Arc::clone(object)));
        }
        Node::PropertiesOf(object) => {
            tasks.push(Task::PropertiesOf);
            tasks.push(Task::Evaluate(Arc::clone(object)));
        }
        Node::HasASetItem(object) => {
            tasks.push(Task::HasASetItem);
            tasks.push(Task::Evaluate(Arc::clone(object)));
        }
        Node::HasProperty { subject, property } => {
            tasks.push(Task::PartialHasProperty {
                property: Arc::clone(property),
            });
            tasks.push(Task::Evaluate(Arc::clone(subject)));
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
        Node::SetUnion { first, second } => {
            tasks.push(Task::SetUnion);
            tasks.push(Task::Evaluate(Arc::clone(second)));
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::SetIntersection { first, second } => {
            tasks.push(Task::SetIntersection);
            tasks.push(Task::Evaluate(Arc::clone(second)));
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::SetDifference { first, second } => {
            tasks.push(Task::PartialSetDifference {
                second: Arc::clone(second),
            });
            tasks.push(Task::Evaluate(Arc::clone(first)));
        }
        Node::SetMap {
            set,
            mapper_function,
        } => {
            tasks.push(Task::SetMap);
            tasks.push(Task::Evaluate(Arc::clone(mapper_function)));
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::SetEvery { set, predicate } => {
            tasks.push(Task::SetEvery);
            tasks.push(Task::Evaluate(Arc::clone(predicate)));
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::SetAny { set, predicate } => {
            tasks.push(Task::SetAny);
            tasks.push(Task::Evaluate(Arc::clone(predicate)));
            tasks.push(Task::Evaluate(Arc::clone(set)));
        }
        Node::SetFilter {
            set,
            filter_function,
        } => {
            tasks.push(Task::SetFilter);
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

/// The outcome of running an [`Evaluator`] for a bounded number of steps.
///
/// Evaluation does not fail; it either finishes or has more steps left. When
/// the budget is exhausted, the evaluator is handed back so that the evaluation
/// can be resumed later.
#[derive(Debug)]
#[must_use]
pub enum RunOutcome {
    /// The evaluation finished and produced this result.
    Finished(LazyObject),
    /// The budget was exhausted before the evaluation finished.
    Paused(Evaluator),
}

impl RunOutcome {
    /// Returns `true` iff the evaluation finished.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Finished(_))
    }
}

/// A resumable evaluator.
///
/// The evaluator is a stack machine. It can be run for a bounded number of
/// steps, in which case its intermediate state is returned so that the
/// evaluation can be resumed later.
pub struct Evaluator {
    knowledge: Knowledge,
    context: EvaluationContext,
    tasks: Vec<Task>,
    evaluated: Vec<LazyObject>,
    steps: u64,
}

impl std::fmt::Debug for Evaluator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Evaluator")
            .field("tasks", &self.tasks.len())
            .field("evaluated", &self.evaluated.len())
            .field("steps", &self.steps)
            .finish_non_exhaustive()
    }
}

impl Evaluator {
    /// Creates an evaluator for the given node and knowledge.
    #[must_use]
    pub fn new(node: &Node, knowledge: &Knowledge) -> Self {
        Self {
            knowledge: knowledge.clone(),
            context: EvaluationContext::default(),
            tasks: vec![Task::Evaluate(Arc::new(node.clone()))],
            evaluated: Vec::new(),
            steps: 0,
        }
    }

    /// Returns the number of steps this evaluator has executed.
    #[must_use]
    pub fn steps(&self) -> u64 {
        self.steps
    }

    /// Returns `true` iff there is no work left.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Runs at most `budget` further steps.
    ///
    /// One step is one task popped from the evaluator's work stack.
    ///
    /// If the evaluation finishes within the budget, [`RunOutcome::Finished`]
    /// is returned. Otherwise, [`RunOutcome::Paused`] carries the evaluator so
    /// that the evaluation can be resumed with another call to [`Self::run`].
    ///
    /// The budget bounds only the strict evaluation. Deferred sets that are
    /// returned as [`LazyObject::Set`] are materialised and iterated later
    /// through fresh, unbudgeted evaluators, as are knowledge constraint
    /// checks.
    pub fn run(mut self, budget: u64) -> RunOutcome {
        for _ in 0..budget {
            let Some(task) = self.tasks.pop() else {
                return RunOutcome::Finished(self.finish());
            };

            self.execute(task);
        }

        if self.is_complete() {
            RunOutcome::Finished(self.finish())
        } else {
            RunOutcome::Paused(self)
        }
    }

    /// Runs the evaluator until it finishes.
    #[must_use]
    pub fn run_to_completion(mut self) -> LazyObject {
        loop {
            match self.run(u64::MAX) {
                RunOutcome::Finished(result) => return result,
                RunOutcome::Paused(next) => self = next,
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn execute(&mut self, task: Task) {
        self.steps += 1;

        let Self {
            knowledge,
            context: evaluation_context,
            tasks,
            evaluated,
            ..
        } = self;

        match task {
            Task::Evaluate(node) => {
                resolve_evaluate_node(&node, knowledge, evaluation_context, tasks, evaluated);
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
            Task::Call { argument } => {
                let callee_lazy_object = evaluated.pop().unwrap();

                if let Some(Node::Function { body, object }) = callee_lazy_object
                    .try_into_node(knowledge)
                    .map(Node::into_plain)
                {
                    tasks.push(Task::PopContext);
                    tasks.push(Task::Evaluate(Arc::clone(&body)));

                    evaluation_context.push(FunctionContext {
                        function: Node::Function { body, object },
                        parameter: argument,
                    });
                } else {
                    evaluated.push(UNDEFINED);
                }
            }
            Task::CacheThunk { thunk } => {
                let value = evaluated.pop().unwrap();

                thunk.cache(value.clone());
                evaluated.push(value);
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
            Task::SetCount => {
                let target = evaluated.pop().unwrap();

                evaluated.push(match target.into_set() {
                    Some(target) => {
                        Object::new_integer(BigInt::from(target.item_count(knowledge))).into()
                    }
                    None => UNDEFINED,
                });
            }
            Task::PropertiesOf => {
                let object = evaluated.pop().unwrap();

                evaluated.push(match object.into_set() {
                    Some(target) => LazyObject::Set(LazySet::PropertiesOf {
                        target: Box::new(target),
                    }),
                    None => UNDEFINED,
                });
            }
            Task::HasASetItem => {
                let object = evaluated.pop().unwrap();

                evaluated.push(match object.into_set() {
                    Some(set) => LazyObject::Eager(set.has_at_least_one_item(knowledge).into()),
                    None => UNDEFINED,
                });
            }
            Task::PartialHasProperty { property } => {
                let subject = evaluated.pop().unwrap();

                match subject.into_set() {
                    Some(subject) => {
                        tasks.push(Task::HasProperty {
                            subject: Box::new(subject),
                        });
                        tasks.push(Task::Evaluate(property));
                    }
                    None => {
                        // A construction is not a subject; the property child
                        // is not evaluated.
                        evaluated.push(UNDEFINED);
                    }
                }
            }
            Task::HasProperty { subject } => {
                let property = evaluated.pop().unwrap().into_object(knowledge);

                evaluated.push(LazyObject::Eager(
                    subject.has_property(&property, knowledge).into(),
                ));
            }
            Task::QueryAtomsWithProperty => {
                let property = evaluated.pop().unwrap().into_object(knowledge);

                evaluated.push(LazyObject::Set(LazySet::AtomsWithProperty { property }));
            }
            Task::QueryConstructionValues => {
                let property_constructor = evaluated.pop().unwrap().into_object(knowledge);
                let object = evaluated.pop().unwrap();

                evaluated.push(match object.into_set() {
                    Some(target) => LazyObject::Set(LazySet::ConstructionValues {
                        target: Box::new(target),
                        property_constructor,
                    }),
                    None => UNDEFINED,
                });
            }
            Task::QueryAtomsWithConstructor => {
                let constructor = evaluated.pop().unwrap().into_object(knowledge);

                evaluated.push(LazyObject::Set(LazySet::AtomsWithConstructor {
                    constructor,
                }));
            }
            Task::QueryConstructions => {
                let constructor = evaluated.pop().unwrap().into_object(knowledge);

                evaluated.push(LazyObject::Set(LazySet::Constructions { constructor }));
            }
            Task::PartialSetContains { element } => {
                let set = evaluated.pop().unwrap();

                match set.into_set() {
                    Some(set) => {
                        tasks.push(Task::SetContains { set: Box::new(set) });
                        tasks.push(Task::Evaluate(element));
                    }
                    None => {
                        // A construction is not a set; the element child is
                        // not evaluated.
                        evaluated.push(UNDEFINED);
                    }
                }
            }
            Task::SetContains { set } => {
                let element = evaluated.pop().unwrap().into_object(knowledge);

                evaluated.push(LazyObject::Eager(set.has_item(&element, knowledge).into()));
            }
            Task::NewConstruction => {
                let value = evaluated.pop().unwrap().into_object(knowledge);
                let constructor = evaluated.pop().unwrap().into_object(knowledge);

                evaluated.push(LazyObject::Eager(
                    Construction::new(constructor, value).into(),
                ));
            }
            Task::NewSet { count } => {
                let mut items = Vec::with_capacity(count);

                for _ in 0..count {
                    items.push(evaluated.pop().unwrap().into_object(knowledge));
                }

                evaluated.push(LazyObject::Eager(Composite::from_set_items(&items).into()));
            }
            Task::NewComposite { count } => {
                let mut properties = Vec::with_capacity(count);

                for _ in 0..count {
                    properties.push(evaluated.pop().unwrap().into_object(knowledge));
                }

                evaluated.push(LazyObject::Eager(
                    Composite::from(properties.as_mut_slice()).into(),
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
            Task::IsComposite => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(object.is_composite().into()));
            }
            Task::IsNode => {
                let object = evaluated.pop().unwrap();

                evaluated.push(LazyObject::Eager(
                    object.try_into_node(knowledge).is_some().into(),
                ));
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
            Task::SetUnion => {
                let first = evaluated.pop().unwrap();
                let second = evaluated.pop().unwrap();

                evaluated.push(match (first.into_set(), second.into_set()) {
                    (Some(first), Some(second)) => LazyObject::Set(LazySet::Union {
                        first: Box::new(second),
                        second: Box::new(first),
                    }),
                    _ => UNDEFINED,
                });
            }
            Task::SetIntersection => {
                let first = evaluated.pop().unwrap();
                let second = evaluated.pop().unwrap();

                evaluated.push(match (first.into_set(), second.into_set()) {
                    (Some(first), Some(second)) => LazyObject::Set(LazySet::Intersection {
                        first: Box::new(second),
                        second: Box::new(first),
                    }),
                    _ => UNDEFINED,
                });
            }
            Task::PartialSetDifference { second } => {
                let first = evaluated.pop().unwrap();

                match first.into_set() {
                    None => {
                        evaluated.push(UNDEFINED);
                    }
                    Some(first) if !first.has_at_least_one_item(knowledge) => {
                        // Nothing can be removed from an empty set, so the second
                        // child is never evaluated.
                        evaluated.push(LazyObject::Eager(Object::Composite(Composite::empty())));
                    }
                    Some(first) => {
                        evaluated.push(first.into());
                        tasks.push(Task::SetDifference);
                        tasks.push(Task::Evaluate(second));
                    }
                }
            }
            Task::SetDifference => {
                let without = evaluated.pop().unwrap();
                let source = evaluated.pop().unwrap();

                evaluated.push(match (source.into_set(), without.into_set()) {
                    (Some(source), Some(without)) => LazyObject::Set(LazySet::Difference {
                        source: Box::new(source),
                        without: Box::new(without),
                    }),
                    _ => UNDEFINED,
                });
            }
            Task::SetMap => {
                let mapper_function = evaluated.pop().unwrap();
                let set = evaluated.pop().unwrap();

                let Some(set) = set.into_set() else {
                    evaluated.push(UNDEFINED);

                    return;
                };

                if let Some(mapper_function @ Node::Function { .. }) = mapper_function
                    .try_into_node(knowledge)
                    .map(Node::into_plain)
                {
                    evaluated.push(LazyObject::Set(LazySet::Map {
                        target: Box::new(set),
                        mapper_function,
                    }));
                } else {
                    evaluated.push(UNDEFINED);
                }
            }
            Task::SetFilter => {
                let filter_function = evaluated.pop().unwrap();
                let set = evaluated.pop().unwrap();

                let Some(set) = set.into_set() else {
                    evaluated.push(UNDEFINED);

                    return;
                };

                if let Some(filter_function @ Node::Function { .. }) = filter_function
                    .try_into_node(knowledge)
                    .map(Node::into_plain)
                {
                    evaluated.push(LazyObject::Set(LazySet::Filter {
                        target: Box::new(set),
                        filter_function,
                    }));
                } else {
                    evaluated.push(UNDEFINED);
                }
            }
            Task::SetEvery => {
                let predicate = evaluated.pop().unwrap();
                let set = evaluated.pop().unwrap();

                let Some(set) = set.into_set() else {
                    evaluated.push(UNDEFINED);

                    return;
                };

                if !set.has_at_least_one_item(knowledge) {
                    // The predicate is never called and may therefore be invalid.
                    evaluated.push(LazyObject::Eager(true.into()));
                } else if let Some(predicate @ Node::Function { .. }) =
                    predicate.try_into_node(knowledge).map(Node::into_plain)
                {
                    tasks.push(Task::EveryStep {
                        values: Box::new(set.into_items(knowledge)),
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
                    tasks.push(Task::Call {
                        argument: Argument::Value(LazyObject::Eager(element)),
                    });
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
            Task::SetAny => {
                let predicate = evaluated.pop().unwrap();
                let set = evaluated.pop().unwrap();

                let Some(set) = set.into_set() else {
                    evaluated.push(UNDEFINED);

                    return;
                };

                if !set.has_at_least_one_item(knowledge) {
                    // The predicate is never called and may therefore be invalid.
                    evaluated.push(LazyObject::Eager(false.into()));
                } else if let Some(predicate @ Node::Function { .. }) =
                    predicate.try_into_node(knowledge).map(Node::into_plain)
                {
                    tasks.push(Task::AnyStep {
                        values: Box::new(set.into_items(knowledge)),
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
                    tasks.push(Task::Call {
                        argument: Argument::Value(LazyObject::Eager(element)),
                    });
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
            Task::SetFold => {
                let initial = evaluated.pop().unwrap();
                let folder = evaluated.pop().unwrap();
                let set = evaluated.pop().unwrap();

                let Some(set) = set.into_set() else {
                    evaluated.push(UNDEFINED);

                    return;
                };

                if !set.has_at_least_one_item(knowledge) {
                    // The folder is never called and may therefore be invalid.
                    evaluated.push(initial);
                } else if let Some(folder @ Node::Function { .. }) =
                    folder.try_into_node(knowledge).map(Node::into_plain)
                {
                    tasks.push(Task::FoldStep {
                        values: Box::new(set.into_items(knowledge)),
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

                    tasks.push(Task::Call {
                        argument: Argument::Value(accumulator),
                    });
                    evaluated.push(LazyObject::Node(folder));
                } else {
                    evaluated.push(accumulator);
                }
            }
            Task::FoldApply { element } => {
                let callee = evaluated.pop().unwrap();

                tasks.push(Task::Call {
                    argument: Argument::Value(LazyObject::Eager(element)),
                });
                evaluated.push(callee);
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
            Task::SetUnwrap => {
                let inner = evaluated.pop().unwrap();

                evaluated.push(match inner.into_set() {
                    Some(inner) => match inner.into_items(knowledge).next_and_last() {
                        Some(inner) => LazyObject::Eager(inner),
                        None => UNDEFINED,
                    },
                    None => UNDEFINED,
                });
            }
        }
    }

    /// Finishes the evaluation and checks the machine invariants.
    ///
    /// # Panics
    ///
    /// On implementation bugs.
    fn finish(mut self) -> LazyObject {
        let last = self.evaluated.pop().unwrap();

        if let Some(top) = self.evaluated.last() {
            unreachable!("internal bug: there is still {top:?} on the stack")
        }

        if let Some(top) = self.context.last() {
            unreachable!("internal bug: there is still {top:?} on the evaluation context stack")
        }

        last
    }
}

/// Evaluates an object in the given knowledge.
///
/// # Panics
///
/// On implementation bugs.
#[must_use]
pub fn evaluate(node: &Node, knowledge: &Knowledge) -> LazyObject {
    Evaluator::new(node, knowledge).run_to_completion()
}
