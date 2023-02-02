#[derive(Debug, Clone, PartialEq)]
pub(super) enum Shape {
    Ellipse(Ellipse),
    Points(Points),
    Text(Text),
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Ellipse {
    filled: bool, x: f32, y: f32, w: f32, h: f32
}
impl Into<Shape> for Ellipse {
    fn into(self) -> Shape { Shape::Ellipse(self) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PointsType { Polygon, Polyline, BSpline }
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Points {
    filled: bool, typ: PointsType, points: Vec<(f32, f32)>
}
impl Into<Shape> for Points {
    fn into(self) -> Shape { Shape::Points(self) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TextAlign { Left, Center, Right }
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Text {
    x: f32, y: f32, align: TextAlign, width: f32, text: String
}
impl Into<Shape> for Text {
    fn into(self) -> Shape { Shape::Text(self) }
}
