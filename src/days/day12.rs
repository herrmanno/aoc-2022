//! # Day 12 Hill Climbing Algorithm
//!
//! Given a 2d map where every square denotes its height (from 'a' - low to 'z' high) and a
//! start point 'S' and target point 'E', find
//!
//! - a) the distance of the shortest path from start to end
//! - b) the distance of the shortest path from end to an 'a' square
//!
//! where height distances between adjacent nodes in a path must not be greater than 1 (smaller
//! than zero is fine).
//!

use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner::Day;

type HeightMap = HashMap<(i32, i32), u32>;

#[derive(Default)]
pub struct Day12 {
    map: HeightMap,
    start: (i32, i32),
    target: (i32, i32),
}

impl Day for Day12 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        let mut map = HeightMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let y = y as i32;
                let x = x as i32;
                match ch {
                    'S' => {
                        map.insert((y, x), 'a'.into());
                        self.start = (y, x);
                    }
                    'E' => {
                        map.insert((y, x), 'z'.into());
                        self.target = (y, x);
                    }

                    ch => {
                        map.insert((y, x), ch.into());
                    }
                }
            }
        }

        self.map = map
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut queue = VecDeque::from([(0, self.start)]);
        let mut visited = HashSet::new();

        while let Some((distance, pos)) = queue.pop_front() {
            if pos == self.target {
                return distance;
            }

            if visited.contains(&pos) {
                continue;
            }

            visited.insert(pos);

            let current_height = *self.map.get(&pos).unwrap();
            let (y, x) = pos;
            for (ny, nx) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                let valid_neighbour = self
                    .map
                    .get(&(ny, nx))
                    .map(|&height| height <= current_height || height == current_height + 1)
                    .unwrap_or(false);

                if valid_neighbour {
                    queue.push_back((distance + 1, (ny, nx)));
                }
            }
        }

        unreachable!("No way fround from start to target");
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut queue = VecDeque::from([(0, self.target)]);
        let mut visited = HashSet::new();

        while let Some((distance, pos)) = queue.pop_front() {
            if self.map.get(&pos) == Some(&'a'.into()) {
                return distance;
            }

            if visited.contains(&pos) {
                continue;
            }

            visited.insert(pos);

            let current_height = *self.map.get(&pos).unwrap();
            let (y, x) = pos;
            for (ny, nx) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                let valid_neighbour = self
                    .map
                    .get(&(ny, nx))
                    .map(|&height| height >= current_height || height + 1 == current_height)
                    .unwrap_or(false);

                if valid_neighbour {
                    queue.push_back((distance + 1, (ny, nx)));
                }
            }
        }

        unreachable!("No way fround from start to target");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part_1() {
        let mut day = Day12::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 31);
    }

    #[test]
    fn part_2() {
        let mut day = Day12::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 29);
    }
}
