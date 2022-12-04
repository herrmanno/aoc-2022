//! Day 01 Calorie Counting
//!
//! Given a list of grouped number find
//! a) the group with the max sum
//! b) the sum of the three groups with most sums

use crate::common::day::Day;

#[derive(Default, Clone)]
pub struct Day02(Vec<(char, char)>);

impl Day for Day02 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .lines()
            .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        fn score_for_round(xs: &(char, char)) -> u32 {
            match xs {
                ('A', 'X') => 3 + 1,
                ('A', 'Y') => 6 + 2,
                ('A', 'Z') => 0 + 3,
                ('B', 'X') => 0 + 1,
                ('B', 'Y') => 3 + 2,
                ('B', 'Z') => 6 + 3,
                ('C', 'X') => 6 + 1,
                ('C', 'Y') => 0 + 2,
                ('C', 'Z') => 3 + 3,
                _ => unreachable!(),
            }
        }
        self.0.iter().map(score_for_round).sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        fn score_for_round(xs: &(char, char)) -> u32 {
            match xs {
                ('A', 'X') => 0 + 3,
                ('A', 'Y') => 3 + 1,
                ('A', 'Z') => 6 + 2,
                ('B', 'X') => 0 + 1,
                ('B', 'Y') => 3 + 2,
                ('B', 'Z') => 6 + 3,
                ('C', 'X') => 0 + 2,
                ('C', 'Y') => 3 + 3,
                ('C', 'Z') => 6 + 1,
                _ => unreachable!(),
            }
        }
        self.0.iter().map(score_for_round).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn part_1() {
        let mut day = Day02::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 15);
    }

    #[test]
    fn part_2() {
        let mut day = Day02::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 12);
    }
}
