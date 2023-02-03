use color_eyre::{
    self,
    eyre::{Report, Result},
};
use graph_ext::{Elem, GraphExt};
use graphviz_rust as gv;
use gv::{
    cmd::{CommandArg, Format, Layout},
    dot_structures::{Attribute, Id},
    printer::PrinterContext,
};
use xdot::{parse, ShapeDraw};

mod graph_ext;
mod xdot;

const TEST: &str = "graph { a -- b }";

fn main() -> Result<()> {
    color_eyre::install()?;
    let graph = gv::parse(TEST).map_err(Report::msg)?;
    let mut ctx = PrinterContext::default();
    let layed_out = gv::exec(
        graph,
        &mut ctx,
        vec![
            CommandArg::Layout(Layout::Dot),
            CommandArg::Format(Format::Xdot),
        ],
    )?;
    // println!("{}", &layed_out);
    let graph = gv::parse(&layed_out).map_err(Report::msg)?;
    let shapes = graph
        .iter_elems()
        .map(handle_elem)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    for shape in shapes {
        dbg!(shape);
    }
    Ok(())
}

const ATTR_NAMES: [&str; 6] = [
    "_draw_", "_ldraw_", "_hdraw_", "_tdraw_", "_hldraw_", "_tldraw_",
];

fn handle_elem(elem: Elem) -> Result<Vec<ShapeDraw>> {
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
                let mut new = parse(&attr_val).map_err(|e| Report::msg(e.input.to_owned()))?;
                shapes.append(&mut new);
            }
        }
    }
    Ok(shapes)
}

fn dot_unescape(input: &str) -> Result<String> {
    // TODO: dedupe error conversion, throw better error if input is not empty
    let (input, s) = dot_unescape_inner(input).map_err(|e| Report::msg(e.to_owned()))?;
    assert_eq!(input, "");
    Ok(s.to_owned())
}

fn dot_unescape_inner(input: &str) -> nom::IResult<&str, &str> {
    use nom::{
        bytes::complete::{tag, take_while},
        combinator::eof,
        sequence::{delimited, terminated},
    };
    // TODO: actually unescape
    terminated(
        delimited(tag("\""), take_while(|c| c != '\\' && c != '\"'), tag("\"")),
        eof,
    )(input)
}

#[test]
fn test_dot_unescape() {
    assert_eq!(dot_unescape_inner("\"\""), Ok(("", "")));
    assert_eq!(dot_unescape_inner("\"xy\""), Ok(("", "xy")));
    assert!(dot_unescape_inner("\"\"\"").is_err());
    assert!(dot_unescape_inner("\"\\\"").is_err()); // so far no actual escape support
}
