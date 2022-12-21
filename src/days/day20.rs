//! # Day 20 Grove Positioning System
//!
//! Given a list of numbers 
//!
//! - a) mix them one time
//! - b) multiply them by a magic number and mix them ten times 
//!
//! calculate a sum of three special values.

use aoc_runner::Day;

type Number = i64;

#[derive(Default, Clone)]
pub struct Day20(Vec<Number>, usize);

impl Day for Day20 {
    type Result1 = i128;
    type Result2 = i128;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .lines()
            .enumerate()
            .map(|(idx, part)| {
                let num = part.parse().unwrap();
                if num == 0 {
                    self.1 = idx;
                }
                num
            })
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        let (idx_map, reverse_idx_map) = shuffle(self.0.as_slice(), self.0.len(), 1, 1);
        let new_zero_idx = idx_map[self.1];
        let total: Number = [1000, 2000, 3000]
            .into_iter()
            .map(|i| self.0[reverse_idx_map[(new_zero_idx + i) % self.0.len()]])
            .sum();
        total.into()
    }

    fn part2(&mut self) -> Self::Result2 {
        const DECRYPTION_KEY: u32 = 811589153;

        let (idx_map, reverse_idx_map) = shuffle(
            self.0.as_slice(),
            self.0.len(),
            DECRYPTION_KEY % (self.0.len() as u32 - 1),
            10,
        );
        let new_zero_idx = idx_map[self.1];
        let total: Number = [1000, 2000, 3000]
            .into_iter()
            .map(|i| self.0[reverse_idx_map[(new_zero_idx + i) % self.0.len()]])
            .sum();
        let total: Self::Result2 = total.into();
        DECRYPTION_KEY as Self::Result2 * total
    }
}

type IndexMap = [usize];
fn shuffle(nums: &[Number], len: usize, factor: u32, rounds: usize) -> (Vec<usize>, Vec<usize>) {
    #[inline(always)]
    fn move_num(from: usize, to: usize, idx_map: &mut IndexMap, idx_rev_map: &mut IndexMap) {
        if from == to {
            return;
        }

        let (start, end, step) = if from < to {
            (from, to, 1isize)
        } else {
            (from - 1, to, -1isize)
        };

        let mut i = start;
        while i != end {
            let orig_idx_i = idx_rev_map[i];
            idx_map[orig_idx_i] += 1;
            let orig_idx_i_succ = idx_rev_map[(i + 1)];
            idx_map[orig_idx_i_succ] -= 1;
            idx_rev_map[i] = orig_idx_i_succ;
            idx_rev_map[i + 1] = orig_idx_i;

            i = (i as isize + step) as usize;
        }
    }

    // Where the number w/ index `i` in the original list is found currently in `nums`
    let mut idx_to_idx: Vec<usize> = (0..len).into_iter().collect();

    // The original index of the number curently at index `i`
    let mut idx_to_idx_reverse: Vec<usize> = (0..len).into_iter().collect();

    for _ in 0..rounds {
        for idx in 0..len {
            let num_idx = idx_to_idx[idx];
            let num = nums[idx] * factor as Number;
            let new_idx = (num_idx + (num.rem_euclid(len as Number - 1) as usize)) % len;
            move_num(
                num_idx,
                new_idx,
                idx_to_idx.as_mut_slice(),
                idx_to_idx_reverse.as_mut_slice(),
            );
        }
    }

    (idx_to_idx, idx_to_idx_reverse)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part_1() {
        let mut day = Day20::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 3);
    }

    #[test]
    fn part_2() {
        let mut day = Day20::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 1623178306);
    }
}
