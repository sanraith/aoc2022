use crate::api::solution::*;

pub enum Part {
    Part1 = 1,
    Part2 = 2,
}
pub use Part::{Part1, Part2};

pub fn assert<T>(part: Part, input: &str, expected: &str)
where
    T: Solution,
{
    let sut = T::new();
    let ctx = Context {
        input: &input,
        ..Default::default()
    };

    let part_num;
    let result = match part {
        Part1 => {
            part_num = 1;
            sut.part1(&ctx)
        }
        Part2 => {
            part_num = 2;
            sut.part2(&ctx)
        }
    };

    assert_eq!(
        result.unwrap(),
        expected,
        "part {} should return {}",
        part_num,
        expected
    );
}

mod day01_test;
