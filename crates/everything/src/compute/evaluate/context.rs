use crate::compute::{LazyObject, MaybeUndefined, Node};

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

    /// Gets the parameter value of the function context at the specified relative
    /// depth, or returns undefined if the index is out of bounds.
    #[must_use]
    pub fn parameter_value(&self, relative_depth: usize) -> MaybeUndefined<LazyObject> {
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
    pub parameter: LazyObject,
}
