#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Shape {
    Ellipse(Ellipse),
    Points(Points),
    Text(Text),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Ellipse {
    pub filled: bool,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
impl Into<Shape> for Ellipse {
    fn into(self) -> Shape {
        Shape::Ellipse(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PointsType {
    Polygon,
    Polyline,
    BSpline,
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Points {
    pub filled: bool,
    pub typ: PointsType,
    pub points: Vec<(f32, f32)>,
}
impl Into<Shape> for Points {
    fn into(self) -> Shape {
        Shape::Points(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextAlign {
    Left,
    Center,
    Right,
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Text {
    pub x: f32,
    pub y: f32,
    pub align: TextAlign,
    pub width: f32,
    pub text: String,
}
impl Into<Shape> for Text {
    fn into(self) -> Shape {
        Shape::Text(self)
    }
}
