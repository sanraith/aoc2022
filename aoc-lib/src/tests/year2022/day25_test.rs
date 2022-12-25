use crate::solution::*;
use crate::solutions::year2022::Day25;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day25>(
        r#"
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#,
    );
    assert_result(day.part1(&ctx), "2=-1=0", "solve part 1");
    assert_result(day.part2(&ctx), "*", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day25>();
    //assert_result(day.part1(&ctx), "day25_part1", "solve part 1");
    assert_result(day.part2(&ctx), "*", "solve part 2");
}
