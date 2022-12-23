use crate::solution::*;
use crate::solutions::year2022::Day20;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day20>(
        r#"
1
2
-3
3
-2
0
4"#,
    );
    assert_result(day.part1(&ctx), "3", "solve part 1");
    assert_result(day.part2(&ctx), "1623178306", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day20>();
    assert_result(day.part1(&ctx), "988", "solve part 1");
    assert_result(day.part2(&ctx), "7768531372516", "solve part 2");
}
