use syn::parse::{Parse, ParseStream};
use syn::{Ident, Lit, LitChar, LitInt, LitStr, Token, parenthesized};

use crate::models::{Modifier, TokenParserState, TokenPart};

#[derive(Debug, Clone)]
pub enum RenderType {
    StringLiteral,
    Identifier
}

pub struct TweldDsl {
    pub render_type: RenderType,
    pub parts: Vec<TokenPart>,
}

fn get_modifiers(content: &syn::parse::ParseBuffer<'_>) -> syn::Result<Vec<Modifier>> {
    let mut modifiers = Vec::new();
    
    while content.peek(Token![|]) {
        content.parse::<Token![|]>()?;
        if content.is_empty() {
            break;
        }

        let mod_name: Ident = content.parse()?;
        match mod_name.to_string().to_lowercase().as_str() {
            "singular" => modifiers.push(Modifier::Singular),
            "plural" => modifiers.push(Modifier::Plural),
            "lower" | "lowercase" => modifiers.push(Modifier::Lowercase),
            "upper" | "uppercase" => modifiers.push(Modifier::Uppercase),
            "pascal" | "pascalcase" | "uppercamelcase" => {
                modifiers.push(Modifier::PascalCase)
            }
            "lowercamelcase" | "camelcase" | "camel" => {
                modifiers.push(Modifier::LowerCamelCase)
            }
            "snakecase" | "snake" | "snekcase" | "snek" => {
                modifiers.push(Modifier::SnakeCase)
            }
            "kebabcase" | "kebab" => modifiers.push(Modifier::KebabCase),
            "shoutysnakecase" | "shoutysnake" | "shoutysnekcase" | "shoutysnek" => {
                modifiers.push(Modifier::ShoutySnakeCase)
            }
            "titlecase" | "title" => modifiers.push(Modifier::TitleCase),
            "shoutykebabcase" | "shoutykebab" => modifiers.push(Modifier::ShoutyKebabCase),
            "traincase" | "train" => modifiers.push(Modifier::TrainCase),
            "replace" => {
                let args;
                syn::braced!(args in content);
                let from = args.parse::<LitStr>()?;
                args.parse::<Token![,]>()?;
                let to = args.parse::<LitStr>()?;
                modifiers.push(Modifier::Replace(from.value(), to.value()));
            }
            "substr" | "substring" => {
                let args;
                syn::braced!(args in content);
                let from = args
                    .parse::<LitInt>()
                    .and_then(|val| val.base10_parse::<usize>())
                    .ok();

                args.parse::<Token![,]>()?;

                let to = args
                    .parse::<LitInt>()
                    .and_then(|val| val.base10_parse::<usize>())
                    .ok();

                modifiers.push(Modifier::Substr(from, to));
            }
            "reverse" | "rev" => modifiers.push(Modifier::Reverse),
            "repeat" | "rep" | "times" => {
                let args;
                syn::braced!(args in content); 
                let times = args
                    .parse::<LitInt>()
                    .and_then(|val| val.base10_parse::<usize>())?;

                modifiers.push(Modifier::Repeat(times));
            },
            "split" => {
                let args;
                syn::braced!(args in content);

                let lit: Lit = args.parse()?; // Consume the literal
        
                match lit {
                    Lit::Char(sep) => {
                        modifiers.push(Modifier::Split(sep.value().to_string()));
                    },
                    Lit::Str(sep) => {
                        modifiers.push(Modifier::Split(sep.value()));
                    }
                    Lit::Int(num) => {
                        let mid = num.base10_parse::<usize>()?;
                        modifiers.push(Modifier::SplitAt(mid));
                    }
                    _ =>  return Err(syn::Error::new(
                        mod_name.span(),
                        format!("Expected a string or integer literal {:?}", mod_name.span()),
                    ))
                }                                                
            },
            "join" => {
                let args;
                syn::braced!(args in content);
                let sep = if args.peek(LitStr) {
                    args.parse::<LitStr>()?.value()
                } else {
                    args.parse::<LitChar>()?.value().to_string()
                };                        
                modifiers.push(Modifier::Join(sep));
            },
            "padstart" | "padleft" | "padl" => {
                let args;
                syn::braced!(args in content);
                let size = args
                    .parse::<LitInt>()
                    .and_then(|val| val.base10_parse::<usize>())?;

                args.parse::<Token![,]>()?;

                let pad = if args.peek(LitStr) {
                    args.parse::<LitStr>()?.value()
                } else {
                    args.parse::<LitChar>()?.value().to_string()
                };

                modifiers.push(Modifier::PadStart(size, pad));
            },
            "padend" | "padright" | "padr" => {
                let args;
                syn::braced!(args in content);
                let size = args
                    .parse::<LitInt>()
                    .and_then(|val| val.base10_parse::<usize>())?;

                args.parse::<Token![,]>()?;

                let pad = if args.peek(LitStr) {
                    args.parse::<LitStr>()?.value()
                } else {
                    args.parse::<LitChar>()?.value().to_string()
                };

                modifiers.push(Modifier::PadEnd(size, pad));
            },

            _ => {
                return Err(syn::Error::new(
                    mod_name.span(),
                    format!("Unknown modifier {:?}", mod_name.span()),
                ));
            }
        }
    }
    Ok(modifiers)
}

fn parse_stream(
    input: &syn::parse::ParseBuffer<'_>, 
    mut dsl: TweldDsl,
    state: TokenParserState,     
    mut indent: usize,
) -> syn::Result<TweldDsl> {
    indent += 1; 
    let sp = "-".repeat(indent);
    let mut words: Vec<String> = vec![];    
    while !input.is_empty() {
        println!("{sp}looping: {state:?} {:?}", dsl.parts);
        match state {
            TokenParserState::InsideBrackets => {
                if input.peek(syn::token::Paren) { 
                        println!("{sp}entering group");
                        let group;
                        parenthesized!(group in input);
                        let mut grouped_dsl = TweldDsl {
                            render_type: dsl.render_type.clone(),
                            parts: vec![],
                        }; 
                        grouped_dsl = parse_stream(&group, grouped_dsl, TokenParserState::InsideGroup, indent)?;
                        
                        dsl.parts.push(TokenPart::Grouped(grouped_dsl.parts));
                        if let RenderType::StringLiteral = grouped_dsl.render_type {
                            dsl.render_type = RenderType::StringLiteral;
                        }

                        println!("{sp}leaving group b");
                        continue;
                    }

                if input.peek(Token![|]) {
                    println!("{sp}entering modifiers");
                    dsl = parse_stream(&input, dsl, TokenParserState::Modifiers, indent)?;                    
                    println!("{sp}leaving modifiers");
                    continue;
                }

                if input.peek(Token![-]) {
                    println!("{sp}found token b -");
                    input.parse::<Token![-]>()?;
                    dsl.parts.push(TokenPart::Plain("-".to_string()));
                    continue;
                }

                if input.peek(Token![_]) {
                    println!("{sp}found token b _");
                    input.parse::<Token![_]>()?;
                    dsl.parts.push(TokenPart::Plain("_".to_string()));
                    continue;
                }
            
                if input.peek(syn::Ident) {
                    let result = input
                        .parse::<Ident>()?
                        .to_string();
                    println!("{sp}save ident b: `{result}`");
                    dsl.parts.push(TokenPart::Plain(result));
                    continue;
                }

                if input.peek(syn::LitStr) {
                    let result = input
                        .parse::<LitStr>()?
                        .value();  
                    println!("{sp}save lit b: `{result}`");
                    dsl.render_type = RenderType::StringLiteral;
                    dsl.parts.push(TokenPart::Plain(result));
                    continue;                      
                }

                if input.peek(syn::LitChar) {
                    let result = input
                        .parse::<LitChar>()?
                        .value()
                        .to_string();  
                    println!("{sp}save lit b: `{result}`");
                    dsl.render_type = RenderType::StringLiteral;
                    dsl.parts.push(TokenPart::Plain(result));
                    continue;                      
                }

                let ignored = input.parse::<proc_macro2::TokenTree>()?;
                println!("{sp}ignored 1 {ignored:?}");
            },
            TokenParserState::InsideGroup => {                    
                while !input.is_empty() {
                    println!("{sp}looping group");

                    if input.peek(syn::token::Paren) { 
                        println!("{sp}entering group");
                        let group;
                        parenthesized!(group in input);
                        let mut grouped_dsl = TweldDsl {
                            render_type: dsl.render_type.clone(),
                            parts: vec![],
                        }; 
                        grouped_dsl = parse_stream(&group, grouped_dsl, TokenParserState::InsideGroup, indent)?;
                        
                        dsl.parts.push(TokenPart::Grouped(grouped_dsl.parts));
                        if let RenderType::StringLiteral = grouped_dsl.render_type {
                            dsl.render_type = RenderType::StringLiteral;
                        }

                        println!("{sp}leaving group g {:?}", dsl.parts);
                        continue;
                    }

                    if input.peek(Token![|]) {
                        println!("{sp}entering modifiers {words:?}");

                        if !words.is_empty() {

                            let Some(last) = words.pop() else {
                                todo!();
                            };
                        
                            dsl.parts.push(TokenPart::Plain(words.join("")));
                            words.clear();
                            dsl.parts.push(TokenPart::Plain(last));
                        }

                        dsl = parse_stream(&input, dsl, TokenParserState::Modifiers, indent)?;
                        continue;
                    }

                    if input.peek(Token![-]) {
                        input.parse::<Token![-]>()?;
                        words.push("-".to_string());
                        println!("{sp}found token g -: {words:?}");
                        continue;
                    }

                    if input.peek(Token![_]) {
                        println!("{sp}found token g _");
                        input.parse::<Token![_]>()?;
                        dsl.parts.push(TokenPart::Plain("_".to_string()));
                        continue;
                    }

                    if input.peek(syn::Ident) {
                        let value = input
                            .parse::<Ident>()?
                            .to_string();
                        
                        words.push(value);
                        println!("{sp}acc ident g: {words:?}");
                        continue;
                    }

                    if input.peek(syn::LitChar) {
                        let value = input
                            .parse::<LitChar>()?
                            .value();
                    
                        words.push(value.to_string());
                        println!("{sp}acc litc g: {words:?}");
                        dsl.render_type = RenderType::StringLiteral;
                        continue;
                    }

                    if input.peek(syn::LitStr) {                        
                        let value = input
                            .parse::<LitStr>()?
                            .value();
                    
                        words.push(value);
                        println!("{sp}acc lits g: {words:?}");
                        dsl.render_type = RenderType::StringLiteral;
                        continue;
                    }

                    let ignored = input.parse::<proc_macro2::TokenTree>()?;
                    println!("{sp}ignored 2 {ignored:?}");
                }
            },
            TokenParserState::Modifiers => {
                println!("{sp}getting modifiers");
                
                let Some(target) = dsl.parts.pop() else {
                    return Err(syn::Error::new(
                        input.span(),
                        "Modifiers need a target"
                    ));
                };                
                
                let modifiers = get_modifiers(input)?;
                dsl.parts.push(TokenPart::Modified(Box::new(target), modifiers));
            },
        }
        
        println!("{sp}end - words: `{words:?}`");

        if !words.is_empty() {
            println!("{sp}inside loop push plain");
            dsl.parts.push(TokenPart::Plain(words.join("")));
            words.clear();
        }
    }
    println!("{sp}end2 - word: `{words:?}`");

    if !words.is_empty() {
        println!("{sp}outside loop push plain");
        dsl.parts.push(TokenPart::Plain(words.join("")));
        words.clear();
    }
    println!("{sp}render_as: {:?}", dsl.render_type);

    Ok(dsl)
}


impl Parse for TweldDsl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut dsl = TweldDsl { 
            render_type: RenderType::Identifier, 
            parts: Vec::new(),
        };
        
        // let b = input
        dsl = parse_stream(
            input, 
            dsl, 
            TokenParserState::InsideBrackets,
            0usize
        )?;

        println!("parts: {:?}", dsl.parts);
        Ok(dsl)
    }
}
