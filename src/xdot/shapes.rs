//! Drawable shapes included in [Shape].

/// A drawable shape including closed shapes, lines, and text.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Ellipse(Ellipse),
    Points(Points),
    Text(Text),
}

/// A horizontal ellipse shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Ellipse {
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

/// Type of shape defined by a sequence of points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointsType {
    Polygon,
    Polyline,
    BSpline,
}
/// Shape defined by a sequence of points (line or closed shape).
#[derive(Debug, Clone, PartialEq)]
pub struct Points {
    pub filled: bool,
    pub typ: PointsType,
    pub points: Vec<(f32, f32)>,
}
impl From<Points> for Shape {
    fn from(val: Points) -> Self {
        Shape::Points(val)
    }
}

/// Horizontal text alignment: left, center, or right.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}
/// Multiline text for node or edge labels.
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
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

/// External image, currently unimplemented.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExternalImage;
