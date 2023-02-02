use nom::error::Error as NomError;
use std::cell::RefCell;

use self::declarative::Pen;
pub(super) use self::declarative::ShapeDraw;

mod attrs;
mod declarative;
mod op_parser;
mod ops;
mod shapes;

#[derive(Debug, Default)]
struct Parser {
    pen: Pen,
}

impl Parser {
    fn parse(&mut self, input: &str) -> Result<Vec<ShapeDraw>, NomError<&str>> {
        let ops = op_parser::parse(input)?;
        let this = RefCell::new(self);
        /*for op in ops {

        }*/
        todo!()
    }
}

pub(super) fn parse(input: &str) -> Result<Vec<ShapeDraw>, NomError<&str>> {
    // Parser::default().parse(input)
    todo!()
}
