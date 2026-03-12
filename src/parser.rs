use std::iter::Peekable;
use std::str::Chars;

use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, LitStr, Token, parenthesized};

use crate::models::{Modifier, StringParserError, StringParserState, TokenPart};

pub struct TweldDsl {
    pub parts: Vec<TokenPart>,
}

impl Parse for TweldDsl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut parts = Vec::new();
        while !input.is_empty() {
            if !input.peek(syn::token::Paren) {
                // println!("paren");
                let tt: TokenTree = input.parse()?;
                parts.push(TokenPart::Plain(tt.to_string()));
                continue;
            }

            let mod_content;
            parenthesized!(mod_content in input);

            let mut target = String::new();

            while mod_content.peek(syn::Ident) {
                target.push_str(&mod_content.parse::<Ident>()?.to_string());
            }

            let mut modifiers = Vec::new();

            while mod_content.peek(Token![|]) {
                mod_content.parse::<Token![|]>()?;
                if mod_content.is_empty() {
                    break;
                }

                let mod_name: Ident = mod_content.parse()?;
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
                            .and_then(|val| val.base10_parse::<usize>())
                            .ok();

                        args.parse::<Token![,]>()?;

                        let to = args
                            .parse::<LitInt>()
                            .and_then(|val| val.base10_parse::<usize>())
                            .ok();

                        modifiers.push(Modifier::Substr(from, to));
                    }
                    _ => {
                        return Err(syn::Error::new(
                            mod_name.span(),
                            format!("Unknown modifier {:?}", mod_name.span()),
                        ));
                    }
                }
            }
            parts.push(TokenPart::Modified(target, modifiers));
        }
        Ok(TweldDsl { parts })
    }
}


fn extract_left_and_right(clean_chars: &mut Peekable<Chars<'_>>) -> (String, String) {
    let mut left_side: bool = true;
    let mut left_word= String::new();
    let mut right_word = String::new();

    while let Some(char) = clean_chars.next() {
        if char == '}' { break; }
    
        let ignore_char = char == ' ' 
            || char == '\t'
            || char == '{' 
            || char == '"' 
            || char == '\''
            || char == '\'';
        
        if ignore_char { continue; }

        if char == ',' { 
            left_side = false; 
            continue;
        }

        if left_side {
            left_word.push(char);
        } else {
            right_word.push(char);
        }
    }

    (left_word, right_word)
}


impl TweldDsl {
    pub fn parse_str(input: &str) -> Result<Self, StringParserError> {
        let mut word = String::new();
        let mut clean_chars = input.chars().peekable();

        let mut state = StringParserState::Idle;

        let mut modifiers = vec![];
        let mut mod_target = String::new();
        let mut parts: Vec<TokenPart> = vec![];

        while let Some(curr_char) = clean_chars.next() {
            println!("curr_char '{curr_char}'");

            match state {
                StringParserState::Idle => {
                    if curr_char == '@' && clean_chars.peek() == Some(&'[') {
                        println!("entering brackets");
                        state = StringParserState::InsideBrackets;
                        clean_chars.next();

                        if word.len() > 0 {
                            println!("flushing word 1: `{word}`");
                            parts.push(TokenPart::Literal(word.clone()));
                            word.clear();
                        }

                        continue;
                    }

                    word.push(curr_char);
                    println!("not inside brackets ch '{curr_char}', word '{word}'");
                }
                StringParserState::InsideBrackets => {
                    if curr_char == ']' {
                        println!("leaving brackets");
                        state = StringParserState::Idle;
                        parts.push(TokenPart::Plain(word.clone()));
                        word.clear();
                        continue;
                    }

                    if curr_char == '(' {
                        println!("entering group");
                        state = StringParserState::InsideGroup;

                        if word.len() > 0 {
                            println!("flushing word 2: `{word}`");
                            parts.push(TokenPart::Literal(word.clone()));
                            word.clear();
                        }
                        continue;
                    }

                    if curr_char != ' ' {
                        word.push(curr_char);
                        // println!("inside brackets ch '{curr_char}', word '{word}'");
                    }

                    let word_terminator = curr_char == ' ' || curr_char == ']';

                    if word_terminator && word.len() > 0 {
                        parts.push(TokenPart::Plain(word.clone()));
                        word.clear();
                        continue;
                    }
                }
                StringParserState::InsideGroup => {
                    if curr_char == ')' {
                        println!("leaving group");

                        if word.len() > 0 {
                            mod_target.push_str(&word);
                            word.clear();
                        }

                        if modifiers.len() > 0 {
                            parts.push(TokenPart::Modified(mod_target.clone(), modifiers));
                        } else {
                            parts.push(TokenPart::Plain(mod_target.clone()));
                        }

                        state = StringParserState::InsideBrackets;

                        mod_target.clear();
                        modifiers = vec![];
                        word.clear();
                        continue;
                    }

                    if curr_char == '|' {
                        println!("entering modifiers");
                        state = StringParserState::Modifiers;
                        word.clear();
                        continue;
                    }

                    if curr_char == ' ' {
                        mod_target.push_str(&word);
                        word.clear();
                        continue;
                    }

                    word.push(curr_char);
                }
                StringParserState::Modifiers => {
                    if curr_char == '|' {
                        continue;
                    }

                    let word_terminator = curr_char == ' ' || curr_char == '{' || curr_char == ')';

                    if !word_terminator {
                        word.push(curr_char);
                        println!("word: '{word}'");
                    }

                    if word_terminator && word.len() > 0 {
                        println!("word: '{word}'");

                        match word.to_lowercase().trim() {
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
                            "shoutykebabcase" | "shoutykebab" => {
                                modifiers.push(Modifier::ShoutyKebabCase)
                            }
                            "traincase" | "train" => modifiers.push(Modifier::TrainCase),
                            "replace" => {
                                let (left_word, right_word) =
                                    extract_left_and_right(&mut clean_chars);

                                modifiers.push(Modifier::Replace(left_word, right_word));
                            }
                            "substr" | "substring" => {
                                let (left_word, right_word) =
                                    extract_left_and_right(&mut clean_chars);
                                let mut start_index: Option<usize> = None;

                                if !left_word.is_empty() {
                                    start_index = left_word
                                        .parse()                                        
                                        .and_then(|r| Ok(Some(r)))
                                        .map_err(|_| StringParserError::NaNParam{name: "start", value: left_word})?;

                                }

                                let mut end_index: Option<usize> = None;
                                if !right_word.is_empty() {
                                    end_index = right_word
                                        .parse()                                        
                                        .and_then(|r| Ok(Some(r)))
                                        .map_err(|_| StringParserError::NaNParam{name: "end", value: right_word})?;
                                }

                                modifiers.push(Modifier::Substr(start_index, end_index));
                            }
                            w => {
                                return Err(StringParserError::UnknownModifier(w.to_string()));
                            }
                        }

                        word.clear();
                    }

                    if curr_char == ')' {
                        println!("leaving group");

                        if modifiers.len() > 0 {
                            parts.push(TokenPart::Modified(mod_target.clone(), modifiers));
                        } else {
                            parts.push(TokenPart::Plain(mod_target.clone()));
                        }

                        state = StringParserState::InsideBrackets;

                        mod_target.clear();
                        modifiers = vec![];
                        word.clear();
                        continue;
                    }
                }
            }
        }

        println!("the end");

        if let StringParserState::InsideBrackets = state {
            println!("inside_brackets");
            return Err(StringParserError::OpenBrackets);
        }

        if let StringParserState::InsideGroup = state {
            println!("inside_group");
            return Err(StringParserError::OpenGroup);
        }

        if let StringParserState::Modifiers = state {
            println!("inside_modifiers");
            return Err(StringParserError::UnfinishModifiers);

        }

        if word.len() > 0 {
            println!("flushing word 3: `{word}`");
            parts.push(TokenPart::Literal(word.clone()));
        }

        Ok(TweldDsl { parts })
    }
}
