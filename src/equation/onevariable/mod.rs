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
        if let QuestionType::Equation(EquationType::OneVariable) = equation {
            //ax+b=result
            let x: i32 = <Self as Equation>::choose_variable(max);
            let a: i32 = <Self as Equation>::choose_variable(max);
            let b: i32 = <Self as Equation>::choose_variable(max);

            let result = OneVariable::calculate(&Self { a, b, result: 0, x });
            Self { a, x, b, result }
        } else {
            panic!("Unexpected Behaviour");
        }
    }

    fn get_correct_answer(&self) -> i32 {
        self.x
    }

    fn calculate(&self) -> i32 {
        let ax = (self.a * self.x) as i32;
        let result: i32 = (ax + self.b as i32) as i32;
        return result;
    }

    fn post_to_cmd(&self) -> bool {
        let correct_answer = self.get_correct_answer();
        self.print();

        let answer = OneVariable::read_answer_from_cmd();
        OneVariable::verify_answer(correct_answer, answer)
    }

    fn read_answer_from_cmd() -> i32 {
        println!("Guess x:");
        number::read_number(-1000000, 1000000)
    }

    fn correct_answer_to_string(&self) -> String {
        format!("x = {}", self.x)
    }
}

#[cfg(test)]
mod tests;
