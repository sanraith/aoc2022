use crate::solution::*;
use crate::solutions::year2022::Day22;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day22>(
        r#"
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#,
    );
    assert_result(day.part1(&ctx), "6032", "solve part 1");
    assert_result(day.part2(&ctx), "5031", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day22>();
    assert_result(day.part1(&ctx), "162186", "solve part 1");
    //assert_result(day.part2(&ctx), "day22_part2", "solve part 2");
}
