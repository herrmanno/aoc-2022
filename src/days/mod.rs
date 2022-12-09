//! Collection of all puzzles and utilities to run a day's puzzle

// pub(crate) mod day00; // only used as example; no valid module
pub(crate) mod day01;
pub(crate) mod day02;
pub(crate) mod day03;
pub(crate) mod day04;
pub(crate) mod day05;
pub(crate) mod day06;
pub(crate) mod day07;

use aoc_runner::{Analyzer, Day};
use derive_aoc_runner::{Analyzer, AoC};

#[derive(Analyzer, AoC)]
pub(crate) struct Days(
    day01::Day01,
    day02::Day02,
    day03::Day03,
    day04::Day04,
    day05::Day05,
    day06::Day06,
    day07::Day07,
);
