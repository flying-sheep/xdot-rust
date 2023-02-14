//! Types reused for drawing things.
use std::str::FromStr;

use bitflags::bitflags;

/// RGBA color representation with 8 bit per channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature= "pyo3", pyo3::pyclass)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Default for Rgba {
    fn default() -> Self {
        Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 0xff,
        }
    }
}

/// Line style for node borders and edges.
/// See [here](https://graphviz.org/docs/attr-types/style/).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature= "pyo3", pyo3::pyclass)]
pub enum Style {
    Dashed,
    Dotted,
    Solid,
    Invis,
    Bold,
    // TODO: "tapered" for edges only
}
impl Default for Style {
    fn default() -> Self {
        Style::Solid
    }
}
impl FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Style::*;
        Ok(match s {
            "dashed" => Dashed,
            "dotted" => Dotted,
            "solid" => Solid,
            "invis" => Invis,
            "bold" => Bold,
            s => return Err(s.to_owned()),
        })
    }
}

bitflags! {
    /// Font weight and decorations.
    /// Matches values defined [here](https://graphviz.org/docs/outputs/canon/#xdot).
    #[derive(Default)]
    #[cfg_attr(feature= "pyo3", pyo3::pyclass)]
    pub struct FontCharacteristics: u128 {
        const BOLD           = 0b00000001;
        const ITALIC         = 0b00000010;
        const UNDERLINE      = 0b00000100;
        const SUPERSCRIPT    = 0b00001000;
        const SUBSCRIPT      = 0b00010000;
        const STRIKE_THROUGH = 0b00100000;
        const OVERLINE       = 0b01000000;
    }
}
