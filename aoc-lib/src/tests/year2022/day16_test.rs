use crate::solution::*;
use crate::solutions::year2022::Day16;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day16>(
        r#"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#,
    );
    assert_result(day.part1(&ctx), "1651", "solve part 1");
    assert_result(day.part2(&ctx), "1707", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day16>();
    assert_result(day.part1(&ctx), "2124", "solve part 1");
    assert_result(day.part2(&ctx), "2775", "solve part 2"); // too slow
}
