use crate::solution::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default)]
pub struct Day12 {
    caves: Caves,
}
impl Solution for Day12 {
    fn info(&self) -> SolutionInfo {
        Title::new(2021, 12, "Passage Pathing")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        self.caves = self.parse_input(&ctx.input());
        let start = Rc::clone(&self.caves["start"]);
        let path_count = self.traverse(&mut vec![start], true);

        return Ok(path_count.to_string());
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        let start = Rc::clone(&self.caves["start"]);
        let path_count = self.traverse(&mut vec![start], false);

        return Ok(path_count.to_string());
    }
}
impl Day12 {
    fn parse_input(&self, input: &str) -> Caves {
        let mut caves: Caves = HashMap::new();
        for line in input.lines() {
            line.split("-").for_each(|key| {
                caves.insert(
                    key.to_string(),
                    Rc::new(Cave::new(key.to_string(), RefCell::new(Vec::new()))),
                );
            });
        }

        for line in input.lines() {
            let parts = line.split("-").collect::<Vec<_>>();
            let a = &caves[parts[0]];
            let b = &caves[parts[1]];
            a.paths.borrow_mut().push(Rc::clone(b));
            b.paths.borrow_mut().push(Rc::clone(a));
        }

        let caves = caves;

        caves
    }

    fn traverse(&self, path: &mut Vec<Rc<Cave>>, has_duplicate: bool) -> i32 {
        let mut path_count = 0;
        let path_last = Rc::clone(path.last().expect("non-empty path"));
        let candidates = path_last.paths.borrow();
        for next in candidates.iter() {
            match Rc::clone(next) {
                c if c.is_end => {
                    path_count += 1;
                    continue;
                }
                c if c.is_start => {
                    continue;
                }
                c if c.is_small && path.contains(&c) => {
                    if has_duplicate {
                        continue;
                    }
                    path.push(c);
                    path_count += self.traverse(path, true);
                    path.pop();
                }
                c => {
                    path.push(c);
                    path_count += self.traverse(path, has_duplicate);
                    path.pop();
                }
            }
        }

        path_count
    }
}

#[derive(Default, PartialEq, Eq)]
struct Cave {
    name: String,
    paths: RefCell<Vec<Rc<Cave>>>,
    is_small: bool,
    is_start: bool,
    is_end: bool,
}
impl Cave {
    fn new(name: String, paths: RefCell<Vec<Rc<Cave>>>) -> Self {
        Self {
            is_small: &name[0..1].to_lowercase() == &name[0..1],
            is_start: &name == "start",
            is_end: &name == "end",
            name,
            paths,
        }
    }
}

type Caves = HashMap<String, Rc<Cave>>;
