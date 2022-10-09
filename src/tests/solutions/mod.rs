use crate::api::solution::*;

pub fn setup<T>(input: &str) -> (T, Context)
where
    T: Solution,
{
    (
        T::new(),
        Context {
            input,
            ..Default::default()
        },
    )
}

mod day01_test;
