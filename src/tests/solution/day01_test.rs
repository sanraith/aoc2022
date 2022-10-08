use super::*;
use crate::solutions::Day01;

#[test]
fn test_input() {
    let input = "asd";
    assert::<Day01>(Part1, &input, "3");
    assert::<Day01>(Part2, &input, "*");
}

#[test]
fn puzzle_input() {}
