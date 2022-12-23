use crate::solution::*;
use crate::solutions::year2022::Day01;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day01>(
        r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#,
    );
    assert_result(day.part1(&ctx), "24000", "solve part 1");
    assert_result(day.part2(&ctx), "45000", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day01>();
    assert_result(day.part1(&ctx), "70613", "solve part 1");
    assert_result(day.part2(&ctx), "205805", "solve part 2");
}
