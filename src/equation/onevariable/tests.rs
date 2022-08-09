use super::QuestionType;
use crate::{equation::onevariable::OneVariable, question::Question, question_type::EquationType};

#[test]
fn test_onevariable() {
    //This test includes testing calculate
    let max = 10;
    let onevariable_equation =
        OneVariable::new(QuestionType::Equation(EquationType::OneVariable), max);
    assert_eq!(
        onevariable_equation.a * onevariable_equation.x + onevariable_equation.b,
        onevariable_equation.result
    );
}
