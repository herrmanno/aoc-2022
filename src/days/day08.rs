//! # Day 08 Treetop Tree House
//!
//! Given a map (forest) of integers (height of tree at that position)
//!
//! - a) calculate how many trees are visible
//! - b) find the location from where the most trees are visible
//!
//! Where for
//!
//! - a) a tree is visible from the edge if there is no bigger tree between the edge and the three
//! in question itself
//! - b) a tree is visible from an inner tree as if there is no tree bigger then the inner tree
//! between the inner tree and the tree in question

use aoc_runner::Day;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day08(Vec<Vec<char>>);

impl Day for Day08 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.0 = input.lines().map(|line| line.chars().collect()).collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut trees: HashSet<(usize, usize)> = HashSet::new();

        // look from left and right edge for every row
        for (y, row) in self.0.iter().enumerate() {
            let mut max_l = '.';
            let mut max_r = '.';
            let mut x_l = 0;
            let mut x_r = row.len() - 1;

            loop {
                if row[x_l] > max_l {
                    trees.insert((x_l, y));
                    max_l = row[x_l];
                }
                if row[x_r] > max_r {
                    trees.insert((x_r, y));
                    max_r = row[x_r];
                }

                if x_r == 0 {
                    break;
                }

                x_l += 1;
                x_r -= 1;
            }
        }

        // look from top and bottom edge for every column
        for x in 0..self.0.len() {
            let mut max_t = '.';
            let mut max_b = '.';
            let mut y_t = 0;
            let mut y_b = self.0[0].len() - 1;

            loop {
                if self.0[y_t][x] > max_t {
                    trees.insert((x, y_t));
                    max_t = self.0[y_t][x];
                }
                if self.0[y_b][x] > max_b {
                    trees.insert((x, y_b));
                    max_b = self.0[y_b][x];
                }

                if y_b == 0 {
                    break;
                }

                y_t += 1;
                y_b -= 1;
            }
        }

        trees.len()
    }

    // SLOWER (!) ALTERNATIVE for part 2
    // fn part2(&mut self) -> Self::Result2 {
    //     let mut distance_map: HashMap<(usize, usize), usize> = HashMap::new();
    //     let mut stack: VecDeque<char> = VecDeque::new();

    //     for (y, row) in self.0.iter().enumerate() {
    //         stack.clear();

    //         for (x, &tree) in row.iter().enumerate() {
    //             let distance = stack.iter().enumerate().find_map(|(idx, &el)| if el >= tree { Some(idx + 1) } else { None }).unwrap_or(stack.len());
    //             distance_map.insert((x,y), distance);
    //             stack.push_front(tree);
    //         }

    //         stack.clear();

    //         for (x, &tree) in row.iter().enumerate().rev() {
    //             let distance = stack.iter().enumerate().find_map(|(idx, &el)| if el >= tree { Some(idx + 1) } else { None }).unwrap_or(stack.len());
    //             *distance_map.get_mut(&(x,y)).unwrap() *= distance;
    //             stack.push_front(tree);
    //         }
    //     }
    //     for x in 0..self.0[0].len() {
    //         stack.clear();

    //         for y in 0..self.0.len() {
    //             let tree = self.0[y][x];
    //             let distance = stack.iter().enumerate().find_map(|(idx, &el)| if el >= tree { Some(idx + 1) } else { None }).unwrap_or(stack.len());
    //             *distance_map.get_mut(&(x,y)).unwrap() *= distance;
    //             stack.push_front(tree);
    //         }

    //         stack.clear();

    //         for y in (0..self.0.len()).rev() {
    //             let tree = self.0[y][x];
    //             let distance = stack.iter().enumerate().find_map(|(idx, &el)| if el >= tree { Some(idx + 1) } else { None }).unwrap_or(stack.len());
    //             *distance_map.get_mut(&(x,y)).unwrap() *= distance;
    //             stack.push_front(tree);
    //         }
    //     }

    //     distance_map.into_values().max().unwrap()
    // }

    fn part2(&mut self) -> Self::Result2 {
        #[inline(always)]
        fn scenic_distance(map: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
            let height = map.len();
            let width = map[0].len();
            let tree = map[y][x];

            let left = {
                let mut count = 0usize;
                let mut x = x as i32 - 1;
                while x >= 0 {
                    count += 1;
                    if map[y][x as usize] >= tree {
                        break;
                    }

                    x -= 1;
                }
                count
            };

            if left == 0 {
                return 0;
            }

            let right = {
                let mut count = 0usize;
                let mut x = x + 1;
                while x < width {
                    count += 1;
                    if map[y][x] >= tree {
                        break;
                    }

                    x += 1;
                }
                count
            };

            if right == 0 {
                return 0;
            }

            let top = {
                let mut count = 0usize;
                let mut y = y as i32 - 1;
                while y >= 0 {
                    count += 1;
                    if map[y as usize][x] >= tree {
                        break;
                    }
                    y -= 1;
                }
                count
            };

            if top == 0 {
                return 0;
            }

            let bottom = {
                let mut count = 0usize;
                let mut y = y + 1;
                while y < height {
                    count += 1;
                    if map[y][x] >= tree {
                        break;
                    }
                    y += 1;
                }
                count
            };

            left * right * top * bottom
        }

        let mut max = 0;
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                max = max.max(scenic_distance(&self.0, x, y));
            }
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part_1() {
        let mut day = Day08::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 21);
    }

    #[test]
    fn part_2() {
        let mut day = Day08::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 8);
    }
}
