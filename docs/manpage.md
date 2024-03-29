% MDFMT(1)
% AFNAN ENAYET
% AUGUST 2019

# NAME

mdfmt - format markdown documents

# SYNOPSIS

**mdfmt** _INPUT_FILE_

**mdfmt \--help**

**mdfmt \--version**

# DESCRIPTION

**mdfmt** is a configurable formatter for markdown documents. It supports
the usage of many extensions, thanks to the *comrak* markdown parsing library
-- it has support for almost all of the GFM extensions. It allows you to
configure some settings, such as line width, what kind of delimiters to use for
lists, emphasis, and so on.

The formatter features opinionated defaults, so if you agree with them, you can
simply use **mdfmt** directly on files without needing to do any
configuration.

There are several locations in which **mdfmt** will look in to try to
resolve the configuration. These are listed in order of precedence. Firstly,
configuration options can be supplied directly from the command line, which are
outlined below. Secondly, you can pass in a flag that directly specifies a
configuration file. Lastly, you can specify a global configuration file that
**mdfmt** will look for automatically on each invocation. **mdfmt**
expects your configuration file to be at
**\$XDG\_CONFIG\_HOME**/mdfmt/config.toml. If **\$XDG\_CONFIG\_HOME** is not set,
this program will use the default value, **\$HOME/.config**.

# GENERAL OPTIONS

**-h**, **\--help**
: Print a helpful error message

**-v**, **\--version**
: Print the version of the program

**-c** [_CONFIG_FILE_], **\--config** [_CONFIG_FILE_]
: Use the supplied configuration file
