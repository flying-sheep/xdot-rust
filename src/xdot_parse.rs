//! `xdot` draw attribute parser without the graph related parts.

use nom::error::Error as NomError;

pub mod draw;
mod op_parser;
mod ops;
pub mod shapes;

pub use self::draw::Pen;
use self::shapes::{PyShape, Shape};

/// A [Shape] together with a [Pen].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct ShapeDraw {
    pub pen: Pen,
    pub shape: Shape,
}
#[cfg(feature = "pyo3")]
#[pyo3::pymethods]
impl ShapeDraw {
    #[getter]
    fn get_shape(&self) -> PyShape {
        PyShape(self.shape.clone())
    }
}

/// Parse an `xdot` draw attribute (as defined [here](https://graphviz.org/docs/outputs/canon/#xdot)).
/// Returns a vector of stateless drawing operations defining shape and style of the drawn node, edge, or label.
pub fn parse(input: &str) -> Result<Vec<ShapeDraw>, NomError<&str>> {
    use ops::Op::*;
    let mut pen = Pen::default();
    let mut shape_draws = vec![];
    for op in op_parser::parse(input)? {
        match op {
            DrawShape(shape) => shape_draws.push(ShapeDraw {
                pen: pen.clone(),
                shape,
            }),
            SetFontCharacteristics(fc) => pen.font_characteristics = fc,
            SetFillColor(color) => pen.fill_color = color,
            SetPenColor(color) => pen.color = color,
            SetFont { size, name } => {
                pen.font_size = size;
                pen.font_name = name;
            }
            SetStyle(style) => pen.line_style = style,
            ExternalImage(_) => todo!("conversion of external image op"),
        }
    }
    Ok(shape_draws)
}

#[cfg(feature = "pyo3")]
#[pyo3::pyfunction]
#[pyo3(name = "parse")]
pub fn parse_py(input: &str) -> pyo3::PyResult<Vec<ShapeDraw>> {
    use pyo3::{exceptions::PyValueError, PyErr};

    parse(input).map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
}
