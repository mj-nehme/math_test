use std::fmt;

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

impl fmt::Display for TwoVariables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x + {} = {}", self.a, self.b, self.result)
    }
}

impl Equation for TwoVariables {}

impl Question for TwoVariables {
    fn new(equation: QuestionType, max: i32) -> Self {
        panic!("TwoVariables not implemented yet");
    }

    fn get_expected_answer(&self) -> i32 {
        self.result
    }

    fn calculate(&self) -> i32 {
        1
    }

    fn print(&self) {}
}

#[cfg(test)]
mod tests;
