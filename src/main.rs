mod config;
mod parser;

use comrak::nodes::{AstNode, NodeValue};
use comrak::Arena;
use comrak::{format_commonmark, ComrakOptions};
use config::Opt;
use failure::Error;
use parser::{iter_nodes, parse};
use std::fs;
use std::io;
use std::str;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    opt.valid()?;
    let contents = fs::read_to_string(opt.input_file)?;
    let arena = Arena::new();
    let root = parse(&arena, &contents);
    format_commonmark(&root, &ComrakOptions::default(), &mut io::stdout())?;
    Ok(())
}
