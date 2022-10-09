use super::*;
use crate::solutions::Day01;

#[test]
fn test_input() {
    let (mut day, ctx) = setup::<Day01>("asd");
    assert_eq!(day.part1(&ctx).unwrap(), "3");
    assert_eq!(day.part2(&ctx).unwrap(), "*");
}

#[test]
fn puzzle_input() {}
