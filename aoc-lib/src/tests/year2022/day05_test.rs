use crate::solution::*;
use crate::solutions::year2022::Day05;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day05>(
        r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#,
    );
    assert_result(day.part1(&ctx), "CMZ", "solve part 1");
    assert_result(day.part2(&ctx), "MCD", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day05>();
    assert_result(day.part1(&ctx), "PTWLTDSJV", "solve part 1");
    assert_result(day.part2(&ctx), "WZMFVGGZP", "solve part 2");
}
