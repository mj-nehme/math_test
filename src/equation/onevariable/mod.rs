use crate::{
    question::Question,
    question_type::{EquationType, QuestionType},
};
use rand::Rng;

#[derive(Debug)]
pub struct OneVariable {
    a: i32,
    x: i32,
    b: i32,
    result: i32,
}

impl Question for OneVariable {
    fn new(operation: QuestionType, max: i32) -> Self {
        match operation {
            QuestionType::Equation(EquationType::OneVariable) => {
                //ax+b=result
                let x: i32 = Self::choose_variable(-max, max);
                let a: i32 = Self::choose_variable(-max, max);
                let b: i32 = Self::choose_variable(-max, max);

                let result = OneVariable::calculate(&Self { a, b, result: 0, x }, operation);
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

    fn choose_variable(min: i32, max: i32) -> i32 {
        let mut rng = rand::thread_rng();
        let mut num = 0;
        loop {
            num = rng.gen_range(min..max);
            //exclude 0
            if num != 0 {
                break;
            }
        }

        num
    }

    fn get_result(&self) -> i32 {
        self.result
    }

    fn calculate(&self, question_type: QuestionType) -> i32 {
        match question_type {
            QuestionType::Equation(EquationType::OneVariable) => {
                let ax = (self.a * self.x) as i32;
                let result: i32 = (ax + self.b as i32) as i32;
                return result;
            }
            QuestionType::Equation(EquationType::TwoVariables) => {
                panic!("Calculate should be handled in TwoVariables");
            }
            QuestionType::Operation(_) => {
                panic!("Calculate should be handled in Operation");
            }
        }
    }

    fn print(&self, question_type: QuestionType) {
        if self.b < 0 {
            println!("{}x{}={}", self.a, self.b, self.result);
        } else {
            println!("{}x+{}={}", self.a, self.b, self.result);
        }
    }
}

#[cfg(test)]
mod tests;
