//! # Day 11 Monkey in the middle
//!
//! Given a list of monkeys, containing of
//! - items (integers) they currently have
//! - a operation the modifies an item
//! - a integer to test (by dividing modulo) a modified item against
//! - two indicies of other monkeys to pass the item to, depending on the test's result
//!
//! simulate the monkeys throwing the items around for
//!
//! - a) 20 rounds (with an addition 'divide by 3' operation)
//! - b) 10000 rounds
//!
//! and find out how often the two monkeys that have thrown the most items have thrown in total
//!

use std::collections::VecDeque;

use aoc_runner::Day;

#[derive(Default)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    lcm: usize,
}

impl Monkeys {
    fn new(monkeys: Vec<Monkey>) -> Self {
        // lcm = ∏ divisors,  because all divisor are prime (in real input and test input)
        let lcm = monkeys.iter().map(|m| m.test.divisor).product();
        Self { monkeys, lcm }
    }
}

/// A babbling, bumbling band of baboons
impl Monkeys {
    /// Simulate all monkeys throwing their items around for one round

    #[inline(always)]
    fn do_round_with_relief(&mut self) {
        for idx in 0..self.monkeys.len() {
            while let Some(mut item) = self.monkeys[idx].items.pop_front() {
                self.monkeys[idx].throw_count += 1;
                item = self.monkeys[idx].operation.apply(item);

                item /= 3;

                let receiver_monkey = if item % self.monkeys[idx].test.divisor == 0 {
                    self.monkeys[idx].test.monkey_true
                } else {
                    self.monkeys[idx].test.monkey_false
                };

                self.monkeys[receiver_monkey].items.push_back(item);
            }
        }
    }

    #[inline(always)]
    fn do_round_without_relief(&mut self) {
        for idx in 0..self.monkeys.len() {
            while let Some(mut item) = self.monkeys[idx].items.pop_front() {
                self.monkeys[idx].throw_count += 1;
                item = self.monkeys[idx].operation.apply(item);

                item %= self.lcm;

                let receiver_monkey = if item % self.monkeys[idx].test.divisor == 0 {
                    self.monkeys[idx].test.monkey_true
                } else {
                    self.monkeys[idx].test.monkey_false
                };

                self.monkeys[receiver_monkey].items.push_back(item);
            }
        }
    }
}

struct Monkey {
    throw_count: usize,
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
}

enum Operation {
    Add(usize),
    Multiply(usize),
    Double,
    Square,
}

impl Operation {
    fn apply(&self, i: usize) -> usize {
        match self {
            Operation::Add(x) => i + x,
            Operation::Multiply(x) => i * x,
            Operation::Double => i + i,
            Operation::Square => i * i,
        }
    }
}

struct Test {
    divisor: usize,
    monkey_true: usize,
    monkey_false: usize,
}

#[derive(Default)]
pub struct Day11(Monkeys);

impl Day for Day11 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        let monkeys: Vec<Monkey> = input
            .split("\n\n")
            .into_iter()
            .map(|lines| {
                let mut lines = lines.split('\n').map(str::trim);
                let (items_line, operation_line, divisible_line, true_line, false_line) = {
                    lines.next();
                    (
                        lines.next().unwrap(),
                        lines.next().unwrap(),
                        lines.next().unwrap(),
                        lines.next().unwrap(),
                        lines.next().unwrap(),
                    )
                };

                let items: VecDeque<usize> = items_line
                    .strip_prefix("Starting items: ")
                    .map(|part| part.split(", ").map(|num| num.parse().unwrap()))
                    .unwrap()
                    .collect();
                let operation = operation_line
                    .strip_prefix("Operation: new = old ")
                    .and_then(|rest| rest.split_once(' '))
                    .map(|(op, num)| match (op, num) {
                        ("+", "old") => Operation::Double,
                        ("*", "old") => Operation::Square,
                        ("+", _) => Operation::Add(
                            num.parse().unwrap_or_else(|_| panic!("Bad num: {}", num)),
                        ),
                        ("*", _) => Operation::Multiply(
                            num.parse().unwrap_or_else(|_| panic!("Bad num {}", num)),
                        ),
                        _ => panic!("Unknown operation {}", op),
                    })
                    .unwrap();
                let test = {
                    let divisor = divisible_line
                        .split(' ')
                        .last()
                        .and_then(|num| num.parse().ok())
                        .unwrap();
                    let monkey_true = true_line
                        .split(' ')
                        .last()
                        .and_then(|num| num.parse().ok())
                        .unwrap();
                    let monkey_false = false_line
                        .split(' ')
                        .last()
                        .and_then(|num| num.parse().ok())
                        .unwrap();
                    Test {
                        divisor,
                        monkey_true,
                        monkey_false,
                    }
                };

                Monkey {
                    throw_count: 0,
                    items,
                    operation,
                    test,
                }
            })
            .collect();

        self.0 = Monkeys::new(monkeys);
    }

    fn part1(&mut self) -> Self::Result1 {
        for _ in 0..20 {
            self.0.do_round_with_relief();
        }
        let throw_counts = {
            let mut counts: Vec<usize> = self.0.monkeys.iter().map(|m| m.throw_count).collect();
            counts.sort_unstable_by(|a, b| b.cmp(a));
            counts
        };
        throw_counts[0] * throw_counts[1]
    }

    fn part2(&mut self) -> Self::Result2 {
        for _ in 0..10000 {
            self.0.do_round_without_relief();
        }
        let throw_counts = {
            let mut counts: Vec<usize> = self.0.monkeys.iter().map(|m| m.throw_count).collect();
            counts.sort_unstable_by(|a, b| b.cmp(a));
            counts
        };
        throw_counts[0] * throw_counts[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1";

    #[test]
    fn part_1() {
        let mut day = Day11::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 10605);
    }

    #[test]
    fn part_2() {
        let mut day = Day11::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 2713310158);
    }
}
