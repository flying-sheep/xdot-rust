
use thiserror::Error;

mod xdot;
#[cfg(feature = "layout")]
mod layout;

pub use xdot::{
    parse, Ellipse, ExternalImage, FontCharacteristics, Pen, Points, PointsType, Rgba, Shape,
    ShapeDraw, Style, Text, TextAlign,
};

#[cfg(feature = "layout")]
pub use layout::layout_and_draw;

#[derive(Error, Debug)]
pub enum XDotError {
    #[error("failed to run xdot")]
    Layout(#[from] std::io::Error),
    #[error("failed to parse dot")]
    ParseDot(String),
    #[error("failed to parse xdot attributes")]
    ParseXDot(#[from] nom::error::Error<String>),
}
impl From<nom::error::Error<&str>> for XDotError {
    fn from(e: nom::error::Error<&str>) -> Self {
        nom::error::Error {
            input: e.input.to_owned(),
            code: e.code,
        }
        .into()
    }
}
