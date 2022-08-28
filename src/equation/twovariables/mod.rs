use super::Equation;
use crate::{
    number,
    question::Question,
    question_type::{EquationType, QuestionType},
};
use std::fmt;

#[derive(Debug)]
pub struct TwoVariables {
    // ax+by=result1
    // cx+dy=result2
    a: i32,
    b: i32,
    result1: i32,
    c: i32,
    d: i32,
    result2: i32,
    x: i32,
    y: i32,
}

impl fmt::Display for TwoVariables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_1: String;
        let line_2: String;
        if self.b < 0 {
            line_1 = format!("{}x {}y = {}", self.a, self.b, self.result1);
        } else {
            line_1 = format!("{}x + {}y = {}", self.a, self.b, self.result1);
        }
        if self.d < 0 {
            line_2 = format!("{}x {}y = {}", self.c, self.d, self.result2);
        } else {
            line_2 = format!("{}x + {}y = {}", self.c, self.d, self.result2);
        }
        write!(f, "{}\n{}", line_1, line_2)
    }
}

impl Equation for TwoVariables {}

impl Question for TwoVariables {
    type Output = (i32, i32);

    fn new(equation: QuestionType, max: i32) -> Self {
        if let QuestionType::Equation(EquationType::TwoVariables) = equation {
            let x: i32 = <Self as Equation>::choose_variable(max);
            let y: i32 = <Self as Equation>::choose_variable(max);

            let a: i32 = <Self as Equation>::choose_variable(max);
            let b: i32 = <Self as Equation>::choose_variable(max);
            let c: i32 = <Self as Equation>::choose_variable(max);
            let d: i32 = <Self as Equation>::choose_variable(max);
            let (result1, result2) = TwoVariables::calculate(&Self {
                a,
                b,
                result1: 0,
                c,
                d,
                result2: 0,
                x,
                y,
            });
            Self {
                a,
                b,
                result1,
                c,
                d,
                result2,
                x,
                y,
            }
        } else {
            panic!("Unexpected Behaviour");
        }
    }

    fn get_correct_answer(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn calculate(&self) -> (i32, i32) {
        let ax = self.a * self.x;
        let by = self.b * self.y;
        let cx = self.c * self.x;
        let dy = self.d * self.y;
        let result1 = ax + by;
        let result2 = cx + dy;

        (result1, result2)
    }

    fn post_to_cmd(&self) -> bool {
        let correct_answer = self.get_correct_answer();
        self.print();

        let answer = TwoVariables::read_answer_from_cmd();
        TwoVariables::verify_answer(correct_answer, answer)
    }

    fn read_answer_from_cmd() -> (i32, i32) {
        println!("Guess x:");
        let x = number::read_number(-1000000, 1000000);
        println!("Guess y:");
        let y = number::read_number(-1000000, 1000000);
        (x, y)
    }

    fn correct_answer_to_string(&self) -> String {
        format!("x = {}, y = {}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests;
