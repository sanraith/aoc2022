use crate::solution::*;
use crate::solutions::year2022::Day24;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day24>(
        r#"
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#,
    );
    assert_result(day.part1(&ctx), "18", "solve part 1");
    //assert_result(day.part2(&ctx), "day24_part2", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day24>();
    assert_result(day.part1(&ctx), "288", "solve part 1");
    //assert_result(day.part2(&ctx), "day24_part2", "solve part 2");
}
