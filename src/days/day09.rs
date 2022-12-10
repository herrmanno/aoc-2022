//! # Day 09 Rope Bridge
//!
//! Given a list of movements (direction + steps) calculate how a rope consisting of
//!
//! - a) two knots (elements)
//! - b) ten knots
//!
//! behaves while moving and track all fields the rope's tail visits

use aoc_runner::Day;
#[cfg(debug_assertions)]
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day09(String);

type Knot = (i32, i32);

/// Given a moved knot `head`, let `tail` follow `head` and return its new position
#[inline(always)]
fn follow_knot(head: Knot, mut tail: Knot) -> Knot {
    if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
        tail.0 += (head.0 - tail.0).signum();
        tail.1 += (head.1 - tail.1).signum();
    }

    tail
}

/// Print rope for debugging reasons
#[allow(dead_code)]
#[cfg(debug_assertions)]
fn print_rope(rope: &[Knot]) {
    let map: HashMap<Knot, usize> = rope
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, knot)| (knot, idx))
        .collect();

    for y in -10..=10 {
        for x in -10..=10 {
            let c = if let Some(idx) = map.get(&(x, y)) {
                match idx {
                    0 => "H".to_owned(),
                    i => i.to_string(),
                }
            } else {
                ".".to_owned()
            };
            if (x, y) == (0, 0) {
                print!("s");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

macro_rules! diff {
    ($dir: expr) => {
        match $dir {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("Bad direction: {}", $dir),
        }
    };
}

impl Day for Day09 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.0 = input.to_owned();
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut head: (i32, i32) = (0, 0);
        let mut tail: (i32, i32) = (0, 0);
        let mut tails: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

        for line in self.0.lines() {
            let (dir, steps): (&str, usize) = line
                .split_once(' ')
                .map(|(dir, steps)| (dir, steps.parse().unwrap()))
                .unwrap();

            let diff = diff!(dir);

            for _ in 0..steps {
                head.0 += diff.0;
                head.1 += diff.1;

                tail = follow_knot(head, tail);

                tails.insert(tail);
            }
        }

        tails.len()
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut knots: [(i32, i32); 10] = [(0, 0); 10];
        let mut tail_positions: HashSet<usize> = HashSet::from([0]);

        for line in self.0.lines() {
            let (dir, steps): (&str, usize) = line
                .split_once(' ')
                .map(|(dir, steps)| (dir, steps.parse().unwrap()))
                .unwrap();

            let diff = diff!(dir);

            for _ in 0..steps {
                let head = &mut knots[0];
                head.0 += diff.0;
                head.1 += diff.1;

                // 'unroll' loop for performance reasons
                knots[1] = follow_knot(knots[0], knots[1]);
                knots[2] = follow_knot(knots[1], knots[2]);
                knots[3] = follow_knot(knots[2], knots[3]);
                knots[4] = follow_knot(knots[3], knots[4]);
                knots[5] = follow_knot(knots[4], knots[5]);
                knots[6] = follow_knot(knots[5], knots[6]);
                knots[7] = follow_knot(knots[6], knots[7]);
                knots[8] = follow_knot(knots[7], knots[8]);
                knots[9] = follow_knot(knots[8], knots[9]);

                tail_positions.insert(
                    (u32::from_be_bytes(knots[9].0.to_be_bytes()) as usize) << 32
                        | (u32::from_be_bytes(knots[9].1.to_be_bytes()) as usize),
                );
            }
        }

        tail_positions.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part_1() {
        let mut day = Day09::default();
        day.parse(INPUT1);
        assert_eq!(day.part1(), 13);
    }

    #[test]
    fn part_2() {
        let mut day = Day09::default();
        day.parse(INPUT2);
        assert_eq!(day.part2(), 36);
    }
}
