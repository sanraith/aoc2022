use crate::solution::*;
use crate::solutions::year2022::Day23;
use crate::tests::util::*;

#[test]
fn small_example_input() {
    let (mut day, ctx) = setup::<Day23>(
        r#"
.....
..##.
..#..
.....
..##.
....."#,
    );
    assert_result(day.part1(&ctx), "25", "solve part 1");
    //assert_result(day.part2(&ctx), "day23_part2", "solve part 2");
}

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day23>(
        r#"
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#,
    );
    assert_result(day.part1(&ctx), "110", "solve part 1");
    assert_result(day.part2(&ctx), "20", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day23>();
    assert_result(day.part1(&ctx), "3966", "solve part 1");
    //assert_result(day.part2(&ctx), "day23_part2", "solve part 2");
}
