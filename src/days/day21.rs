//! # Day 21 Monkey Math
//!
//! Given a term represented by an ast
//!
//! - a) Calculate the term's value
//! - b) treat 'root: a x b' as 'a == b' and solve for variable 'humn'
//!

use aoc_runner::Day;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

type Number = i64;

#[derive(Debug, Clone)]
enum Monkey {
    Number(Number),
    Calculation(String, String, char, fn(Number, Number) -> Number),
}

#[derive(Default, Clone)]
pub struct Day21(HashMap<String, Monkey>);

impl Day for Day21 {
    type Result1 = Number;
    type Result2 = Number;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .lines()
            .map(|line| {
                let mut words = line.split(' ');
                let name = words.next().and_then(|w| w.strip_suffix(':')).unwrap();
                let a = words.next().unwrap();
                if a.chars().next().unwrap().is_numeric() {
                    (name.to_string(), Monkey::Number(a.parse().unwrap()))
                } else {
                    let op = words.next().unwrap();
                    let b = words.next().unwrap();
                    let op_name = op.chars().next().unwrap();
                    let op = match op {
                        "+" => std::ops::Add::add,
                        "-" => std::ops::Sub::sub,
                        "*" => std::ops::Mul::mul,
                        "/" => std::ops::Div::div,
                        _ => panic!("Unknown operator: {}", op),
                    };
                    (
                        name.to_string(),
                        Monkey::Calculation(a.to_string(), b.to_string(), op_name, op),
                    )
                }
            })
            .collect()
    }

    fn part1(&mut self) -> Self::Result1 {
        eval(&self.0, "root")
    }

    fn part2(&mut self) -> Self::Result2 {
        let monkeys = &self.0;
        let Monkey::Calculation(a, b, _, _) = &monkeys["root"] else {
            panic!("Root is no calculation");
        };

        let traces = trace(&self.0, "humn");

        let (mut unknown, mut value) = if traces.contains(a) {
            (&self.0[a], eval(monkeys, b))
        } else {
            (&self.0[b], eval(monkeys, a))
        };

        loop {
            match unknown {
                Monkey::Number(_) => break value,
                Monkey::Calculation(lhs, rhs, '+', _) => {
                    if traces.contains(lhs) {
                        value -= eval(monkeys, rhs);
                        unknown = &monkeys[lhs];
                    } else {
                        value -= eval(monkeys, lhs);
                        unknown = &monkeys[rhs];
                    }
                }
                Monkey::Calculation(lhs, rhs, '-', _) => {
                    if traces.contains(lhs) {
                        value += eval(monkeys, rhs);
                        unknown = &monkeys[lhs];
                    } else {
                        value = eval(monkeys, lhs) - value;
                        unknown = &monkeys[rhs];
                    }
                }
                Monkey::Calculation(lhs, rhs, '*', _) => {
                    if traces.contains(lhs) {
                        value /= eval(monkeys, rhs);
                        unknown = &monkeys[lhs];
                    } else {
                        value /= eval(monkeys, lhs);
                        unknown = &monkeys[rhs];
                    }
                }
                Monkey::Calculation(lhs, rhs, '/', _) => {
                    if traces.contains(lhs) {
                        value *= eval(monkeys, rhs);
                        unknown = &monkeys[lhs];
                    } else {
                        value = eval(monkeys, lhs) / value;
                        unknown = &monkeys[rhs];
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn eval(monkeys: &HashMap<String, Monkey>, name: impl AsRef<str>) -> Number {
    match &monkeys[name.as_ref()] {
        Monkey::Number(n) => *n,
        Monkey::Calculation(a, b, _, op) => {
            let a = eval(monkeys, a);
            let b = eval(monkeys, b);
            op(a, b)
        }
    }
}

fn trace(monkeys: &HashMap<String, Monkey>, name: impl AsRef<str>) -> HashSet<String> {
    let mut names = Default::default();
    trace_for(monkeys, "root", name, &mut names);
    names
}

fn trace_for(
    monkeys: &HashMap<String, Monkey>,
    curr: impl AsRef<str>,
    name: impl AsRef<str>,
    names: &mut HashSet<String>,
) -> bool {
    if curr.as_ref() == name.as_ref() {
        names.insert(name.as_ref().to_string());
        true
    } else {
        match &monkeys[curr.as_ref()] {
            Monkey::Number(_) => false,
            Monkey::Calculation(a, b, _, _) => {
                let in_a = trace_for(monkeys, a, name.as_ref(), names);
                let in_b = trace_for(monkeys, b, name.as_ref(), names);

                if in_a || in_b {
                    names.insert(curr.as_ref().to_string());
                }

                in_a || in_b
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn part_1() {
        let mut day = Day21::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 152);
    }

    #[test]
    fn part_2() {
        let mut day = Day21::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 301);
    }
}
