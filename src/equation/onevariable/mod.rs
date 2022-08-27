use std::fmt;

use crate::{
    number,
    question::Question,
    question_type::{EquationType, QuestionType},
};

use super::Equation;

#[derive(Debug)]
pub struct OneVariable {
    a: i32,
    x: i32,
    b: i32,
    result: i32,
}
impl fmt::Display for OneVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.b < 0 {
            write!(f, "{}x{} = {}", self.a, self.b, self.result)
        } else {
            write!(f, "{}x+{} = {}", self.a, self.b, self.result)
        }
    }
}

impl Equation for OneVariable {}

impl Question for OneVariable {
    type Output = i32;

    fn new(equation: QuestionType, max: i32) -> Self {
        match equation {
            QuestionType::Equation(EquationType::OneVariable) => {
                //ax+b=result
                let x: i32 = <Self as Equation>::choose_variable(max);
                let a: i32 = <Self as Equation>::choose_variable(max);
                let b: i32 = <Self as Equation>::choose_variable(max);

                let result = OneVariable::calculate(&Self { a, b, result: 0, x });
                Self { a, x, b, result }
            }
            QuestionType::Equation(EquationType::TwoVariables) => {
                panic!("Should be handled in TwoVariables");
            }
            QuestionType::Operation(_) => {
                panic!("Should be handled in Operation");
            }
        }
    }

    fn get_expected_answer(&self) -> i32 {
        self.x
    }

    fn calculate(&self) -> i32 {
        let ax = (self.a * self.x) as i32;
        let result: i32 = (ax + self.b as i32) as i32;
        return result;
    }

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

#[cfg(test)]
mod tests;
