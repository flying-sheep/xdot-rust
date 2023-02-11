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
pub use self::layout::{draw_graph, layout_and_draw_graph, LayoutError};
pub use self::xdot::{draw, parse, shapes, ShapeDraw};

/// Known node/edge attribute names holding `xdot` draw instructions that [parse] can handle.
///
/// If the [feature flag](crate#feature-flags) `layout` is active, this is by [draw_graph] when traversing the graph.
pub static ATTR_NAMES: [&str; 6] = [
    "_draw_", "_ldraw_", "_hdraw_", "_tdraw_", "_hldraw_", "_tldraw_",
];


/// Python module TODO
#[cfg_attr(feature= "pyo3", pyo3::pymodule)]
fn rust(_py: pyo3::Python, m: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
    m.add_class::<ShapeDraw>()?;
    // TODO
    Ok(())
}
