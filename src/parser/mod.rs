mod modifiers;

use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitChar, LitStr, Token, bracketed, parenthesized};

use crate::models::{RenderType, TokenParserState, TokenPart};
use crate::parser::modifiers::parse_modifiers;

const MAX_DEPTH: usize = 20;

pub struct TweldDsl {
    pub render_type: RenderType,
    pub parts: Vec<TokenPart>,
}

impl Parse for TweldDsl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut dsl = TweldDsl {
            render_type: RenderType::Identifier,
            parts: Vec::new(),
        };

        dsl = parse_stream(input, dsl, TokenParserState::InsideBrackets, 0usize)?;

        Ok(dsl)
    }
}

fn parse_stream(
    input: &syn::parse::ParseBuffer<'_>,
    mut dsl: TweldDsl,
    state: TokenParserState,
    mut depth: usize,
) -> syn::Result<TweldDsl> {
    depth += 1;

    if depth >= MAX_DEPTH {
        return Err(syn::Error::new(input.span(), "Maximum nesting reached!"));
    }

    let mut words: Vec<String> = vec![];
    while !input.is_empty() {
        match state {
            TokenParserState::InsideBrackets => {
                if input.peek(syn::token::Paren) {
                    parse_concat_group(input, &mut dsl, depth)?;

                    continue;
                }

                if input.peek(syn::token::Bracket) {
                    parse_list_group(input, &mut dsl, depth)?;

                    continue;
                }

                if input.peek(Token![|]) {
                    dsl = parse_stream(input, dsl, TokenParserState::Modifiers, depth)?;
                    continue;
                }

                if input.peek(syn::Ident) {
                    let result = input.parse::<Ident>()?.to_string();
                    dsl.parts.push(TokenPart::Plain(result));
                    continue;
                }

                if input.peek(syn::LitStr) {
                    let result = input.parse::<LitStr>()?.value();
                    dsl.render_type = RenderType::StringLiteral;
                    dsl.parts.push(TokenPart::Plain(result));
                    continue;
                }

                if input.peek(syn::LitChar) {
                    let result = input.parse::<LitChar>()?.value().to_string();
                    dsl.render_type = RenderType::StringLiteral;
                    dsl.parts.push(TokenPart::Plain(result));
                    continue;
                }

                if input.peek(Token![-]) {
                    input.parse::<Token![-]>()?;
                    dsl.parts.push(TokenPart::Plain("-".to_string()));
                    continue;
                }

                if input.peek(Token![_]) {
                    input.parse::<Token![_]>()?;
                    dsl.parts.push(TokenPart::Plain("_".to_string()));
                    continue;
                }

                let _ = input.parse::<proc_macro2::TokenTree>()?;
            }
            TokenParserState::InsideGroup(_) => {
                while !input.is_empty() {
                    if input.peek(syn::token::Paren) {
                        parse_concat_group(input, &mut dsl, depth)?;

                        continue;
                    }

                    if input.peek(syn::token::Bracket) {
                        parse_list_group(input, &mut dsl, depth)?;

                        continue;
                    }

                    if input.peek(Token![|]) {
                        if !words.is_empty() {
                            let Some(last) = words.pop() else {
                                todo!();
                            };

                            dsl.parts.push(TokenPart::Plain(words.join("")));
                            words.clear();
                            dsl.parts.push(TokenPart::Plain(last));
                        }

                        dsl = parse_stream(input, dsl, TokenParserState::Modifiers, depth)?;
                        continue;
                    }

                    if input.peek(syn::Ident) {
                        let value = input.parse::<Ident>()?.to_string();

                        words.push(value);
                        continue;
                    }

                    if input.peek(syn::LitChar) {
                        let value = input.parse::<LitChar>()?.value();

                        words.push(value.to_string());
                        dsl.render_type = RenderType::StringLiteral;
                        continue;
                    }

                    if input.peek(syn::LitStr) {
                        let value = input.parse::<LitStr>()?.value();

                        words.push(value);
                        dsl.render_type = RenderType::StringLiteral;
                        continue;
                    }

                    if input.peek(Token![-]) {
                        input.parse::<Token![-]>()?;
                        words.push("-".to_string());
                        continue;
                    }

                    if input.peek(Token![_]) {
                        input.parse::<Token![_]>()?;
                        words.push("_".to_string());
                        continue;
                    }

                    let _ = input.parse::<proc_macro2::TokenTree>()?;
                }
            }
            TokenParserState::Modifiers => {
                let Some(target) = dsl.parts.pop() else {
                    return Err(syn::Error::new(input.span(), "Modifiers need a target"));
                };

                let modifiers = parse_modifiers(input)?;
                dsl.parts
                    .push(TokenPart::Modified(Box::new(target), modifiers));
            }
        }

        if !words.is_empty() {
            parse_words(&mut dsl, &state, &mut words);
        }
    }
    if !words.is_empty() {
        parse_words(&mut dsl, &state, &mut words);
    }

    Ok(dsl)
}

fn parse_words(dsl: &mut TweldDsl, state: &TokenParserState, words: &mut Vec<String>) {
    match state {
        TokenParserState::InsideGroup(single_value) if !single_value => {
            words
                .iter()
                .for_each(|word| dsl.parts.push(TokenPart::Plain(word.clone())));
        }
        _ => dsl.parts.push(TokenPart::Plain(words.join(""))),
    }

    words.clear();
}

fn parse_concat_group(
    input: &syn::parse::ParseBuffer<'_>,
    dsl: &mut TweldDsl,
    depth: usize,
) -> Result<(), syn::Error> {
    let group;
    parenthesized!(group in input);
    let mut grouped_dsl = TweldDsl {
        render_type: dsl.render_type.clone(),
        parts: vec![],
    };
    grouped_dsl = parse_stream(
        &group,
        grouped_dsl,
        TokenParserState::InsideGroup(true),
        depth,
    )?;

    dsl.parts.push(TokenPart::ConcatGroup(grouped_dsl.parts));

    if let RenderType::StringLiteral = grouped_dsl.render_type {
        dsl.render_type = RenderType::StringLiteral;
    }

    Ok(())
}

fn parse_list_group(
    input: &syn::parse::ParseBuffer<'_>,
    dsl: &mut TweldDsl,
    depth: usize,
) -> Result<(), syn::Error> {
    let group;
    bracketed!(group in input);
    let mut grouped_dsl = TweldDsl {
        render_type: dsl.render_type.clone(),
        parts: vec![],
    };
    grouped_dsl = parse_stream(
        &group,
        grouped_dsl,
        TokenParserState::InsideGroup(false),
        depth,
    )?;

    dsl.parts.push(TokenPart::ListGroup(grouped_dsl.parts));

    if let RenderType::StringLiteral = grouped_dsl.render_type {
        dsl.render_type = RenderType::StringLiteral;
    }

    Ok(())
}
