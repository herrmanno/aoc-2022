//! # Day 19
//!
//!
//!
//! - a)
//! - b)
//!

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use rustc_hash::FxHashSet as HashSet;

use aoc_runner::Day;

type Time = u16;
// Using u16 instead of usize seems to get a speedup of ~ 3x
type Resource = u16;
type Cost = u16;

#[derive(Debug, Clone)]
struct Blueprint {
    ore_cost: Cost,
    clay_cost: Cost,
    obsidian_cost: (Cost, Cost),
    geode_cost: (Cost, Cost),
}

impl Blueprint {
    fn max_ore_cost(&self) -> Cost {
        [self.clay_cost, self.obsidian_cost.0, self.geode_cost.0]
            .into_iter()
            .max()
            .unwrap()
    }

    fn max_clay_cost(&self) -> Cost {
        self.obsidian_cost.1
    }

    fn max_obsidian_cost(&self) -> Cost {
        self.geode_cost.1
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BlueprintResult {
    resources: [Resource; 4],
    robots: [Resource; 4],
    robots_building: [Resource; 4],
}

#[derive(Default, Clone)]
pub struct Day19(Vec<Blueprint>);

impl Day for Day19 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .lines()
            .map(|line| {
                let mut words = line.split(' ');
                let ore = words.nth(6).unwrap().parse().unwrap();
                let clay = words.nth(5).unwrap().parse().unwrap();
                let obsidian = (
                    words.nth(5).unwrap().parse().unwrap(),
                    words.nth(2).unwrap().parse().unwrap(),
                );
                let geode = (
                    words.nth(5).unwrap().parse().unwrap(),
                    words.nth(2).unwrap().parse().unwrap(),
                );

                Blueprint {
                    ore_cost: ore,
                    clay_cost: clay,
                    obsidian_cost: obsidian,
                    geode_cost: geode,
                }
            })
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        const TIME: Time = 24;
        let mut total = 0;
        let start = BlueprintResult {
            robots: [1, 0, 0, 0],
            ..Default::default()
        };
        for (idx, blueprint) in self.0.iter().enumerate() {
            let result = search(blueprint, start.clone(), TIME);
            #[cfg(debug_assertions)]
            {
                println!("{:#?}", result);
            }
            total += (idx + 1) * (result.resources[3] as Self::Result1);
        }

        total
    }

    fn part2(&mut self) -> Self::Result2 {
        const TIME: Time = 32;
        let mut total = 1;
        let start = BlueprintResult {
            robots: [1, 0, 0, 0],
            ..Default::default()
        };
        for blueprint in self.0.iter().take(3) {
            let result = search(blueprint, start.clone(), TIME);
            #[cfg(debug_assertions)]
            {
                println!("{:#?}", result);
            }
            total *= result.resources[3] as Self::Result2;
        }

        total
    }
}

fn search(blueprint: &Blueprint, start: BlueprintResult, max_time: Time) -> BlueprintResult {
    let max_ore_robots = blueprint.max_ore_cost();
    let max_clay_robots = blueprint.max_clay_cost();
    let max_obsidian_robots = blueprint.max_obsidian_cost();

    type State = (Reverse<Time>, Resource, Resource, BlueprintResult);
    let mut queue: BinaryHeap<State> = Default::default();
    queue.push((Reverse(0), 0, 0, start));

    let mut visited: HashSet<BlueprintResult> = Default::default();

    while let Some((Reverse(time), _, geode_robots, mut state)) = queue.pop() {
        if time == max_time {
            return state;
        }

        if !visited.insert(state.clone()) {
            continue;
        }

        // capture resource from time - 1 for later comparison
        // if we had the chance to build robot x in last round we should have done so and forbid
        // building that robot type this round!
        let [old_ore, old_clay, old_obsidian, _] = state.resources;

        for i in 0..4 {
            state.resources[i] += state.robots[i];
        }

        for i in 0..4 {
            state.robots[i] += state.robots_building[i];
        }

        state.robots_building = [0; 4];

        let [ore, clay, obsidian, geode] = state.resources;
        let [ore_robots, clay_robots, obsidian_robots, _] = state.robots;

        if ore >= blueprint.geode_cost.0
            && obsidian >= blueprint.geode_cost.1
            && (old_ore < blueprint.geode_cost.0 || old_obsidian < blueprint.geode_cost.1)
        {
            let new_state = BlueprintResult {
                robots_building: [0, 0, 0, 1],
                resources: [
                    ore - blueprint.geode_cost.0,
                    clay,
                    obsidian - blueprint.geode_cost.1,
                    geode,
                ],
                ..state
            };
            queue.push((Reverse(time + 1), geode, geode_robots, new_state));
            continue;
        }

        let max_obsidian_demand = max_obsidian_robots * (max_time - time);
        let obsidian_supply = obsidian + obsidian_robots * (max_time - time);
        if obsidian_robots < max_obsidian_robots
            && max_obsidian_demand > obsidian_supply
            && ore >= blueprint.obsidian_cost.0
            && clay >= blueprint.obsidian_cost.1
            && (old_ore < blueprint.obsidian_cost.0 || old_clay < blueprint.obsidian_cost.1)
        {
            let new_state = BlueprintResult {
                robots_building: [0, 0, 1, 0],
                resources: [
                    ore - blueprint.obsidian_cost.0,
                    clay - blueprint.obsidian_cost.1,
                    obsidian,
                    geode,
                ],
                ..state
            };
            let new_state_ore = new_state.resources[0];
            queue.push((Reverse(time + 1), geode, geode_robots, new_state));

            // continue if building a obsidian robot does not interfer with building the maximum of geode robots
            if new_state_ore >= (max_time - time) * blueprint.geode_cost.0 {
                continue;
            }
        }

        let max_clay_demand = max_clay_robots * (max_time - time);
        let clay_supply = clay + clay_robots * (max_time - time);
        if clay_robots < max_clay_robots
            && max_clay_demand > clay_supply
            && ore >= blueprint.clay_cost
            && old_ore < blueprint.clay_cost
        {
            let new_state = BlueprintResult {
                robots_building: [0, 1, 0, 0],
                resources: [ore - blueprint.clay_cost, clay, obsidian, geode],
                ..state
            };
            queue.push((Reverse(time + 1), geode, geode_robots, new_state));
        }

        let max_ore_demand = max_ore_robots * (max_time - time);
        let ore_supply = ore + ore_robots * (max_time - time);
        if ore_robots < max_ore_robots
            && max_ore_demand > ore_supply
            && ore >= blueprint.ore_cost
            && old_ore < blueprint.ore_cost
        {
            let new_state = BlueprintResult {
                robots_building: [1, 0, 0, 0],
                resources: [ore - blueprint.ore_cost, clay, obsidian, geode],
                ..state
            };
            queue.push((Reverse(time + 1), geode, geode_robots, new_state));
        }

        queue.push((Reverse(time + 1), geode, geode_robots, state));
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part_1() {
        let mut day = Day19::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 33);
    }

    #[test]
    fn part_2() {
        let mut day = Day19::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 56 * 62);
    }
}
