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
mod xdot_parse;

#[cfg(feature = "layout")]
pub use self::layout::{draw_graph, layout_and_draw_graph, LayoutError};
#[cfg(feature = "pyo3")]
use self::xdot_parse::parse_py;
pub use self::xdot_parse::{draw, parse, shapes, ShapeDraw};

/// Known node/edge attribute names holding `xdot` draw instructions that [parse] can handle.
///
/// If the [feature flag](crate#feature-flags) `layout` is active, this is by [draw_graph] when traversing the graph.
pub static ATTR_NAMES: [&str; 6] = [
    "_draw_", "_ldraw_", "_hdraw_", "_tdraw_", "_hldraw_", "_tldraw_",
];

/// Python module TODO
#[cfg(feature = "pyo3")]
#[pyo3::pymodule]
#[pyo3(name = "xdot_rs")]
pub fn pymodule(py: pyo3::Python, m: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
    m.add_class::<ShapeDraw>()?;
    m.add_function(pyo3::wrap_pyfunction!(parse_py, m)?)?;
    let shapes = pyo3::wrap_pymodule!(shapes::pymodule);
    let draw = pyo3::wrap_pymodule!(draw::pymodule);
    let m_dict = py.import("sys")?.getattr("modules")?;
    m.add_wrapped(shapes)?;
    m_dict.set_item("xdot_rs.shapes", shapes(py))?;
    m.add_wrapped(draw)?;
    m_dict.set_item("xdot_rs.draw", draw(py))?;
    Ok(())
}
