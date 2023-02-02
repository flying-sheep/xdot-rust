///! xdot drawing and pen manipulation operation

use super::{attrs::*, shapes::*};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Never {}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Op {
    DrawShape(Shape),
    SetFontCharacteristics(FontCharacteristics),
    SetFillColor(Rgba),
    SetPenColor(Rgba),
    SetFont { size: f32, name: String },
    SetStyle(Style), // TODO: is it just one?
    ExternalImage(Never), // FIXME
}

// shapes

impl Into<Op> for Shape {
    fn into(self) -> Op { Op::DrawShape(self) }
}

impl Into<Op> for Ellipse {
    fn into(self) -> Op { Into::<Shape>::into(self).into() }
}
impl Into<Op> for Points {
    fn into(self) -> Op { Into::<Shape>::into(self).into() }
}
impl Into<Op> for Text {
    fn into(self) -> Op { Into::<Shape>::into(self).into() }
}

// rest

impl Into<Op> for FontCharacteristics {
    fn into(self) -> Op { Op::SetFontCharacteristics(self) }
}
