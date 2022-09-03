mod equation;
mod exam;
mod number;
mod operation;
mod pdf;
mod question;
mod question_type;
use std::env;

/// Main function that lunches the app
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("uncorrect_answer arguments");
    }

    let number_of_questions = 10;

    println!("Choose number (1-6):");
    math_test::print_list();
    let question_type_i32 = number::read_number(1, 6);

    println!("Choose level (1-10):");
    let level = number::read_number(1, 10);

    println!("Level: {}", level);
    println!("Quiz: {}", question_type_i32.to_string());
    println!("===========================");

    if args.len() == 1 {
        math_test::generate_exam(question_type_i32, level, number_of_questions, 0);
    } else {
        if args[1] == "pdf" {
            math_test::generate_exam(question_type_i32, level, number_of_questions, 1);
        } else if args[1] == "txt" {
            math_test::generate_exam(question_type_i32, level, number_of_questions, 2);
        } else {
            panic!("Unknown argument");
        }
    }
}
