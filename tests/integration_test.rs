#[cfg(test)]
mod tests {

    use math_test::{
        exam::Exam, operation::Operation, question::Question, question_type::OperationType,
    };

    #[test]
    #[allow(dead_code)]
    fn generate_addition_exam_test() {
        let question_type = OperationType::Addition;
        let level = 7;
        let number_of_questions = 5;
        let exam = Exam::<Operation>::new(
            math_test::QuestionType::Operation(question_type),
            level,
            number_of_questions,
        );

        for question in exam.get_questions() {
            assert_eq!(question.calculate(), question.get_correct_answer());
        }
    }
}
