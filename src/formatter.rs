//! The formatting module
//!
//! This module contains all of the logic to create the strings that will be output as a result of
//! the format operation.
//!
//! The general idea is to have each AST type be formatted separately, since they all have
//! different logic.

use comrak::nodes::{AstNode, NodeValue};
use failure::Error;
use std::io::Write;
use std::str;

/// Dispatch the proper routine for an `AstNode`
pub fn format_node(node: &AstNode, w: &mut Write) -> Result<(), Error> {
    match node.data.borrow().value {
        NodeValue::Paragraph => {
            if let Ok(s) = str::from_utf8(&node.data.borrow().content) {
                // Replace the newlines with spaces
                let stripped: String = s.replace("\n", " ");
                write!(w, "{}\n\n", stripped)?;
            }
        }
        _ => (),
    }
    Ok(())
}
