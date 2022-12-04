//! Day 03 Rucksack Reorganization
//!
//! Find chars contained in
//! a) both halfs of every line
//! b) every triplet of lines
//! and sum them up

use std::collections::BTreeSet;

use crate::common::day::Day;

#[derive(Default, Clone)]
pub struct Day03(Vec<Vec<u8>>);

// TODO: use u64 bitset instead of BTreeSet
impl Day for Day03 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        fn char_to_priority(c: char) -> u8 {
            let i: u32 = c.into();
            (match c {
                'a'..='z' => i - 96, // map 97 to 1
                'A'..='Z' => i - 38, // map 65 to 27
                _ => panic!("Got bad char '{}'", c),
            }) as u8
        }

        self.0 = input
            .lines()
            .map(|line| line.chars().into_iter().map(char_to_priority).collect())
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        fn find_duplicate(v: &Vec<u8>) -> u32 {
            let (l, r) = v.split_at(v.len() / 2);
            let l = l.iter().cloned().collect::<BTreeSet<u8>>();
            let r = r.iter().cloned().collect::<BTreeSet<u8>>();
            *l.intersection(&r).min().unwrap() as u32
        }

        self.0.iter().map(find_duplicate).sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.0
            .chunks(3)
            .map(|chunk| {
                *(chunk
                    .iter()
                    .map(|items| items.iter().cloned().collect::<BTreeSet<u8>>())
                    .reduce(|a, b| a.intersection(&b).into_iter().cloned().collect())
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap()) as u32
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn part_1() {
        let mut day = Day03::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 157);
    }

    #[test]
    fn part_2() {
        let mut day = Day03::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 70);
    }
}
