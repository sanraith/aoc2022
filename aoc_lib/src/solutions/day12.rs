use crate::helpers::*;
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
        let path_count = self.count_paths(0);
        Ok(path_count.to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        let path_count = self.count_paths(1);
        Ok(path_count.to_string())
    }
}
impl Day12 {
    fn count_paths(&mut self, allowed_duplicates: i32) -> i32 {
        let start = self.cave_system.as_ref().unwrap().start_id;
        self._count_paths(start, &mut Vec::new(), allowed_duplicates)
    }

    fn _count_paths(&self, current: usize, visited: &mut Vec<usize>, remaining: i32) -> i32 {
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
                    path_count += self._count_paths(next_index, visited, next_remaining_duplicates);
                }
                _ => continue,
            }
        }

        visited.pop();
        path_count
    }

    fn parse_input(&self, ctx: &Context) -> CaveSystem {
        let mut next_id = 0;
        let caves: HashMap<String, Cave> =
            ctx.input().lines().fold(HashMap::new(), |mut caves, line| {
                let pair = line.split("-").collect::<Vec<_>>();
                for &name in pair.iter() {
                    caves.entry(name.to_string()).or_insert_with(|| Cave {
                        id: post_increment(&mut next_id),
                        _name: name.to_string(),
                        paths: Vec::new(),
                        is_small: name == name.to_lowercase(),
                    });
                }
                for id_a in 0..pair.len() {
                    let id_b = caves[pair[(id_a + 1) % 2]].id;
                    caves.get_mut(pair[id_a]).unwrap().paths.push(id_b);
                }
                caves
            });

        CaveSystem {
            start_id: caves["start"].id,
            end_id: caves["end"].id,
            caves: caves
                .into_values()
                .into_sorted_by(|a, b| a.id.partial_cmp(&b.id).unwrap()),
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
    _name: String,
    paths: Vec<usize>,
    is_small: bool,
}
