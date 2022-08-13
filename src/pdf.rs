use genpdf::Document;
use crate::exam::Exam;

use crate::question::Question;

pub fn generate_pdf<T:Question>(exam: Exam<T>) {

    println!("Generating PDF file...");
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

  let questions = exam.get();
  let mut counter = 1;
    for question in questions {

        push_question (question, counter,  &mut doc);
        counter +=1;
    }  

    let file_name = "math_test.pdf";
    // Render the document and write it to a file
    doc.render_to_file(file_name)
        .expect("Failed to write PDF file");

    println!("File {} generated", file_name);
}

fn push_question<T:Question> (question : &T, id: i32, doc: &mut Document){
    let title= format!("Question {}: ", id);
    doc.push(genpdf::elements::Paragraph::new(title));
    
    doc.push(genpdf::elements::Paragraph::new(question.to_string()));

    doc.push(genpdf::elements::Paragraph::new(""));
    let separator = "=============";
    doc.push(genpdf::elements::Paragraph::new(separator));
    doc.push(genpdf::elements::Paragraph::new(""));
}