#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

//! Parse and draw [`xdot`](https://graphviz.org/docs/attr-types/xdot/) shapes.
//!
//! ## Example
//! ```rust
//! use xdot::parse;
//! let shapes = parse("c 7 -#ff0000 p 4 4 4 36 4 36 36 4 36");
//! ```
//!
//! ## Feature flags
#![cfg_attr(all(doc, feature = "document-features"), doc = document_features::document_features!())]

#[cfg(feature = "layout")]
mod layout;
mod xdot;

#[cfg(feature = "layout")]
pub use self::layout::{layout_and_draw, LayoutError};
pub use self::xdot::shapes;
pub use self::xdot::{parse, FontCharacteristics, Pen, Rgba, ShapeDraw, Style};
