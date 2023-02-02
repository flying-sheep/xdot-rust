use super::{attrs::*, shapes::Shape};

/// Store pen attributes.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Pen {
    color: Rgba,
    fill_color: Rgba,
    line_width: f32,
    line_style: Style,
    font_size: f32,
    font_name: String,
    font_characteristics: FontCharacteristics,
}
impl Default for Pen {
    fn default() -> Self {
        Pen {
            color: Default::default(),
            fill_color: Default::default(),
            line_width: 1.0,
            line_style: Default::default(),
            font_size: 14.0,
            font_name: "Times-Roman".to_owned(),
            font_characteristics: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShapeDraw {
    pen: Pen,
    desc: Shape,
}
