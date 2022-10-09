use crate::api::solution::*;

pub enum Part {
    Part1 = 1,
    Part2 = 2,
}
pub use Part::{Part1, Part2};

pub trait TestableSolution {
    fn assert(&self, part: Part, input: &str, expected: &str);
}

impl<T: Solution> TestableSolution for T {
    fn assert(&self, part: Part, input: &str, expected: &str) {
        let ctx = Context {
            input,
            ..Default::default()
        };

        let part_num;
        let result = match part {
            Part1 => {
                part_num = 1;
                self.part1(&ctx)
            }
            Part2 => {
                part_num = 2;
                self.part2(&ctx)
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
}

mod day01_test;
