///! Stateless parser extracting xdot operations.
///! See https://graphviz.org/docs/outputs/canon/#xdot
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while_m_n},
    character::complete::{char, multispace0, multispace1, one_of},
    combinator::{eof, flat_map, map, map_parser, map_res, recognize, value},
    error::{Error as NomError, ParseError},
    multi::{count, many0, many1, separated_list1},
    number::complete::float,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

use super::shapes::ExternalImage;

use super::{
    attrs::{FontCharacteristics, Rgba, Style},
    ops::Op,
    shapes::{Ellipse, Points, PointsType, Text, TextAlign},
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

fn tagged<'a, F, O, E: ParseError<&'a str>>(
    t: &'static str,
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(tuple((tag(t), multispace1)), inner)
}

// Data type parsers

/// Parse xdot’s “n -b₁b₂...bₙ” pattern
fn parse_string(input: &str) -> IResult<&str, &str> {
    // TODO: take bytes, not chars
    flat_map(map_res(decimal, usize::from_str), |n| {
        preceded(tuple((multispace1, tag("-"))), take(n))
    })(input)
}

fn parse_text_align(input: &str) -> IResult<&str, TextAlign> {
    alt((
        value(TextAlign::Left, tag("-1")),
        value(TextAlign::Center, tag("0")),
        value(TextAlign::Right, tag("1")),
    ))(input)
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Rgba> {
    let (input, _) = tag("#")(input)?;
    let (input, (r, g, b)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;
    Ok((input, Rgba { r, g, b, a: 0xff }))
}

// Op parsers

fn parse_op_draw_shape_ellipse(input: &str) -> IResult<&str, Op> {
    let (input, (c, x, y, w, h)) = tuple((
        one_of("Ee"),
        preceded(multispace1, float),
        preceded(multispace1, float),
        preceded(multispace1, float),
        preceded(multispace1, float),
    ))(input)?;
    let ellip = Ellipse {
        filled: c == 'E',
        x,
        y,
        w,
        h,
    };
    Ok((input, ellip.into()))
}

fn parse_op_draw_shape_points(input: &str) -> IResult<&str, Op> {
    let (input, (c, points)) = tuple((
        terminated(one_of("PpLBb"), multispace1),
        flat_map(map_res(decimal, usize::from_str), |n| {
            count(
                tuple((preceded(multispace1, float), preceded(multispace1, float))),
                n,
            )
        }),
    ))(input)?;
    let points = Points {
        filled: c == 'P' || c == 'b',
        typ: match c {
            'P' | 'p' => PointsType::Polygon,
            'L' => PointsType::Polyline,
            'B' | 'b' => PointsType::BSpline,
            _ => unreachable!(),
        },
        points,
    };
    Ok((input, points.into()))
}

fn parse_op_draw_shape_text(input: &str) -> IResult<&str, Op> {
    let (input, (x, y, align, width, text)) = tagged(
        "T",
        tuple((
            terminated(float, multispace1),            // x
            terminated(float, multispace1),            // y
            terminated(parse_text_align, multispace1), // align
            terminated(float, multispace1),            // width
            parse_string,
        )),
    )(input)?;
    let text = Text {
        x,
        y,
        align,
        width,
        text: text.to_owned(),
    };
    Ok((input, text.into()))
}

fn parse_op_draw_shape(input: &str) -> IResult<&str, Op> {
    alt((
        parse_op_draw_shape_ellipse,
        parse_op_draw_shape_points,
        parse_op_draw_shape_text,
    ))(input)
}

fn parse_op_set_font_characteristics(input: &str) -> IResult<&str, Op> {
    tagged(
        "t",
        map_res(decimal, |value| {
            u128::from_str(value).map(|n| FontCharacteristics::from_bits_truncate(n).into())
        }),
    )(input)
}

fn parse_op_set_color<'a>(
    t: &'static str,
    op: impl FnMut(Rgba) -> Op,
) -> impl FnMut(&'a str) -> IResult<&'a str, Op> {
    map(tagged(t, map_parser(parse_string, hex_color)), op)
}

fn parse_op_set_fill_color(input: &str) -> IResult<&str, Op> {
    parse_op_set_color("C", Op::SetFillColor)(input)
}

fn parse_op_set_pen_color(input: &str) -> IResult<&str, Op> {
    parse_op_set_color("c", Op::SetPenColor)(input)
}

fn parse_op_set_font(input: &str) -> IResult<&str, Op> {
    let (input, (size, name)) =
        tagged("F", separated_pair(float, multispace1, parse_string))(input)?;
    Ok((
        input,
        Op::SetFont {
            size,
            name: name.to_owned(),
        },
    ))
}

fn parse_op_set_style(input: &str) -> IResult<&str, Op> {
    let (input, style) = tagged("S", map_res(parse_string, Style::from_str))(input)?;
    Ok((input, style.into()))
}

fn parse_op_external_image(input: &str) -> IResult<&str, Op> {
    // TODO: implement
    use nom::combinator::{cut, fail};
    map(tagged("I", cut(fail)), |_: ()| ExternalImage.into())(input)
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
    terminated(ws(separated_list1(multispace1, parse_op)), eof)(input)
        .finish()
        .map(|(rest, ops)| {
            assert_eq!(rest, "");
            ops
        })
}

#[test]
fn test_ellipse() {
    assert_eq!(
        parse_op_draw_shape_ellipse("e 27 90 27 18"),
        Ok((
            "",
            Ellipse {
                filled: false,
                x: 27.,
                y: 90.,
                w: 27.,
                h: 18.,
            }
            .into()
        ))
    )
}

#[test]
fn test_b_spline() {
    assert_eq!(
        parse_op_draw_shape_points("B 4 27 71.7 27 60.85 27 46.92 27 36.1"),
        Ok((
            "",
            Points {
                filled: false,
                typ: PointsType::BSpline,
                points: vec![(27., 71.7), (27., 60.85), (27., 46.92), (27., 36.1)]
            }
            .into()
        ))
    )
}
