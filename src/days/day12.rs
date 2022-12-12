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

use std::collections::VecDeque;

use aoc_runner::Day;

type HeightMap = Vec<Vec<u32>>;

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
        let map_height = input.lines().count();
        let map_width = input.lines().next().unwrap().len();
        let mut map = vec![vec![0; map_width]; map_height];
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    'S' => {
                        map[y][x] = 'a'.into();
                        self.start = (y as i32, x as i32);
                    }
                    'E' => {
                        map[y][x] = 'z'.into();
                        self.target = (y as i32, x as i32);
                    }
                    ch => {
                        map[y][x] = ch.into();
                    }
                }
            }
        }

        self.map = map
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut queue = VecDeque::from([(0, self.start)]);
        let map_height = self.map.len();
        let map_width = self.map[0].len();
        let mut visited = vec![vec![false; map_width]; map_height];

        while let Some((distance, pos)) = queue.pop_front() {
            if pos == self.target {
                return distance;
            }

            if visited[pos.0 as usize][pos.1 as usize] {
                continue;
            }

            visited[pos.0 as usize][pos.1 as usize] = true;

            let (y, x) = pos;
            let current_height = self.map[y as usize][x as usize];
            for (ny, nx) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                if ny < 0 || nx < 0 {
                    continue;
                }

                let valid_neighbour = self
                    .map
                    .get(ny as usize)
                    .and_then(|row| row.get(nx as usize))
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
        let map_height = self.map.len();
        let map_width = self.map[0].len();
        let mut visited = vec![vec![false; map_width]; map_height];

        while let Some((distance, pos)) = queue.pop_front() {
            if self
                .map
                .get(pos.0 as usize)
                .and_then(|row| row.get(pos.1 as usize))
                == Some(&'a'.into())
            {
                return distance;
            }

            if visited[pos.0 as usize][pos.1 as usize] {
                continue;
            }

            visited[pos.0 as usize][pos.1 as usize] = true;

            let (y, x) = pos;
            let current_height = self.map[y as usize][x as usize];
            for (ny, nx) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                if ny < 0 || nx < 0 {
                    continue;
                }

                let valid_neighbour = self
                    .map
                    .get(ny as usize)
                    .and_then(|row| row.get(nx as usize))
                    .map(|&height| height >= current_height || height + 1 == current_height)
                    .unwrap_or(false);

                if valid_neighbour {
                    queue.push_back((distance + 1, (ny, nx)));
                }
            }
        }

        unreachable!("No way fround from target to 'a'");
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
