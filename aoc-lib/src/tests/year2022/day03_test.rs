use crate::solution::*;
use crate::solutions::year2022::Day03;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day03>(
        r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#,
    );
    assert_result(day.part1(&ctx), "157", "solve part 1");
    assert_result(day.part2(&ctx), "70", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day03>();
    assert_result(day.part1(&ctx), "7766", "solve part 1");
    assert_result(day.part2(&ctx), "2415", "solve part 2");
}
