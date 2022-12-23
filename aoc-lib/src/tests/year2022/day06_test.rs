use crate::solution::*;
use crate::solutions::year2022::Day06;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day06>(r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#);
    assert_result(day.part1(&ctx), "7", "solve part 1");
    assert_result(day.part2(&ctx), "19", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day06>();
    assert_result(day.part1(&ctx), "1816", "solve part 1");
    assert_result(day.part2(&ctx), "2625", "solve part 2");
}
