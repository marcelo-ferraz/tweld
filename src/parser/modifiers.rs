use std::str::FromStr;

use syn::{Ident, Lit, LitChar, LitInt, LitStr, Token};

use crate::models::{Output, Modifier};

fn parse_lit<T>(args: &syn::parse::ParseBuffer<'_>) -> syn::Result<T>
where T: FromStr, 
    <T as FromStr>::Err: std::fmt::Display {
    let result = args
        .parse::<LitInt>()
        .and_then(|val| val.base10_parse::<T>());
    result
}

fn parse_splice(input: &syn::parse::ParseBuffer<'_>, output_set: Option<Output>) -> syn::Result<Modifier> {
    let mut start = None;
    let mut end = None;
    let mut insert = None;

    let args;
    syn::braced!(args in input);
    println!("splice");
    
    let output_type;
    
    match output_set {
        Some(val) => output_type = val,
        None => {
            match args.parse::<syn::Ident>() {
                Err(_) => output_type = Output::Value,
                Ok(val) => {
                    match val.to_string().to_lowercase().as_str() {
                        "into" | "value" | "val" => output_type = Output::Value,
                        "out" | "rm" | "removed" => output_type = Output::Removed,
                        v => {
                            return Err(syn::Error::new(
                                val.span(), 
                                format!("Splice output invalid \"{v}\"")
                            ))
                        }
                    }
                },                    
            };
            
            if !args.peek(Token![,]) { 
                return Ok(Modifier::Splice(output_type, start, end, insert));
            }
            args.parse::<Token![,]>()?;
        },        
    }
       
    start = args
        .parse::<LitInt>()
        .and_then(|val| val.base10_parse::<i32>())
        .ok();

    if !args.peek(Token![,]) { 
        return Ok(Modifier::Splice(output_type, start, end, insert));
    }
    
    args.parse::<Token![,]>()?;
    end = args
        .parse::<LitInt>()
        .and_then(|val| val.base10_parse::<i32>())
        .ok();
    
    if !args.peek(Token![,]) { 
        return Ok(Modifier::Splice(output_type, start, end, insert));
    }

    args.parse::<Token![,]>()?;
    insert = parse_lit_str_char(&args).ok();     
    
    Ok(Modifier::Splice(output_type, start, end, insert))  
}


fn parse_lit_str_char(args: &syn::parse::ParseBuffer<'_>) -> syn::Result<String> {
    let result = if args.peek(LitStr) {
        args.parse::<LitStr>()?.value()
    } else {
        args.parse::<LitChar>()?.value().to_string()
    };
    Ok(result)
}

fn parse_pad_args(input: &syn::parse::ParseBuffer<'_>) -> Result<(usize, String), syn::Error> {
    let args;
    syn::braced!(args in input);
    let size = parse_lit(&args)?;
    args.parse::<Token![,]>()?;
    let pad = parse_lit_str_char(&args)?;
    Ok((size, pad))
}

pub(crate) fn parse_modifiers(input: &syn::parse::ParseBuffer<'_>) -> syn::Result<Vec<Modifier>> {
    let mut modifiers = Vec::new();
    
    while input.peek(Token![|]) {
        input.parse::<Token![|]>()?;
        if input.is_empty() {
            break;
        }

        let mod_name: Ident = input.parse()?;
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
                syn::braced!(args in input);
                let from = parse_lit_str_char(&args)?;                
                args.parse::<Token![,]>()?;
                let to = parse_lit_str_char(&args)?;
                modifiers.push(Modifier::Replace(from, to));
            }
            "substr" | "substring" => {
                let args;
                syn::braced!(args in input);
                let from = parse_lit(&args).ok();
                args.parse::<Token![,]>()?;
                let to = parse_lit(&args).ok();

                modifiers.push(Modifier::Substr(from, to));
            }
            "reverse" | "rev" => modifiers.push(Modifier::Reverse),
            "repeat" | "rep" | "times" => {
                let args;
                syn::braced!(args in input); 
                let times = args
                    .parse::<LitInt>()
                    .and_then(|val| val.base10_parse::<usize>())?;

                modifiers.push(Modifier::Repeat(times));
            },
            "splitat" => {
                let args;
                syn::braced!(args in input);
                let mid = parse_lit(&args)?;
                modifiers.push(Modifier::SplitAt(mid));
                
            },
            "each" => {
                if !input.peek(syn::token::Brace) {
                    modifiers.push(Modifier::Split(" ".to_owned()));
                    continue;
                }

                let args;
                syn::braced!(args in input);
                let sep = parse_lit_str_char(&args)?;
                modifiers.push(Modifier::Split(sep.to_owned()));
            },
            "split" => {
                let args;
                syn::braced!(args in input);
                
                let lit: Lit = args.parse()?;
        
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
                        format!("Expected a string, char, or integer literal {:?}", mod_name.span()),
                    ))
                }                                                
            },
            "join" => {
                if input.peek(Token![|]) {
                    modifiers.push(Modifier::Join("".to_string())); 
                    continue;   
                }

                let args;
                syn::braced!(args in input);
                let sep = parse_lit_str_char(&args)?;                        
                modifiers.push(Modifier::Join(sep));
            },
            "padstart" | "padleft" | "padl" => {
                let (size, pad) = parse_pad_args(input)?;

                modifiers.push(Modifier::PadStart(size, pad));
            },
            "padend" | "padright" | "padr" => {
                let (size, pad) = parse_pad_args(input)?;

                modifiers.push(Modifier::PadEnd(size, pad));
            },
            "slice" => {
                let args;
                syn::braced!(args in input);
                let start = parse_lit(&args).ok();

                let mut end = None;
                if let Ok(_) = args.parse::<Token![,]>() {
                    end = parse_lit(&args).ok();
                }

                modifiers.push(Modifier::Slice(start, end));
            },
            "spliceout" | "splice_out" => {
                let modifier = parse_splice(input, Some(Output::Removed))?;
                modifiers.push(modifier);
                println!("spliceout");
            },
            "spliceinto" | "splice_into" => {
                let modifier = parse_splice(input, Some(Output::Value))?;
                modifiers.push(modifier);
                println!("splicein");
            },
            "splice" => {
                let modifier = parse_splice(input, None)?;                
                modifiers.push(modifier);
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

