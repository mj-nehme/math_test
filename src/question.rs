use crate::{question_type::{QuestionType}};

pub trait Question {
    fn new(question_type: QuestionType, max: i32) -> Self;
    fn choose_variable(min: i32, max: i32) -> i32;
    fn get_expected_answer(&self) -> i32;
    fn calculate(&self, question_type: QuestionType) -> i32;
    fn print(&self, question_type: QuestionType);
}
