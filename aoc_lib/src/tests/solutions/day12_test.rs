use crate::solution::*;
use crate::solutions::Day12;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day12>(
        r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end"#,
    );
    assert_result(day.part1(&ctx), "10", "solve part 1");
    assert_result(day.part2(&ctx), "36", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day12>();
    assert_result(day.part1(&ctx), "5104", "solve part 1");
    assert_result(day.part2(&ctx), "149220", "solve part 2");
}
