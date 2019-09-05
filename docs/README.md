# mdfmt

[![Build Status](https://dev.azure.com/afnanenayet/mdformat/_apis/build/status/afnanenayet.mdformat?branchName=master)](https://dev.azure.com/afnanenayet/mdformat/_build/latest?definitionId=5&branchName=master)

## Synopsis

This is a Markdown formatting tool that takes valid markdown files and outputs
well formatted Markdown files. It uses the
[comrak](https://github.com/kivikakk/comrak) parsing library to create an AST
from your markdown file, then formats as necessary.

Example input:

```
# test

And on the first day, God created Markdown files and let us run wild. People wrote markdown files with no regard for line widths and whatnot.

* Even with lists, people write toooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo much
    * Let's add some depth to this list

And why not a page break?

---
* second list
[link](example.com)

I hate indented code blocks by the way, or really anything that depends on whitespace.

    #include "stdio.h"

    int main(void) {
        return 0;
    }
```

Output:

    # test

    And on the first day, God created Markdown files and let us run wild. People
    wrote markdown files with no regard for line widths and whatnot.

    * Even with lists, people write
      toooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo
      much
        * Let’s add some depth to this list

    And why not a page break?

    ---

    * second list [link](example.com)

    I hate indented code blocks by the way, or really anything that depends on
    whitespace.

    ```
    #include "stdio.h"

    int main(void) {
        return 0;
    }

    ```

_NOTE: This formatter is alpha stage software and not all markdown elements properly
work with the formatter._

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
