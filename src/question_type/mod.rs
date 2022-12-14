use std::fmt;

/// QuestionType is an enum that derives into OperationType and EquationType
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum QuestionType {
    Operation(OperationType),
    Equation(EquationType),
}

/// OperationType is an enum that has
/// - Addition
/// - Subtraction
/// - Multiplication
/// - Division
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

/// EquationType is an enum that inherits QuestionType and derives to OneVariable or TwoVariables
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EquationType {
    OneVariable,
    TwoVariables,
}

#[allow(dead_code)]
impl QuestionType {
    pub fn get(game_option: i32) -> Result<Self, &'static str> {
        match game_option {
            1 => Ok(QuestionType::Operation(OperationType::Addition)),
            2 => Ok(QuestionType::Operation(OperationType::Subtraction)),
            3 => Ok(QuestionType::Operation(OperationType::Multiplication)),
            4 => Ok(QuestionType::Operation(OperationType::Division)),
            5 => Ok(QuestionType::Equation(EquationType::OneVariable)),
            6 => Ok(QuestionType::Equation(EquationType::TwoVariables)),
            _ => Err("Unrecognized option"),
        }
    }

    pub fn print_list() {
        // The number "7" should be handled automatically
        for i in 1..7 {
            println!("{}. {}", i, Self::get(i).unwrap());
        }
    }
}

impl fmt::Display for QuestionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuestionType::Operation(x) => {
                write!(f, "Operation::{}", x.to_string())
            }

            QuestionType::Equation(x) => {
                write!(f, "Equation::{}", x.to_string())
            }
        }
    }
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Addition => write!(f, "Addition"),
            Self::Subtraction => write!(f, "Subtraction"),
            Self::Multiplication => write!(f, "Multiplication"),
            Self::Division => write!(f, "Division"),
        }
    }
}

impl fmt::Display for EquationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OneVariable => write!(f, "OneVariable"),
            Self::TwoVariables => write!(f, "TwoVariables"),
        }
    }
}

#[cfg(test)]
mod tests;
