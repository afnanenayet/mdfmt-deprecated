mod config;
mod formatter;
mod parser;

use comrak::Arena;
use config::{Config, Opt};
use failure::Error;
use formatter::Formatter;
use parser::parse;
use std::{fs, rc::Rc};
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    // Check whether the CLI arguments are valid inputs
    opt.valid()?;
    let contents = fs::read_to_string(opt.clone().input_file)?;
    let arena = Arena::new();
    let root = parse(&arena, &contents);
    let config = Rc::new(Config::from(opt.clone()));

    // TODO(afnan) use some sort of stack setup so we can figure out whatever we need to prepend
    // (be it a list or an indent, etc), and apply the preface to all of the children in a block
    // We can use a stack to keep track of the things we need to prepend, and the preface will be
    // the combination of all of the things in the stack
    let mut fmt = Formatter::new(config);
    let formatted_doc = fmt.format_md(&root);

    // TODO(afnan) remove
    println!("\n\n[START DOCUMENT]\n{}\n[END DOCUMENT]", formatted_doc);
    Ok(())
}
