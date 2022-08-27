use std::fmt::Display;

use crate::question_type::QuestionType;

pub trait Question: Display {
    type Output;

    fn new(question_type: QuestionType, max: i32) -> Self;
    fn get_expected_answer(&self) -> Self::Output;
    fn calculate(&self) -> Self::Output;
    fn post(&self) -> bool;
    fn print(&self) {
        println!("{}", self.to_string());
    }
}
