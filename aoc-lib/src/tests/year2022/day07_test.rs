use crate::solution::*;
use crate::solutions::year2022::Day07;
use crate::tests::util::*;

#[test]
fn example_input() {
    let (mut day, ctx) = setup::<Day07>(
        r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#,
    );
    assert_result(day.part1(&ctx), "95437", "solve part 1");
    assert_result(day.part2(&ctx), "24933642", "solve part 2");
}

#[test]
fn puzzle_input() {
    let (mut day, ctx) = setup_from_file::<Day07>();
    assert_result(day.part1(&ctx), "1118405", "solve part 1");
    assert_result(day.part2(&ctx), "12545514", "solve part 2");
}
