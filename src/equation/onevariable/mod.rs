use crate::{
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

impl Equation for OneVariable {}
impl Question for OneVariable {
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

    fn print(&self) {
        if self.b < 0 {
            println!("{}x{}={}", self.a, self.b, self.result);
        } else {
            println!("{}x+{}={}", self.a, self.b, self.result);
        }
    }
}

#[cfg(test)]
mod tests;
