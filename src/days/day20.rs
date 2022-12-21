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
    type Result1 = i64;
    type Result2 = i64;

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
        let nums = shuffle(self.0.as_slice(), self.0.len(), 1, 1);
        let new_zero_idx = nums
            .iter()
            .position(|r| std::ptr::eq(&self.0[self.1], *r))
            .unwrap();
        let total: Number = [1000, 2000, 3000]
            .into_iter()
            .map(|i| *nums[(new_zero_idx + i) % self.0.len()])
            .sum();
        total
    }

    fn part2(&mut self) -> Self::Result2 {
        const DECRYPTION_KEY: u32 = 811589153;

        let nums = shuffle(
            self.0.as_slice(),
            self.0.len(),
            DECRYPTION_KEY % (self.0.len() as u32 - 1),
            10,
        );
        let new_zero_idx = nums
            .iter()
            .position(|r| std::ptr::eq(&self.0[self.1], *r))
            .unwrap();
        let total: Number = [1000, 2000, 3000]
            .into_iter()
            .map(|i| *nums[(new_zero_idx + i) % self.0.len()])
            .sum();
        DECRYPTION_KEY as Self::Result2 * total
    }
}

fn shuffle(nums: &[Number], len: usize, factor: u32, rounds: usize) -> Vec<&Number> {
    let mut ref_vec: Vec<_> = nums.iter().collect();

    for _ in 0..rounds {
        for num in nums.iter() {
            let idx = ref_vec.iter().position(|r| std::ptr::eq(num, *r)).unwrap();
            let num = *num * factor as Number;
            let new_idx = (idx + (num.rem_euclid(len as Number - 1) as usize)) % len;

            if idx == new_idx {
                continue;
            }

            if new_idx > idx {
                ref_vec[idx..=new_idx].rotate_left(1);
            } else {
                ref_vec[new_idx + 1..=idx].rotate_right(1);
            }
        }
    }

    ref_vec
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
