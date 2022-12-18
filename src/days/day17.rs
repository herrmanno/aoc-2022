//! # Day 17 Pyroclastic Flow
//!
//! Given a set of 'jet streams' (< 'left' and > 'right' movement), simulate a kind of Tetris game
//! were five different rock types are falling down and pushed sideways by the pattern described
//! by the jet streams.
//! Find the maximal height of the Tetris tower after
//!
//! - a) 2022
//! - b) 1000000000000
//!
//! falling rocks.

use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

use aoc_runner::Day;

#[derive(Default, Clone)]
pub struct Day17(String);

impl Day for Day17 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.0 = input.to_owned();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.play_for_n_rounds(2022)
    }

    fn part2(&mut self) -> Self::Result2 {
        self.play_for_n_rounds(1000000000000)
    }
}

impl Day17 {
    const PIECES: usize = 5;
    fn play_for_n_rounds(&self, n: usize) -> usize {
        let jets = self.0.as_bytes();
        let jets_len = jets.len();
        let mut piece_idx = 0;
        let mut jet_idx = 0;
        let mut tower: HashSet<(usize, u8)> = Default::default();
        // max y value for every column
        let mut floor = [0usize; 7];
        let mut cycle_cache: HashMap<(usize, usize), ([i128; 6], usize, usize)> =
            HashMap::default();

        let mut bonus: Option<usize> = None;
        let mut round = 0;
        loop {
            if round == n {
                break;
            }

            let max_height = *floor.iter().max().unwrap();

            // cycle detection
            if bonus.is_none() {
                let cache_key = (piece_idx, jet_idx);
                let cache_value = {
                    let mut value = [0; 6];
                    for i in 0..5 {
                        value[i] = floor[i] as i128 - floor[i + 1] as i128;
                    }
                    (value, round, max_height)
                };

                if let Some((floor_old, round_old, max_height_old)) = cycle_cache.get(&cache_key) {
                    if floor_old == &cache_value.0 {
                        let remaining_rounds = n - round;
                        let cycle_length = round - round_old;
                        let cycles_to_skip = remaining_rounds / cycle_length;
                        let cycle_height = max_height - max_height_old;

                        round += cycles_to_skip * cycle_length;
                        bonus = Some(cycles_to_skip * cycle_height);
                    }
                } else {
                    cycle_cache.insert(cache_key, cache_value);
                }
            }

            // rock creation
            let m = max_height + 4;
            let mut rock: Vec<(usize, u8)> = match piece_idx {
                0 => {
                    vec![(m, 2), (m, 3), (m, 4), (m, 5)]
                }
                1 => {
                    vec![(m + 1, 2), (m, 3), (m + 1, 3), (m + 2, 3), (m + 1, 4)]
                }
                2 => {
                    vec![(m, 2), (m, 3), (m, 4), (m + 1, 4), (m + 2, 4)]
                }
                3 => {
                    vec![(m, 2), (m + 1, 2), (m + 2, 2), (m + 3, 2)]
                }
                4 => {
                    vec![(m, 2), (m + 1, 2), (m, 3), (m + 1, 3)]
                }
                _ => unreachable!(),
            };

            // rock movement
            loop {
                // jet movement
                let jet = jets[jet_idx];

                let can_jet_left = rock
                    .iter()
                    .all(|(y, x)| *x > 0 && !tower.contains(&(*y, *x - 1)));
                let can_jet_right = rock
                    .iter()
                    .all(|(y, x)| *x < 6 && !tower.contains(&(*y, *x + 1)));

                if jet == 60 && can_jet_left {
                    for pos in rock.iter_mut() {
                        pos.1 -= 1;
                    }
                } else if jet == 62 && can_jet_right {
                    for pos in rock.iter_mut() {
                        pos.1 += 1;
                    }
                } else {
                }

                jet_idx += 1;
                jet_idx %= jets_len;

                // fall / rest movement
                let can_fall = rock
                    .iter()
                    .all(|(y, x)| *y > 1 && !tower.contains(&(*y - 1, *x)));
                if !can_fall {
                    break;
                }

                for pos in rock.iter_mut() {
                    pos.0 -= 1;
                }
            }

            // updating tower
            for pos in rock.into_iter() {
                floor[pos.1 as usize] = floor[pos.1 as usize].max(pos.0);
                tower.insert(pos);
            }

            piece_idx += 1;
            piece_idx %= Self::PIECES;

            round += 1;
        }

        floor.into_iter().max().unwrap() + bonus.unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part_1() {
        let mut day = Day17::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 3068);
    }

    #[test]
    fn part_2() {
        let mut day = Day17::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 1514285714288);
    }
}
