use proc_macro2::TokenStream;
use proc_macro2::{TokenTree, Delimiter, Group};
use quote::format_ident;
use syn::parse2;

use crate::{builder::build_string, parser::NamingDsl};

pub fn scan_tokens(input: TokenStream) -> TokenStream {
    let mut output = Vec::new();
    let mut tokens = input.into_iter().peekable();

    while let Some(tree) = tokens.next() {
        match tree {
            // Checking for the '@' hook
            TokenTree::Punct(ref p) if p.as_char() == '@' => {
                if let Some(TokenTree::Group(g)) = tokens.peek() {
                    if g.delimiter() == Delimiter::Bracket {
                        // We found @[ ... ]! Consume the bracket group.
                        let bracket_group = if let TokenTree::Group(g) = tokens.next().unwrap() { g } else { unreachable!() };
                        
                        // Process the naming DSL inside the brackets
                        let dsl: NamingDsl = parse2(bracket_group.stream()).expect("Invalid Naming DSL");
                        let generated_name = build_string(dsl);
                        
                        output.push(TokenTree::Ident(format_ident!("{}", generated_name)));
                        continue;
                    }
                }
                output.push(tree);
            }
            
            TokenTree::Group(g) => {
                let inner_expanded = scan_tokens(g.stream());
                let mut new_group = Group::new(g.delimiter(), inner_expanded);
                new_group.set_span(g.span());
                output.push(TokenTree::Group(new_group));
            }
            _ => output.push(tree),
        }
    }
    TokenStream::from_iter(output)
}