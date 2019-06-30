//! The configuration details for `mdformat`. This class defines the configuration options for
//! serialization and what is acceptable from the command line.

use failure::Fail;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use structopt::StructOpt;

/// Format markdown files
#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Opt {
    /// Whether the file should be modified in place (this is a potentially destructive change)
    #[structopt(short = "i", long = "in-place")]
    pub in_place: bool,

    /// The input file to format
    #[structopt(parse(from_os_str))]
    pub input_file: PathBuf,

    /// A configuration file specifying the options to use when formatting the markdown file. Any
    /// command line options will override the options from the config file.
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    pub config_file: Option<PathBuf>,
}

impl Opt {
    /// Return whether the presented command line parameters are valid
    pub fn valid(&self) -> Result<(), OptError> {
        if !self.input_file.is_file() {
            return Err(OptError::InvalidFile {
                filename: self.input_file.to_path_buf(),
                parameter: "input_file".to_string(),
            });
        }

        if let Some(config) = &self.config_file {
            if !config.is_file() {
                return Err(OptError::InvalidFile {
                    filename: config.to_path_buf(),
                    parameter: "config_file".to_string(),
                });
            }
        }
        Ok(())
    }
}

/// Validation errors for command line arguments
///
/// This struct represents errors that can arise from validation
#[derive(Debug, Fail)]
pub enum OptError {
    #[fail(display = "Invalid file: {:#?} for {}", filename, parameter)]
    InvalidFile {
        filename: PathBuf,
        parameter: String,
    },
}

/// The struct representing the configuration options for the app.
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// The max line width for the output file.
    #[serde(rename = "line-width")]
    line_width: usize,

    /// The indent width to use for the output file. This must be less than the line width.
    #[serde(rename = "indent-width")]
    indent_width: u32,

    /// The symbol to use to denote lists. This can either be `-` or `*`.
    #[serde(rename = "list-delim")]
    list_delim: ListDelimiter,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            line_width: 80,
            indent_width: 4,
            list_delim: ListDelimiter::Asterisk,
        }
    }
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
