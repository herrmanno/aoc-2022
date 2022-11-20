//! Common utility parses

use std::str::FromStr;

use combine::{
    optional,
    parser::{
        char::{char, digit},
        combinator::recognize,
    },
    skip_many1, Parser, Stream,
};
use num_traits::{Float, PrimInt};

#[allow(unused_macros)]
macro_rules! number_parser {
    ($name: ident) => {
        pub fn $name<Input>() -> impl Parser<Input, Output = $name>
        where
            Input: Stream<Token = char>,
        {
            many1(digit()).map(|s: String| {
                s.parse()
                    .unwrap_or_else(|_| panic!("Expected number, got {}", s))
            })
        }
    };
}

pub fn int<Input, Output: PrimInt + FromStr>() -> impl Parser<Input, Output = Output>
where
    Input: Stream<Token = char>,
{
    recognize((optional(char('-')), skip_many1(digit()))).map(|s: String| {
        s.parse::<Output>()
            .unwrap_or_else(|_| panic!("Expected number, got {}", s))
    })
}

pub fn float<Input, Output: Float + FromStr>() -> impl Parser<Input, Output = Output>
where
    Input: Stream<Token = char>,
{
    recognize((
        optional(char('-')),
        skip_many1(digit()),
        optional(char('.')),
        skip_many1(digit()),
    ))
    .map(|s: String| {
        s.parse::<Output>()
            .unwrap_or_else(|_| panic!("Expected number, got {}", s))
    })
}
