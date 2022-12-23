//! # Day 23 Unstable Diffusion
//!
//! Given a map of occupied positions and a specific set of rules
//!
//! - a) do 10 rounds of 'game of life'
//! - b) count number of rounds until fixpointn is reached
//!

use aoc_runner::Day;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

type C = i32;
type Coord = (C, C);

#[derive(Default, Clone)]
pub struct Day23(HashSet<Coord>, bool);

impl Day for Day23 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, ch)| {
                    if ch == '#' {
                        Some((y as C, x as C))
                    } else {
                        None
                    }
                })
            })
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        do_rounds(10, &mut self.0);
        let (min_x, max_x, min_y, max_y) = get_min_max(&self.0);

        self.1 = true;

        (min_y..=max_y)
            .map(move |y| {
                (min_x..=max_x)
                    .map(|x| !self.0.contains(&(y, x)) as usize)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut n = if self.1 { 11 } else { 1 };
        let mut direction = if self.1 { 10 % 4 } else { 0 };

        while do_round(&mut self.0, direction) != 0 {
            direction = (direction + 1) % 4;
            n += 1;
        }

        n
    }
}

fn do_rounds(n: usize, map: &mut HashSet<Coord>) {
    let mut dir = 0;
    for _ in 0..n {
        do_round(map, dir);
        dir = (dir + 1) % 4;
    }
}

fn do_round(map: &mut HashSet<Coord>, direction: u8) -> usize {
    #[inline(always)]
    fn neighbours8((y, x): Coord) -> [Coord; 8] {
        [
            (y, x - 1),
            (y, x + 1),
            (y - 1, x - 1),
            (y - 1, x),
            (y - 1, x + 1),
            (y + 1, x - 1),
            (y + 1, x),
            (y + 1, x + 1),
        ]
    }

    #[inline(always)]
    fn direction_lookouts(direction: u8) -> [Coord; 3] {
        match direction {
            // north
            0 => [(-1, 0), (-1, -1), (-1, 1)],
            // south
            1 => [(1, 0), (1, -1), (1, 1)],
            // west
            2 => [(0, -1), (-1, -1), (1, -1)],
            // east
            3 => [(0, 1), (-1, 1), (1, 1)],
            _ => unreachable!(),
        }
    }

    /// Description of a tile
    enum Proposal {
        /// Tile is blocked because more than one elf already proposed this tile
        Blocked,
        /// Tile was proposed by elf currently at [Proposal::ProposedBy(0)]
        ProposedBy(Coord)
    }

    let mut proposals: HashMap<Coord, Proposal> = Default::default();
    'positions: for &pos in map.iter() {
        if neighbours8(pos).into_iter().any(|p| map.contains(&p)) {
            for offset in 0..4 {
                let lookouts = direction_lookouts((direction + offset) % 4);
                let proposed_dir = lookouts[0];
                if lookouts
                    .into_iter()
                    .all(|p| !map.contains(&(pos.0 + p.0, pos.1 + p.1)))
                {
                    let proposed_pos = (pos.0 + proposed_dir.0, pos.1 + proposed_dir.1);
                    proposals.entry(proposed_pos)
                        .and_modify(|prop| *prop = Proposal::Blocked)
                        .or_insert(Proposal::ProposedBy(pos));

                    continue 'positions;
                }
            }
        }
    }

    let mut num_moves = 0;
    for (to, from) in proposals.into_iter() {
        if let Proposal::ProposedBy(from) = from {
            num_moves += 1;
            map.remove(&from);
            map.insert(to);
        }
    }

    num_moves
}

#[inline(always)]
fn get_min_max(map: &HashSet<Coord>) -> (C, C, C, C) {
    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        map.iter().next().map(|(y, x)| (*x, *x, *y, *y)).unwrap();

    for pos in map.iter() {
        min_x = min_x.min(pos.1);
        max_x = max_x.max(pos.1);
        min_y = min_y.min(pos.0);
        max_y = max_y.max(pos.0);
    }

    (min_x, max_x, min_y, max_y)
}

#[cfg(debug_assertions)]
fn print_map(map: &HashSet<Coord>) {
    let (min_x, max_x, min_y, max_y) = get_min_max(map);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", if map.contains(&(y, x)) { '#' } else { '.' });
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn part_1() {
        let mut day = Day23::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 110);
    }

    #[test]
    fn part_2() {
        let mut day = Day23::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 20);
    }
}
