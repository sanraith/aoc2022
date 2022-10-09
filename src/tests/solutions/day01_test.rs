use super::*;
use crate::solutions::Day01;

#[test]
fn test_input() {
    let input = "asd";
    let mut day = Day01::new();
    day.assert(Part1, input, "3");
    day.assert(Part2, input, "*");
}

#[test]
fn puzzle_input() {}
