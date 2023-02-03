use nom::error::Error as NomError;

mod attrs;
mod declarative;
mod op_parser;
mod ops;
mod shapes;

pub use self::attrs::{FontCharacteristics, Rgba, Style};
pub use self::declarative::{Pen, ShapeDraw};
pub use self::shapes::{Ellipse, ExternalImage, Points, PointsType, Shape, Text, TextAlign};

pub fn parse(input: &str) -> Result<Vec<ShapeDraw>, NomError<&str>> {
    use ops::Op::*;
    let mut pen = Pen::default();
    let mut shape_draws = vec![];
    for op in op_parser::parse(input)? {
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
