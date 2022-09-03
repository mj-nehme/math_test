use crate::pdf;
use crate::question::Question;
use crate::question_type::QuestionType;

#[derive(PartialEq, Clone)]
#[repr(i32)]
pub enum ExamType {
    Cmd = 0, // Terminal
    Pdf = 1, // Generate pdf file
    Txt = 2, // Generate txt file
    Jsc = 3, // For JS use
    Arr = 4, // Bare Array
}

impl From<i32> for ExamType {
    fn from(num: i32) -> Self {
        match num {
            0 => ExamType::Cmd,
            1 => ExamType::Pdf,
            2 => ExamType::Txt,
            3 => ExamType::Jsc,
            _ => ExamType::Arr,
        }
    }
}

#[derive(Clone)]
pub struct Exam<T: Question> {
    level: i32,
    question_type: QuestionType,
    questions: Vec<T>,
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
        }
    }

    pub fn print(&self) {
        for question in self.questions.iter() {
            question.print();
        }
    }

    pub fn get_questions(&self) -> &Vec<T> {
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

        println!("Score: {}/{}\n--------", score, self.get_questions().len());
    }

    pub fn post(&self, exam_type: i32) {
        let exam_type: ExamType = exam_type.into();
        if exam_type == ExamType::Cmd {
            self.post_to_cmd();
        } else if exam_type == ExamType::Pdf {
            pdf::generate_pdf(self);
        } else if exam_type == ExamType::Txt {
            // To be handled later
            //self.post_to_txt();
        } else if exam_type == ExamType::Jsc {
            // To be handled later
            //self.post_to_JS();
        } else {
            // To be handled later
        }
    }

    pub fn level_complexity(level: i32) -> i32 {
        level * 10
    }
}

#[cfg(test)]
mod tests;
