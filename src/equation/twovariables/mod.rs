use crate::{
    question::Question,
    question_type::{EquationType, QuestionType},
};
use rand::Rng;

#[derive(Debug)]
pub struct TwoVariables {
    a: i32,
    x: i32,
    b: i32,
    result: i32,
}

impl Question for TwoVariables {
    fn new(operation: QuestionType, max: i32) -> Self {
        panic!("TwoVariables not implemented yet");
    }

    fn choose_variable(min: i32, max: i32) -> i32 {
        1
    }

    fn get_result(&self) -> i32 {
        self.result
    }

    fn calculate(&self, question_type: QuestionType) -> i32 {
        1
    }

    fn print(&self, question_type: QuestionType) {}
}

#[cfg(test)]
mod tests;
