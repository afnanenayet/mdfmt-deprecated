//! The configuration details for `mdformat`. This class defines the configuration options for
//! serialization and what is acceptable from the command line.

use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use structopt::StructOpt;

/// CLI parameters
///
/// These are the CLI parameters that will be taken from the command line
#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Opt {
    /// Whether the file should be modified in place (this is a potentially destructive change)
    #[structopt(short = "i", long = "in-place")]
    in_place: bool,

    /// The input file to format
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
}

/// The struct representing the configuration options for the app.
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// The max line width for the output file.
    #[serde(rename = "line-width")]
    line_width: u32,

    /// The indent width to use for the output file. This must be less than the line width.
    #[serde(rename = "indent-width")]
    indent_width: u32,

    /// The symbol to use to denote lists. This can either be `-` or `*`.
    #[serde(rename = "list-delim")]
    list_delim: ListDelimiter,
}

/// The valid symbols that can denote a markdown list.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ListDelimiter {
    /// The "*" symbol
    Asterisk,

    /// The "-" symbol
    Dash,
}

impl fmt::Display for ListDelimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ListDelimiter::Dash => write!(f, "-")?,
            ListDelimiter::Asterisk => write!(f, "*")?,
        };
        Ok(())
    }
}
