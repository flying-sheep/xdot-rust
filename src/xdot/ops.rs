//! xdot drawing and pen manipulation operation
use super::{
    draw::{FontCharacteristics, Rgba, Style},
    shapes::*,
};

/// An operation to draw a shape or modify drawing attributes like color or line style.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum Op {
    DrawShape(Shape),
    SetFontCharacteristics(FontCharacteristics),
    SetFillColor(Rgba),
    SetPenColor(Rgba),
    SetFont { size: f32, name: String },
    SetStyle(Style), // TODO: is it just one?
    ExternalImage(ExternalImage),
}

// shapes

impl From<Shape> for Op {
    fn from(val: Shape) -> Self {
        Op::DrawShape(val)
    }
}

impl From<Ellipse> for Op {
    fn from(val: Ellipse) -> Self {
        Into::<Shape>::into(val).into()
    }
}
impl From<Points> for Op {
    fn from(val: Points) -> Self {
        Into::<Shape>::into(val).into()
    }
}
impl From<Text> for Op {
    fn from(val: Text) -> Self {
        Into::<Shape>::into(val).into()
    }
}

// rest

impl From<FontCharacteristics> for Op {
    fn from(val: FontCharacteristics) -> Self {
        Op::SetFontCharacteristics(val)
    }
}

impl From<Style> for Op {
    fn from(val: Style) -> Self {
        Op::SetStyle(val)
    }
}

impl From<ExternalImage> for Op {
    fn from(val: ExternalImage) -> Self {
        Op::ExternalImage(val)
    }
}
