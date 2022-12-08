use crate::solution::*;
use crate::solutions::year2022::Day08;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day08>(
        r#"
30373
25512
65332
33549
35390"#,
    );
    assert_result(day.part1(&ctx), "21", "solve part 1");
    assert_result(day.part2(&ctx), "8", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day08>();
    assert_result(day.part1(&ctx), "1814", "solve part 1");
    assert_result(day.part2(&ctx), "330786", "solve part 2");
}
