//! A wrapper for comrak's parsing options

use comrak::{nodes::AstNode, parse_document, Arena, ComrakOptions};

/// Parse the contents of a document to a tree
///
/// Return the root node of the tree. You must construct the arena that nodes will be allocated to
/// and pass them to this method.
pub fn parse<'a>(arena: &'a Arena<AstNode<'a>>, contents: &str) -> &'a AstNode<'a> {
    let options = ComrakOptions {
        smart: true,
        width: 80,
        ext_strikethrough: true,
        ext_tagfilter: true,
        ext_autolink: true,
        ext_tasklist: true,
        ext_superscript: true,
        ..ComrakOptions::default()
    };
    parse_document(arena, contents, &options)
}

/// Iterate through the nodes of an AST and apply a method to each node.
///
/// Given some root node, this method will iterate through each node and apply some provided
/// function to each node.
pub fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}
