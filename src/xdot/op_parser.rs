///! Stateless parser extracting xdot operations.
///! See https://graphviz.org/docs/outputs/canon/#xdot
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1, one_of},
    combinator::{flat_map, map, map_res, recognize},
    error::{Error as NomError, ParseError},
    multi::{count, many0, many1, separated_list0},
    number::complete::float,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

use super::{
    attrs::FontCharacteristics,
    ops::Op,
    shapes::{Ellipse, Points, PointsType},
};

// Combinators

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

// Op parsers

fn parse_op_draw_shape_ellipse(input: &str) -> IResult<&str, Op> {
    map(
        separated_pair(
            one_of("Ee"),
            multispace1,
            tuple((float, float, float, float)),
        ),
        |(c, (x, y, w, h))| {
            Ellipse {
                filled: c == 'E',
                x,
                y,
                w,
                h,
            }
            .into()
        },
    )(input)
}
fn parse_op_draw_shape_points(input: &str) -> IResult<&str, Op> {
    map(
        tuple((
            one_of("PpLBb"),
            flat_map(map_res(decimal, usize::from_str), |n| {
                count(
                    tuple((preceded(multispace1, float), preceded(multispace1, float))),
                    n,
                )
            }),
        )),
        |(c, points)| -> Op {
            Points {
                filled: c == 'P' || c == 'b',
                typ: match c {
                    'P' | 'p' => PointsType::Polygon,
                    'L' => PointsType::Polyline,
                    'B' | 'b' => PointsType::BSpline,
                    _ => unreachable!(),
                },
                points,
            }
            .into()
        },
    )(input)
}
fn parse_op_draw_shape_text(input: &str) -> IResult<&str, Op> {
    todo!("parsing of text draw op")
}
fn parse_op_draw_shape(input: &str) -> IResult<&str, Op> {
    alt((
        parse_op_draw_shape_ellipse,
        parse_op_draw_shape_points,
        parse_op_draw_shape_text,
    ))(input)
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
    todo!("parsing of fill color set op")
}
fn parse_op_set_pen_color(input: &str) -> IResult<&str, Op> {
    todo!("parsing of pen color set op")
}
fn parse_op_set_font(input: &str) -> IResult<&str, Op> {
    todo!("parsing of font set op")
}
fn parse_op_set_style(input: &str) -> IResult<&str, Op> {
    todo!("parsing of style set op")
}
fn parse_op_external_image(input: &str) -> IResult<&str, Op> {
    todo!("parsing of external image op")
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        parse_op_draw_shape,
        parse_op_set_font_characteristics,
        parse_op_set_fill_color,
        parse_op_set_pen_color,
        parse_op_set_font,
        parse_op_set_style,
        parse_op_external_image,
    ))(input)
}

pub(super) fn parse(input: &str) -> Result<Vec<Op>, NomError<&str>> {
    // TODO: what to do instead of swallowing rest?
    ws(separated_list0(multispace1, parse_op))(input)
        .finish()
        .map(|(_rest, ops)| ops)
}
