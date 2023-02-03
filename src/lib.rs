#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

#[cfg(feature = "layout")]
mod layout;
mod xdot;

pub use self::xdot::{
    parse, Ellipse, ExternalImage, FontCharacteristics, Pen, Points, PointsType, Rgba, Shape,
    ShapeDraw, Style, Text, TextAlign,
};

#[cfg(feature = "layout")]
pub use self::layout::{layout_and_draw, LayoutError};
