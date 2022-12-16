//! # Day 16 FIXME: add description
//!
//!
//! - a)
//! - b)
//!

use std::collections::VecDeque;

use aoc_runner::Day;
use im::HashSet as ImutableSet;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
struct Valve {
    name: String,
    flow: usize,
    tunnels: Vec<String>,
}

type Valves = HashMap<String, Valve>;

#[derive(Default)]
pub struct Day16(Valves, Distances);

impl Day for Day16 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .lines()
            .map(|line| {
                let mut words = line.split(' ');
                let name = words.nth(1).unwrap().to_owned();
                let flow = words
                    .nth(2)
                    .and_then(|part| part.split_once('='))
                    .and_then(|(_, num)| num.strip_suffix(';'))
                    .and_then(|num| num.parse::<usize>().ok())
                    .unwrap();
                let tunnels = words
                    .skip(4)
                    .map(|name| name.strip_suffix(',').unwrap_or(name).to_owned())
                    .collect();
                let valve = Valve {
                    name: name.clone(),
                    flow,
                    tunnels,
                };

                (name, valve)
            })
            .collect();

        self.1 = shortest_distances(&self.0);
    }

    // FIXME: use DFS as it seems to be faster for n = 30
    fn part1(&mut self) -> Self::Result1 {
        const TIME: usize = 30;
        let non_null_valves: Valves = self
            .0
            .clone()
            .into_iter()
            .filter(|(_, valve)| valve.flow > 0)
            .collect();
        let total = knapsack(
            "AA".to_string(),
            &non_null_valves,
            ImutableSet::default(),
            &self.1,
            TIME,
            &mut Default::default(),
        );

        total
    }

    fn part2(&mut self) -> Self::Result2 {
        const TIME: usize = 26;
        let non_null_valves: Valves = self
            .0
            .clone()
            .into_iter()
            .filter(|(_, valve)| valve.flow > 0)
            .collect();
        let (total1, path1) = knapsack_with_path(
            "AA".to_string(),
            &non_null_valves,
            ImutableSet::default(),
            &self.1,
            TIME,
            &mut Default::default(),
        );

        let non_null_valves: Valves = non_null_valves
            .into_iter()
            .filter(|(name, _)| !path1.contains(name))
            .collect();
        let total2 = knapsack(
            "AA".to_string(),
            &non_null_valves,
            ImutableSet::default(),
            &self.1,
            TIME,
            &mut Default::default(),
        );

        /*
           I don't really understand why this works yet...
        */
        total1 + total2
    }
}

fn knapsack(
    pos: String,
    valves: &Valves,
    choosen: ImutableSet<String>,
    distances: &Distances,
    time: usize,
    cache: &mut HashMap<(String, usize, ImutableSet<String>), usize>,
) -> usize {
    if let Some(result) = cache.get(&(pos.clone(), time, choosen.clone())) {
        return *result;
    }

    let result = valves
        .clone()
        .iter()
        .filter(|(name, _)| !choosen.contains(*name))
        .map(|(name, valve)| {
            let distance = distances.get(&pos).unwrap().get(name).expect("Foo");
            (name, valve, *distance)
        })
        .filter(|(_, _, distance)| distance + 1 < time)
        .map(|(name, valve, distance)| {
            let choosen = {
                let mut copy = choosen.clone();
                copy.insert(name.clone());
                copy
            };
            let rest_total = knapsack(
                name.clone(),
                valves,
                choosen,
                distances,
                time - distance - 1,
                cache,
            );
            let reward = valve.flow * (time - distance - 1);
            rest_total + reward
        })
        .max()
        .unwrap_or(0);

    cache.insert((pos, time, choosen), result.clone());

    result
}

fn knapsack_with_path(
    pos: String,
    valves: &Valves,
    choosen: ImutableSet<String>,
    distances: &Distances,
    time: usize,
    cache: &mut HashMap<(String, usize, ImutableSet<String>), (usize, Vec<String>)>,
) -> (usize, Vec<String>) {
    if let Some(result) = cache.get(&(pos.clone(), time, choosen.clone())) {
        return result.clone();
    }

    let result = valves
        .clone()
        .iter()
        .filter(|(name, _)| !choosen.contains(*name))
        .map(|(name, valve)| {
            let distance = distances.get(&pos).unwrap().get(name).expect("Foo");
            (name, valve, *distance)
        })
        .filter(|(_, _, distance)| distance + 1 < time)
        .map(|(name, valve, distance)| {
            let choosen = {
                let mut copy = choosen.clone();
                copy.insert(name.clone());
                copy
            };
            let (rest_total, mut rest_path) = knapsack_with_path(
                name.clone(),
                valves,
                choosen,
                distances,
                time - distance - 1,
                cache,
            );
            rest_path.push(name.clone());
            let reward = valve.flow * (time - distance - 1);
            let total = rest_total + reward;

            (total, rest_path)
        })
        .max_by_key(|(total, _)| *total)
        .unwrap_or((0, vec![]));

    cache.insert((pos, time, choosen), result.clone());

    result
}

fn shortest_distance(valves: &HashMap<String, Valve>, from: &str, to: &str) -> Option<usize> {
    let mut queue: VecDeque<(usize, &str)> = Default::default();
    queue.push_front((0, from));

    let mut visited: HashSet<&str> = Default::default();

    while let Some((distance, pos)) = queue.pop_front() {
        if pos == to {
            return Some(distance);
        }

        for tunnel in valves.get(pos).unwrap().tunnels.as_slice() {
            if !visited.contains(tunnel.as_str()) {
                queue.push_back((distance + 1, tunnel.as_str()));
            }
        }

        visited.insert(pos);
    }
    None
}

type Distances = HashMap<String, HashMap<String, usize>>;

fn shortest_distances(valves: &HashMap<String, Valve>) -> Distances {
    let mut distances: Distances = Default::default();

    for (idx, from) in valves.keys().enumerate() {
        for to in valves.keys().skip(idx + 1) {
            if from == "AA"
                || to == "AA"
                || (valves.get(from).unwrap().flow > 0 && valves.get(to).unwrap().flow > 0)
            {
                if let Some(d) = shortest_distance(valves, from.as_str(), to.as_str()) {
                    distances
                        .entry(from.clone())
                        .or_default()
                        .insert(to.clone(), d);

                    distances
                        .entry(to.clone())
                        .or_default()
                        .insert(from.clone(), d);
                }
            }
        }
    }

    distances
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part_1() {
        let mut day = Day16::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 1651);
    }

    #[test]
    fn part_2() {
        let mut day = Day16::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 1707);
    }
}
