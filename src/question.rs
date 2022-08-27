use crate::question_type::QuestionType;
use std::fmt::Debug;
use std::fmt::Display;

/// Question Trait is the supertrait that can be either an Equation or an Operation
///
///            Question
///      _________|_________
///      |                 |
///  Operation         Equation
///               _________|_________
///               |                 |
///          OneVariable      TwoVariables
///
pub trait Question: Display {
    type Output: PartialEq + Debug;

    fn new(question_type: QuestionType, max: i32) -> Self;
    fn get_correct_answer(&self) -> Self::Output;
    fn calculate(&self) -> Self::Output;
    fn post_to_cmd(&self) -> bool;
    fn read_answer_from_cmd() -> Self::Output;
    fn correct_answer_to_string(&self) -> String;
    fn verify_answer(correct_answer: Self::Output, answer: Self::Output) -> bool {
        answer == correct_answer
    }
    fn print(&self) {
        println!("{}", self.to_string());
    }
}
