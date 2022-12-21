use crate::solution::*;
use crate::solutions::year2022::Day21;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day21>(
        r#"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#,
    );
    assert_result(day.part1(&ctx), "152", "solve part 1");
    assert_result(day.part2(&ctx), "301", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day21>();
    assert_result(day.part1(&ctx), "194501589693264", "solve part 1");
    assert_result(day.part2(&ctx), "3887609741189", "solve part 2");
}
