use super::QuestionType;
use crate::{
    equation::twovariables::TwoVariables, question::Question, question_type::EquationType,
};

#[test]
fn test_twovariables() {
    let max = 10;
    let twovariable_equations =
        TwoVariables::new(QuestionType::Equation(EquationType::TwoVariables), max);
    assert_eq!(
        twovariable_equations.a * twovariable_equations.x
            + twovariable_equations.b * twovariable_equations.y,
        twovariable_equations.result1
    );
    assert_eq!(
        twovariable_equations.c * twovariable_equations.x
            + twovariable_equations.d * twovariable_equations.y,
        twovariable_equations.result2
    );
}
