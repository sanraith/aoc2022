use crate::solution::*;
use std::collections::HashMap;

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
        let path_count = self.traverse(0);
        Ok(path_count.to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        let path_count = self.traverse(1);
        Ok(path_count.to_string())
    }
}
impl Day12 {
    fn traverse(&mut self, allowed_duplicates: i32) -> i32 {
        let start = self.cave_system.as_ref().unwrap().start_id;
        self._traverse(start, &mut Vec::new(), allowed_duplicates)
    }

    fn _traverse(&self, current: usize, visited: &mut Vec<usize>, remaining: i32) -> i32 {
        let sys = self.cave_system.as_ref().unwrap();
        let mut path_count = 0;
        visited.push(current);

        for &next_index in &sys.caves[current].paths {
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
                    path_count += self._traverse(next_index, visited, next_remaining_duplicates);
                }
                _ => continue,
            }
        }

        visited.pop();
        path_count
    }

    fn parse_input(&self, ctx: &Context) -> CaveSystem {
        let mut next_id = 0;
        let mut caves: HashMap<String, Cave> = HashMap::new();
        for line in ctx.input().lines() {
            let names = line.split("-").collect::<Vec<_>>();
            names
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

            let id0 = caves[names[0]].id;
            let id1 = caves[names[1]].id;
            caves.get_mut(names[0]).unwrap().paths.push(id1);
            caves.get_mut(names[1]).unwrap().paths.push(id0);
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
