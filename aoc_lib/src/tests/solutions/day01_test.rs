use crate::solution::*;
use crate::solutions::Day01;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day01>(
        r#"
199
200
208
210
200
207
240
269
260
263"#,
    );
    assert_result(day.part1(&ctx), "7", "solve part 1");
    assert_result(day.part2(&ctx), "5", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day01>();
    assert_result(day.part1(&ctx), "1292", "solve part 1");
    assert_result(day.part2(&ctx), "1262", "solve part 2");
}
