use std::sync::Arc;

use crate::compute::{LazyObject, MaybeUndefined, Node};

/// The argument bound to a function parameter.
///
/// A call argument is not evaluated eagerly. It is kept as a closed expression
/// until the parameter is referenced. Already evaluated values, for example the
/// elements passed to a mapper or filter, are kept as values instead of being
/// materialised into an object first.
#[derive(Debug, Clone)]
pub enum Argument {
    /// An already evaluated value.
    Value(LazyObject),
    /// A closed expression that is evaluated when the parameter is referenced.
    Unevaluated(Arc<Node>),
}

/// An evaluation context is a stack of function contexts,
/// each holding a function and the parameter argument from
/// call.
///
/// Instead of capturing every variable on each call,
/// the evaluation engine can just lookup parameter arguments
/// from this Composite.
#[derive(Debug, Clone, Default)]
pub struct EvaluationContext {
    stack: Vec<FunctionContext>,
}

impl EvaluationContext {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)] // function should not panic (!)
    pub fn function_context(&self, relative_depth: usize) -> Option<&FunctionContext> {
        self.stack
            .len()
            .checked_sub(1 + relative_depth)
            .map(|index| self.stack.get(index).unwrap())
    }

    /// Gets the parameter argument of the function context at the specified
    /// relative depth, or returns undefined if the index is out of bounds.
    #[must_use]
    pub fn parameter(&self, relative_depth: usize) -> MaybeUndefined<Argument> {
        self.function_context(relative_depth)
            .map(|context| context.parameter.clone())
            .into()
    }

    /// Gets the function itself of the function context at the specified relative
    /// depth, or returns undefined if the index is out of bounds.
    #[must_use]
    pub fn function_self(&self, relative_depth: usize) -> MaybeUndefined<Node> {
        self.function_context(relative_depth)
            .map(|context| context.function.clone())
            .into()
    }

    /// Pushes a new function context onto the stack.
    pub fn push(&mut self, fc: FunctionContext) {
        self.stack.push(fc);
    }

    /// Pops the top-most function context off the stack.
    pub fn pop(&mut self) {
        self.stack.pop();
    }

    #[must_use]
    pub fn last(&self) -> Option<&FunctionContext> {
        self.stack.last()
    }

    #[must_use]
    pub fn length(&self) -> usize {
        self.stack.len()
    }
}

#[derive(Debug, Clone)]
pub struct FunctionContext {
    pub function: Node,
    pub parameter: Argument,
}
