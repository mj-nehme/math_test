use std::io::{self};

pub fn read_number() -> i32 {
    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let answer = input_text.trim();

        match answer.parse::<i32>() {
            Ok(answer) => {
                return answer;
            }
            Err(e) => {
                println!("Invalid argument: {} \nPlease try again", e);
                continue;
            }
        }
    }
}
