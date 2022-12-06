//! # Day 06 Tuning Trouble
//!
//! Given a list of chars find the position where for the first time
//!
//! - a) the prior 4
//! - b) the prior 14
//!
//! chars are all different

use aoc_runner::Day;

#[derive(Default)]
pub struct Day06(String);

impl Day06 {
    fn all_different(slice: &[u8]) -> bool {
        for i in 0..slice.len() - 1 {
            for j in i + 1..slice.len() {
                if slice[i] == slice[j] {
                    return false;
                }
            }
        }

        true
    }

    fn find_marker(&self, marker_length: usize) -> usize {
        let bytes = self.0.as_bytes();
        bytes
            .windows(marker_length)
            .enumerate()
            .find_map(|(idx, window)| {
                if Self::all_different(window) {
                    Some(idx + marker_length)
                } else {
                    None
                }
            })
            .unwrap()
    }
}

impl Day for Day06 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.0 = input.to_owned();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.find_marker(4)
    }

    fn part2(&mut self) -> Self::Result2 {
        self.find_marker(14)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    const INPUT2: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    #[test]
    fn part_1() {
        let mut day = Day06::default();
        day.parse(INPUT1);
        assert_eq!(day.part1(), 11);
        day.parse(INPUT2);
        assert_eq!(day.part1(), 10);
    }

    #[test]
    fn part_2() {
        let mut day = Day06::default();
        day.parse(INPUT1);
        assert_eq!(day.part2(), 26);
        day.parse(INPUT2);
        assert_eq!(day.part2(), 29);
    }
}
