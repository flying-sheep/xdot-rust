///! Types reused for drawing things.

use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Rgba {
    r: u8, g: u8, b: u8, a: u8
}
impl Default for Rgba {
    fn default() -> Self { Rgba { r: 0, g: 0, b: 0, a: 0xff } }
}

/// See https://graphviz.org/docs/attr-types/style/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Style {
    Dashed,
    Dotted,
    Solid,
    Invis,
    Bold,
    // TODO: "tapered" for edges only
}
impl Default for Style {
    fn default() -> Self { Style::Solid }
}
    
bitflags!{
    /// Matches values in https://graphviz.org/docs/outputs/canon/#xdot
    #[derive(Default)]
    pub(super) struct FontCharacteristics: u128 {
        const BOLD           = 0b00000001;
        const ITALIC         = 0b00000010;
        const UNDERLINE      = 0b00000100;
        const SUPERSCRIPT    = 0b00001000;
        const SUBSCRIPT      = 0b00010000;
        const STRIKE_THROUGH = 0b00100000;
        const OVERLINE       = 0b01000000;
    }
}
