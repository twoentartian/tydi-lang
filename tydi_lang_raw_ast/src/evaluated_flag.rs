use deep_clone::DeepClone;

#[derive(Clone, Debug)]
pub enum EvaluatedState {
    NotEvaluate,
    Evaluating,
    Evaluated,
}

pub trait EvaluatedFlag {
    fn get_evaluate_flag(&self) -> EvaluatedState;
    fn set_evaluate_flag(&mut self, flag: EvaluatedState);
}

impl DeepClone for EvaluatedState {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}