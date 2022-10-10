use super::*;
use crate::solutions::Day01;

#[test]
fn test_input() {
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
    assert_eq!(day.part1(&ctx).unwrap(), "7");
    assert_eq!(day.part2(&ctx).unwrap(), "5");
}

#[test]
fn puzzle_input() {
    let (mut _day, _ctx) = setup_from_file::<Day01>();
}
