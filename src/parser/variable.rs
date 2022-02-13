use crate::structures::Variable;
use nom::{
    character::complete::{alphanumeric1, char, digit1, multispace0},
    combinator::{map_res, opt},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

pub fn parse_variable(input: &str) -> IResult<&str, Variable> {
    let (input, (name, _, (lower, upper), _)) = tuple((
        opt(alphanumeric1),
        char('('),
        separated_pair(
            delimited(
                multispace0,
                map_res(digit1, |s: &str| s.parse()),
                multispace0,
            ),
            char(','),
            delimited(
                multispace0,
                map_res(digit1, |s: &str| s.parse()),
                multispace0,
            ),
        ),
        char(')'),
    ))(input)?;

    Ok((
        input,
        Variable {
            name: name,
            lower: lower,
            upper: upper,
        },
    ))
}
