use crate::solution::*;
use crate::solutions::year2022::Day17;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day17>(r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#);
    assert_result(day.part1(&ctx), "3068", "solve part 1");
    //assert_result(day.part2(&ctx), "day17_part2", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day17>();
    //assert_result(day.part1(&ctx), "day17_part1", "solve part 1");
    //assert_result(day.part2(&ctx), "day17_part2", "solve part 2");
}
