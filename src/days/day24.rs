//! # Day 24 Blizzard Basin
//!
//! Given a 2d map and a set of blizzard moving across that map
//!
//! - a) find the shortest path (time) from start to target
//! - b) find the shortest path (time) from start -> target -> start -> target

use std::collections::BinaryHeap;

use aoc_runner::Day;
use rustc_hash::{FxHashSet as HashSet};

type C = i32;
type Coord = (C, C);

#[derive(Default, Clone)]
pub struct Day24 {
    row_length: usize,
    col_length: usize,
    rows: Vec<Vec<(usize, i8)>>,
    cols: Vec<Vec<(usize, i8)>>,
}

impl Day for Day24 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.col_length = input.lines().count() - 2;
        self.row_length = input.lines().next().unwrap().len() - 2;
        self.rows = vec![vec![]; self.col_length + 2];
        self.cols = vec![vec![]; self.row_length + 2];

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| {
                match ch {
                    '>' => {
                        self.rows.get_mut(y).unwrap().push((x, -1));
                    }
                    '<' => {
                        self.rows.get_mut(y).unwrap().push((x, 1));
                    }
                    'v' => {
                        self.cols.get_mut(x).unwrap().push((y, -1));
                    }
                    '^' => {
                        self.cols.get_mut(x).unwrap().push((y, 1));
                    }
                    _ => {}
                };
            });
        });
    }

    fn part1(&mut self) -> Self::Result1 {
        self.shortest_path((0, 1), (self.col_length as C + 1, self.row_length as C), 0)
    }

    fn part2(&mut self) -> Self::Result2 {
        let start = (0, 1);
        let target = (self.col_length as C + 1, self.row_length as C);
        let a = self.shortest_path(start, target, 0);
        let b = self.shortest_path(target, start, a);
        self.shortest_path(start, target, b)
    }
}

impl Day24 {
    fn shortest_path(&self, start: Coord, target: Coord, time: usize) -> usize {
        #[derive(Debug, PartialEq, Eq)]
        struct State(usize, Coord, u32);

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                other.0.cmp(&self.0).then(other.2.cmp(&self.2))
            }
        }

        let mut visited: HashSet<(usize, Coord)> = Default::default();
        let mut queue: BinaryHeap<State> = Default::default();
        let start_state = State(time, start, 0);
        queue.push(start_state);

        while let Some(State(time, pos, _)) = queue.pop() {
            if !visited.insert((time, pos)) {
                continue;
            }

            for (dy, dx) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                let new_pos = (pos.0 + dy, pos.1 + dx);
                if new_pos == target {
                    return time + 1;
                }

                if new_pos != start
                    && (new_pos.0 <= 0
                        || new_pos.0 > self.col_length as C
                        || new_pos.1 <= 0
                        || new_pos.1 > self.row_length as C)
                {
                    continue;
                }

                if !self.is_blizzard_at(new_pos, time + 1) {
                    let distance = new_pos.0.abs_diff(target.0) + new_pos.1.abs_diff(target.1);
                    let new_state = State(time + 1, new_pos, distance);
                    queue.push(new_state);
                }
            }
        }

        panic!("Not path found");
    }

    fn is_blizzard_at(&self, coord: Coord, time: usize) -> bool {
        let row_blizz = self
            .rows
            .get(coord.0 as usize)
            .map(|blizzards| {
                blizzards.iter().any(|blizz| {
                    (coord.1 - 1 + blizz.1 as i32 * time as C).rem_euclid(self.row_length as C)
                        == blizz.0 as i32 - 1
                })
            })
            .unwrap_or(false);

        let col_blizz = self
            .cols
            .get(coord.1 as usize)
            .map(|blizzards| {
                blizzards.iter().any(|blizz| {
                    (coord.0 - 1 + blizz.1 as i32 * time as C).rem_euclid(self.col_length as C)
                        == blizz.0 as i32 - 1
                })
            })
            .unwrap_or(false);

        row_blizz || col_blizz
    }

    #[cfg(debug_assertions)]
    fn print_map(&self, time: usize, pos: Coord, start: Coord, target: Coord) {
        for y in 0..self.col_length as C + 2 {
            for x in 0..self.row_length as C + 2 {
                if (y, x) == start {
                    print!("S");
                } else if (y, x) == target {
                    print!("T");
                } else if pos == (y, x) {
                    print!("X");
                } else if y == 0
                    || x == 0
                    || y == self.col_length as C + 1
                    || x == self.row_length as C + 1
                {
                    print!("#");
                } else {
                    print!(
                        "{}",
                        if self.is_blizzard_at((y as C, x as C), time) {
                            "@"
                        } else {
                            "."
                        }
                    );
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn part_1() {
        let mut day = Day24::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 18);
    }

    #[test]
    fn part_2() {
        let mut day = Day24::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 54);
    }
}
