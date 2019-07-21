mod config;
mod formatter;
mod parser;

use comrak::{
    arena_tree::{NodeEdge, Traverse},
    Arena,
};
use config::Opt;
use failure::Error;
use parser::{iter_nodes, parse};
use std::fs;
use std::io;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    opt.valid()?;
    let contents = fs::read_to_string(opt.input_file)?;
    let arena = Arena::new();
    let root = parse(&arena, &contents);

    // TODO(afnan) use some sort of stack setup so we can figure out whatever we need to prepend
    // (be it a list or an indent, etc), and apply the preface to all of the children in a block
    // We can use a stack to keep track of the things we need to prepend, and the preface will be
    // the combination of all of the things in the stack
    for edge in root.traverse() {
        match edge {
            NodeEdge::Start(node) => println!("[start] {:?}", node.data),
            NodeEdge::End(node) => println!("[end] {:?}", node.data),
        }
    }
    //let mut stdout = io::stdout();
    //iter_nodes(&root, &mut stdout, &formatter::format_node)?;
    Ok(())
}
