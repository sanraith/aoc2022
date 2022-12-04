use crate::solution::*;
use crate::solutions::year2022::Day04;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day04>(
        r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#,
    );
    assert_result(day.part1(&ctx), "2", "solve part 1");
    assert_result(day.part2(&ctx), "4", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day04>();
    assert_result(day.part1(&ctx), "532", "solve part 1");
    assert_result(day.part2(&ctx), "854", "solve part 2");
}
