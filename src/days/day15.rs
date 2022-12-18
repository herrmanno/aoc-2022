//! # Day 15 Beacon Exclusion Zone
//!
//! Given a list of (sender, beacon) pairs, where sender and beacon are (x,y) points and the
//! receiving radius of every sender equals its manhattan distance to the corresponding beacon
//!
//! - a) find how many coordinates in row 2000000 are covered by a sensor
//! - b) find the one coordinate that is not covered by any sensor
//!
//! where 'covered' means the points distance to any sensor is equal to or less than that sensors
//! radius.
//!
//! The basic idea for part 2 is the following:
//! If there is a unique (!) coordinate that is not covered by any sensor, then all its neighbour
//! must be covered by sensors (otherwise it wouldn't be the only uncovered spot).
//! For this to be true the four neighbouring coordinates must all be covered by different sensors
//! because no sensor would be able to cover two of the beacon's neighours (if it would, its range)
//! would 'curve' around the beacon, which is not possible when the sensor's coverage has the shape
//! of a rhombus (diamond).
//! But if the four neighbours of the beacon are all covered by different sensors, then those
//! coordinates are likely [0] intersections between different sensor's coverages. So the best
//! guesses of coordinates where the distress beacon may be are neighbours of coordinates where
//! the coverages areas of two sensors intersect.
//!
//! [0]: It would be possible that the neighbours of the beacon are all covered by the 'spikes' of
//! a sensor's coverage area. Luckily, that is not the case with our input.

use rustc_hash::FxHashMap as HashMap;

use aoc_runner::Day;

const ROW_TO_CHECK: usize = 2000000;
const AREA_TO_CHECK: i32 = 4000000;

type Point = (i32, i32);
type Radius = u32;
type Sensor = (Point, Radius);
type Range = (i32, i32);

/// Collapses two range into one bigger range, if the overlap
fn collapse_ranges(a: Range, b: Range) -> Option<Range> {
    if a.0 <= b.1 && b.0 <= a.1 {
        Some((a.0.min(b.0), a.1.max(b.1)))
    } else {
        None
    }
}

/// finds all possible intersection points between to sensors
fn intersection_points(a: Sensor, b: Sensor) -> Vec<(i32, i32)> {
    let dist = a.0 .0.abs_diff(b.0 .0) + a.0 .1.abs_diff(b.0 .1);
    if dist > a.1 + b.1 {
        return vec![];
    }

    /*
       Image the four sides of the diamons
           ^
    a --> / \  <-- b
         /   \
         \   /
    c --> \ /  <-- d
           v
    */

    type Slope = i32;
    type Y0 = i32;
    type Line = (Slope, Y0);

    fn get_sides(a: Sensor) -> [Line; 4] {
        // let top = (a.0.0, a.0.1 + a.1 as i32);
        // let bottom = (a.0.0, a.0.1 - a.1 as i32);
        let left = (a.0 .0 - a.1 as i32, a.0 .1);
        let right = (a.0 .0 + a.1 as i32, a.0 .1);
        let a = (1, -left.0 + left.1);
        let b = (-1, right.0 + right.1);
        let c = (-1, left.0 + left.1);
        let d = (1, -right.0 + right.1);
        [a, b, c, d]
    }

    fn find_crosspoint(a: Line, b: Line) -> (f32, f32) {
        let x = (b.1 - a.1) as f32 / (a.0 - b.0) as f32;
        let y = a.0 as f32 * x + a.1 as f32;
        (x, y)
    }

    let [a1, b1, c1, d1] = get_sides(a);
    let [a2, b2, c2, d2] = get_sides(b);
    let crosspoints = [
        find_crosspoint(a1, b2),
        find_crosspoint(a1, c2),
        find_crosspoint(b1, a2),
        find_crosspoint(b1, d2),
        find_crosspoint(c1, a2),
        find_crosspoint(c1, d2),
        find_crosspoint(d1, b2),
        find_crosspoint(d1, c2),
    ];

    crosspoints
        .into_iter()
        .map(|(x, y)| (x.round() as i32, y.round() as i32))
        .filter(|(x, y)| {
            let in_a = (x.abs_diff(a.0 .0) + y.abs_diff(a.0 .1)) as u32 <= a.1;
            let in_b = (x.abs_diff(b.0 .0) + y.abs_diff(b.0 .1)) as u32 <= b.1;
            in_a && in_b
        })
        .collect()
}

#[derive(Default)]
pub struct Day15(Vec<Sensor>);

impl Day15 {
    fn num_covered_positions_in_row(&self, row: usize) -> usize {
        let mut ranges: Vec<Range> = vec![];
        for (sensor, radius) in self.0.iter() {
            let y_diff = sensor.1.abs_diff(row as i32);
            if y_diff > *radius {
                continue;
            }
            let remaining_distance = radius.saturating_sub(y_diff);
            let min_x = sensor.0 - remaining_distance as i32;
            let max_x = sensor.0 + remaining_distance as i32;

            ranges.push((min_x, max_x));
        }

        ranges.sort_unstable();

        let collapsed_ranges: Vec<Range> = {
            let mut collapsed_ranges: Vec<Range> = vec![];
            for range in ranges {
                if let Some(last_range) = collapsed_ranges.last() {
                    if let Some(new_range) = collapse_ranges(*last_range, range) {
                        collapsed_ranges.pop();
                        collapsed_ranges.push(new_range);
                        continue;
                    }
                }
                collapsed_ranges.push(range);
            }
            collapsed_ranges
        };

        collapsed_ranges
            .into_iter()
            .map(|(low, high)| (high - low) as usize)
            .sum()
    }

    fn get_beacon_position_in_area(&self, max_area: i32) -> (i32, i32) {
        let mut points: HashMap<Point, usize> = Default::default();
        for (idx, a) in self.0.iter().enumerate() {
            for b in self.0.iter().skip(idx + 1) {
                let intersections = intersection_points(*a, *b).into_iter();
                for point in intersections
                    .flat_map(|(x, y)| [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)])
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x <= max_area && *y <= max_area)
                {
                    let n = points.entry(point).or_insert(0);
                    *n += 1;
                    if *n == 4 && self.check_point(point) {
                        return point;
                    }
                }
            }
        }

        panic!("Beacon not found");
    }

    /// checks if a point is the holy beacon we are looking for
    fn check_point(&self, point: (i32, i32)) -> bool {
        for (sender, radius) in self.0.iter() {
            let distance = sender.0.abs_diff(point.0) + sender.1.abs_diff(point.1);
            if distance <= *radius {
                return false;
            }
        }

        true
    }
}

impl Day for Day15 {
    type Result1 = usize;
    type Result2 = u128;

    fn parse(&mut self, input: &str) {
        fn extract_num<'a>(words: &mut impl Iterator<Item = &'a str>, nth: usize) -> i32 {
            words
                .nth(nth)
                .and_then(|part| part.split_once('='))
                .and_then(|(_, part)| {
                    part.strip_suffix(',')
                        .or_else(|| part.strip_suffix(':'))
                        .or(Some(part))
                })
                .and_then(|num| num.parse().ok())
                .unwrap()
        }
        self.0 = input
            .lines()
            .map(|line| {
                let mut words = line.split(' ');
                let sensor_x = extract_num(&mut words, 2);
                let sensor_y = extract_num(&mut words, 0);
                let beacon_x = extract_num(&mut words, 4);
                let beacon_y = extract_num(&mut words, 0);
                let radius = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);
                ((sensor_x, sensor_y), radius)
            })
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.num_covered_positions_in_row(ROW_TO_CHECK)
    }

    fn part2(&mut self) -> Self::Result2 {
        let (x, y) = self.get_beacon_position_in_area(AREA_TO_CHECK);
        x as u128 * 4000000 + y as u128
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part_1() {
        let mut day = Day15::default();
        day.parse(INPUT);
        assert_eq!(day.num_covered_positions_in_row(10), 26);
    }

    #[test]
    fn part_2() {
        let mut day = Day15::default();
        day.parse(INPUT);
        assert_eq!(day.get_beacon_position_in_area(20), (14, 11));
    }
}
