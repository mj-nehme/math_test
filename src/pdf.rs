use crate::{exam::Exam, question_type};
use genpdf::{style::Style, Document};

use crate::question::Question;

/// Genrates a Pdf file given an Exam
#[allow(dead_code)]
pub fn generate_pdf<T: Question>(exam: &Exam<T>) {
    println!("Generating PDF file...");

    let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)
        .expect("Failed to load font family");
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Math Test");
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    if exam.get_question_type()
        == question_type::QuestionType::Equation(question_type::EquationType::TwoVariables)
    {
        doc.set_font_size(11);
    } else {
        doc.set_font_size(12);
    }

    let questions = exam.get_questions();
    let mut counter = 1;
    for question in questions {
        push_question(question, counter, &mut doc);
        counter += 1;
    }

    let file_name = "math_test.pdf";
    doc.render_to_file(file_name)
        .expect("Failed to write PDF file");

    println!("File {} generated", file_name);
}

/// Writes a Question into the Pdf file
fn push_question<T: Question>(question: &T, id: i32, doc: &mut Document) {
    let title = format!("Question {}: ", id);

    let mut title_element = genpdf::elements::Paragraph::new("");
    let style = Style::new();
    let style = style.bold();
    title_element.push_styled(title, style);

    doc.push(title_element);
    for line in question.to_string().lines() {
        doc.push(genpdf::elements::Paragraph::new(line));
    }

    doc.push(genpdf::elements::Paragraph::new(""));
    let separator = "=============";
    doc.push(genpdf::elements::Paragraph::new(separator));
    doc.push(genpdf::elements::Paragraph::new(""));
}
