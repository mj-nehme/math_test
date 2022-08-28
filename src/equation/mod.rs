use rand::Rng;
pub mod onevariable;
pub mod twovariables;
use crate::question::Question;

pub trait Equation: Question {
    fn choose_variable(max: i32) -> i32
    where
        Self: Sized,
    {
        let mut rng = rand::thread_rng();
        rng.gen_range(-max..0, 0..max);
    }
}
