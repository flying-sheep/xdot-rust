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
impl From<Ellipse> for Shape {
    fn from(val: Ellipse) -> Self {
        Shape::Ellipse(val)
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
impl From<Points> for Shape {
    fn from(val: Points) -> Self {
        Shape::Points(val)
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
impl From<Text> for Shape {
    fn from(val: Text) -> Self {
        Shape::Text(val)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ExternalImage;
