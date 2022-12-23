use crate::solution::*;
use crate::solutions::year2022::Day18;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day18>(
        r#"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#,
    );
    assert_result(day.part1(&ctx), "64", "solve part 1");
    assert_result(day.part2(&ctx), "58", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day18>();
    assert_result(day.part1(&ctx), "4512", "solve part 1");
    assert_result(day.part2(&ctx), "2554", "solve part 2");
}
