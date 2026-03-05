use proc_macro2::TokenTree;
use syn::{Ident, LitInt, LitStr, Token, parenthesized, token};
use syn::parse::{Parse, ParseStream};

use crate::models::{Modifier, TokenPart};


pub struct NamingDsl {
    pub parts: Vec<TokenPart>
}

impl Parse for NamingDsl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut parts = Vec::new();
        while !input.is_empty() {
            if !input.peek(syn::token::Paren) {
                let tt: TokenTree = input.parse()?;
                parts.push(TokenPart::Plain(tt.to_string()));
            }

            let mod_content;
            parenthesized!(mod_content in input);
            let target = mod_content.parse::<Ident>()?.to_string();
            let mut modifiers = Vec::new();

            while mod_content.peek(Token![|]) {
                mod_content.parse::<Token![|]>()?;
                if mod_content.is_empty() { break; }
                
                let mod_name: Ident = mod_content.parse()?;
                match mod_name.to_string().to_lowercase().as_str() {
                    "singular" => modifiers.push(Modifier::Singular),
                    "plural" => modifiers.push(Modifier::Plural),
                    "lower" | "lowercase" => modifiers.push(Modifier::Lowercase),
                    "pascal" | "pascalcase" | "uppercamelcase" => modifiers.push(Modifier::PascalCase),
                    "lowercamelcase" | "camelcase" | "camel" => modifiers.push(Modifier::LowerCamelCase),
                    "snakecase" | "snake" | "snekcase" | "snek" => modifiers.push(Modifier::SnakeCase),
                    "kebabcase" | "kebab" => modifiers.push(Modifier::KebabCase),
                    "shoutysnakecase" | "shoutysnake"  | "shoutysnekcase"  | "shoutysnek" => modifiers.push(Modifier::ShoutySnakeCase),
                    "titlecase" | "title" => modifiers.push(Modifier::TitleCase),
                    "shoutykebabcase" | "shoutykebab" => modifiers.push(Modifier::ShoutyKebabCase),
                    "traincase" | "train" => modifiers.push(Modifier::TrainCase),
                    "uppercase" | "upper" => modifiers.push(Modifier::Uppercase),
                    "replace" => {
                        let args;
                        syn::braced!(args in mod_content);
                        let from = args.parse::<LitStr>()?;
                        args.parse::<Token![,]>()?;
                        let to = args.parse::<LitStr>()?;
                        modifiers.push(Modifier::Replace(from.value(), to.value()));
                    }
                    "substr" | "substring" => {
                        let args;
                        syn::braced!(args in mod_content);
                        let from = args
                            .parse::<LitInt>()
                            .and_then(| val | val.base10_parse::<usize>())
                            .ok();

                        args.parse::<Token![,]>()?;
                        
                        let to = args
                            .parse::<LitInt>()
                            .and_then(| val | val.base10_parse::<usize>())
                            .ok();
                        
                        modifiers.push(Modifier::Substr(from, to));
                    }
                    _ => return Err(syn::Error::new(mod_name.span(), format!("Unknown modifier {:?}", mod_name.span()))),
                }
            }
            parts.push(TokenPart::Modified(target, modifiers));
            
        }
        Ok(NamingDsl { parts })
    }
}

