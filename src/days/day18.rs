//! # Day 18 Boiling Boulders
//!
//! Given a list of 3d coordinates that describe a lava droplet find
//!
//! - a) all cube faces that are not connected to an adjacent cube
//! - b) all free cube faces (like in a) that are reachable from the outside (irgnore holes insied the droplet)
//!

use rustc_hash::FxHashSet as HashSet;

use aoc_runner::Day;

type Cube = (i8, i8, i8);

#[derive(Default, Clone)]
pub struct Day18(HashSet<Cube>, (Cube, Cube));

impl Day for Day18 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        let mut min_x: Option<i8> = None;
        let mut max_x: Option<i8> = None;
        let mut min_y: Option<i8> = None;
        let mut max_y: Option<i8> = None;
        let mut min_z: Option<i8> = None;
        let mut max_z: Option<i8> = None;

        self.0 = input
            .lines()
            .map(|line| {
                let mut nums = line.split(',').map(|num| num.parse().unwrap());
                let x = nums.next().unwrap();
                let y = nums.next().unwrap();
                let z = nums.next().unwrap();

                min_x = Some(min_x.unwrap_or(x).min(x));
                max_x = Some(max_x.unwrap_or(x).max(x));
                min_y = Some(min_y.unwrap_or(y).min(y));
                max_y = Some(max_y.unwrap_or(y).max(y));
                min_z = Some(min_z.unwrap_or(z).min(z));
                max_z = Some(max_z.unwrap_or(z).max(z));

                (x, y, z)
            })
            .collect();

        self.1 = (
            (min_x.unwrap(), min_y.unwrap(), min_z.unwrap()),
            (max_x.unwrap(), max_y.unwrap(), max_z.unwrap()),
        );
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut total = 0;
        for &(x, y, z) in self.0.iter() {
            total += !self.0.contains(&(x + 1, y, z)) as usize;
            total += !self.0.contains(&(x - 1, y, z)) as usize;
            total += !self.0.contains(&(x, y + 1, z)) as usize;
            total += !self.0.contains(&(x, y - 1, z)) as usize;
            total += !self.0.contains(&(x, y, z + 1)) as usize;
            total += !self.0.contains(&(x, y, z - 1)) as usize;
        }
        total
    }

    fn part2(&mut self) -> Self::Result2 {
        #[inline(always)]
        fn is_safe(min: &Cube, max: &Cube, cube: &Cube) -> bool {
            (min.0 - 1..=max.0 + 1).contains(&cube.0)
                && (min.1 - 1..=max.1 + 1).contains(&cube.1)
                && (min.2 - 1..=max.2 + 1).contains(&cube.2)
        }

        let (min, max) = self.1;
        let mut total = 0;
        let mut visited: HashSet<Cube> = Default::default();
        let mut queue: Vec<Cube> = Default::default();
        queue.push((min.0 - 1, min.1 - 1, min.2 - 1));

        while let Some((x, y, z)) = queue.pop() {
            if !visited.insert((x, y, z)) {
                continue;
            }

            let neighbours = [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]
            .into_iter()
            .filter(|cube| is_safe(&min, &max, cube));

            for neighbour in neighbours {
                if self.0.contains(&neighbour) {
                    total += 1;
                } else {
                    queue.push(neighbour);
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part_1() {
        let mut day = Day18::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 64);
    }

    #[test]
    fn part_2() {
        let mut day = Day18::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 58);
    }
}
