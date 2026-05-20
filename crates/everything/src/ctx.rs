use everything_structures::{Object, Structure};
use tracing::instrument;

use crate::LazyObject;

/// An evaluation context is a stack of function contexts,
/// each holding a function and the parameter value from
/// call.
///
/// Instead of capturing every variable on each call,
/// the evaluation engine can just lookup parameter values
/// from this structure.
#[derive(Debug, Clone, Default)]
pub struct EvaluationContext {
    stack: Vec<FunctionContext>,
}

impl EvaluationContext {
    pub fn function_context(&self, relative_depth: usize) -> Option<&FunctionContext> {
        self.stack
            .len()
            .checked_sub(1 + relative_depth)
            .map(|index| self.stack.get(index).unwrap())
    }

    #[instrument(ret)]
    pub fn parameter_value(&self, relative_depth: usize) -> LazyObject {
        self.function_context(relative_depth)
            .map_or(Object::Structure(Structure::Empty).into(), |context| {
                context.parameter.clone()
            })
    }

    #[instrument(ret)]
    pub fn function_self(&self, relative_depth: usize) -> Object {
        self.function_context(relative_depth)
            .map_or(Object::Structure(Structure::Empty), |context| {
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
