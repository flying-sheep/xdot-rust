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
use xdot::parse;

mod graph_ext;
mod xdot;

const TEST: &'static str = "graph { a -- b }";

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
    graph.iter_elems().map(handle_elem).collect::<Result<_>>()?;
    Ok(())
}

const ATTR_NAMES: [&'static str; 6] = [
    "_draw_", "_ldraw_", "_hdraw_", "_tdraw_", "_hldraw_", "_tldraw_",
];

fn handle_elem(elem: Elem) -> Result<()> {
    let attributes: &[Attribute] = match elem {
        Elem::Edge(edge) => edge.attributes.as_ref(),
        Elem::Node(node) => node.attributes.as_ref(),
    };
    for attr in attributes.iter() {
        if let Id::Plain(ref attr_name) = attr.0 {
            if !ATTR_NAMES.contains(&attr_name.as_str()) {
                continue;
            }
        };
        if let Id::Escaped(ref attr_val) = attr.1 {
            let shapes = parse(&attr_val).map_err(|e| Report::msg(e.input.to_owned()))?;
            dbg!(attr_val, shapes);
        }
    }
    Ok(())
}
