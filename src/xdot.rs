use std::cell::RefCell;
use nom::IResult;

use self::declarative::Pen;
pub(super) use self::declarative::ShapeDraw;

mod attrs;
mod shapes;
mod ops;
mod op_parser;
mod declarative;

#[derive(Debug, Default)]
struct Parser {
    pen: Pen,
}

impl Parser {
    fn parse(&mut self, spec: &str) -> IResult<&str, Vec<ShapeDraw>> {
        let this = RefCell::new(self);
        todo!()
    }
}
  
pub(super) fn parse(spec: &str) -> IResult<&str, Vec<ShapeDraw>> {
    // Parser::default().parse(spec)
    todo!()
}
