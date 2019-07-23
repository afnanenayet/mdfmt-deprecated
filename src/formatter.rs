//! The formatting module
//!
//! This module contains all of the logic to create the strings that will be output as a result of
//! the format operation.
//!
//! The general idea is to have each AST type be formatted separately, since they all have
//! different logic.

use crate::config::Config;
use comrak::{arena_tree::NodeEdge, nodes::NodeValue};
use std::convert::TryInto;
use std::{mem::discriminant, rc::Rc, str};

/// A regular ole' stack
type Stack<T> = Vec<T>;

/// A type alias for a reference to a node
type NodeRef<'a> = &'a comrak::arena_tree::Node<'a, std::cell::RefCell<comrak::nodes::Ast>>;

/// A member of the prefix stack
///
/// It contains a reference to a prefix, which level it was added at, and what type of node added
/// it
struct PrefixStackElement {
    /// The variant of the node associated with this prefix
    pub node_value: NodeValue,

    /// The contents of the prefix
    pub prefix: String,

    /// Which depth this prefix was added at
    pub depth: usize,
}

/// Routines to format a markdown file
pub struct Formatter {
    /// The formatting configuration
    ///
    /// This is a reference counted heap variable because multiple Formatters can refer to the same
    /// config file, so it makes sense to have multiple ownership.
    config: Rc<Config>,

    /// An internal stack containing the prefix for a markdown element
    prefix_stack: Stack<PrefixStackElement>,
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
    pub fn format_md(&mut self, root: NodeRef) -> String {
        let mut formatted = String::new();
        let mut depth = 0;
        for edge in root.traverse() {
            let node = match edge {
                // TODO(afnan) if the node is an inline, just dump the text without the prefix, and
                // use the text wrapping routines
                NodeEdge::Start(node) => {
                    println!("[START {}] {:?}", depth, &node.data.borrow().value);
                    depth += 1;
                    // TODO create a method to get a prefix, pass the prefix to the format method
                    // Check to see whether this node allocates a new prefix. If so, add the prefix
                    // to the stack with the metadata so we know when to pop it.
                    let prefix_candidate = self.node_prefix(node, depth);

                    if let Some(prefix) = prefix_candidate.clone() {
                        self.prefix_stack.push(PrefixStackElement {
                            prefix,
                            depth,
                            node_value: node.data.borrow().value.clone(),
                        });
                    }

                    // Get an (`Option`) reference to the prefix so we can pass it around
                    let prefix_opt = match self.prefix_stack.last() {
                        Some(p) => Some(p.prefix.as_str()),
                        None => None,
                    };

                    if let Some(formatted_string) = self.format_node(node, prefix_opt) {
                        formatted.push_str(&formatted_string);
                    }
                    node
                }
                NodeEdge::End(node) => {
                    println!("[END {}] {:?}", depth, &node.data.borrow().value);
                    // FIXME(afnan) try to prevent trailing newlines
                    // only add a suffix if the next node is not trailing, otherwise we will end up
                    // with a bunch of trailing newline characters
                    if let Some(suffix) = node_suffix(node) {
                        formatted.push_str(&suffix);
                    }

                    // Determine whether the prefix stack can be popped by matching the node type
                    // and depth. This is similar to how we would create HTML </end> tags if we
                    // were converting this document to HTML
                    if let Some(last_prefix) = self.prefix_stack.last() {
                        let node_type = discriminant(&node.data.borrow().value);
                        let prefix_type = discriminant(&last_prefix.node_value);

                        if node_type == prefix_type && depth == last_prefix.depth {
                            self.prefix_stack.pop();
                        }
                    }
                    depth -= 1;
                    node
                }
            };
        }
        formatted.trim().to_owned()
    }

    /// Format the contents of a node to text (if applicable)
    ///
    /// This function takes a reference to an AST node and formats a string according the the
    /// formatting configuration options. It takes an optional prefix, which should be the current
    /// prefix on the stack.
    fn format_node(&self, node: NodeRef, prefix: Option<&str>) -> Option<String> {
        match &node.data.borrow().value {
            NodeValue::Link(link) => {
                let url_str = String::from_utf8(link.url.clone()).unwrap();
                let link_text = collect_text(node.first_child().unwrap());
                let formatted_link = format!("[{}]({})", link_text, url_str);
                Some(formatted_link)
            }
            NodeValue::Paragraph => {
                let raw_text = collect_text(node);
                let wrapped = self.wrap_text(prefix, &raw_text);
                Some(wrapped)
            }
            NodeValue::Heading(h) => {
                // This is guaranteed to never panic because there can be at most 6 levels
                let hashtags = "#".repeat(h.level.try_into().unwrap());
                Some(hashtags + " " + &collect_text(node))
            }
            NodeValue::HtmlBlock(html_block) => {
                Some(String::from_utf8(html_block.literal.clone()).unwrap())
            }
            _ => None,
        }
    }

    /// Determine the optional prefix of a node
    ///
    /// Some nodes, such as list elements, will have a prefix for the text which will determine the
    /// text output before the inner text element, as well as the indentation for word wrapping. You
    /// need to supply the `depth` of the node in the syntax tree.
    ///
    /// Most nodes do not have a prefix, which is indicated by the `None` type.
    fn node_prefix(&self, node: NodeRef, depth: usize) -> Option<String> {
        match &node.data.borrow().value {
            NodeValue::Item(_) => {
                // We calculate the depth of a list by subtracting one for the document (which
                // doesn't contribute to the nested depth of a list), and recognize that the depth
                // of the elements inside of a list are doubled because we have a `List` which we
                // don't take text from, and then the `Item`, which we process here
                let indent = " ".repeat(self.indent_offset((depth / 2) - 1));
                let res = format!("{}{} ", indent, self.config.list_delim().to_string());
                Some(res)
            }
            _ => None,
        }
    }

    /// Wrap text according to the config options
    ///
    /// You may supply an optional prefix that will be used to determine the leading indent width
    /// for the whole text block.
    ///
    /// For example "  * " will ensure that for every line after the first line of text, we lead
    /// with four spaces.
    fn wrap_text(&self, prefix: Option<&str>, text: &str) -> String {
        // Because we don't want to split up links, we tokenize links first, and then tokenize the
        // remaining strings by words
        //
        let tokenized: Vec<&str> = text.split(' ').collect();

        // The resulting vector, in which each string is a separate line
        let mut res_vec = Vec::new();

        // We already know the max line width, so we can reserve the memory ahead of time for some
        // performance gains
        let mut current_line = String::with_capacity(*self.config.line_width());

        // Calculate the padding for the text "box" on the left side
        let space_prefix = " ".repeat(prefix.unwrap_or("").len());

        // Push the prefix only onto the first line
        if let Some(p) = prefix {
            current_line.push_str(p);
        }

        // Loop through each word, either pushing to the current line or creating a new line based
        // on whether the word would fit on the current line
        for word in tokenized {
            let space_left = self.config.line_width() - current_line.len();
            if word.len() + 1 > space_left {
                res_vec.push(current_line);
                current_line = String::with_capacity(*self.config.line_width());

                if prefix.is_some() {
                    current_line.push_str(&space_prefix);
                }
            }
            current_line.push_str(word);
            current_line.push_str(" ");
        }

        // Push the last line
        res_vec.push(current_line);
        res_vec.join("\n")
    }
}

/// Recursively the text from a node, if it exists
///
/// This function takes a reference to an existing unicode vector so it can recursively extend
/// the output.
fn collect_text_helper(node: NodeRef, output: &mut Vec<u8>) {
    match node.data.borrow().value {
        // Links should handle their own text so we ignore any text inside of a link node
        NodeValue::Link(_) => (),
        NodeValue::Text(ref literal) | NodeValue::Code(ref literal) => {
            output.extend_from_slice(literal)
        }
        NodeValue::SoftBreak => output.push(b' '),
        NodeValue::LineBreak => output.push(b'\n'),
        _ => {
            for child in node.children() {
                collect_text_helper(child, output);
            }
        }
    };
}

/// Recursively get all of the text from a node
///
/// This is a wrapper for the actual recursive function
fn collect_text(node: NodeRef) -> String {
    let mut unicode: Vec<u8> = Vec::new();
    collect_text_helper(node, &mut unicode);
    String::from_utf8(unicode).unwrap_or_else(|_| "".to_owned())
}

/// Determine the suffix of a node
///
/// This returns the suffix of a node, if it is applicable. This should be used with the `End`
/// variant of a node.
fn node_suffix(node: NodeRef) -> Option<String> {
    let node_variant = &node.data.borrow().value;

    // if we have an element that is ending a direct descendant of the document node and a block,
    // then give it an extra newline
    if let Some(parent) = node.parent() {
        let parent_type = discriminant(&parent.data.borrow().value);
        let document_type = discriminant(&NodeValue::Document);

        // These are high-level blocks that should be separated by newlines
        // We special case lists because they already having an extra newline from their internal
        // `Paragraph` blocks
        if parent_type == document_type && node_variant.block() {
            return match node_variant {
                NodeValue::List(_) => Some("\n".to_owned()),
                _ => Some("\n\n".to_owned()),
            };
        }

        // Otherwise we just want to end the element with a single newline
        return match node_variant {
            NodeValue::List(_) | NodeValue::Paragraph => Some("\n".to_owned()),
            _ => None,
        };
    }

    // This branch only triggers when the `document` node is passed in, which is a sentinel node
    // that doesn't have any information
    None
}
