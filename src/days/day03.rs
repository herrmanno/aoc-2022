//! # Day 03 Rucksack Reorganization
//!
//! Find chars contained in
//!
//! - a) both halfs of every line
//! - b) every triplet of lines
//!
//! and sum them up

use aoc_runner::Day;

#[derive(Default, Clone)]
pub struct Day03(Vec<Vec<u8>>);

fn find_duplicate<T: AsRef<[U]>, U: AsRef<[u8]>>(slices: T) -> u8 {
    let len = slices.as_ref().len() as u8;
    let mut total = [0u8; 256];
    for slice in slices.as_ref() {
        let mut seen = [false; 256];
        for el in slice.as_ref() {
            let el = *el as usize;
            if seen[el] {
                continue;
            }
            seen[el] = true;
            total[el] += 1;
            if total[el] == len {
                return el as u8;
            }
        }
    }

    panic!("No duplicate element found");
}

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
        self.0
            .iter()
            .map(|v| {
                let (l, r) = v.split_at(v.len() / 2);
                find_duplicate([l, r]) as u32
            })
            .sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.0
            .chunks(3)
            .map(|chunk| find_duplicate(chunk) as u32)
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
