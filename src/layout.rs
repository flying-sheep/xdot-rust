use graphviz_rust::{
    cmd::{CommandArg, Format, Layout},
    dot_structures::{Attribute, Graph, Id},
    printer::PrinterContext,
};
use nom::{error::Error as NomError, Finish};
use thiserror::Error;

mod graph_ext;

use self::graph_ext::{Elem, GraphExt};
use super::{
    xdot_parse::{parse, ShapeDraw},
    ATTR_NAMES,
};

/// Error wrapping possible errors that can occur when running [draw_graph].
#[derive(Error, Debug)]
pub enum LayoutError {
    #[error("failed to run xdot")]
    Layout(#[from] std::io::Error),
    #[error("failed to parse dot")]
    ParseDot(String),
    #[error("failed to parse xdot attributes")]
    ParseXDot(#[from] NomError<String>),
}
impl From<NomError<&str>> for LayoutError {
    fn from(e: NomError<&str>) -> Self {
        nom2owned(e).into()
    }
}
fn nom2owned(e: NomError<&str>) -> NomError<String> {
    NomError {
        input: e.input.to_owned(),
        code: e.code,
    }
}

/// Run `xdot` layout algorithm on a [Graph](graphviz_rust::dot_structures::Graph) and extract all [ShapeDraw] operations.
pub fn layout_and_draw_graph(graph: Graph) -> Result<Vec<ShapeDraw>, LayoutError> {
    let layed_out = layout_graph(graph)?;
    Ok(draw_graph(layed_out)?)
}

fn layout_graph(graph: Graph) -> Result<Graph, LayoutError> {
    let mut ctx = PrinterContext::default();
    let layed_out = graphviz_rust::exec(
        graph,
        &mut ctx,
        vec![
            CommandArg::Layout(Layout::Dot),
            CommandArg::Format(Format::Xdot),
        ],
    )?;
    // println!("{}", &layed_out);
    graphviz_rust::parse(&layed_out).map_err(LayoutError::ParseDot)
}

/// Extract [ShapeDraw] operations from a graph annotated with `xdot` draw attributes.
pub fn draw_graph(graph: Graph) -> Result<Vec<ShapeDraw>, NomError<String>> {
    Ok(graph
        .iter_elems()
        .map(handle_elem)
        .collect::<Result<Vec<_>, _>>()
        .map_err(nom2owned)?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>())
}

fn handle_elem(elem: Elem) -> Result<Vec<ShapeDraw>, NomError<&str>> {
    let attributes: &[Attribute] = match elem {
        Elem::Edge(edge) => edge.attributes.as_ref(),
        Elem::Node(node) => node.attributes.as_ref(),
    };
    let mut shapes = vec![];
    for attr in attributes.iter() {
        if let Id::Plain(ref attr_name) = attr.0 {
            if !ATTR_NAMES.contains(&attr_name.as_str()) {
                continue;
            }
            if let Id::Escaped(ref attr_val_raw) = attr.1 {
                let attr_val = dot_unescape(attr_val_raw)?;
                dbg!(&attr_name, &attr_val);
                let mut new = parse(attr_val)?;
                shapes.append(&mut new);
            }
        }
    }
    Ok(shapes)
}

fn dot_unescape(input: &str) -> Result<&str, NomError<&str>> {
    use nom::{
        bytes::complete::{tag, take_while},
        combinator::eof,
        sequence::{delimited, terminated},
    };
    // TODO: actually unescape
    let (_, inner) = terminated(
        delimited(tag("\""), take_while(|c| c != '\\' && c != '\"'), tag("\"")),
        eof,
    )(input)
    .finish()?;
    Ok(inner)
}

#[test]
fn test_dot_unescape() {
    assert_eq!(dot_unescape("\"\""), Ok(""));
    assert_eq!(dot_unescape("\"xy\""), Ok("xy"));
    assert!(dot_unescape("\"\"\"").is_err());
    assert!(dot_unescape("\"\\\"").is_err()); // so far no actual escape support
}
