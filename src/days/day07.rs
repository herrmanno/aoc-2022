//! # Day 07 No Space Left On Device
//!
//! Given a list of 'commands' (cd, ls) and 'outputs' (dir & file entries), find
//!
//! - a) all dirs that contain less than 100000 bytes in files
//! - b) the smallest dir to remove to obtain 30000000 bytes of free disk space

use aoc_runner::Day;
use std::collections::HashMap;

const DISK_SIZE: usize = 70000000;
const NEEDED_SPACE: usize = 30000000;
const SMALL_FILE: usize = 100000;

#[derive(Default)]
pub struct Day07(Vec<String>, Vec<usize>);

impl Day for Day07 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.0 = input.split('\n').map(str::to_owned).collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut dir_map = HashMap::new();
        let mut cwds: Vec<String> = vec![];
        for entry in self.0.iter() {
            if let Some(dir) = entry.strip_prefix("$ cd ") {
                if dir == ".." {
                    cwds.pop();
                } else {
                    let new_dir = [cwds.last().unwrap_or(&"".to_owned()), dir].join("/");
                    cwds.push(new_dir);
                }
            } else if entry == "$ ls" {
                dir_map.insert(cwds.last().unwrap().clone(), 0);
            } else if !entry.starts_with("dir") {
                let size: usize = entry.split(' ').next().unwrap().parse().unwrap();
                for dir_name in cwds.iter() {
                    *dir_map.get_mut(dir_name).unwrap() += size;
                }
            }
        }

        self.1 = {
            let mut dirs: Vec<usize> = dir_map.into_values().collect();
            dirs.sort_unstable();
            dirs
        };

        self.1.iter().take_while(|v| *v < &SMALL_FILE).sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        // initialize directory sizes
        if self.1.is_empty() {
            self.part1();
        }

        let free_size = DISK_SIZE - self.1.last().unwrap();
        let needed_size = NEEDED_SPACE - free_size;
        for &size in self.1.iter() {
            if size >= needed_size {
                return size;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "$ cd /
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
7214296 k";

    #[test]
    fn part_1() {
        let mut day = Day07::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 95437);
    }

    #[test]
    fn part_2() {
        let mut day = Day07::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 24933642);
    }
}
