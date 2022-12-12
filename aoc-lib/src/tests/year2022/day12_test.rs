use crate::solution::*;
use crate::solutions::year2022::Day12;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day12>(
        r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#,
    );
    assert_result(day.part1(&ctx), "31", "solve part 1");
    assert_result(day.part2(&ctx), "29", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day12>();
    assert_result(day.part1(&ctx), "528", "solve part 1");
    assert_result(day.part2(&ctx), "522", "solve part 2");
}
