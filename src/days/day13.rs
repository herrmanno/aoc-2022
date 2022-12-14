//! # Day 13 Distress Signal
//!
//! Given a list of packets, where every packet is a list of packets or a number
//!
//! - a) find tuples of packets that are in correct order (first tuple element is smaller than second)
//! - b) insert to 'divider packages' and find indices of dividers after sorting all packets
//!

use std::{
    cmp::Ordering,
    iter::Peekable,
    str::{Chars, FromStr},
};

use aoc_runner::Day;

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

#[cfg(debug_assertions)]
impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Self::Num(n) => f.write_fmt(format_args!("{}", n)),
            Self::List(l) => {
                f.write_char('[')?;
                let len = l.len();
                for (idx, el) in l.iter().enumerate() {
                    el.fmt(f)?;
                    if idx + 1 < len {
                        f.write_char(',')?;
                    }
                }
                f.write_char(']')
            }
        }
    }
}

impl PartialOrd for Packet {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Num(a), Packet::Num(b)) => a.cmp(b),
            (a @ Packet::Num(_), b @ Packet::List(_)) => Packet::List(vec![a.clone()]).cmp(b),
            (a @ Packet::List(_), b @ Packet::Num(_)) => a.cmp(&Packet::List(vec![b.clone()])),
            (Packet::List(a), Packet::List(b)) => {
                let len = a.len().min(b.len());
                for i in 0..len {
                    let result = a[i].cmp(&b[i]);
                    if result != Ordering::Equal {
                        return result;
                    }
                }

                a.len().cmp(&b.len())
            }
        }
    }
}

impl Packet {
    fn parse(stream: &mut Peekable<Chars>) -> Self {
        if stream.peek() == Some(&'[') {
            Self::parse_list(stream)
        } else {
            Self::parse_num(stream)
        }
    }

    fn parse_list(stream: &mut Peekable<Chars>) -> Self {
        assert_eq!(stream.next(), Some('['));
        let mut vec = vec![];
        loop {
            if stream.peek() == Some(&']') {
                stream.next();
                break;
            }

            let el = Self::parse(stream);
            vec.push(el);

            if stream.peek() == Some(&',') {
                stream.next();
            }
        }

        Packet::List(vec)
    }

    fn parse_num(stream: &mut Peekable<Chars>) -> Self {
        let mut num = 0u8;
        while stream.peek().unwrap().is_numeric() {
            num *= 10;
            num += stream.next().unwrap().to_digit(10).unwrap() as u8;
        }

        Packet::Num(num)
    }
}

impl FromStr for Packet {
    type Err = ();

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Packet::parse(&mut s.chars().peekable()))
    }
}

#[derive(Default)]
pub struct Day13(Vec<(Packet, Packet)>);

impl Day for Day13 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .split("\n\n")
            .filter_map(|parts| parts.split_once('\n'))
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(idx, (a, b))| if a < b { Some(idx + 1) } else { None })
            .sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        let divider_packet_1 = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
        let divider_packet_2 = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);

        // index of divider_package_1 start at 1 b/c we have 1-based indices
        let mut idx_1 = 1;
        // index of divider_package_2 start at 2 b/c we have 1-based indices and it is also
        // greater than divider_package_1
        let mut idx_2 = 2;

        for packet in self.0.iter().flat_map(|(a, b)| [a, b].into_iter()) {
            if &divider_packet_1 > packet {
                idx_1 += 1;
            }
            if &divider_packet_2 > packet {
                idx_2 += 1;
            }
        }

        idx_1 * idx_2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part_1() {
        let mut day = Day13::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 13);
    }

    #[test]
    fn part_2() {
        let mut day = Day13::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 140);
    }
}
