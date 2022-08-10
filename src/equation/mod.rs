use rand::Rng;
pub(crate) mod onevariable;
pub(crate) mod twovariables;
use crate::question::Question;

pub trait Equation: Question
{

    fn choose_variable(max: i32) -> i32 where
        Self: Sized,
    {
        loop {
            let mut rng = rand::thread_rng();
            let num = rng.gen_range(-max..max);
            //exclude 0
            if num != 0 {
                return num;
            }
        }
    }
}
