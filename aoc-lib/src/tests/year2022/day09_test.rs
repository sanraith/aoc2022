use crate::solution::*;
use crate::solutions::year2022::Day09;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day09>(
        r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
    );
    assert_result(day.part1(&ctx), "13", "solve part 1");
    assert_result(day.part2(&ctx), "1", "solve part 2");
}

#[test]
fn example_input_2() {
    let (mut day, ctx) = setup::<Day09>(
        r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#,
    );
    assert_result(day.part2(&ctx), "36", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day09>();
    assert_result(day.part1(&ctx), "6067", "solve part 1");
    assert_result(day.part2(&ctx), "2471", "solve part 2");
}
