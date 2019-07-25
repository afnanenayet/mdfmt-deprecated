mod config;

#[macro_use]
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
    let mut fmt = Formatter::new(config);
    let formatted_doc = fmt.format_md(&root);

    // TODO(afnan) remove
    debugln!("[START DOCUMENT]");
    print!("{}", formatted_doc);
    debugln!("[END DOCUMENT]");
    Ok(())
}
