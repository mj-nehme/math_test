use crate::question::{self, Question};
use crate::question_type::QuestionType;

pub struct Exam<T: Question> {
    level: i32,
    question_type: QuestionType,
    questions: Vec<T>,
}

impl<T> Exam<T>
where
    T: Question,
{
    pub fn new(
        question_type: QuestionType,
        level: i32,
        number_of_questions: i32,
    ) -> Exam<impl Question> {
        let mut questions: Vec<T> = Vec::new();
        let max = Self::level_complexity(level);
        for i in 0..number_of_questions {
            let question = Question::new(question_type, max);
            questions.push(question);
        }
        Exam::<T> {
            questions,
            level,
            question_type,
        }
    }

    pub fn print(&self) {
        for question in self.questions.iter() {
            question.print();
        }
    }

    pub fn get(&self) -> &Vec<T> {
        &self.questions
    }

    pub fn get_level(&self) -> i32 {
        self.level
    }
    pub fn get_question_type(&self) -> QuestionType {
        self.question_type
    }

    pub fn post_cmd(&self) {
        let mut score = 0;
        let mut posted = 0;

        for question in &self.questions {
            let answer = question.post();
            if answer {
                score += 1;
            }

            posted += 1;
            println!("Score: {}/{}\n--------", score, posted);
        }

        println!("Score: {}/{}\n--------", score, self.get().len());
    }

    fn level_complexity(level: i32) -> i32 {
        level * 10
    }
}

#[cfg(test)]
mod tests;
