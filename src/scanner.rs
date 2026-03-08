use proc_macro2::{Literal, TokenStream};
use proc_macro2::{TokenTree, Delimiter, Group};
use quote::{ToTokens, format_ident};
use syn::{LitStr, parse2};

use crate::models::{Modifier, TokenPart};
use crate::{builder::build_string, parser::BrazeDsl};

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
                        let bracket_group = if let TokenTree::Group(grp) = tokens.next().unwrap() { grp } else { unreachable!() };
                        
                        // Process the naming DSL inside the brackets
                        let dsl: BrazeDsl = parse2(bracket_group.stream()).expect("Invalid Braze welding DSL");
                        let ident_name = build_string(dsl.parts);
                        
                        output.push(TokenTree::Ident(format_ident!("{}", ident_name)));
                        continue;
                    }
                }
                output.push(tree);
            }

            TokenTree::Punct(ref p) => {
                // println!("maybe here (punc)? {p:?}");
                let token = tokens.next().unwrap();
                // println!("token {token}");
            }
            
            TokenTree::Group(g) => {
                // println!("maybe here (grp)? {g:?}");
                let inner_expanded = scan_tokens(g.stream());
                let mut new_group = Group::new(g.delimiter(), inner_expanded);
                new_group.set_span(g.span());
                output.push(TokenTree::Group(new_group));
            }

            TokenTree::Literal(lit) => {
                // println!("maybe here (lit)? {:?}", &lit);                
                
                let tokens = quote::quote!(#lit);
                
                let Ok(lit_str) = parse2::<LitStr>(tokens) else {                                      
                    output.push(TokenTree::Literal(lit));
                    continue;
                };
                
                let clean_string: String = lit_str.value();
                
                let mut word = String::new();
                let mut clean_chars = clean_string.chars().peekable(); //.collect::<Vec<char>>();
                let mut inside_brackets = false;
                let mut inside_group = false;
                let mut inside_modifiers = false;
                let mut modifiers = vec![];
                let mut mod_target = String::new();
                let mut parts: Vec<TokenPart> = vec![];

                while let Some(curr_char) = clean_chars.next() {
                    println!("curr_char '{curr_char}'");

                    if curr_char == '@' && clean_chars.peek() == Some(&'[') {
                        println!("getting inside brackets");
                        inside_brackets = true;
                        clean_chars.next();
                        continue;
                    }

                    if inside_brackets && curr_char == ']' {
                        println!("leaving brackets");
                        inside_brackets = false;
                        inside_group = false;
                        inside_modifiers = false;
                        clean_chars.next();
                        continue;
                    }

                    if !inside_brackets || (curr_char != ' ' && curr_char != '(' && curr_char != '|' && curr_char != ')') {
                        word.push(curr_char);                        
                    }                    

                    if inside_brackets {
                        // if curr_char == ' ' { continue; }

                        if curr_char == '(' {
                            println!("inside group");
                            inside_group = true;
                            continue;
                        }

                        if inside_group {
                            if curr_char == ')' {
                                println!("leaving group");
                                parts.push(TokenPart::Modified(mod_target.clone(), modifiers));

                                inside_group = false;
                                inside_modifiers = false;
                                mod_target.clear();
                                modifiers = vec![];
                                word.clear();
                                continue;
                            }
                            
                            if curr_char == '|' {
                                inside_modifiers = true;
                                continue;
                            }
                        }
                    }

                    let Some(peeked) = clean_chars.peek() else {
                        break;
                    };

                    let word_terminator = peeked == &' ' || curr_char == ' ' 
                        || (inside_brackets && (curr_char == ']'))
                        || (inside_group && (peeked == &'|' || curr_char == ')' || peeked == &')'))
                        || (inside_modifiers && (peeked == &'{'));
                    
                    if word_terminator {
                        println!("word: '{word}'");                        
                        // if !inside_modifiers {
                        //     parts.push(TokenPart::Plain(word.clone())); 
                        //     word.clear();
                        //     continue;
                        // } 

                        match word.to_lowercase().trim() {
                            "" => continue,
                            "singular" => modifiers.push(Modifier::Singular),
                            "plural" => modifiers.push(Modifier::Plural),
                            "lower" | "lowercase" => modifiers.push(Modifier::Lowercase),
                            "upper" | "uppercase" => modifiers.push(Modifier::Uppercase),
                            "pascal" | "pascalcase" | "uppercamelcase" => modifiers.push(Modifier::PascalCase),
                            "lowercamelcase" | "camelcase" | "camel" => modifiers.push(Modifier::LowerCamelCase),
                            "snakecase" | "snake" | "snekcase" | "snek" => modifiers.push(Modifier::SnakeCase),
                            "kebabcase" | "kebab" => modifiers.push(Modifier::KebabCase),
                            "shoutysnakecase" | "shoutysnake"  | "shoutysnekcase"  | "shoutysnek" => modifiers.push(Modifier::ShoutySnakeCase),
                            "titlecase" | "title" => modifiers.push(Modifier::TitleCase),
                            "shoutykebabcase" | "shoutykebab" => modifiers.push(Modifier::ShoutyKebabCase),
                            "traincase" | "train" => modifiers.push(Modifier::TrainCase),
                            "replace" => {
                                if peeked != &'{' {
                                    // throw compilation error
                                }
                                
                                clean_chars.next();
                                
                                let mut left_word = String::new();
                                let mut right_word = String::new();
                                let mut left_side = true;
                                while let Some(repl_char) = clean_chars.next() {
                                    println!("replace char {repl_char}");
                                    if repl_char == '}' { break; }
                                    if repl_char == ' ' || repl_char == '"' || repl_char == '\'' { continue; }

                                    if repl_char == ',' { 
                                        left_side = false; 
                                        continue;
                                    }

                                    if left_side {
                                        left_word.push(repl_char);
                                    } else {
                                        right_word.push(repl_char);
                                    }
                                } 
                                
                                modifiers.push(Modifier::Replace(left_word, right_word));
                            }
                            "substr" | "substring" => {
                                if peeked != &'{' {
                                    // throw compilation error
                                }
                                
                                clean_chars.next();
                                
                                let mut left_word = String::new();
                                let mut right_word = String::new();
                                let mut left_side = true;
                                
                                while let Some(subs_char) = clean_chars.next() {
                                    if subs_char == '}' { break; }
                                    if subs_char == ' ' { continue; }

                                    if subs_char == ',' { 
                                        left_side = false; 
                                        continue;
                                    }

                                    if left_side {
                                        left_word.push(subs_char);
                                    } else {
                                        right_word.push(subs_char);
                                    }
                                } 

                                let mut start_index: Option<usize> = None;
                                
                                if !left_word.is_empty() {
                                    start_index = left_word.parse()
                                        // TODO: refactor it to handle errors better
                                        .and_then(|r| Ok(Some(r)))
                                        .expect("modify to return a compilation error");
                                }

                                let mut end_index: Option<usize> = None;                                    
                                if !right_word.is_empty() {
                                    end_index = right_word.parse()
                                        // TODO: refactor it to handle errors better
                                        .and_then(|r| Ok(Some(r)))
                                        .expect("modify to return a compilation error");
                                }

                                modifiers.push(Modifier::Substr(start_index, end_index));
                            }                                
                            w => {
                                println!("rest w: '{w}', curr_char: '{curr_char}'");
                                
                                if inside_modifiers {
                                    todo!() // this should return compilation error
                                }

                                if inside_group {
                                    mod_target.push_str(&word);
                                    mod_target.push(' ');
                                    println!("mod_target: {mod_target}")
                                } else {
                                    parts.push(TokenPart::Plain(word.clone()));                                    
                                }

                            }
                        }           

                        word.clear();
                    }
                }
           
                println!("the end");
                
                if inside_brackets {
                    println!("inside_brackets");
                    todo!() // this should return compilation error
                } 

                if inside_group {
                    println!("inside_group");
                    todo!() // this should return compilation error
                }                

                if inside_modifiers {
                    println!("inside_modifiers");
                    todo!() // this should return compilation error
                }

                if word.len() > 0 {
                    parts.push(TokenPart::Plain(word.clone()));
                }
       
                println!("parts: {parts:?}");
                let result = build_string(parts);
                println!("result: {result:?}");
                                
                let mut new_lit = TokenTree::Literal(Literal::string(&result));
                new_lit.set_span(lit.span());

                output.push(new_lit);
                continue;
            }

            t => {                

                output.push(t);
            },
        }
    }
    TokenStream::from_iter(output)
}
#[cfg(test)]
mod tests {
    use quote::quote; 
    use proc_macro2::TokenStream; 
    use super::scan_tokens;
    
    #[allow(dead_code)]
    struct TestStruct {}

    #[allow(dead_code)]
    struct TestItems (Vec<i64>);

    fn assert_simple_transforms(arguments: Vec<TokenStream>, expected: &str) {
        for input in arguments {
            let result = scan_tokens(TokenStream::from(input));
            let result = result.to_string(); 
            assert_eq!(result, expected, "Welded tokens didnt match: {{ res: {result}, exp: {expected} }}",);
        }
    }

    #[test]
    fn should_transform_to_lower_case() {
        let arguments = vec![
            quote!{ @[(get_ TestStruct | lower) ] },
            quote!{ @[(get_ TestStruct | lowercase) ] },
        ];

        assert_simple_transforms(arguments, "get_teststruct"); 
    }

    #[test]
    fn should_transform_to_upper_case() {
        let arguments = vec![
            quote!{ @[(get_ TestStruct | uppercase) ] },
            quote!{ @[(get_ TestStruct | upper) ] },
        ];

        assert_simple_transforms(arguments, "GET_TESTSTRUCT");
    }

    #[test]
    fn should_transform_to_pascal_case() {
        let arguments = vec![
            quote!{ @[(get_ TestStruct | pascal) ] },
            quote!{ @[(get_ TestStruct | pascalcase) ] },
            quote!{ @[(get_ TestStruct | uppercamelcase) ] },            
        ];

        assert_simple_transforms(arguments, "GetTestStruct");
    }

    #[test]
    fn should_transform_to_camel_case() {
        let arguments = vec![
            quote!{ @[(get_ TestStruct | lowercamelcase) ] },
            quote!{ @[(get_ TestStruct | camelcase) ] }, 
            quote!{ @[(get_ TestStruct | camel) ] },
        ];

        assert_simple_transforms(arguments, "getTestStruct");
    }

    #[test]
    fn should_transform_to_snake_case() {
        let arguments = vec![
            quote!{ @[get_ (TestStruct | snakecase) ] },
            quote!{ @[get_ (TestStruct | snake) ] }, 
            quote!{ @[get_ (TestStruct | snekcase) ] },
            quote!{ @[get_ (TestStruct | snek) ] }, 
        ];

        assert_simple_transforms(arguments, "get_test_struct");
    }

    #[test]
    fn should_transform_to_shouty_snake_case() {
        let arguments = vec![
            quote!{ @[(get_ TestStruct | shoutysnakecase) ] },
            quote!{ @[(get_ TestStruct | shoutysnake) ] }, 
            quote!{ @[(get_ TestStruct | shoutysnekcase) ] },
            quote!{ @[(get_ TestStruct | shoutysnek) ] }, 
        ];

        assert_simple_transforms(arguments, "GET_TEST_STRUCT");
    }

    #[test]    
    fn should_transform_to_kebab_case() {
        let arguments = vec![
            quote!{ "@[(get_ TestStruct | kebabcase) ]" },
            quote!{ "@[(get_ TestStruct | kebab) ]" }, 
        ];

        assert_simple_transforms(arguments, "\"get-test-struct\"");
    }

    #[test]    
    fn should_transform_to_shouty_kebab_case() {
        let arguments = vec![
            (quote!{ "@[(get_ TestStruct | shoutykebabcase) ]" }, "\"GET-TEST-STRUCT\""),
            (quote!{ "@[(get_ TestStruct | shoutykebab) ]" }, "\"GET-TEST-STRUCT\""),
        ];

        for (input, expected) in arguments {
            
            let result = scan_tokens(TokenStream::from(input));
            let result = result.to_string(); 
            assert_eq!(result, expected, "Welded tokens didnt match: {{ res: {result}, exp: {expected} }}",);
        } 
    }

    #[test]    
    fn should_transform_to_misc_cases() {
        let arguments = vec![
            (quote!{ @[(get__ TestStruct | titlecase) ] }, "GetTestStruct"),
            (quote!{ @[(get_ TestStruct | title) ] }, "GetTestStruct"),
            
            (quote!{ "@[(get_ TestStruct | traincase) ]" }, "\"Get-Test-Struct\""),
            (quote!{ "@[(get_ TestStruct | train) ]" }, "\"Get-Test-Struct\""),
        ];

        for (input, expected) in arguments {
            
            let result = scan_tokens(TokenStream::from(input));
            let result = result.to_string(); 
            assert_eq!(result, expected, "Welded tokens didnt match: {{ res: {result}, exp: {expected} }}",);
        } 
    }

    #[test]
    fn should_transform_to_singular() {
        let arguments = vec![
            quote!{ @[(get_ TestItems | singular) ] },
            quote!{ @[(get_ TestItem | singular) ] },

        ];

        assert_simple_transforms(arguments, "get_TestItem");
    }

    #[test]
    fn should_transform_to_plural() {
        let arguments = vec![
            quote!{ @[(get_ TestItems | plural) ] },
            quote!{ @[(get_ TestItem | plural) ] },            
        ];

        assert_simple_transforms(arguments, "get_TestItems");
    }

    #[test]    
    fn should_chain_piped_modifiers() {
        let arguments = vec![            
            (quote!{ @[(get__ TestStruct | upper | snek) ] }, "get_teststruct"),
            (quote!{ @[(get__ TestStruct | snek | upper) ] }, "GET_TEST_STRUCT"),
            (quote!{ @[(get_ TestStruct | replace{"Struct", "_Info"} | camel )] }, "getTestInfo"),
            (quote!{ @[(get_ TestStruct | replace{"Struct", "_Info"} | camel ) ById] }, "getTestInfoById"),
            
            (quote!{ "@[(get_ TestStruct | replace{'Struct', '_Info'} | camel ) ById]" }, "\"getTestInfoById\""),
            (quote!{ "@[(get_ TestStruct | snek | camel ) ById]" }, "\"getTestStructById\""),             
            (quote!{ "@[(get_ TestStruct | replace{\"Struct\", \"_Info\"} | camel ) ById]" }, "\"getTestInfoById\""),
            (quote!{ "@[(get_ TestStruct | replace{\"Struct\", \"_Info\"} | kebab) - by -id]" }, "\"get-test-info-by-id\""),
        ];

        for (input, expected) in arguments {
            
            let result = scan_tokens(TokenStream::from(input));
            let result = result.to_string(); 
            assert_eq!(result, expected, "Welded tokens didnt match: {{ res: {result}, exp: {expected} }}",);
        } 
    }
}