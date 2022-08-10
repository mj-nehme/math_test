mod equation;
mod exam;
mod number;
mod operation;
mod pdf;
mod question;
mod question_type;
use math_test::question_type::QuestionType;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("unexpected arguments");
    }

    let number_of_questions = 10;

    println!("Choose number (1-6):");
    math_test::print_list();
    let question_type_i32 = number::read_number();
    validate_quiz_type(question_type_i32);

    println!("Choose level (1-10):");
    let level = number::read_number();
    validate_level(level);

    println!("Level: {}", level);
    println!("Quiz: {}", question_type_i32.to_string());
    println!("===========================");

    let question_type = QuestionType::get(question_type_i32).unwrap();

    math_test::run(args, question_type, level, number_of_questions);
}

pub fn validate_level(level: i32) {
    if level < 1 || level > 10 {
        panic!("Sorry you're not smart enough to follow the rules!");
    }
}

pub fn validate_quiz_type(quiz_type: i32) {
    if quiz_type < 1 || quiz_type > 6 {
        panic!("Sorry you're not smart enough to follow the rules!");
    } else if quiz_type == 6 {
        panic!("Sorry this option (TwoVariables) isn't implemented yet!");
    }
}
