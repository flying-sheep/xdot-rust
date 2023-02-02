///! Stateless parser extracting xdot operations
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1, one_of},
    combinator::{map_res, recognize},
    error::{Error as NomError, ParseError},
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult,
};

use super::{attrs::FontCharacteristics, ops::Op};

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_op_draw_shape(input: &str) -> IResult<&str, Op> {
    todo!()
}
fn parse_op_set_font_characteristics(input: &str) -> IResult<&str, Op> {
    preceded(
        tuple((tag("t"), multispace1)),
        map_res(decimal, |value| {
            u128::from_str(value).map(|n| FontCharacteristics::from_bits_truncate(n).into())
        }),
    )(input)
}
fn parse_op_set_fill_color(input: &str) -> IResult<&str, Op> {
    todo!()
}
fn parse_op_set_pen_color(input: &str) -> IResult<&str, Op> {
    todo!()
}
fn parse_op_set_font(input: &str) -> IResult<&str, Op> {
    todo!()
}
fn parse_op_set_style(input: &str) -> IResult<&str, Op> {
    todo!()
}
fn parse_op_external_image(input: &str) -> IResult<&str, Op> {
    todo!()
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        ws(parse_op_draw_shape),
        ws(parse_op_set_font_characteristics),
        ws(parse_op_set_fill_color),
        ws(parse_op_set_pen_color),
        ws(parse_op_set_font),
        ws(parse_op_set_style),
        ws(parse_op_external_image),
    ))(input)
}

pub(super) fn parse(input: &str) -> Result<Vec<Op>, NomError<&str>> {
    // TODO: what to do instead of swallowing rest?
    many0(parse_op)(input).finish().map(|(_rest, ops)| ops)
}
