## Links

- [GitHub: `wooorm/markdown-rs`][repo]
- [`crates.io`: `markdown`][crate]
- [`docs.rs`: `markdown`][docs]

## When should I use this?

- if you _just_ want to turn markdown into HTML (with maybe a few extensions)
- if you want to do _really complex things_ with markdown

## What is this?

`markdown-rs` is an open source markdown parser written in Rust.
It’s implemented as a state machine (`#![no_std]` + `alloc`) that emits
concrete tokens,
so that every byte is accounted for,
with positional info.
The API then exposes this information as an AST,
which is easier to work with,
or it compiles directly to HTML.
