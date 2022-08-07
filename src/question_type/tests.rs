use super::QuestionType;
use crate::question_type::{EquationType, OperationType};

#[test]
fn test_get_question_type() {
    assert_eq!(
        QuestionType::get(1).unwrap(),
        QuestionType::Operation(OperationType::Addition)
    );
    assert_eq!(
        QuestionType::get(2).unwrap(),
        QuestionType::Operation(OperationType::Subtraction)
    );
    assert_eq!(
        QuestionType::get(3).unwrap(),
        QuestionType::Operation(OperationType::Multiplication)
    );
    assert_eq!(
        QuestionType::get(4).unwrap(),
        QuestionType::Operation(OperationType::Division)
    );
    assert_eq!(
        QuestionType::get(5).unwrap(),
        QuestionType::Equation(EquationType::OneVariable)
    );
    assert_eq!(
        QuestionType::get(6).unwrap(),
        QuestionType::Equation(EquationType::TwoVariables)
    );
}

#[test]
fn test_display_methods() {
    assert_eq!(
        QuestionType::Operation(OperationType::Addition).to_string(),
        "Operation::Addition"
    );
    assert_eq!(
        QuestionType::Operation(OperationType::Subtraction).to_string(),
        "Operation::Subtraction"
    );
    assert_eq!(
        QuestionType::Operation(OperationType::Multiplication).to_string(),
        "Operation::Multiplication"
    );
    assert_eq!(
        QuestionType::Operation(OperationType::Division).to_string(),
        "Operation::Division"
    );
    assert_eq!(
        QuestionType::Equation(EquationType::OneVariable).to_string(),
        "Equation::OneVariable"
    );
    assert_eq!(
        QuestionType::Equation(EquationType::TwoVariables).to_string(),
        "Equation::TwoVariables"
    );
}
