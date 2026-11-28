mod context;
mod evaluator;
#[cfg(test)]
mod tests;

pub use context::{Argument, EvaluationContext, FunctionContext, Thunk};
pub use evaluator::{Evaluator, RunOutcome, evaluate};
