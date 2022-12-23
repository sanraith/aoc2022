use crate::solution::*;
use crate::solutions::year2022::Day13;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day13>(
        r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#,
    );
    assert_result(day.part1(&ctx), "13", "solve part 1");
    assert_result(day.part2(&ctx), "140", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day13>();
    assert_result(day.part1(&ctx), "6428", "solve part 1");
    assert_result(day.part2(&ctx), "22464", "solve part 2");
}
