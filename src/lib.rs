mod equation;
pub mod exam;
mod number;
mod operation;
mod question;
pub mod question_type;
use equation::onevariable::OneVariable;
use equation::twovariables::TwoVariables;
use exam::Exam;
use question_type::QuestionType;
mod pdf;
use crate::exam::ExamType;
use crate::operation::Operation;

pub fn generate_exam(
    question_type_i32: i32,
    level: i32,
    number_of_questions: i32,
    exam_type: ExamType,
) {
    verify_level(level);
    let question_type = QuestionType::get(question_type_i32).unwrap();
    match question_type {
        QuestionType::Operation(_) => {
            let exam = Exam::<Operation>::new(question_type, level, number_of_questions, exam_type);
            exam.post();
        }
        QuestionType::Equation(equation) => match equation {
            question_type::EquationType::OneVariable => {
                let exam =
                    Exam::<OneVariable>::new(question_type, level, number_of_questions, exam_type);
                exam.post();
            }
            question_type::EquationType::TwoVariables => {
                let exam =
                    Exam::<TwoVariables>::new(question_type, level, number_of_questions, exam_type);
                exam.post();
            }
        },
    }
}

pub fn print_list() {
    QuestionType::print_list();
}

fn verify_level(level: i32) {
    match level {
        1..=10 => (),
        _ => panic!("Unhandled level!"),
    }
}
