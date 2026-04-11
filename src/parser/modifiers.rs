use std::str::FromStr;

use syn::{Ident, Lit, LitChar, LitInt, LitStr, Token};

use crate::models::{Modifier, Output};

fn parse_lit_int<T>(args: &syn::parse::ParseBuffer<'_>) -> syn::Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    args.parse::<LitInt>()
        .and_then(|val| val.base10_parse::<T>())
}

fn parse_splice(
    input: &syn::parse::ParseBuffer<'_>,
    output_set: Option<Output>,
) -> syn::Result<Modifier> {
    let mut start = None;
    let mut end = None;
    let mut insert = None;

    let args;
    syn::braced!(args in input);

    let output_type;

    match output_set {
        Some(val) => output_type = val,
        None => {
            match args.parse::<syn::Ident>() {
                Err(_) => output_type = Output::Value,
                Ok(val) => match val.to_string().to_lowercase().as_str() {
                    "into" | "value" | "val" => output_type = Output::Value,
                    "out" | "rm" | "removed" => output_type = Output::Removed,
                    v => {
                        return Err(syn::Error::new(
                            val.span(),
                            format!("Splice output invalid \"{v}\""),
                        ));
                    }
                },
            };

            if !args.peek(Token![,]) {
                return Ok(Modifier::Splice(output_type, start, end, insert));
            }
            args.parse::<Token![,]>()?;
        }
    }

    start = parse_lit_int(&args).ok();

    if !args.peek(Token![,]) {
        return Ok(Modifier::Splice(output_type, start, end, insert));
    }

    args.parse::<Token![,]>()?;
    end = parse_lit_int(&args).ok();

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
    let size = parse_lit_int(&args)?;
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
            "pascal" | "pascalcase" | "uppercamelcase" => modifiers.push(Modifier::PascalCase),
            "lowercamelcase" | "camelcase" | "camel" => modifiers.push(Modifier::LowerCamelCase),
            "snakecase" | "snake" | "snekcase" | "snek" => modifiers.push(Modifier::SnakeCase),
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
                let mut to = "".to_string();
                if args.parse::<Token![,]>().is_ok() {
                    to = parse_lit_str_char(&args)?;
                }
                modifiers.push(Modifier::Replace(from, to));
            }
            "substr" | "substring" => {
                if !input.peek(syn::token::Brace) {
                    modifiers.push(Modifier::Substr(None, None));
                    continue;
                }

                let args;
                syn::braced!(args in input);
                let from = parse_lit_int(&args).ok();
                let mut to = None;
                if args.parse::<Token![,]>().is_ok() {
                    to = parse_lit_int(&args).ok();
                }

                modifiers.push(Modifier::Substr(from, to));
            }
            "reverse" | "rev" => modifiers.push(Modifier::Reverse),
            "repeat" | "rep" | "times" => {
                let args;
                syn::braced!(args in input);
                let times = parse_lit_int(&args)?;

                modifiers.push(Modifier::Repeat(times));
            }
            "splitat" => {
                let args;
                syn::braced!(args in input);
                let mid = parse_lit_int(&args)?;
                modifiers.push(Modifier::SplitAt(mid));
            }
            "split" => {
                let args;
                syn::braced!(args in input);

                let lit: Lit = args.parse()?;

                match lit {
                    Lit::Char(sep) => {
                        modifiers.push(Modifier::Split(sep.value().to_string()));
                    }
                    Lit::Str(sep) => {
                        modifiers.push(Modifier::Split(sep.value()));
                    }
                    Lit::Int(num) => {
                        let mid = num.base10_parse::<usize>()?;
                        modifiers.push(Modifier::SplitAt(mid));
                    }
                    _ => {
                        return Err(syn::Error::new(
                            mod_name.span(),
                            format!(
                                "Expected a string, char, or integer literal {:?}",
                                mod_name.span()
                            ),
                        ));
                    }
                }
            }
            "join" => {
                if !input.peek(syn::token::Brace) {
                    modifiers.push(Modifier::Join("".to_string()));
                    continue;
                }

                let args;
                syn::braced!(args in input);
                let sep = parse_lit_str_char(&args).unwrap_or_default();
                modifiers.push(Modifier::Join(sep));
            }
            "padstart" | "padleft" | "padl" => {
                let (size, pad) = parse_pad_args(input)?;

                modifiers.push(Modifier::PadStart(size, pad));
            }
            "padend" | "padright" | "padr" => {
                let (size, pad) = parse_pad_args(input)?;

                modifiers.push(Modifier::PadEnd(size, pad));
            }
            "slice" => {
                let args;
                syn::braced!(args in input);
                let start = parse_lit_int(&args).ok();

                let mut end = None;
                if args.parse::<Token![,]>().is_ok() {
                    end = parse_lit_int(&args).ok();
                }

                modifiers.push(Modifier::Slice(start, end));
            }
            "spliceout" | "splice_out" => {
                let modifier = parse_splice(input, Some(Output::Removed))?;
                modifiers.push(modifier);
            }
            "spliceinto" | "splice_into" => {
                let modifier = parse_splice(input, Some(Output::Value))?;
                modifiers.push(modifier);
            }
            "splice" => {
                let modifier = parse_splice(input, None)?;
                modifiers.push(modifier);
            }
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

#[cfg(test)]
mod tests {    
    use crate::{models::Modifier, parser::modifiers::parse_modifiers};    
    use syn::parse::Parser;

    #[macro_export]
    macro_rules! test_syntax {
        ($name: ident, $modi:pat, $($input:expr),+ ) => {
            #[test]
            fn $name() {
                vec![$($input),+]
                    .iter()
                    .for_each(|input| {
                        let result = Parser::parse_str(parse_modifiers, &input).unwrap();
                        assert!(matches!(&result[0], $modi));
                    });
            }
        };

        ($(($name:ident, $modi:pat, $($input:expr),+)),+ $(,)?) => {
            $(test_syntax!($name, $modi, $($input),+);)+
        };
    }

    test_syntax![
        (singular, Modifier::Singular, "|singular"),
        (plural, Modifier::Plural, "|plural"),
        (lower, Modifier::Lowercase, "|lower" , "|lowercase"),
        (upper, Modifier::Uppercase, "|upper" , "|uppercase" ),
        (pascal, Modifier::PascalCase, "|pascal" , "|pascalcase" , "|uppercamelcase" ),
        (camel, Modifier::LowerCamelCase, "|lowercamelcase" , "|camelcase" , "|camel" ),
        (snek, Modifier::SnakeCase, "|snakecase" , "|snake" , "|snekcase" , "|snek"),
        (kebab, Modifier::KebabCase, "|kebabcase" , "|kebab"),
        (shouty_nek, Modifier::ShoutySnakeCase, "|shoutysnakecase" , "|shoutysnake" , "|shoutysnekcase" , "|shoutysnek"),
        (title, Modifier::TitleCase, "|titlecase" , "|title"),
        (shouty_kebab, Modifier::ShoutyKebabCase, "|shoutykebabcase" , "|shoutykebab"),
        (train, Modifier::TrainCase, "|traincase" , "|train"),
        (replace, Modifier::Replace(_, _), "|replace{'a','b'}", "|replace{\"a\",'b'}", "|replace{'a',\"b\"}", "|replace{\"a\",\"b\"}"),
        (substring, Modifier::Substr(_,_), 
            "|substr{,}", "|substr{1,}", "|substr{,2}", "|substr{1,2}", 
            "|substring{,}", "|substring{1,}", "|substring{,2}", "|substring{1,2}"
        ),
        (reverse, Modifier::Reverse, "|reverse", "|rev"),
        (repeat, Modifier::Repeat(_), "|repeat{1}", "|rep{1}", "|times{1}"),
        (split_at, Modifier::SplitAt(_), "|splitat{1}"),
        (split, Modifier::Split(_), "|split{\"a\"}", "|split{'a'}"),
        (join, Modifier::Join(_), "|join", "|join{\"a\"}", "|join{'a'}"),
        (pad_start, Modifier::PadStart(_, _), 
            "|padstart{1, 'a'}", "|padstart{1, \"a\"}",
            "|padleft{1, 'a'}", "|padleft{1, \"a\"}",            
            "|padl{1, 'a'}", "|padl{1, \"a\"}"
        ),
        (pad_end, Modifier::PadEnd(_, _), 
            "|padend{1, 'a'}", "|padend{1, \"a\"}",
            "|padright{1, 'a'}", "|padright{1, \"a\"}",            
            "|padr{1, 'a'}", "|padr{1, \"a\"}"
        ),
        (slice, Modifier::Slice(_, _), 
            "|slice{}", "|slice{,}", "|slice{1,}", "|slice{,2}", "|slice{1,2}"
        ),
        (splice_out, Modifier::Splice(_,_,_,_),
            "|spliceout{,,}",
            "|spliceout{1,,}",
            "|spliceout{,1,}",
            "|spliceout{1,1,}",
            "|spliceout{,,'a'}",
            "|spliceout{,,\"a\"}",
            "|spliceout{1,,'a'}",
            "|spliceout{1,,\"a\"}",
            "|spliceout{,1,'a'}",
            "|spliceout{,1,\"a\"}",
            "|spliceout{1,1,'a'}",
            "|spliceout{1,1,\"a\"}",
            "|splice_out{,,}",
            "|splice_out{1,,}",
            "|splice_out{,1,}",
            "|splice_out{1,1,}",
            "|splice_out{,,'a'}",
            "|splice_out{,,\"a\"}",
            "|splice_out{1,,'a'}",
            "|splice_out{1,,\"a\"}",
            "|splice_out{,1,'a'}",
            "|splice_out{,1,\"a\"}",
            "|splice_out{1,1,'a'}",
            "|splice_out{1,1,\"a\"}"
        ),
        (splice_into, Modifier::Splice(_,_,_,_),
            "|spliceinto{,,}",
            "|spliceinto{1,,}",
            "|spliceinto{,1,}",
            "|spliceinto{1,1,}",
            "|spliceinto{,,'a'}",
            "|spliceinto{,,\"a\"}",
            "|spliceinto{1,,'a'}",
            "|spliceinto{1,,\"a\"}",
            "|spliceinto{,1,'a'}",
            "|spliceinto{,1,\"a\"}",
            "|spliceinto{1,1,'a'}",
            "|spliceinto{1,1,\"a\"}",
            "|splice_into{,,}",
            "|splice_into{1,,}",
            "|splice_into{,1,}",
            "|splice_into{1,1,}",
            "|splice_into{,,'a'}",
            "|splice_into{,,\"a\"}",
            "|splice_into{1,,'a'}",
            "|splice_into{1,,\"a\"}",
            "|splice_into{,1,'a'}",
            "|splice_into{,1,\"a\"}",
            "|splice_into{1,1,'a'}",
            "|splice_into{1,1,\"a\"}"
        ),
        (splice, Modifier::Splice(_,_,_,_),
            "|splice{into,,,}",
            "|splice{into,1,,}",
            "|splice{into,,1,}",
            "|splice{into,1,1,}",
            "|splice{into,,,'a'}",
            "|splice{into,,,\"a\"}",
            "|splice{into,1,,'a'}",
            "|splice{into,1,,\"a\"}",
            "|splice{into,,1,'a'}",
            "|splice{into,,1,\"a\"}",
            "|splice{into,1,1,'a'}",
            "|splice{into,1,1,\"a\"}",
            "|splice{value,,,}",
            "|splice{value,1,,}",
            "|splice{value,,1,}",
            "|splice{value,1,1,}",
            "|splice{value,,,'a'}",
            "|splice{value,,,\"a\"}",
            "|splice{value,1,,'a'}",
            "|splice{value,1,,\"a\"}",
            "|splice{value,,1,'a'}",
            "|splice{value,,1,\"a\"}",
            "|splice{value,1,1,'a'}",
            "|splice{value,1,1,\"a\"}",
            "|splice{val,,,}",
            "|splice{val,1,,}",
            "|splice{val,,1,}",
            "|splice{val,1,1,}",
            "|splice{val,,,'a'}",
            "|splice{val,,,\"a\"}",
            "|splice{val,1,,'a'}",
            "|splice{val,1,,\"a\"}",
            "|splice{val,,1,'a'}",
            "|splice{val,,1,\"a\"}",
            "|splice{val,1,1,'a'}",
            "|splice{val,1,1,\"a\"}",
            "|splice{,,,}",
            "|splice{,1,,}",
            "|splice{,,1,}",
            "|splice{,1,1,}",
            "|splice{,,,'a'}",
            "|splice{,,,\"a\"}",
            "|splice{,1,,'a'}",
            "|splice{,1,,\"a\"}",
            "|splice{,,1,'a'}",
            "|splice{,,1,\"a\"}",
            "|splice{,1,1,'a'}",
            "|splice{,1,1,\"a\"}",
            "|splice{out,,,}",
            "|splice{out,1,,}",
            "|splice{out,,1,}",
            "|splice{out,1,1,}",
            "|splice{out,,,'a'}",
            "|splice{out,,,\"a\"}",
            "|splice{out,1,,'a'}",
            "|splice{out,1,,\"a\"}",
            "|splice{out,,1,'a'}",
            "|splice{out,,1,\"a\"}",
            "|splice{out,1,1,'a'}",
            "|splice{out,1,1,\"a\"}",
            "|splice{value,,,}",
            "|splice{value,1,,}",
            "|splice{value,,1,}",
            "|splice{value,1,1,}",
            "|splice{value,,,'a'}",
            "|splice{value,,,\"a\"}",
            "|splice{value,1,,'a'}",
            "|splice{value,1,,\"a\"}",
            "|splice{value,,1,'a'}",
            "|splice{value,,1,\"a\"}",
            "|splice{value,1,1,'a'}",
            "|splice{value,1,1,\"a\"}",
            "|splice{rm,,,}",
            "|splice{rm,1,,}",
            "|splice{rm,,1,}",
            "|splice{rm,1,1,}",
            "|splice{rm,,,'a'}",
            "|splice{rm,,,\"a\"}",
            "|splice{rm,1,,'a'}",
            "|splice{rm,1,,\"a\"}",
            "|splice{rm,,1,'a'}",
            "|splice{rm,,1,\"a\"}",
            "|splice{rm,1,1,'a'}",
            "|splice{rm,1,1,\"a\"}"
        ),
    ];
}