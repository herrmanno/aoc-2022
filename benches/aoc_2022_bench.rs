use std::time::Duration;

const WARM_UP_TIME: Duration = Duration::from_secs(1);
const MEASUREMENT_TIME: Duration = Duration::from_secs(1);
const SAMPLE_SIZE: usize = 100;

macro_rules! get_input {
    ($day: expr) => {{
        let input_file_path = format!("./input/{}.txt", $day);
        let input = std::fs::read_to_string(input_file_path).expect("Could not read input file");
        input
    }};
}

macro_rules! bench_day {
    ($day: expr, $name: ident, $Day: ty) => {
    mod $name {
        use criterion::{black_box, criterion_group, Criterion};
        use aoc_runner::Day;

        fn parse(c: &mut Criterion) {
            let input = get_input!($day);
            let mut day = <$Day>::default();
            let name = format!("day {} - parse", $day);
            c.bench_function(&name, |b| b.iter(|| day.parse(black_box(&input))));
        }

        fn part1(c: &mut Criterion) {
            let input = get_input!($day);
            let mut day = <$Day>::default();
            day.parse(&input);
            let name = format!("day {} - part 1", $day);
            c.bench_function(&name, |b| b.iter(|| black_box(day.part1())));
        }

        fn part2(c: &mut Criterion) {
            let input = get_input!($day);
            let mut day = <$Day>::default();
            day.parse(&input);
            let name = format!("day {} - part 2", $day);
            c.bench_function(&name, |b| b.iter(|| black_box(day.part2())));
        }

        criterion_group!(
            name = bench;
            config = Criterion::default().sample_size(super::SAMPLE_SIZE).warm_up_time(super::WARM_UP_TIME).measurement_time(super::MEASUREMENT_TIME);
            targets = parse, part1, part2
        );
    }
    };
}

bench_day!("01", day_01, aoc2022::days::day01::Day01);
bench_day!("02", day_02, aoc2022::days::day02::Day02);
bench_day!("03", day_03, aoc2022::days::day03::Day03);
bench_day!("04", day_04, aoc2022::days::day04::Day04);
bench_day!("05", day_05, aoc2022::days::day05::Day05);
bench_day!("06", day_06, aoc2022::days::day06::Day06);
bench_day!("07", day_07, aoc2022::days::day07::Day07);
bench_day!("08", day_08, aoc2022::days::day08::Day08);
bench_day!("09", day_09, aoc2022::days::day09::Day09);
bench_day!("10", day_10, aoc2022::days::day10::Day10);
bench_day!("11", day_11, aoc2022::days::day11::Day11);
bench_day!("12", day_12, aoc2022::days::day12::Day12);
bench_day!("13", day_13, aoc2022::days::day13::Day13);
bench_day!("14", day_14, aoc2022::days::day14::Day14);

criterion::criterion_main!(
    day_01::bench,
    day_02::bench,
    day_03::bench,
    day_04::bench,
    day_05::bench,
    day_06::bench,
    day_07::bench,
    day_08::bench,
    day_09::bench,
    day_10::bench,
    day_11::bench,
    day_12::bench,
    day_13::bench,
    day_14::bench,
);
