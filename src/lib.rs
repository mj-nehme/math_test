mod number;
mod operation;
mod question;
mod question_type;
use operation::Operation;
use question::Question;
use question_type::QuestionType;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration};

fn generate_question(question_type: &QuestionType, level: i32) -> bool {

    let timeout = 10;
    let max = 10;

    // TODO: Make a smarter leveling mechanism (Something like exponential)
    let function = Operation::new(*question_type, max * level as i32);

    function.print(*question_type);
    let expected = function.get_result();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
    let answer = number::read_number();
    tx.send(answer).unwrap();
    });

    let mut timer = 0; 
    loop {

        timer +=1;
        if timer > timeout {
            
            println!("Timeout!");
            return false;}
    
        let millis = Duration::from_millis(500);
        thread::sleep(millis);

        match rx.try_recv() {
            Ok(answer) => {
                if answer == expected {
                    println!("Correct Answer!");
                    return true;
                } else {
                    println!("Wrong! The correct Answer was {}", expected);
                    return false;
                }
            }
            Err (_) => {}
        }
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
