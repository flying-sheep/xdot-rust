use nom::error::Error as NomError;

use self::declarative::Pen;
pub(super) use self::declarative::ShapeDraw;

mod attrs;
mod declarative;
mod op_parser;
mod ops;
mod shapes;

pub(super) fn parse<'a>(input: &'a str) -> Result<Vec<ShapeDraw>, NomError<&'a str>> {
    use ops::Op::*;
    let mut pen = Pen::default();
    let mut shape_draws = vec![];
    for op in op_parser::parse(input)? {
        dbg!(&op);
        match op {
            DrawShape(shape) => shape_draws.push(ShapeDraw {
                pen: pen.clone(),
                shape,
            }),
            SetFontCharacteristics(fc) => pen.font_characteristics = fc,
            SetFillColor(color) => pen.fill_color = color,
            SetPenColor(color) => pen.color = color,
            SetFont { size, name } => {
                pen.font_size = size;
                pen.font_name = name;
            }
            SetStyle(style) => pen.line_style = style,
            ExternalImage(_) => todo!("conversion of external image op"),
        }
    }
    Ok(shape_draws)
}
