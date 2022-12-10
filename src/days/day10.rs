//! # Day 10 Cathode-Ray Tube
//!
//! Given a list of instructions ('noop' or 'add x') where instruction
//!
//! - `noop` does nothing for one cycle
//! - `add x` adds `x` to the register after two(!) cycles (the instruction blocks for two cycles)
//!
//! do for part
//!
//! - a) calculate the register values, multiplied with the cycle value at cycles 20, 60, 100, 140, 180, 220
//! - b) treat the register value at cycle `n` as position of a '###' sprite at time `n` and draw
//! a 40px * 60px based on the fact if the pixel that will be drawn at time `n` is blocked by the sprite
//!

use std::fmt::Write;

use aoc_runner::Day;

/// Visual representation of the 40px * 6px CRT output
pub struct Crt([bool; 240]);

impl std::fmt::Debug for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for y in 0..6 {
            for x in 0..40 {
                f.write_char(if self.0[y * 40 + x] { '#' } else { '.' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct Day10(String);

impl Day for Day10 {
    type Result1 = i32;
    type Result2 = Crt;

    fn parse(&mut self, input: &str) {
        self.0 = input.to_owned();
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut cycle = 1;
        let mut register = 1;
        let mut final_value = 0;

        for line in self.0.lines() {
            let num = line
                .split_once(' ')
                .and_then(|(_, num)| num.parse::<i32>().ok());

            if cycle > 0 && (cycle + 20) % 40 == 0 {
                final_value += cycle * register;
            }

            if let Some(num) = num {
                if cycle > 0 && (cycle + 21) % 40 == 0 {
                    final_value += (cycle + 1) * register;
                }
                register += num;
                cycle += 2;
            } else {
                cycle += 1;
            }
        }

        final_value
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut lines = self.0.lines();
        let mut cycle: usize = 1;
        let mut register = 1i32;
        let mut pixels = [false; 40 * 6];

        while cycle <= 240 {
            let pixel_pos = (cycle as i32 - 1) % 40;
            let line = lines.next().unwrap();
            let num = line
                .split_once(' ')
                .and_then(|(_, num)| num.parse::<i32>().ok());
            if let Some(num) = num {
                pixels[cycle - 1] = pixel_pos.abs_diff(register) <= 1;
                pixels[cycle] = ((pixel_pos + 1) % 40).abs_diff(register) <= 1;
                register += num;
                cycle += 2;
            } else {
                pixels[cycle - 1] = pixel_pos.abs_diff(register) <= 1;
                cycle += 1;
            }
        }

        Crt(pixels)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part_1() {
        let mut day = Day10::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 13140);
    }

    #[test]
    fn part_2() {
        let mut day = Day10::default();
        day.parse(INPUT);
        let expected_str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        let expected_vec = expected_str
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| c == '#')
            .collect::<Vec<bool>>();
        assert_eq!(expected_vec.len(), 240);
        let expected = Crt(expected_vec.as_slice().try_into().unwrap());
        let actual = day.part2();
        println!("Expected: {:?}", expected);
        println!("Actual: {:?}", actual);
        assert_eq!(expected.0, actual.0);
    }
}
