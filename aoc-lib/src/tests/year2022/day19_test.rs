use crate::solution::*;
use crate::solutions::year2022::Day19;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day19>(
        r#"
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#,
    );
    assert_result(day.part1(&ctx), "33", "solve part 1");
    // assert_result(day.part2(&ctx), "3472", "solve part 2"); // too slow
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day19>();
    assert_result(day.part1(&ctx), "817", "solve part 1");
    assert_result(day.part2(&ctx), "4216", "solve part 2");
}
