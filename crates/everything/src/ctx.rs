use everything_structures::Object;

#[derive(Default, Debug)]
pub struct EvaluationContext {
    pub parameters: Vec<Object>,
    pub functions: Vec<Object>,
}
