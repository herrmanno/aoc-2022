/// Collection of all puzzles and utilities to run a day's puzzle
// pub(crate) mod day00; // only used as example; no valid module
pub(crate) mod day01;

use crate::common::day::Day;

#[allow(unused_macros)]
macro_rules! run_day {
    ($self: expr, $day: expr, $input: expr) => {{
        run_day!($self, $day, None, $input)
    }};

    ($self: expr, $day: expr, $part: expr, $input: expr) => {{
        match $day {
            1 => run_day!(@ $self, 0, $part, $input),
            _ => panic!("Invalid day: {:?}", $day)
        }
    }};

    (@ $self: expr, $day: tt, $part: expr, $input: expr) => {{
        $self.$day.parse($input);

        println!("Day {}", $day + 1);

        match $part {
            Some(1) => {
                let result = $self.$day.part1();
                $self.$day.print_part1(result);
            },
            Some(2) => {
                let result = $self.$day.part2();
                $self.$day.print_part2(result);
            },
            None => {
                let result = $self.$day.part1();
                $self.$day.print_part1(result);
                let result = $self.$day.part2();
                $self.$day.print_part2(result);
            },
            Some(part) => panic!("Invalid part: {:?}", part)
        }
    }};
}

pub(crate) struct Days(
    // Vec<Box<dyn Day<Result1 = dyn Any, Result2 = dyn Any>>>
    day01::Day01,
);

/// Container for all puzzles
impl Days {
    pub fn new() -> Self {
        Self(
            day01::Day01::default(),
            // vec![
            //     Box::new(day01::Day01::default()),
            // ]
        )
    }

    pub fn len(&self) -> usize {
        1
    }

    // fn get_mut(&mut self, day: usize) -> &mut impl Day<Result1 = dyn Any, Result2 = dyn Any> {
    //     match day {
    //         1 => &mut self.0,
    //         _ => panic!("{} is not a valid day", day)
    //     }
    //     // self.0.get_mut(day - 1)
    //     //     .unwrap_or_else(|| panic!("{} is not a valid day", day))
    // }

    pub fn run_day(&mut self, day: usize, input: &str) {
        run_day!(self, day, input);
        // let day = self.get_mut(day);
        // day.parse(input);
        // day.print_part1(day.part1());
        // day.print_part2(day.part2());
    }

    pub fn run_part(&mut self, day: usize, part: usize, input: &str) {
        run_day!(self, day, Some(part), input);
        // let day = self.get_mut(day);
        // day.parse(input);
        // match day.part(part) {
        //     Either::Left(_) => todo!(),
        //     Either::Right(_) => todo!(),
        // }
    }

    pub fn run_all<I: AsRef<str>>(&mut self, inputs: &[I]) {
        assert_eq!(inputs.len(), self.len());

        for i in 1..=self.len() {
            let input = inputs[i - 1].as_ref();
            self.run_day(i, input);
        }
    }
}
