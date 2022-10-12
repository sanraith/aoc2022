use super::*;
use crate::solutions::Day01;

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
    assert_eq!(day.part1(&ctx).unwrap(), "7", "solve part 1");
    assert_eq!(day.part2(&ctx).unwrap(), "5", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day01>();
    assert_eq!(day.part1(&ctx).unwrap(), "1292", "solve part 1");
    assert_eq!(day.part2(&ctx).unwrap(), "1262", "solve part 2");
}
