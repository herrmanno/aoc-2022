//! # Day 16 Proboscidea Volcanium
//!
//! Given a graph of connected rooms where every room hosts a valve that has a given flow rate
//! find the optimal route between routes such that the release pressure by every open valve
//! is miximal. (Every opened valve releases pressure corresponding to its flow rate *for every
//! remaning time slot*).
//!
//! - a) with a time limit of 30 minutes
//! - b) with a time limit of 26 minutes but with two people
//!

use std::collections::VecDeque;

use aoc_runner::Day;
use rustc_hash::FxHashMap as HashMap;
use std::cmp::Reverse;
use std::collections::BTreeMap;

type ValveID = u64;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
struct Valve {
    id: ValveID,
    flow: usize,
    tunnels: Vec<ValveID>,
}

type Valves = HashMap<ValveID, Valve>;

#[derive(Default)]
pub struct Day16(Valves, Distances);

impl Day for Day16 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        let mut lines: Vec<&str> = input.lines().collect();
        // sort so that valve "AA" always gets ID 1 << 0 == 1
        lines.sort_unstable();

        assert!(
            lines.len() < 64,
            "Can not handle graph with more than 63 entries for now"
        );

        let name_id_map: HashMap<String, ValveID> = lines
            .iter()
            .enumerate()
            .map(|(idx, line)| (line.split(' ').nth(1).unwrap().to_string(), 1 << idx))
            .collect();

        let valves: Valves = lines
            .iter()
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
                    .map(|name| *name_id_map.get(&name).unwrap())
                    .collect();
                let valve = Valve {
                    id: *name_id_map.get(&name).unwrap(),
                    flow,
                    tunnels,
                };

                (valve.id, valve)
            })
            .collect();

        self.1 = shortest_distances(&valves);

        self.0 = valves
            .clone()
            .into_iter()
            .filter(|(_, valve)| valve.flow > 0)
            .map(|(id, mut valve)| {
                valve.tunnels.retain(|name| valves[name].flow > 0);
                (id, valve)
            })
            .collect();

        self.0 = valves;
    }

    fn part1(&mut self) -> Self::Result1 {
        const TIME: usize = 30;
        dfs_max(&self.0, &self.1, TIME)
    }

    fn part2(&mut self) -> Self::Result2 {
        const TIME: usize = 26;
        let results = dfs_with_paths(&self.0, &self.1, TIME);

        let results: BTreeMap<Reverse<usize>, Path> = results
            .into_iter()
            .map(|(path, score)| (Reverse(score), path))
            .collect();

        let Reverse(max_score) = *results.iter().next().unwrap().0;

        let mut best_score: usize = 0;
        for (idx, (Reverse(score_1), path_1)) in results.iter().enumerate() {
            'inner: for (Reverse(score_2), path_2) in results.iter().skip(idx + 1) {
                if score_1 + score_2 < max_score {
                    break 'inner;
                }
                if *path_1 & *path_2 == 0 {
                    best_score = best_score.max(score_1 + score_2);
                }
            }
        }

        best_score
    }
}

/// find score of best path (by score) that is possible in `max_time`
fn dfs_max(valves: &Valves, distances: &Distances, max_time: usize) -> usize {
    let mut best_score = 0;

    let max_flow: usize = valves.values().map(|valve| valve.flow).sum();

    type State = (ValveID, Path, usize, usize);
    let mut queue: Vec<State> = Vec::from([(1, 0, 0, 0)]);
    while let Some((pos, path, score, time)) = queue.pop() {
        best_score = best_score.max(score);

        if score + (max_time - time).saturating_sub(2) * max_flow < best_score {
            continue;
        }

        let targets = &distances[&pos];
        for (tunnel, distance) in targets {
            if *distance < max_time - time && (path & tunnel == 0) {
                if let Some(flow) = valves.get(tunnel).map(|valve| valve.flow) {
                    let new_pos = *tunnel;
                    let new_path = path | new_pos;
                    let new_score = score + (flow * (max_time - time - distance - 1));
                    let new_time = time + (distance + 1);
                    queue.push((new_pos, new_path, new_score, new_time));
                }
            }
        }
    }

    best_score
}

type Path = ValveID;

/// find all paths (with scores) that are possible in `max_time`
fn dfs_with_paths(valves: &Valves, distances: &Distances, max_time: usize) -> HashMap<Path, usize> {
    let mut results: HashMap<Path, usize> = Default::default();

    let max_flow: usize = valves.values().map(|valve| valve.flow).sum();

    type State = (ValveID, Path, usize, usize);
    let mut queue: Vec<State> = Vec::from([(1, 0, 0, 0)]);
    while let Some((pos, path, score, time)) = queue.pop() {
        let path_best_score = *results.get(&path).unwrap_or(&0);
        if score > path_best_score {
            results.insert(path, score);
        }

        if score + (max_time - time).saturating_sub(2) * max_flow < path_best_score {
            continue;
        }

        let targets = &distances[&pos];
        for (tunnel, distance) in targets {
            if *distance < max_time - time && (path & tunnel == 0) {
                if let Some(flow) = valves.get(tunnel).map(|valve| valve.flow) {
                    let new_pos = *tunnel;
                    let new_path = path | new_pos;
                    let new_score = score + (flow * (max_time - time - distance - 1));
                    let new_time = time + (distance + 1);
                    queue.push((new_pos, new_path, new_score, new_time));
                }
            }
        }
    }

    results
}

fn shortest_distance(valves: &Valves, from: ValveID, to: ValveID) -> Option<usize> {
    let mut queue: VecDeque<(usize, ValveID)> = Default::default();
    queue.push_front((0, from));

    let mut visited: Path = 0;

    while let Some((distance, pos)) = queue.pop_front() {
        if pos == to {
            return Some(distance);
        }

        for tunnel in valves.get(&pos).unwrap().tunnels.as_slice() {
            if visited & tunnel == 0 {
                queue.push_back((distance + 1, *tunnel));
            }
        }

        visited |= pos;
    }
    None
}

type Distances = HashMap<ValveID, HashMap<ValveID, usize>>;

fn shortest_distances(valves: &Valves) -> Distances {
    let mut distances: Distances = Default::default();

    for (idx, from) in valves.keys().enumerate() {
        for to in valves.keys().skip(idx + 1) {
            if (valves.get(from).unwrap().flow > 0 && valves.get(to).unwrap().flow > 0)
                || (*from == 1 && valves.get(to).unwrap().flow > 0)
                || (*to == 1 && valves.get(from).unwrap().flow > 0)
            {
                if let Some(d) = shortest_distance(valves, *from, *to) {
                    distances.entry(*from).or_default().insert(*to, d);

                    distances.entry(*to).or_default().insert(*from, d);
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
