//! # Day 25 Full of Hot Air
//!
//! Sum up a list of numbers which are in base 5 and only use the digis 0,1,2,-1,-2

use std::fmt::Display;

use aoc_runner::Day;

#[derive(Clone)]
pub struct Snafu(Vec<i8>);

impl Snafu {
    fn from_str(s: &str) -> Self {
        let values = s
            .chars()
            .map(|c| match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!("Read non-SNAFU character {}", c),
            })
            .rev()
            .collect();

        Self(values)
    }

    fn add(a: Self, b: Self) -> Self {
        let mut values: Vec<i8> = Default::default();
        let mut carry = 0;
        for i in 0..a.0.len().max(b.0.len()) {
            let value = a.0.get(i).unwrap_or(&0) + b.0.get(i).unwrap_or(&0) + carry;
            match value {
                3 => {
                    carry = 1;
                    values.push(-2);
                }
                4 => {
                    carry = 1;
                    values.push(-1);
                }
                5 => {
                    carry = 1;
                    values.push(0);
                }
                -3 => {
                    carry = -1;
                    values.push(2);
                }
                -4 => {
                    carry = -1;
                    values.push(1);
                }
                -5 => {
                    carry = -1;
                    values.push(0);
                }
                _ => {
                    values.push(value);
                    carry = 0;
                }
            };
        }

        if carry != 0 {
            values.push(carry);
        }

        Self(values)
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .0
            .iter()
            .rev()
            .map(|num| match num {
                0 => '0',
                1 => '1',
                2 => '2',
                -1 => '-',
                -2 => '=',
                _ => panic!("Illegal Snafu digit {}", num),
            })
            .collect();
        f.write_fmt(format_args!("{}", s))?;
        Ok(())
    }
}

impl std::fmt::Debug for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

#[derive(Default, Clone)]
pub struct Day25(Vec<Snafu>);

impl Day for Day25 {
    type Result1 = Snafu;
    type Result2 = String;

    fn parse(&mut self, input: &str) {
        self.0 = input.lines().map(Snafu::from_str).collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.0.iter().cloned().reduce(Snafu::add).unwrap()
    }

    fn part2(&mut self) -> Self::Result2 {
        "".to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn part_1() {
        let mut day = Day25::default();
        day.parse(INPUT);
        assert_eq!(day.part1().to_string(), "2=-1=0");
    }
}
