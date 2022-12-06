//! Day 04 Camp Cleanup
//!
//! Given two ranges per line check for how many pairs
//! - a) one pair is fully contained inside the other
//! - b) the paris overlap

use aoc_runner::Day;

type Range = (u32, u32);

#[derive(Default, Clone)]
pub struct Day04(Vec<(Range, Range)>);

impl Day for Day04 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        fn to_range(s: &str) -> Range {
            let mut iter = s.split('-').map(|s| s.parse().expect("Bad number"));
            (iter.next().unwrap(), iter.next().unwrap())
        }

        fn to_ranges(s: &str) -> (Range, Range) {
            let mut iter = s.split(',').map(to_range);
            (iter.next().unwrap(), iter.next().unwrap())
        }

        self.0 = input.lines().map(to_ranges).collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        fn ranges_overlap_fully(ranges: &&(Range, Range)) -> bool {
            let ((a, b), (c, d)) = ranges;
            a <= c && d <= b || c <= a && b <= d
        }
        self.0.iter().filter(ranges_overlap_fully).count()
    }

    fn part2(&mut self) -> Self::Result2 {
        fn ranges_overlap(ranges: &&(Range, Range)) -> bool {
            let ((a, b), (c, d)) = ranges;
            a <= c && c <= b || c <= a && a <= d
        }
        self.0.iter().filter(ranges_overlap).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1() {
        let mut day = Day04::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 2);
    }

    #[test]
    fn part_2() {
        let mut day = Day04::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 4);
    }
}
