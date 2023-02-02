use std::iter::{empty, once};

use graphviz_rust::dot_structures::{Graph, Subgraph, Stmt, Node, Edge};

pub(super) enum Elem<'a> { Node(&'a Node), Edge(&'a Edge) }

pub(super) trait GraphExt {
    fn iter_elems<'a>(&'a self) -> Box<dyn Iterator<Item=Elem<'a>> + 'a>;
}


impl GraphExt for Stmt {
    fn iter_elems<'a>(&'a self) -> Box<dyn Iterator<Item=Elem<'a>> + 'a> {
        match self {
            Stmt::Edge(edge) => Box::new(once(Elem::Edge(edge))),
            Stmt::Node(node) => Box::new(once(Elem::Node(node))),
            Stmt::Subgraph(sg) => sg.iter_elems(),
            _ => Box::new(empty()),
        }
    }
}

impl GraphExt for Subgraph {
    fn iter_elems<'a>(&'a self) -> Box<dyn Iterator<Item=Elem<'a>> + 'a> {
        Box::new(self.stmts.iter().flat_map(Stmt::iter_elems))
    }
}

impl GraphExt for Graph {
    fn iter_elems<'a>(&'a self) -> Box<dyn Iterator<Item=Elem<'a>> + 'a> {
        let stmts = match self {
            Graph::Graph   { stmts, .. } => stmts,
            Graph::DiGraph { stmts, .. } => stmts,
        };
        Box::new(stmts.iter().flat_map(Stmt::iter_elems))
    }
}
