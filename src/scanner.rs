use proc_macro2::{Delimiter, Group, TokenTree};
use proc_macro2::{Literal, TokenStream};
use quote::format_ident;
use syn::{LitStr, parse2};

use crate::{builder::build_string, parser::TweldDsl};

pub fn scan_tokens(input: TokenStream) -> TokenStream {
    let mut output = Vec::new();
    let mut tokens = input.into_iter().peekable();

    while let Some(tree) = tokens.next() {
        // println!("parsing");
        match tree {
            // Checking for the '@' hook
            TokenTree::Punct(ref p) if p.as_char() == '@' => {
                // println!("maybe here (the @)? {p:?}");
                if let Some(TokenTree::Group(grp)) = tokens.peek() {
                    if grp.delimiter() == Delimiter::Bracket {
                        // We found @[ ... ]! Consume the bracket group.
                        let bracket_group = if let TokenTree::Group(grp) = tokens.next().unwrap() {
                            grp
                        } else {
                            unreachable!()
                        };

                        // Process the naming DSL inside the brackets
                        let dsl: TweldDsl =
                            parse2(bracket_group.stream()).expect("Invalid Braze welding DSL");
                        let ident_name = build_string(dsl.parts).replace(" ", "");

                        output.push(TokenTree::Ident(format_ident!("{}", ident_name)));
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

            TokenTree::Literal(lit) => {
                let tokens = quote::quote!(#lit);

                let Ok(lit_str) = parse2::<LitStr>(tokens) else {
                    output.push(TokenTree::Literal(lit));
                    continue;
                };

                let clean_string: String = lit_str.value();

                let dsl = TweldDsl::parse_str(&clean_string)
                    .expect("There was an error when parsing the string");

                println!("parts: {:?}", dsl.parts);
                let result = build_string(dsl.parts);
                println!("result: {result:?}");

                let mut new_lit = TokenTree::Literal(Literal::string(&result));
                new_lit.set_span(lit.span());

                output.push(new_lit);
                continue;
            }

            t => {
                output.push(t);
            }
        }
    }
    TokenStream::from_iter(output)
}
