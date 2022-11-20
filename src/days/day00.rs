//! Day x <name>
//!
//! <short description of problem>
//!
//! <short description of solution>

pub fn part1(input: &str) {
    let commands = parser::parse(input).unwrap().0;
    println!("{:?}", commands)
}

pub fn part2(_input: &str) {
    println!("part 2")
}

#[derive(Debug)]
enum Command {
    Rect(u32, u32),
    Rotate(RotationType, RotationAxis, i32, i32),
}

#[derive(Debug)]
enum RotationType {
    Row,
    Column,
}

#[derive(Debug)]
enum RotationAxis {
    X,
    Y,
}

mod parser {
    use super::*;
    use crate::common::parser::int;
    use combine::{
        attempt,
        choice,
        sep_end_by,
        parser::char::{char, string, spaces, digit},
        stream::easy::ParseError,
        stream::easy::Stream,
        EasyParser, Parser,
    };

    pub(super) fn parse(input: &str) -> Result<(Vec<Command>, &str), ParseError<&str>> {
        parse_commands().easy_parse(input)
    }

    fn parse_commands<'a>() -> impl Parser<Stream<&'a str>, Output = Vec<Command>> {
        sep_end_by(parse_command(), char('\n'))
    }

    fn parse_command<'a>() -> impl Parser<Stream<&'a str>, Output = Command> {
        choice((
            attempt(parse_rect()),
            attempt(parse_rotate()),
        ))
    }

    fn parse_rect<'a>() -> impl Parser<Stream<&'a str>, Output = Command> {
        (string("rect "), int(), char('x'), int())
            .map(|(_, x, _, y)| Command::Rect(x, y))
    }

    fn parse_rotate<'a>() -> impl Parser<Stream<&'a str>, Output = Command> {
        (
            string("rotate "),
            parse_rotation_type(),
            spaces(),
            parse_rotation_axis(),
            char('='),
            int(),
            string(" by "),
            int(),
        )
            .map(
                |(_, rotation_type, _, rotation_axis, _, value, _, by_value)| {
                    Command::Rotate(rotation_type, rotation_axis, value, by_value)
                },
            )
    }

    fn parse_rotation_type<'a>() -> impl Parser<Stream<&'a str>, Output = RotationType> {
        choice((
            string("row ").map(|_| RotationType::Row),
            string("column ").map(|_| RotationType::Column),
        ))
    }

    fn parse_rotation_axis<'a>() -> impl Parser<Stream<&'a str>, Output = RotationAxis> {
        choice((
            string("x").map(|_| RotationAxis::X),
            string("y").map(|_| RotationAxis::Y),
        ))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_part1() {
        assert!(false);
    }

    #[test]
    fn test_part2() {
        assert!(false);
    }
}
