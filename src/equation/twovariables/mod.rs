use crate::{
    question::Question,
    question_type::{EquationType, QuestionType},
};
use rand::Rng;

use super::Equation;

#[derive(Debug)]
pub struct TwoVariables {
    a: i32,
    x: i32,
    b: i32,
    result: i32,
}

impl Question for TwoVariables {
    fn new(question_type: QuestionType, max: i32) -> Self {
        todo!()
    }

    fn choose_variable(min: i32, max: i32) -> i32 {
        todo!()
    }

    fn get_expected_answer(&self) -> i32 {
        todo!()
    }

    fn calculate(&self, question_type: QuestionType) -> i32 {
        todo!()
    }

    fn print(&self, question_type: QuestionType) {
        todo!()
    }
}

impl Equation for TwoVariables {
    fn new(equation: QuestionType, max: i32) -> Self {
        panic!("TwoVariables not implemented yet");
    }

    fn choose_variable(min: i32, max: i32) -> i32 {
        1
    }

    fn get_expected_answer(&self) -> i32 {
        self.result
    }

    fn calculate(&self, question_type: QuestionType) -> i32 {
        1
    }

    fn print(&self, question_type: QuestionType) {}
}

#[cfg(test)]
mod tests;
