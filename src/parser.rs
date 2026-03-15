use std::iter::Peekable;
use std::str::{Chars, FromStr};

use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, LitStr, Token, parenthesized};

use crate::models::{Modifier, StringParserState, TokenPart};

pub struct TweldDsl {
    pub parts: Vec<TokenPart>,
}

impl Parse for TweldDsl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut parts = Vec::new();
        while !input.is_empty() {
            if !input.peek(syn::token::Paren) {                
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
                    "reverse" | "rev" => modifiers.push(Modifier::Reverse),
                    "repeat" | "rep" | "times" => todo!(),
                    "split" => todo!(),
                    "join" => todo!(),
                    "padstart" | "padleft" | "padl" => todo!(),
                    "padend" | "padright" | "padr" => todo!(),

                    _ => {
                        return Err(syn::Error::new(
                            mod_name.span(),
                            format!("Unknown modifier {:?}", mod_name.span()),
                        ));
                    }
                }
            }
            parts.push(TokenPart::Modified(vec![target], modifiers));
        }
        Ok(TweldDsl { parts })
    }
}

fn extract_word(clean_chars: &mut Peekable<Chars<'_>>, modifier: &str, lit: &LitStr) -> syn::Result<(String, bool)> {
    let mut word = String::new();
        
    while let Some(char) = clean_chars.next() {
        if char == '}' { break; }
    
        let ignore_char = char == ' ' 
            || char == '\t'
            || char == '{'            
            || char == ',';
        
        if ignore_char { continue; }
        word.push(char);        
    }

    let double = word.chars().filter(|c| *c == '"').count();
    let single = word.chars().filter(|c| *c == '\'').count();

    if double % 2 != 0 || single % 2 != 0 {
        return Err(syn::Error::new(
            lit.span(),
            format!("String badly formed for {modifier} {:?}", lit.span()),
        ));
    }

    Ok((word, double > 0 || single > 0))
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

fn try_num<T: FromStr>(input: &str, field: &str, parent: &LitStr) -> syn::Result<T> {
    input.parse::<T>()        
        .map_err(|_| syn::Error::new(
            parent.span(), 
            format!("The value for '{field}' is not a number ('{input}')!")
        ))
}

impl TweldDsl {
    pub fn parse_lit_str(input_lit: &LitStr) -> syn::Result<Self> {
        let input_str: String = input_lit.value();

        let mut word = String::new();
        let mut clean_chars = input_str.chars().peekable();

        let mut state = StringParserState::Idle;

        let mut modifiers = vec![];
        let mut mod_target = String::new();
        let mut parts: Vec<TokenPart> = vec![];

        let mut row_num = 1;
        let mut col_num = 1;
        while let Some(curr_char) = clean_chars.next() {
            println!("curr_char '{curr_char}'");

            if curr_char == '\n' {
                row_num += 1;
                col_num = 0;
            } 

            if curr_char == '\r' {                 
                continue;
            }

            col_num += 1;

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

                    if curr_char == '|' {
                        println!("entering modifiers");
                        state = StringParserState::Modifiers;
                        mod_target.push_str(&word);                            
                        word.clear();
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
                            parts.push(TokenPart::Modified(vec![mod_target.clone()], modifiers));
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
                                    let val = try_num(&left_word, "start", input_lit)?;
                                    start_index = Some(val);
                                }

                                let mut end_index: Option<usize> = None;
                                if !right_word.is_empty() {
                                    let val = try_num(&right_word, "end", input_lit)?;                                    
                                    end_index = Some(val)
                                }

                                modifiers.push(Modifier::Substr(start_index, end_index));
                            }                            
                            "reverse" | "rev" => modifiers.push(Modifier::Reverse),
                            "repeat" | "rep" | "times" => {
                                let (result, _)  = extract_word(&mut clean_chars, "split", input_lit)?;
                                
                                if result.is_empty() {
                                    return Err(syn::Error::new(
                                        input_lit.span(),
                                        format!("No parameter was informed for {word} at line: {row_num}, col: {col_num} in the literal"),
                                    ));
                                }
                                
                                let times = try_num(&word, "start", input_lit)?;

                                modifiers.push(Modifier::SplitAt(times));
                            },
                            "split" => {
                                let (result, is_str)  = extract_word(&mut clean_chars, "split", input_lit)?;
                                
                                if result.is_empty() {
                                    return Err(syn::Error::new(
                                        input_lit.span(),
                                        format!("No parameter was informed for {word} at line: {row_num}, col: {col_num} in the literal"),
                                    ));
                                }

                                if is_str {
                                    modifiers.push(Modifier::Split(result));
                                    continue;
                                }

                                let mid = try_num(&result, "start", input_lit)?;

                                modifiers.push(Modifier::SplitAt(mid));
                            }
                            "join" => {
                                let (result, _) = extract_word(&mut clean_chars, "join", input_lit)?;
                                if result.is_empty() {
                                    return Err(syn::Error::new(
                                        input_lit.span(),
                                        format!("No parameter was informed for join at line: {row_num}, col: {col_num} in the literal"),
                                    ));
                                }
                                modifiers.push(Modifier::Join(result));
                            }
                            "padstart" | "padleft" | "padl" => {
                                let (left_word, right_word) =
                                    extract_left_and_right(&mut clean_chars);
                                let mut index = 0;
                                if !left_word.is_empty() {
                                    index = try_num(&left_word, "start", input_lit)?;
                                }
                                
                                if index < 1 {
                                    return Err(syn::Error::new(
                                        input_lit.span(),
                                        format!("The mid parameter needs to be more than 0 for {word} at line: {row_num}, col: {col_num} in the literal"),
                                    ));
                                }

                                if right_word.is_empty() {
                                    return Err(syn::Error::new(
                                        input_lit.span(),
                                        format!("No parameter was informed for {word} at line: {row_num}, col: {col_num} in the literal"),
                                    ));
                                }                                

                                modifiers.push(Modifier::PadStart(index, right_word));
                            }
                            "padend" | "padright" | "padr" => {
                                let (left_word, right_word) =
                                    extract_left_and_right(&mut clean_chars);
                                let mut index = 0;
                                if !left_word.is_empty() {
                                    index = try_num(&left_word, "start", input_lit)?;
                                }
                                
                                if index < 1 {
                                    return Err(syn::Error::new(
                                        input_lit.span(),
                                        format!("The mid parameter needs to be more than 0 for {word} at line: {row_num}, col: {col_num} in the literal"),
                                    ));
                                }

                                modifiers.push(Modifier::PadEnd(index, right_word));
                            }

                            modifier => {                                
                                return Err(syn::Error::new(
                                    input_lit.span(),
                                    format!("Unknown modifier {modifier} at line: {row_num}, col: {col_num} in the literal"),
                                ));                                
                            }
                        }

                        word.clear();
                    }

                    if curr_char == ')' || curr_char == ']' {
                        println!("leaving modifiers");

                        if modifiers.len() > 0 {
                            parts.push(TokenPart::Modified(vec![mod_target.clone()], modifiers));
                        } else {
                            parts.push(TokenPart::Plain(mod_target.clone()));
                        }

                        if curr_char == ')' {
                            state = StringParserState::InsideBrackets;
                        } else {
                            state = StringParserState::Idle;
                        }

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
            return Err(syn::Error::new(
                input_lit.span(),
                format!("Brackets were not closed! (line: {row_num}, col: {col_num} in the literal)"),
            ));  
        }

        if let StringParserState::InsideGroup = state {
            println!("inside_group");            
            return Err(syn::Error::new(
                input_lit.span(),
                format!("Group was left open! (line: {row_num}, col: {col_num} in the literal)"),
            ));  
        }

        if let StringParserState::Modifiers = state {
            println!("inside_modifiers");
            return Err(syn::Error::new(
                input_lit.span(),
                format!("Modifiers are incomplete! (line: {row_num}, col: {col_num} in the literal)"),
            ));  

        }

        if word.len() > 0 {
            println!("flushing word 3: `{word}`");
            parts.push(TokenPart::Literal(word.clone()));
        }

        Ok(TweldDsl { parts })
    }
}
