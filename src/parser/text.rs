use crate::parser::parse_variable;
use crate::structures::Entity;
use nom::{branch::alt, bytes::complete::take, multi::many0, IResult};

fn parse_variable_wrapper(input: &str) -> IResult<&str, Entity> {
    let (input, var) = parse_variable(input)?;

    Ok((input, Entity::Variable(var)))
}

fn parse_rest_wrapper(input: &str) -> IResult<&str, Entity> {
    // TODO: Make this capture everything until variable
    let (input, text) = take(1_usize)(input)?; 

    Ok((input, Entity::Text(text)))
}

pub fn parse_text(input: &str) -> IResult<&str, Vec<Entity>> {
    let input = many0(alt((parse_variable_wrapper, parse_rest_wrapper)))(input)?;

    Ok(input)
}
