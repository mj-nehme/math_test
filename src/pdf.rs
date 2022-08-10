use genpdf::Document;
use crate::exam::Exam;

use crate::question::Question;

pub fn generate_pdf<T>(exam: &T) {
    // Load a font from the file system
    let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)
        .expect("Failed to load font family");
    // Create a document and set the default font family
    let mut doc = genpdf::Document::new(font_family);
    // Change the default settings
    doc.set_title("Math Test");
    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

  /*   let questions = exam.get();
    for i in questions {
        push_question (&doc);
    }  */

    // Render the document and write it to a file
    doc.render_to_file("math_test.pdf")
        .expect("Failed to write PDF file");
}

fn push_question (doc: &mut Document){
    doc.push(genpdf::elements::Paragraph::new("Question"));
}