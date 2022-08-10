mod equation;
mod number;
mod operation;
mod question;
mod question_type;
use equation::onevariable::OneVariable;
use equation::twovariables::TwoVariables;
use equation::Equation;
use operation::Operation;
use question::Question;
use question_type::QuestionType;

fn generate_question(question_type: &QuestionType, level: i32) -> bool {
    let timeout = 10000; //in milliseconds
    let max_baseline = 10;
    let max = max_baseline * level as i32;
    let expected = match question_type {
        QuestionType::Operation(t) => {
            let function = Operation::new(QuestionType::Operation(*t), max);
            function.print(*question_type);
            function.get_expected_answer()
        }
        QuestionType::Equation(t) => match t {
            question_type::EquationType::OneVariable => {
                let function = <OneVariable as Equation>::new(
                    QuestionType::Equation(question_type::EquationType::OneVariable),
                    max,
                );
                Equation::print(&function, *question_type);
                Equation::get_expected_answer(&function)
            }
            question_type::EquationType::TwoVariables => {
                let function = <TwoVariables as Equation>::new(
                    QuestionType::Equation(question_type::EquationType::TwoVariables),
                    max,
                );
                Equation::print(&function, *question_type);
                Equation::get_expected_answer(&function)
            }
        },
    };

    let answer = number::read_number();

    if answer == expected {
        println!("Correct Answer!");
        return true;
    } else {
        println!("Wrong! The correct Answer was {}", expected);
        return false;
    }
}

pub fn generate_exam(game_option: i32, level: i32, number_of_questions: i32) {
    let mut score = 0;
    let mut total = 0;

    let question_type = QuestionType::get(game_option)
        .unwrap_or_else(|e| panic!("Unable to get question type:\n{}", e));

    println!("Level: {}", level);
    println!("Quiz: {}", question_type.to_string());
    println!("===========================");

    loop {
        let answer = generate_question(&question_type, level);
        if answer {
            score += 1;
        }

        total += 1;
        println!("Score: {}/{}\n--------", score, total);

        if total == number_of_questions {
            break;
        }
    }
}

pub fn print_list() {
    QuestionType::print_list();
}
