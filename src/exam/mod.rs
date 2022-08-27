use crate::pdf;
use crate::question::Question;
use crate::question_type::QuestionType;

#[derive(PartialEq, Clone)]
pub enum ExamType {
    Cmd,
    Pdf,
}

#[derive(Clone)]
pub struct Exam<T: Question> {
    level: i32,
    question_type: QuestionType,
    questions: Vec<T>,
    exam_type: ExamType,
}

#[allow(dead_code)]
impl<T> Exam<T>
where
    T: Question,
{
    pub fn new(
        question_type: QuestionType,
        level: i32,
        number_of_questions: i32,
        exam_type: ExamType,
    ) -> Exam<impl Question> {
        let mut questions: Vec<T> = Vec::new();
        let max = Self::level_complexity(level);
        for _ in 0..number_of_questions {
            let question = Question::new(question_type, max);
            questions.push(question);
        }
        Exam::<T> {
            questions,
            level,
            question_type,
            exam_type,
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

    pub fn post_to_cmd(&self) {
        let mut score = 0;
        let mut posted = 0;

        for question in &self.questions {
            let correct_answer = question.post_to_cmd();

            if correct_answer {
                score += 1;
                println!("Correct Answer!");
            } else {
                println!(
                    "Wrong! The correct Answer was {}",
                    question.correct_answer_to_string()
                );
            }

            posted += 1;
            println!("Score: {}/{}\n--------", score, posted);
        }

        println!("Score: {}/{}\n--------", score, self.get().len());
    }

    pub fn post(&self) {
        if self.exam_type == ExamType::Cmd {
            self.post_to_cmd();
        } else if self.exam_type == ExamType::Pdf {
            pdf::generate_pdf(self);
        }
    }

    fn level_complexity(level: i32) -> i32 {
        level * 10
    }
}

#[cfg(test)]
mod tests;
