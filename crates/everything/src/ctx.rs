use everything_structures::{Object, Structure};
use tracing::instrument;

#[derive(Debug, Clone, Default)]
pub struct EvaluationContext {
    stack: Vec<FunctionContext>,
}

impl EvaluationContext {
    #[instrument(ret)]
    pub fn parameter_value(&self, relative_depth: usize) -> Object {
        self.stack
            .len()
            .checked_sub(1 + relative_depth)
            .map(|index| self.stack.get(index).unwrap().parameter.clone())
            .unwrap_or(Object::Structure(Structure::Empty))
    }

    pub fn push(&mut self, fc: FunctionContext) {
        self.stack.push(fc);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}

#[derive(Debug, Clone)]
pub struct FunctionContext {
    pub function: Object,
    pub parameter: Object,
}
