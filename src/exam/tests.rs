use super::Exam;
use crate::{operation::Operation, question_type::OperationType, question_type::QuestionType};

#[test]
fn test_exam() {
    let question_type = QuestionType::Operation(OperationType::Addition);
    let level = 1;
    let number_of_questions = 10;

    let exam = Exam::<Operation>::new(question_type, level, number_of_questions);

    assert_eq!(exam.get_question_type(), question_type);
    assert_eq!(exam.get_level(), level);
    assert_eq!(exam.get().len(), number_of_questions as usize);
}
