# mdfmt

[![Build Status](https://dev.azure.com/afnanenayet/mdformat/_apis/build/status/afnanenayet.mdformat?branchName=master)](https://dev.azure.com/afnanenayet/mdformat/_build/latest?definitionId=5&branchName=master)

## Synopsis

This is a Markdown formatting tool that takes valid markdown files and outputs
well formatted Markdown files. It uses the
[comrak](https://github.com/kivikakk/comrak) parsing library to create an AST
from your markdown file, then formats as necessary.

This formatter is alpha stage software and not all markdown elements properly
work with the formatter.

## Configuration

There are several options you can configure when using this tool.

Options:

* Max line width (`line-width`): The maximum allowed line width for the output
  file. Defaults to 80.
* List delimiter (`list-delim`): The symbol used to denote a list (either `-`
  or `*`). Defaults to `*`.
* Indent width (`indent-width`): How many spaces to use for indentation.
  Defaults to 4.

## Development

This app is tested against Rust stable. You can build it using Cargo.
