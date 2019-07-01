mod config;
mod formatter;
mod parser;

use comrak::Arena;
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
    let mut stdout = io::stdout();
    iter_nodes(&root, &mut stdout, &formatter::format_node)?;
    Ok(())
}
