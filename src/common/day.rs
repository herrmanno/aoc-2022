//! Defines a common interface for all puzzles

/// A day's challenge
pub(crate) trait Day {
    type Result1: std::fmt::Debug + Sized;
    type Result2: std::fmt::Debug + Sized;

    /// Part 1 of this day's challenge
    fn part1(&mut self) -> Self::Result1;

    /// Part 2 of this day's challenge
    fn part2(&mut self) -> Self::Result2;

    // fn part(&mut self, part: usize) -> Either<Self::Result1, Self::Result2> {
    //     match part {
    //         1 => Either::Left(Box::new(self.part1())),
    //         2 => Either::Right(Box::new(self.part2())),
    //         _ => panic!("Invalid part {}", part)
    //     }
    // }

    /// Print result of part 1
    fn print_part1(&self, result: Self::Result1) {
        println!("Part 1: {:?}", result)
    }

    /// Print result of part 2
    fn print_part2(&self, result: Self::Result2) {
        println!("Part 2: {:?}", result)
    }

    /// Optional: parse input to use later in part1/part2
    fn parse(&mut self, input: &str);
}
