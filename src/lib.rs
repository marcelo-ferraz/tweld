mod models;
mod parser;
mod builder;
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
