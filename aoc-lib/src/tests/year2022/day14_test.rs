use crate::solution::*;
use crate::solutions::year2022::Day14;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day14>(
        r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#,
    );
    assert_result(day.part1(&ctx), "24", "solve part 1");
    assert_result(day.part2(&ctx), "93", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day14>();
    assert_result(day.part1(&ctx), "592", "solve part 1");
    assert_result(day.part2(&ctx), "30367", "solve part 2");
}
