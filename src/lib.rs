pub mod equation;
pub mod exam;
pub mod number;
pub mod operation;
pub mod question;
pub mod question_type;
pub use equation::onevariable::OneVariable;
pub use equation::twovariables::TwoVariables;
use exam::Exam;
pub use question_type::{EquationType, OperationType, QuestionType};
mod pdf;
use crate::operation::Operation;

/// Generates an Exam object given the question type, exam level, number of questions and the exam type (pdf or cmd)
pub fn generate_exam(question_type_i32: i32, level: i32, number_of_questions: i32, exam_type: i32) {
    let question_type = QuestionType::get(question_type_i32).unwrap();
    match question_type {
        QuestionType::Operation(_) => {
            let exam = Exam::<Operation>::new(question_type, level, number_of_questions);
            exam.post(exam_type);
        }
        QuestionType::Equation(equation) => match equation {
            question_type::EquationType::OneVariable => {
                let exam = Exam::<OneVariable>::new(question_type, level, number_of_questions);
                exam.post(exam_type);
            }
            question_type::EquationType::TwoVariables => {
                let exam = Exam::<TwoVariables>::new(question_type, level, number_of_questions);
                exam.post(exam_type);
            }
        },
    }
}

/// Prints the list of Exam types
pub fn print_list() {
    QuestionType::print_list();
}
