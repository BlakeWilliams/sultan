# Sultan

Sultan is a fuzzy finder written in Rust. The name "Sultan" comes from the very
fuzzy [chicken](sultans) breed of the same name.

[sultans]: https://en.wikipedia.org/wiki/Sultan_chicken

## Installation

To install Sultan, clone this repo and run `cargo build --release` and then move
`./target/release/sultan` to somewhere in your `$PATH`.

## Usage

Sultan reads from `stdin`, and then outputs the selected value to `stdout`. This
makes it great for use with piping and subcommands.

eg: `vim $(ls | sultan)` to fuzzy find a file and open it in vim.

## TODO

[ ] Add highlighting to results.
[ ] Show match and total counts.
[ ] Asynchronously read stdin.
[ ] Add pretty colors.
