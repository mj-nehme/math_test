use std::{fmt, str};

use crate::{
    number,
    question::Question,
    question_type::{OperationType, QuestionType},
};
use rand::Rng;

#[derive(Debug)]
pub struct Operation {
    pub a: i32,
    pub op: char,
    pub b: i32,
    pub result: i32,
}
impl Operation {
    fn choose_variable(min: i32, max: i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..max)
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{} = ", self.a, self.op, self.b)
    }
}

impl Question for Operation {
    type Output = i32;

    fn new(operation: QuestionType, max: i32) -> Operation {
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
                Operation {
                    a: numerator,
                    op: '/',
                    b: denominator,
                    result,
                }
            }
            QuestionType::Operation(operation) => {
                let a = Self::choose_variable(0, max);
                let b = Self::choose_variable(0, max);
                let op = match operation {
                    OperationType::Addition => '+',
                    OperationType::Subtraction => '-',
                    OperationType::Multiplication => '*',
                    OperationType::Division => {
                        panic!("Division is erroneously handled")
                    }
                };
                let operation = Operation {
                    a,
                    op,
                    b,
                    result: 0,
                };
                let result = Operation::calculate(&operation);
                Operation { a, op, b, result }
            }
            QuestionType::Equation(_) => {
                panic!("Should be handled in Equation");
            }
        }
    }

    fn get_correct_answer(&self) -> i32 {
        self.result
    }

    fn calculate(&self) -> i32 {
        match self.op {
            '+' => {
                let (result, overloaded) = self.a.overflowing_add(self.b);
                if overloaded {
                    Self::panic(&self, "Addition Overloaded");
                }
                result
            }
            '-' => {
                let (result, overloaded) = self.a.overflowing_sub(self.b);
                if overloaded {
                    Self::panic(&self, "Subtraction Overloaded");
                }
                result
            }
            '*' => {
                let (result, overloaded) = self.a.overflowing_mul(self.b);
                if overloaded {
                    Self::panic(&self, "Multiplication Overloaded");
                }
                result
            }
            '/' => {
                let (result, overloaded) = self.a.overflowing_div(self.b);
                if overloaded {
                    Self::panic(&self, "Division Overloaded");
                }
                result
            }
            _ => {
                panic!("Unknown operator");
            }
        }
    }

    fn post_to_cmd(&self) -> bool {
        let correct_answer = self.get_correct_answer();
        self.print();

        let answer = Operation::read_answer_from_cmd();
        Operation::verify_answer(correct_answer, answer)
    }

    fn read_answer_from_cmd() -> i32 {
        number::read_number(-1000000, 1000000)
    }

    fn correct_answer_to_string(&self) -> String {
        format!("{}", self.result)
    }
}

impl Operation {
    pub fn panic(&self, e: &str) {
        panic!("{}: {} / {} = {} ", e, self.a, self.b, self.result);
    }
}

#[cfg(test)]
mod tests;
