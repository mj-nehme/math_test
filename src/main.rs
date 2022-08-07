mod equation;
mod number;
mod operation;
mod question;
mod question_type;
use math_test::{generate_exam, print_list};

fn main() {
    let number_of_questions = 10;

    println!("Choose number (1-6):");
    print_list();
    let question_type = number::read_number();
    validate_quiz_type(question_type);

    println!("Choose level (1-10):");
    let level = number::read_number();
    validate_level(level);

    generate_exam(question_type, level, number_of_questions);
}

pub fn validate_level(level: i32) {
    // TODO: levels to be read from Config file
    // TODO: levels to be handled automatically

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
