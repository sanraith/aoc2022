use crate::solution::*;
use itertools::Itertools;
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Default)]
pub struct Day07;
impl Solution for Day07 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 7, "No Space Left On Device")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let (_, directories) = parse_input(ctx);
        let sum_of_small_directories = directories
            .iter()
            .map(|dir| calc_size(&dir))
            .filter(|&size| size <= 100000)
            .sum::<u64>();

        Ok(sum_of_small_directories.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let (root, directories) = parse_input(ctx);
        let disk_size = 70000000;
        let total_required_space = 30000000;
        let current_space = disk_size - calc_size(&root);
        let required_space = total_required_space - current_space;

        let size_of_dir_to_delete = directories
            .iter()
            .map(|dir| calc_size(&dir))
            .sorted()
            .find(|&size| size >= required_space)
            .unwrap();

        Ok(size_of_dir_to_delete.to_string())
    }
}

fn calc_size(dir: &Rc<Directory>) -> u64 {
    dir.files.borrow().values().map(|f| f.size).sum::<u64>()
        + dir
            .directories
            .borrow()
            .values()
            .map(|x| calc_size(x))
            .sum::<u64>()
}

fn parse_input(ctx: &Context) -> (Rc<Directory>, Vec<Rc<Directory>>) {
    let root = Rc::new(Directory {
        _name: "/".to_owned(),
        ..Default::default()
    });
    let mut directories = vec![Rc::clone(&root)];
    let mut current = Rc::clone(&root);

    for line in ctx.input().lines() {
        let cmd = line.split(" ").collect_vec();
        match cmd[0] {
            "$" if line == "$ cd /" => current = Rc::clone(&root),
            "$" if line == "$ cd .." => {
                let parent = current.parent.upgrade().unwrap();
                current = Rc::clone(&parent);
            }
            "$" if cmd[1] == "cd" => {
                let children = current.directories.borrow();
                let child = Rc::clone(&children.get(cmd[2]).unwrap());
                drop(children);
                current = child;
            }
            "$" if cmd[1] == "ls" => (),
            "dir" => {
                let name = cmd[1];
                let dir = Rc::new(Directory {
                    _name: name.to_owned(),
                    parent: Rc::downgrade(&current),
                    ..Default::default()
                });
                current
                    .borrow_mut()
                    .directories
                    .borrow_mut()
                    .insert(name.to_owned(), Rc::clone(&dir));
                directories.push(Rc::clone(&dir));
            }
            _ => {
                let name = cmd[1];
                let file = File {
                    _name: name.to_owned(),
                    size: cmd[0].parse().unwrap(),
                };
                current
                    .borrow_mut()
                    .files
                    .borrow_mut()
                    .insert(name.to_owned(), file);
            }
        }
    }

    (root, directories)
}

#[derive(Debug)]
struct File {
    _name: String,
    size: u64,
}
#[derive(Default, Debug)]
struct Directory {
    _name: String,
    parent: Weak<Directory>,
    files: RefCell<HashMap<String, File>>,
    directories: RefCell<HashMap<String, Rc<Directory>>>,
}
