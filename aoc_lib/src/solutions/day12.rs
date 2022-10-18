use crate::solution::*;
use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
pub struct Day12 {
    cave_system: Option<CaveSystem>,
}
impl Solution for Day12 {
    fn info(&self) -> SolutionInfo {
        Title::new(2021, 12, "Passage Pathing")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        self.cave_system = Some(self.parse_input(ctx));
        let path_count = self.traverse(false);
        Ok(path_count.to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        let path_count = self.traverse(true);
        Ok(path_count.to_string())
    }
}
impl Day12 {
    fn traverse(&mut self, allow_duplicate: bool) -> i32 {
        let start = self.cave_system.as_ref().unwrap().start_id;
        self._traverse(
            start,
            &mut BTreeSet::from([start]),
            if allow_duplicate { 1 } else { 0 },
        )
    }

    fn _traverse(&self, current: usize, visited: &mut BTreeSet<usize>, remaining: i32) -> i32 {
        let mut path_count = 0;
        let sys = self.cave_system.as_ref().unwrap();
        let candidates = &sys.caves[current].paths;
        for &next_index in candidates {
            let next = &sys.caves[next_index];
            let is_duplicate = next.is_small && visited.contains(&next_index);
            let next_remaining_duplicates = remaining - if is_duplicate { 1 } else { 0 };
            match next_index {
                _ if next_index == sys.start_id => continue,
                _ if next_index == sys.end_id => {
                    path_count += 1;
                    continue;
                }
                _ if !next.is_small || (!is_duplicate || remaining > 0) => {
                    let could_insert = visited.insert(next_index);
                    path_count += self._traverse(next_index, visited, next_remaining_duplicates);
                    if could_insert {
                        visited.remove(&next_index);
                    }
                }
                _ => continue,
            }
        }

        path_count
    }

    fn parse_input(&self, ctx: &Context) -> CaveSystem {
        let mut next_id = 0;
        let mut caves: HashMap<String, Cave> = HashMap::new();
        for line in ctx.input().lines() {
            let parts = line.split("-").collect::<Vec<_>>();
            parts
                .iter()
                .filter_map(|&key| match caves.contains_key(key) {
                    true => None,
                    false => {
                        let result = Some(Cave {
                            id: next_id,
                            name: key.to_string(),
                            paths: Vec::new(),
                            is_small: key == key.to_lowercase(),
                        });
                        next_id += 1;
                        result
                    }
                })
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|c| {
                    caves.insert(c.name.clone(), c);
                });

            let id0 = caves[parts[0]].id;
            let id1 = caves[parts[1]].id;
            caves.get_mut(parts[0]).unwrap().paths.push(id1);
            caves.get_mut(parts[1]).unwrap().paths.push(id0);
        }

        CaveSystem {
            start_id: caves["start"].id,
            end_id: caves["end"].id,
            caves: {
                let mut caves = caves.into_iter().map(|x| x.1).collect::<Vec<_>>();
                caves.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
                caves
            },
        }
    }
}

#[derive(Debug)]
struct CaveSystem {
    caves: Vec<Cave>,
    start_id: usize,
    end_id: usize,
}

#[derive(Debug)]
struct Cave {
    id: usize,
    name: String,
    paths: Vec<usize>,
    is_small: bool,
}
