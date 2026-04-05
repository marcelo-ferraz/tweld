//! # Tweld
//! [![Crates.io](https://img.shields.io/crates/v/tweld.svg)](https://crates.io/crates/tweld)
//! [![Docs.rs](https://docs.rs/tweld/badge.svg)](https://docs.rs/tweld)
//! [![License](https://img.shields.io/crates/l/tweld.svg)](https://choosealicense.com/licenses/)
//!
//! > *You can read it as tiny-weld, token-weld, or just tweld. The important thing is that it compiles.*
//!  
//! Tweld is a procedural macro toolkit and naming DSL for Rust. It lets you dynamically generate, modify, and compose identifiers directly in your source code using a clean `@[]` syntax — because sometimes the identifier you need doesn't quite exist yet, and writing a full proc-macro just to rename a function feels like bringing a freight train to post a letter.
//! One can only hope the syntax is clean and intuitive enough.
//!
//! ```rust,ignore
//! weld!("## @[(the idea | title)]");
//! ```
//!  
//! The name comes from the idea of fusing tokens together. It started as a tool for writing macros for macros (which sounds recursive, because it is), and then grew somewhat beyond its original remit.
//! See [`weld!`] for full documentation.

mod builder;
mod models;
mod parser;
mod scanner;
mod tests;

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use crate::scanner::scan_tokens;

#[doc = include_str!("../docs/weld.md")]
#[proc_macro]
pub fn weld(input: TokenStream) -> TokenStream {
    scan_tokens(TokenStream2::from(input))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
