use crate::solution::*;
use crate::solutions::year2022::Day02;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day02>(
        r#"
A Y
B X
C Z"#,
    );
    assert_result(day.part1(&ctx), "15", "solve part 1");
    assert_result(day.part2(&ctx), "12", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day02>();
    assert_result(day.part1(&ctx), "15523", "solve part 1");
    assert_result(day.part2(&ctx), "15702", "solve part 2");
}
