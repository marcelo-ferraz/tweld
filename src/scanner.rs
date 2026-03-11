use std::iter::Peekable;
use std::str::Chars;

use proc_macro2::{Literal, TokenStream};
use proc_macro2::{TokenTree, Delimiter, Group};
use quote::{ToTokens, format_ident};
use syn::{LitStr, parse2};

use crate::models::{Modifier, TokenPart};
use crate::{builder::build_string, parser::BrazeDsl};

enum StringParserState {
    Idle,
    InsideBrackets,
    InsideGroup,
    Modifiers
}

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
                        let ident_name = build_string(dsl.parts).replace(" ", "");
                        
                        output.push(TokenTree::Ident(format_ident!("{}", ident_name)));
                        continue;
                    }
                }
                output.push(tree);
            }
            
            TokenTree::Group(g) => {                
                let inner_expanded = scan_tokens(g.stream());
                let mut new_group = Group::new(g.delimiter(), inner_expanded);
                new_group.set_span(g.span());
                output.push(TokenTree::Group(new_group));
            }

            TokenTree::Literal(lit)  => {
                let tokens = quote::quote!(#lit);
                
                let Ok(lit_str) = parse2::<LitStr>(tokens) else {                                      
                    output.push(TokenTree::Literal(lit));
                    continue;
                };
                
                let clean_string: String = lit_str.value();
                
                let dsl = BrazeDsl::parse_str(&clean_string)
                    .expect("There was an error when parsing the string");

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
            },
        }
    }
    TokenStream::from_iter(output)
}

#[cfg(test)]
mod tests {    use quote::quote;     use proc_macro2::TokenStream;     use super::scan_tokens;

    #[test]
    fn should_transform_to_singular2() {
        let arguments = vec![
            quote!{ "@[(get_ TestItems | replace{'est', 'urd'} | singular) ]" },
        ];

        assert_simple_transforms(arguments, "\"get_TurdItem\"");
    }

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
    fn should_transform_to_title_case() {
        let arguments = vec![
            quote!{ "@[(get__ TestStruct | titlecase) ]" },
            quote!{ "@[(get_ TestStruct | title) ]" },
            quote!{ "@[(get- TestStruct | title) ]" },
            quote!{ "@[(get-- TestStruct | title) ]" },
            quote!{ "@[(get--_ TestStruct | title) ]" },
            quote!{ "@[(get _TestStruct | title) ]" },
            quote!{ "@[(get -TestStruct | title) ]" },
        ];

        assert_simple_transforms(arguments, "\"Get Test Struct\"");
    }

    #[test]    
    fn should_transform_to_train_case() {
        let arguments = vec![
            quote!{ "@[(get__ TestStruct | traincase) ]" },
            quote!{ "@[(get_ TestStruct | train) ]" },
            quote!{ "@[(get- TestStruct | train) ]" },
            quote!{ "@[(get-- TestStruct | train) ]" },
            quote!{ "@[(get--_ TestStruct | train) ]" },
            quote!{ "@[(get _TestStruct | train) ]" },
            quote!{ "@[(get -TestStruct | train) ]" },
        ];

        assert_simple_transforms(arguments, "\"Get-Test-Struct\"");
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
    fn should_apply_replace() {
        let arguments = vec![                        
            (quote!{ @[(get_ TestStruct | replace{"Struct", "_Info"} )] }, "get_Test_Info"),
            (quote!{ @[(get_ TestStruct | replace{"Struct", "_Info"} ) ById] }, "get_Test_InfoById"),
            
            (quote!{ "@[(get_ TestStruct | replace{'Struct', '_Info'} ) ById]" }, "\"get_Test_InfoById\""),
            (quote!{ "@[(get_ TestStruct | replace{\"Struct\", \"_Info\"} ) ById]" }, "\"get_Test_InfoById\""),
            (quote!{ "@[(get_ TestStruct | replace{\"Struct\", \"_Info\"}) - by -id]" }, "\"get_Test_Info-by-id\""),
        ];

        for (input, expected) in arguments {
            
            let result = scan_tokens(TokenStream::from(input));
            let result = result.to_string(); 
            assert_eq!(result, expected, "Welded tokens didnt match: {{ res: {result}, exp: {expected} }}",);
        } 
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

    #[test]    
    fn should_transform_multiple_groups_in_a_str() {
        let arguments = vec![            
            (
                quote!{ 
                    "The functions are diferent @[(get TestStruct)] != @[(get TestStruct | replace {'Struct', 'Info'})] !" 
                }, 
                "\"The functions are diferent getTestStruct != getTestInfo !\""
            ),
            (
                quote!{ 
                    "The functions are diferent @[(get TestStruct)] != @[(get TestStruct | replace {'Struct', 'Info'})] !" 
                }, 
                "\"The functions are diferent getTestStruct != getTestInfo !\""
            ),
            // (
            //     quote!{ 
            //         "@[(get_ TestStruct | replace{'Struct', '_Info'} | camel ) ById] != @[(_get_ TestStruct | plural | substr{1,} ) ById]" 
            //     }, 
            //     "\"getTestInfoById != get_TestStructsById\""
            // ),

        ];

        for (input, expected) in arguments {
            
            let result = scan_tokens(TokenStream::from(input));
            let result = result.to_string(); 
            assert_eq!(result, expected, "Welded tokens didnt match: {{ res: {result}, exp: {expected} }}",);
        } 
    }

    
}