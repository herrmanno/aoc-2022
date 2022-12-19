//! # Day 14 Regolith Reservoir
//!
//! Given a list of 'draw' commands that define lines, construct a maze (cave) from those lines
//! and let single sand elements fall from coordinate x=500 y=0 until
//!
//! - a) the first sand particle falls into infinity
//! - b) no more particles can fall, given there is a floor at max(y) + 2
//!

use std::collections::VecDeque;

use rustc_hash::FxHashSet as HashSet;

use aoc_runner::Day;

type Cave = HashSet<(i32, i32)>;

#[derive(Default)]
pub struct Day14(Cave, i32);

impl Day for Day14 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        let mut cave = Cave::default();
        let mut max_y = 0;
        let mut seen_lines = HashSet::default();
        for line in input.lines() {
            // it seems there are duplicate lines in the input
            if seen_lines.contains(&line) {
                continue;
            }
            seen_lines.insert(line);

            let instructions = line
                .split(" -> ")
                .map(|part| {
                    part.split_once(',')
                        .and_then(|(x, y)| Some((x.parse::<i32>().ok()?, y.parse::<i32>().ok()?)))
                        .expect("Bad coordinates")
                })
                .collect::<Vec<(i32, i32)>>();

            for window in instructions.windows(2) {
                match window {
                    [(x1, y1), (x2, y2)] => {
                        max_y = max_y.max(*y1).max(*y2);

                        for x in *(x1.min(x2))..=*(x1.max(x2)) {
                            for y in *(y1.min(y2))..=*(y1.max(y2)) {
                                cave.insert((x, y));
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        self.0 = cave;
        self.1 = max_y;
    }

    fn part1(&mut self) -> Self::Result1 {
        let max_y = self.1;
        let mut n = 0;

        let mut stack: VecDeque<(i32, i32)> = VecDeque::new();
        'outer: loop {
            let (mut x, mut y) = stack.pop_front().unwrap_or((500, 0));
            loop {
                if !self.0.contains(&(x, y + 1)) {
                    stack.push_front((x, y));
                    y += 1;
                } else if !self.0.contains(&(x - 1, y + 1)) {
                    stack.push_front((x, y));
                    x -= 1;
                    y += 1;
                } else if !self.0.contains(&(x + 1, y + 1)) {
                    stack.push_front((x, y));
                    x += 1;
                    y += 1;
                } else {
                    self.0.insert((x, y));
                    n += 1;

                    continue 'outer;
                }

                if y > max_y {
                    break 'outer;
                }
            }
        }

        n
    }

    fn part2(&mut self) -> Self::Result2 {
        let floor_y = self.1 + 2;

        let mut n = 0;
        let mut stack: VecDeque<(i32, i32)> = VecDeque::new();
        'outer: loop {
            let (mut x, mut y) = stack.pop_front().unwrap_or((500, 0));
            loop {
                if !self.0.contains(&(x, y + 1)) {
                    stack.push_front((x, y));
                    y += 1;
                } else if !self.0.contains(&(x - 1, y + 1)) {
                    stack.push_front((x, y));
                    x -= 1;
                    y += 1;
                } else if !self.0.contains(&(x + 1, y + 1)) {
                    stack.push_front((x, y));
                    x += 1;
                    y += 1;
                } else {
                    self.0.insert((x, y));
                    n += 1;
                    if y == 0 {
                        break 'outer;
                    } else {
                        continue 'outer;
                    }
                }

                if y + 1 == floor_y {
                    self.0.insert((x, y));
                    n += 1;
                    continue 'outer;
                }
            }
        }

        n
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part_1() {
        let mut day = Day14::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 24);
    }

    #[test]
    fn part_2() {
        let mut day = Day14::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 93);
    }
}
