use proc_macro2::{Delimiter, Group, TokenTree};
use proc_macro2::{Literal, TokenStream};
use syn::{LitStr, parse_str, parse2};

use crate::parser::RenderType;
use crate::{builder::build_string, parser::TweldDsl};

pub fn scan_tokens(input: TokenStream) -> syn::Result<TokenStream> {
    let mut output = Vec::new();
    let mut tokens = input.into_iter().peekable();

    while let Some(tree) = tokens.next() {
        // println!("parsing");
        match tree {
            // Checking for the '@' hook
            TokenTree::Punct(ref p) if p.as_char() == '@' => {
                // println!("maybe here (the @)? {p:?}");
                if let Some(TokenTree::Group(grp)) = tokens.peek() {
                    let span = grp.span();
                    if grp.delimiter() == Delimiter::Bracket {
                        // We found @[ ... ]! Consume the bracket group.
                        let Some(TokenTree::Group(bracket_group)) = tokens.next() else {
                            return Err(syn::Error::new(
                                span, "There was an error when consuming the bracket group!")
                            );
                        };
                                                
                        let dsl: TweldDsl = parse2(bracket_group.stream())?;                        
                        println!("render {:?}", dsl.render_type);
                        
                        let result = build_string(dsl.parts);

                         match dsl.render_type {
                            RenderType::Identifier => {
                                println!("result: {result}");
                                let result = result.replace(" ", "");
                                let identifier = parse_str::<proc_macro2::Ident>(&result)
                                    .or_else(|_| parse_str::<proc_macro2::Ident>(&format!("r#{result}")))?;
                                
                                output.push(TokenTree::Ident(identifier));
                            },
                            RenderType::StringLiteral => {
                                output.push(TokenTree::Literal(Literal::string(&result)));
                            },
                        }
                            
                        continue;
                    }
                }
                output.push(tree);
            }

            TokenTree::Group(g) => {
                let inner_expanded = scan_tokens(g.stream())?;
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

                let dsl = TweldDsl::parse_lit_str(&lit_str)?;

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
    Ok(TokenStream::from_iter(output))
}
