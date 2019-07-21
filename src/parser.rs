//! A wrapper for comrak's parsing options
//!
//! This module exists to separate the implementation of comrak from our own code.

use comrak::{nodes::AstNode, parse_document, Arena, ComrakOptions};

/// Parse the contents of a document to a tree
///
/// Return the root node of the tree. You must construct the arena that nodes will be allocated to
/// and pass them to this method.
pub fn parse<'a>(arena: &'a Arena<AstNode<'a>>, contents: &str) -> &'a AstNode<'a> {
    let options = ComrakOptions {
        smart: true,
        width: 79,
        ext_strikethrough: true,
        ext_tagfilter: true,
        ext_autolink: true,
        ext_tasklist: true,
        ext_superscript: true,
        ..ComrakOptions::default()
    };
    parse_document(arena, contents, &options)
}
