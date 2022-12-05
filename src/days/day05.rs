//! Day 05 Supply Stacks
//!
//! Parse stacks of chars and move command and
//! a) perform the moves element wise
//! b) moving multiple elements at once
//! between stack. Then return the chars at the top of the stacks.

use std::collections::VecDeque;

use crate::common::day::Day;

type Tower = VecDeque<char>;
type Towers = Vec<Tower>;
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Default)]
pub struct Day05 {
    towers: Towers,
    moves: Vec<Move>,
}

impl Day for Day05 {
    type Result1 = String;
    type Result2 = String;

    fn parse(&mut self, input: &str) {
        fn parse_towers(input: &str) -> Towers {
            let mut towers: Towers = Default::default();
            for line in input.lines() {
                let chars = line.chars().collect::<Vec<char>>();
                for (idx, chunk) in chars.chunks(4).enumerate() {
                    if towers.get(idx).is_none() {
                        towers.push(Default::default());
                    }
                    if chunk[0] == '[' {
                        towers[idx].push_back(chunk[1])
                    }
                }
            }
            towers
        }

        fn parse_moves(input: &str) -> Vec<Move> {
            input.lines().map(parse_move).collect()
        }

        fn parse_move(input: &str) -> Move {
            let words = input.split(' ').collect::<Vec<&str>>();
            Move {
                count: words[1].parse().unwrap(),
                from: words[3].parse::<usize>().unwrap() - 1,
                to: words[5].parse::<usize>().unwrap() - 1,
            }
        }

        let (towers, moves) = {
            let mut parts = input.split("\n\n");
            let (towers, moves) = (parts.next().unwrap(), parts.next().unwrap());
            (parse_towers(towers), parse_moves(moves))
        };

        self.towers = towers;
        self.moves = moves;
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut towers = self.towers.clone();
        for m in self.moves.iter() {
            for _ in 0..m.count {
                let el = towers[m.from].pop_front().unwrap();
                towers[m.to].push_front(el);
            }
        }

        towers.iter().map(|tower| tower[0]).collect::<String>()
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut towers = self.towers.clone();
        for m in self.moves.iter() {
            let mut tmp = vec![];
            for _ in 0..m.count {
                tmp.push(towers[m.from].pop_front().unwrap());
            }
            tmp.reverse();
            for el in tmp {
                towers[m.to].push_front(el);
            }
        }

        towers.iter().map(|tower| tower[0]).collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn part_1() {
        let mut day = Day05::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), String::from("CMZ"));
    }

    #[test]
    fn part_2() {
        let mut day = Day05::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), String::from("MCD"));
    }
}
