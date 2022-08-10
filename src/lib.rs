mod equation;
mod exam;
mod number;
mod operation;
mod question;
pub mod question_type;
use equation::onevariable::OneVariable;
use equation::twovariables::TwoVariables;
use exam::Exam;
use operation::Operation;
use question::Question;
use question_type::QuestionType;
mod pdf;

pub fn run(args: Vec<String>, question_type: QuestionType, level: i32, number_of_questions: i32) {

    match question_type {
        QuestionType::Operation(_) => {
            let exam = Exam::<Operation>::new(question_type, level, number_of_questions);
            execute_exam (args, exam);
        }
        QuestionType::Equation(equation) => {
            match equation {
                question_type::EquationType::OneVariable => {
                    let exam = Exam::<OneVariable>::new(question_type, level, number_of_questions);
                    execute_exam (args, exam);
                }
                question_type::EquationType::TwoVariables => {
                    let exam = Exam::<TwoVariables>::new(question_type, level, number_of_questions);
                    execute_exam (args, exam);
                }
            };
        }
    }
}

fn execute_exam<T: Question>(args: Vec<String>, exam: Exam<T>) {
    if args.len() == 1 {
        exam.post_cmd();
    } else {
        if args[1] == "--pdf" {
            pdf::generate_pdf(&exam);
        } else {
            panic!("unknown arguments");
        }
    }
}

pub fn print_list() {
    QuestionType::print_list();
}
