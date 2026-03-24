use std::iter::Peekable;
use std::str::{Chars, FromStr};

use syn::parse::{Parse, ParseStream};
use syn::{Ident, Lit, LitChar, LitInt, LitStr, Token, parenthesized};

use crate::models::{Modifier, StringParserState, TokenParserState, TokenPart};

#[derive(Debug)]
pub enum RenderAs {
    StringLiteral,
    Identifier
}

pub struct TweldDsl {
    pub render_as: RenderAs,
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
                let sep = args.parse::<LitStr>()?;                        
                modifiers.push(Modifier::Join(sep.value()));
            },
            "padstart" | "padleft" | "padl" => {
                let args;
                syn::braced!(args in content);
                let size = args
                    .parse::<LitInt>()
                    .and_then(|val| val.base10_parse::<usize>())?;

                args.parse::<Token![,]>()?;

                let pad = args.parse::<LitStr>()?;

                modifiers.push(Modifier::PadStart(size, pad.value()));
            },
            "padend" | "padright" | "padr" => {
                let args;
                syn::braced!(args in content);
                let size = args
                    .parse::<LitInt>()
                    .and_then(|val| val.base10_parse::<usize>())?;

                args.parse::<Token![,]>()?;

                let pad = args.parse::<LitStr>()?;

                modifiers.push(Modifier::PadEnd(size, pad.value()));
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
    mut word: String,
    mut indent: usize,
) -> syn::Result<TweldDsl> {
    indent += 1; 
    let sp = "-".repeat(indent);
    while !input.is_empty() {
        println!("{sp}looping: {state:?} {:?}", dsl.parts);
        match state {
            TokenParserState::InsideBrackets => {
                if input.peek(syn::token::Paren) { 
                    println!("{sp}entering group");
                    let group;
                    parenthesized!(group in input); 
                    dsl = parse_stream(&group, dsl, TokenParserState::InsideGroup, word.clone(), indent)?;
                    word.clear();
                    println!("{sp}leaving group");
                    continue;
                }

                if input.peek(Token![|]) {
                    println!("{sp}entering modifiers");
                    dsl = parse_stream(&input, dsl, TokenParserState::Modifiers, word.clone(), indent)?;
                    word.clear();
                    println!("{sp}leaving modifiers");
                    continue;
                }

                if !word.is_empty() {
                    println!("{sp}pushing plain: `{word}`");
                    dsl.parts.push(TokenPart::Plain(word.clone()));
                    word.clear();
                }

                if input.peek(Token![-]) {
                    println!("{sp}found token b -");
                    input.parse::<Token![-]>()?;
                    dsl.parts.push(TokenPart::Plain("-".to_string()));
                    continue;
                }
            
                if input.peek(syn::Ident) {
                    word = input
                        .parse::<Ident>()?
                        .to_string();
                    println!("{sp}save ident b: `{word}`");
                    continue;
                }

                if input.peek(syn::LitStr) {
                    word = input
                        .parse::<LitStr>()?
                        .value();  
                    dsl.render_as = RenderAs::StringLiteral;
                    println!("{sp}save lit b: `{word}`");
                    continue;                      
                }

                if input.peek(syn::LitChar) {
                    word = input
                        .parse::<LitChar>()?
                        .value()
                        .to_string();  
                    dsl.render_as = RenderAs::StringLiteral;
                    println!("{sp}save lit b: `{word}`");
                    continue;                      
                }

                let ignored = input.parse::<proc_macro2::TokenTree>()?;
                println!("{sp}ignored 1 {ignored:?}");
            },
            TokenParserState::InsideGroup => {                    
                while !input.is_empty() {
                    println!("{sp}looping group");

                    if input.peek(Token![|]) {
                        println!("{sp}entering modifiers");
                        dsl = parse_stream(&input, dsl, TokenParserState::Modifiers, word.clone(), indent)?;
                        word.clear();
                        continue;
                    }

                    if input.peek(Token![-]) {
                        input.parse::<Token![-]>()?;
                        word.push('-');
                        println!("{sp}found token g -: {word}");
                        continue;
                    }

                    if input.peek(syn::Ident) {
                        let value = input
                            .parse::<Ident>()?
                            .to_string();
                        
                        word.push_str(&value);
                        println!("{sp}acc ident g: {word}");
                        continue;
                    }

                    if input.peek(syn::LitChar) {
                        let value = input
                            .parse::<LitChar>()?
                            .value();
                    
                        word.push_str(&value.to_string());
                        println!("{sp}acc litc g: {word}");
                        dsl.render_as = RenderAs::StringLiteral;
                        continue;
                    }

                    if input.peek(syn::LitStr) {                        
                        let value = input
                            .parse::<LitStr>()?
                            .value();
                    
                        word.push_str(&value);
                        println!("{sp}acc lits g: {word}");
                        dsl.render_as = RenderAs::StringLiteral;
                        continue;
                    }

                    let ignored = input.parse::<proc_macro2::TokenTree>()?;
                    println!("{sp}ignored 2 {ignored:?}");
                }
            },
            TokenParserState::Modifiers => {
                println!("{sp}getting modifiers");
                let mut modifiers = get_modifiers(input)?;
                let target;
                
                if !word.is_empty() { 
                    target = vec![word.clone()];
                } else {
                    let Some(last_part) = dsl.parts.pop() else {
                        return Err(syn::Error::new(
                            input.span(),
                            "Modifiers need a target"
                        ));
                    };

                    match last_part {
                        TokenPart::Literal(text) => target = vec![text.clone()],
                        TokenPart::Plain(text) => target = vec![text.clone()],
                        TokenPart::Modified(items, mut prev_modifiers) => {
                            target = items;
                            prev_modifiers.append(&mut modifiers);
                            modifiers = prev_modifiers;
                        },
                    }
                }

                dsl.parts.push(TokenPart::Modified(target, modifiers));
                word.clear();
            },
        }
        
        println!("{sp}end - word: `{word}`");

        if !word.is_empty() {
            println!("{sp}inside loop push plain");
            dsl.parts.push(TokenPart::Plain(word.clone()));
            word.clear();
        }
    }
    println!("{sp}end2 - word: `{word}`");

    if !word.is_empty() {
        println!("{sp}outside loop push plain");
        dsl.parts.push(TokenPart::Plain(word.clone()));
        word.clear();
    }
    println!("{sp}render_as: {:?}", dsl.render_as);
    Ok(dsl)
}


impl Parse for TweldDsl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut dsl = TweldDsl { 
            render_as: RenderAs::Identifier, 
            parts: Vec::new(),
        };
        
        // let b = input
        dsl = parse_stream(
            input, 
            dsl, 
            TokenParserState::InsideBrackets, 
            String::new(),
            0usize
        )?;

        println!("parts: {:?}", dsl.parts);
        Ok(dsl)
    }
}



fn extract_word(clean_chars: &mut Peekable<Chars<'_>>, modifier: &str, lit: &LitStr) -> syn::Result<(String, bool)> {    
    let mut contains_str = false;
    let mut str_kind = StrKind::None;    
    let mut word = String::new();

    while let Some(char) = clean_chars.next() {
        println!("extracting char: `{char}`");
        match str_kind {
            StrKind::SingleQuotes => {
                if char == '\'' {
                    contains_str = true;
                    str_kind = StrKind::None;
                    println!("leaving s quotes");
                    continue;
                }
                if char == '}' {
                    return Err(syn::Error::new(
                        lit.span(),
                format!("String badly formed for {modifier} {:?}", lit.span()),
                    ));
                }
            },
            StrKind::DoubleQuotes => {                
                if char == '"' {
                    contains_str = true;
                    str_kind = StrKind::None;
                    println!("leaving d quotes");
                    continue;
                }
                if char == '}' {
                    return Err(syn::Error::new(
                        lit.span(),
                format!("String badly formed for {modifier} {:?}", lit.span()),
                    ));
                }
            },
            StrKind::None => {
                if char == '"' {
                    str_kind = StrKind::DoubleQuotes;
                    println!("entering d quotes");
                    continue;
                }

                if char == '\'' {
                    str_kind = StrKind::SingleQuotes;
                    println!("entering s quotes");
                    continue;
                }

                if char == ' ' 
                    || char == '\t'
                    || char == '{'  {
                    continue;
                }
                    
                if char == '}' {
                    println!("leaving extraction");
                    break; 
                }
            },
        }

        println!("pushing: `{char}`");

        word.push(char);
    }

    Ok((word, contains_str))
}

enum StrKind {
    SingleQuotes,
    DoubleQuotes,
    None
}

fn extract_left_and_right(clean_chars: &mut Peekable<Chars<'_>>, modifier: &str, lit: &LitStr) -> syn::Result<(String, String)> {
    let mut left_side = true;
    let mut str_kind = StrKind::None;
    let mut left_word= String::new();
    let mut right_word = String::new();

    while let Some(char) = clean_chars.next() {
        match str_kind {
            StrKind::SingleQuotes => {
                if char == '\'' {
                    str_kind = StrKind::None;
                    println!("leaving s quotes");
                    continue;
                }
                if char == '}' {
                    return Err(syn::Error::new(
                        lit.span(),
                format!("String badly formed for {modifier} {:?}", lit.span()),
                    ));
                }
            },
            StrKind::DoubleQuotes => {
                if char == '"' {
                    str_kind = StrKind::None;
                    println!("leaving d quotes");
                    continue;
                }
                if char == '}' {
                    return Err(syn::Error::new(
                        lit.span(),
                format!("String badly formed for {modifier} {:?}", lit.span()),
                    ));
                }
            },
            StrKind::None => {
                if char == '"' {
                    str_kind = StrKind::DoubleQuotes;
                    println!("entering d quotes");
                    continue;
                }

                if char == '\'' {
                    str_kind = StrKind::SingleQuotes;
                    println!("entering s quotes");
                    continue;
                }

                if char == ' ' 
                    || char == '\t'
                    || char == '{'  {
                    continue;
                }
                    
                if char == '}' {
                    break; 
                }
                
                if char == ',' { 
                    left_side = false; 
                    continue;
                }
            },
        }

        println!("pushing: `{char}`");

        if left_side {
            left_word.push(char);
        } else {
            right_word.push(char);
        }
    }

    Ok((left_word, right_word))
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

        let mut space = false;

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

            if curr_char == ' ' {
                println!("space!");
                space = true;
            }

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
                            space = false;
                        }

                        continue;
                    }

                    word.push(curr_char);
                    println!("not inside brackets ch '{curr_char}', word '{word}'");
                }
                StringParserState::InsideBrackets => {
                    if curr_char == ']' {
                        println!("leaving brackets");

                        if !mod_target.is_empty() {
                            if modifiers.len() > 0 {
                                parts.push(TokenPart::Modified(vec![mod_target.clone()], modifiers));
                            } else {
                                parts.push(TokenPart::Plain(mod_target.clone()));
                            }
                        } else if !word.is_empty() {
                            if modifiers.len() > 0 {
                                parts.push(TokenPart::Modified(vec![word.clone()], modifiers));
                            } else {
                                parts.push(TokenPart::Plain(word.clone()));
                            }
                        } 


                        state = StringParserState::Idle;
                        
                        mod_target.clear();
                        modifiers = vec![];
                        word.clear();
                        space = false;
                        continue;
                    }

                    if curr_char == '(' {
                        println!("entering group");
                        state = StringParserState::InsideGroup;

                        if !word.is_empty() {
                            println!("flushing word 2: `{word}`");
                            parts.push(TokenPart::Plain(word.clone()));
                            word.clear();
                            space = false;
                        }
                        continue;
                    }

                    if curr_char == '|' {
                        state = StringParserState::Modifiers;
                        mod_target.push_str(&word.replace(" ", ""));
                        word.clear();
                        space = false;
                        println!("entering modifiers w:`{word}` t:`{mod_target}`");
                        continue;
                    }

                    if curr_char != ' ' {
                        // another way to do it, a vec of words and the latest will be sent as either modified or plain
                        if space && !word.is_empty() {
                            println!("pushing Plain: `{word}`");
                            parts.push(TokenPart::Plain(word.clone()));
                            word.clear();
                        }

                        word.push(curr_char);
                        println!("adding to the word: `{word}`");
                        space = false;
                    }
                }
                StringParserState::InsideGroup => {                    
                    if curr_char == ')' {
                        println!("leaving modifiers2");

                        if !mod_target.is_empty() {
                            if modifiers.len() > 0 {
                                parts.push(TokenPart::Modified(vec![mod_target.clone()], modifiers));
                            } else {
                                parts.push(TokenPart::Plain(mod_target.clone()));
                            }
                        } else if !word.is_empty() {
                            if modifiers.len() > 0 {
                                parts.push(TokenPart::Modified(vec![word.clone()], modifiers));
                            } else {
                                parts.push(TokenPart::Plain(word.clone()));
                            }
                        }

                        state = StringParserState::InsideBrackets;                        

                        mod_target.clear();
                        modifiers = vec![];
                        word.clear();
                        space = false;
                        continue;
                    }

                    if curr_char == '|' {
                        println!("entering modifiers");
                        state = StringParserState::Modifiers;
                        mod_target.push_str(&word.replace(" ", ""));                        
                        word.clear();
                        space = false;
                        println!("entering modifiers w:`{word}` t:`{mod_target}`");
                        continue;
                    }

                    // if curr_char == ' ' {
                    //     mod_target.push_str(&word);
                    //     word.clear();
                    //     continue;
                    // }

                    word.push(curr_char);
                }
                StringParserState::Modifiers => {
                    // if word.len() > 0 {
                    //     println!("1");
                    //     mod_target.push_str(&word);
                    //     word.clear();                        
                    // }

                    if curr_char == '|' {
                        println!("2");
                        continue;
                    }

                    let word_terminator = curr_char == ' ' || curr_char == '{' || curr_char == ')' || curr_char == ']';

                    if !word_terminator {
                        println!("3");
                        word.push(curr_char);
                        println!("word: '{word}'");
                    }

                    if word_terminator && word.len() > 0 {
                        println!("4");
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
                                    extract_left_and_right(&mut clean_chars, &word, input_lit)?;

                                    println!("left : `{left_word}`, right: `{right_word}`");
                                modifiers.push(Modifier::Replace(left_word, right_word));
                            }
                            "substr" | "substring" => {
                                let (left_word, right_word) =
                                    extract_left_and_right(&mut clean_chars, &word, input_lit)?;
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
                                
                                let times = try_num(&result, "start", input_lit)?;

                                modifiers.push(Modifier::Repeat(times));
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
                                    
                                } else {
                                    let mid = try_num(&result, "start", input_lit)?;
                                    modifiers.push(Modifier::SplitAt(mid));
                                }                                
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
                                    extract_left_and_right(&mut clean_chars, &word, input_lit)?;
                                
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
                                    extract_left_and_right(&mut clean_chars, &word, input_lit)?;
                                
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
println!("looping mods");
                        word.clear();
                    }

                    if curr_char == ')' || curr_char == ']' {
                        println!("leaving modifiers");

                        if !mod_target.is_empty() {
                            if modifiers.len() > 0 {
                                parts.push(TokenPart::Modified(vec![mod_target.clone()], modifiers));
                            } else {
                                parts.push(TokenPart::Plain(mod_target.clone()));
                            }
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

        Ok(TweldDsl { 
            render_as: RenderAs::StringLiteral, 
            parts 
        })
    }
}
