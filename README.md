# Sultan

Sultan is a fuzzy finder written in Rust. The name "Sultan" comes from the very
fuzzy [chicken breed] of the same name.

[chicken breed]: https://en.wikipedia.org/wiki/Sultan_chicken

## Installation

To install Sultan, clone this repo and run `cargo build --release` and then move
`./target/release/sultan` to somewhere in your `$PATH`.

## Usage

Sultan reads from `stdin`, and then outputs the selected value to `stdout`. This
makes it great for use with piping and subcommands.

eg: `vim $(ls | sultan)` to fuzzy find a file and open it in vim.

## TODO

* [x] Add highlighting to results.
* [x] Show match and total counts.
* [ ] Asynchronously read stdin.
* [ ] Add pretty colors.
