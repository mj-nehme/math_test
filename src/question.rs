use std::fmt::Display;

use crate::{number, question_type::QuestionType};

pub trait Question : Display {
    fn new(question_type: QuestionType, max: i32) -> Self;
    fn get_expected_answer(&self) -> i32;
    fn calculate(&self) -> i32;
    fn print(&self);

    fn post(&self) -> bool {
        let expected = self.get_expected_answer();
        self.print();

        let answer = number::read_number();

        if answer == expected {
            println!("Correct Answer!");
            return true;
        } else {
            println!("Wrong! The correct Answer was {}", expected);
            return false;
        }
    }

}
