# The Directory Time Comparator (dirtcomp)

## Motivation

This utility allows you to compare two globs to determine which describes newer
file(s). Its motivating purpose is the reproduction of file dependencies (a
useful part of the Makefile feature called ["rules"][mrul]) in [Just][]. More
generally, though, this utility can be used in any scripting context on any
system that the [Rust glob crate][glob] supports.

[glob]: https://docs.rs/glob/latest/glob/ 
[Just]: https://just.systems/
[mrul]: https://www.gnu.org/software/make/manual/make.html#Rules 

## Installation

Cross-platform builds are not yet configured. That will come. Until then, you
can install by using `cargo install dirtcomp`. If you prefer, you can also clone
the repository and build it using `cargo build --release` or by running `just
install [path]`. See the [justfile][] for more options.

[justfile]: ./justfile

## Usage

There are two subcommands currently defined:

1. `check` accepts two globs as positional arguments, `<BASE>` and `<TARGET>`.
2. `multi-check` accepts repeated arguments, `--base <GLOB>` and
   `--target <GLOB>`.

In both cases, if the base globs describe newer files, a successful Unix status
code (0) is returned. If the targets are newer, an "error" status code 1 is
returned. Other status codes represent genuinely erroneous states.

A reasonable example, from one of my other projects, is:

```sh
#!/bin/sh
if $(dirtcomp check 'src/**/*.ts' 'dist/**/*'); then
    pnpm tsc
else
    echo "Nothing to do; skipping compilation."
fi
```
