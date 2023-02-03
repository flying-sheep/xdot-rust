use graphviz_rust::{
    cmd::{CommandArg, Format, Layout},
    dot_structures::{Attribute, Graph, Id},
    printer::PrinterContext,
};
use nom::Finish;

mod graph_ext;

use self::graph_ext::{Elem, GraphExt};
use super::xdot::{ShapeDraw, parse};
use super::XDotError;

pub fn layout_and_draw(graph: Graph) -> Result<Vec<ShapeDraw>, XDotError> {
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
    let graph = graphviz_rust::parse(&layed_out).map_err(XDotError::ParseDot)?;
    let shapes = graph
        .iter_elems()
        .map(handle_elem)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    Ok(shapes)
}

const ATTR_NAMES: [&str; 6] = [
    "_draw_", "_ldraw_", "_hdraw_", "_tdraw_", "_hldraw_", "_tldraw_",
];

fn handle_elem(elem: Elem) -> Result<Vec<ShapeDraw>, nom::error::Error<&str>> {
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

fn dot_unescape(input: &str) -> Result<&str, nom::error::Error<&str>> {
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
