//! The formatting module
//!
//! This module contains all of the logic to create the strings that will be output as a result of
//! the format operation.
//!
//! The general idea is to have each AST type be formatted separately, since they all have
//! different logic.

use crate::config::Config;
use comrak::{
    arena_tree::{NodeEdge, Traverse},
    nodes::{AstNode, NodeValue},
};
use std::rc::Rc;

/// A regular ole' stack
type Stack<T> = Vec<T>;

/// A type alias for a reference to a node
type NodeRef<'a> = &'a comrak::arena_tree::Node<'a, std::cell::RefCell<comrak::nodes::Ast>>;

/// Routines to format a markdown file
pub struct Formatter {
    /// The formatting configuration
    ///
    /// This is a reference counted heap variable because multiple Formatters can refer to the same
    /// config file, so it makes sense to have multiple ownership.
    config: Rc<Config>,

    /// An internal stack containing the prefix for a markdown element
    prefix_stack: Stack<String>,
}

impl Formatter {
    /// Create a new `Formatter`, given a formatting config
    pub fn new(config: Rc<Config>) -> Self {
        Self {
            config,
            prefix_stack: Vec::new(),
        }
    }

    /// Get the number of spaces to indent by
    fn indent_offset(&self, indents: usize) -> usize {
        self.config.indent_width() * indents
    }

    /// Format a markdown document from the AST
    ///
    /// This method requires the root node of a markdown file. This will also function on a subset
    /// of the AST (we don't need the *actual* root).
    pub fn format_md<'a>(
        &self,
        root: NodeRef,
    ) -> String {
        let mut formatted = String::new();
        for edge in root.traverse() {
            match edge {
                NodeEdge::Start(node) => formatted.push_str("[start]\n"),
                NodeEdge::End(node) => formatted.push_str("[end]\n"),
            }
        }
        formatted
    }

    /// Determine the prefix for a given node
    ///
    /// Given the depth and type of a node, figure out the proper prefix for the corresponding
    /// markdown element. This function is essentially a big lookup table that creates a mapping
    /// between various markdown elements and their corresponding prefixes
    fn node_prefix<'a> (&self, node: NodeRef) -> String {

        unimplemented!();
    }
}
