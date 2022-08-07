use std::str;

use crate::{
    question::Question,
    question_type::{self, OperationType, QuestionType},
};
use rand::Rng;

#[derive(Debug)]
pub struct Operation {
    pub a: i32,
    pub b: i32,
    pub result: i32,
}

impl Question for Operation {
    fn new(operation: QuestionType, max: i32) -> Self {
        match operation {
            QuestionType::Operation(OperationType::Division) => {
                //Calculate the result first in order to avoid fractions
                let result = Self::choose_variable(0, max);
                let denominator = Self::choose_variable(1, max);
                let (numerator, overloaded) = denominator.overflowing_mul(result);
                if overloaded {
                    panic!(
                        "Multiplication Overloaded (): {} * {} = {} ",
                        numerator, denominator, result
                    );
                }
                Self {
                    a: numerator,
                    b: denominator,
                    result,
                }
            }
            QuestionType::Operation(operation) => {
                let a = Self::choose_variable(0, max);
                let b = Self::choose_variable(0, max);
                let result = Operation::calculate(
                    &Self { a, b, result: 0 },
                    question_type::QuestionType::Operation(operation),
                );
                Self { a, b, result }
            }
            QuestionType::Equation(_) => {
                panic!("Should be handled in Equation");
            }
        }
    }

    fn choose_variable(min: i32, max: i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..max)
    }

    fn get_result(&self) -> i32 {
        self.result
    }

    fn calculate(&self, question_type: QuestionType) -> i32 {
        match question_type {
            QuestionType::Operation(OperationType::Addition) => {
                let (result, overloaded) = self.a.overflowing_add(self.b);
                if overloaded {
                    Self::panic(&self, "Addition Overloaded");
                }
                result
            }
            QuestionType::Operation(OperationType::Subtraction) => {
                let (result, overloaded) = self.a.overflowing_sub(self.b);
                if overloaded {
                    Self::panic(&self, "Subtraction Overloaded");
                }
                result
            }
            QuestionType::Operation(OperationType::Multiplication) => {
                let (result, overloaded) = self.a.overflowing_mul(self.b);
                if overloaded {
                    Self::panic(&self, "Multiplication Overloaded");
                }
                result
            }
            QuestionType::Operation(OperationType::Division) => {
                let (result, overloaded) = self.a.overflowing_div(self.b);
                if overloaded {
                    Self::panic(&self, "Division Overloaded");
                }
                result
            }
            QuestionType::Equation(_) => {
                panic!("Calculate should be handled in Equation");
            }
        }
    }

    fn print(&self, question_type: QuestionType) {
        let mut output: String = self.a.to_string();
        match question_type {
            QuestionType::Operation(OperationType::Addition) => {
                output.push_str(&"+".to_string());
            }
            QuestionType::Operation(OperationType::Subtraction) => {
                output.push_str(&"-".to_string());
            }
            QuestionType::Operation(OperationType::Multiplication) => {
                output.push_str(&"*".to_string());
            }
            QuestionType::Operation(OperationType::Division) => {
                output.push_str(&"/".to_string());
            }
            QuestionType::Equation(_) => {
                panic!("Print should be handled in Equation");
            }
        }
        output.push_str(&self.b.to_string());
        println!("{}", output);
    }
}

impl Operation {
    pub fn panic(&self, e: &str) {
        panic!("{}: {} / {} = {} ", e, self.a, self.b, self.result);
    }
}

#[cfg(test)]
mod tests;
