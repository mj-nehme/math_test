use std::fmt;

use crate::{
    number,
    question::Question,
    question_type::{EquationType, QuestionType},
};

use super::Equation;

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
            //ax+b=result
            let x: i32 = <Self as Equation>::choose_variable(max);
            let a: i32 = <Self as Equation>::choose_variable(max);
            let b: i32 = <Self as Equation>::choose_variable(max);
            let y: i32 = <Self as Equation>::choose_variable(max);
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
            panic!("Unexpected Question Type");
        }
    }

    fn get_expected_answer(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn calculate(&self) -> (i32, i32) {
        let ax = self.a * self.x;
        let by = self.b * self.y;
        let cx = self.c * self.x;
        let dy = self.d * self.y;
        let result1: i32 = ax + by;
        let result2: i32 = cx + dy;
        return (result1, result2);
    }

    fn post(&self) -> bool {
        let expected = self.get_expected_answer();
        self.print();

        let answer_x = number::read_number();
        let answer_y = number::read_number();

        if (answer_x, answer_y) == expected {
            println!("Correct Answer!");
            return true;
        } else {
            println!(
                "Wrong! The correct Answer was x = {}, y = {}",
                expected.0, expected.1
            );
            return false;
        }
    }
}

#[cfg(test)]
mod tests;
