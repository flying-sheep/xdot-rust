use super::{attrs::*, shapes::Shape};

/// Store pen attributes.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Pen {
    pub color: Rgba,
    pub fill_color: Rgba,
    pub line_width: f32,
    pub line_style: Style,
    pub font_size: f32,
    pub font_name: String,
    pub font_characteristics: FontCharacteristics,
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
    pub pen: Pen,
    pub shape: Shape,
}
