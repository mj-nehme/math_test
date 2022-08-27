use std::io::{self};

pub fn read_number(min: i32, max: i32) -> i32 {
    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let answer = input_text.trim();

        match answer.parse::<i32>() {
            Ok(answer) if answer >= min && answer <= max => {
                return answer;
            }
            Ok(_) => {
                println!("Invalid integer: {}Please try again", input_text);
                continue;
            }
            Err(_) => {
                println!("Invalid input: {}Please try again", input_text);
                continue;
            }
        }
    }
}
