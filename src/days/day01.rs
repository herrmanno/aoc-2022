//! # Day 01 Calorie Counting
//!
//! Given a list of grouped number find
//!
//! - a) the group with the max sum
//! - b) the sum of the three groups with most sums

use aoc_runner::Day;

#[derive(Default, Clone)]
pub struct Day01(Vec<Vec<u32>>);

impl Day for Day01 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .split("\n\n")
            .map(|s| s.lines().map(str::parse).map(Result::unwrap).collect())
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.0.iter().map(|v| v.iter().sum::<u32>()).max().unwrap()
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut sums = self
            .0
            .iter()
            .map(|v| v.iter().sum::<u32>())
            .collect::<Vec<u32>>();
        sums.sort_by(|a, b| b.cmp(a));
        sums.iter().take(3).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn part_1() {
        let mut day = Day01::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 24000);
    }

    #[test]
    fn part_2() {
        let mut day = Day01::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 45000);
    }
}
