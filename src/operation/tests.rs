use super::QuestionType;
use crate::{operation::Operation, question::Question, question_type::OperationType};

#[test]
fn test_operation() {
    //This test includes testing calculate
    let max = 10;
    let add_operation = Operation::new(QuestionType::Operation(OperationType::Addition), max);
    assert_eq!(add_operation.a + add_operation.b, add_operation.result);
    let sub_operation = Operation::new(QuestionType::Operation(OperationType::Subtraction), max);
    assert_eq!(sub_operation.a - sub_operation.b, sub_operation.result);
    let mul_operation = Operation::new(QuestionType::Operation(OperationType::Multiplication), max);
    assert_eq!(mul_operation.a * mul_operation.b, mul_operation.result);
    let div_operation = Operation::new(QuestionType::Operation(OperationType::Division), max);
    assert_eq!(div_operation.a / div_operation.b, div_operation.result);
}
