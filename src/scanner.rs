use proc_macro2::TokenStream;
use proc_macro2::{TokenTree, Delimiter, Group};
use quote::format_ident;
use syn::parse2;

use crate::{builder::build_string, parser::BrazeDsl};

pub fn scan_tokens(input: TokenStream) -> TokenStream {
    let mut output = Vec::new();
    let mut tokens = input.into_iter().peekable();

    while let Some(tree) = tokens.next() {
        match tree {
            // Checking for the '@' hook
            TokenTree::Punct(ref p) if p.as_char() == '@' => {
                if let Some(TokenTree::Group(g)) = tokens.peek() {
                    if g.delimiter() == Delimiter::Bracket {
                        // We found @[ ... ]! Consume the bracket group.
                        let bracket_group = if let TokenTree::Group(g) = tokens.next().unwrap() { g } else { unreachable!() };
                        
                        // Process the naming DSL inside the brackets
                        let dsl: BrazeDsl = parse2(bracket_group.stream()).expect("Invalid Naming DSL");
                        let generated_name = build_string(dsl);
                        
                        output.push(TokenTree::Ident(format_ident!("{}", generated_name)));
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
            _ => output.push(tree),
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

    // #[test]
    #[allow(dead_code, reason = "will make sense once it allows for string interpolation")]
    fn should_transform_to_kebab_case() {
        let arguments = vec![
            quote!{ @[(get_ TestStruct | kebabcase) ] },
            quote!{ @[(get_ TestStruct | kebab) ] }, 
        ];

        assert_simple_transforms(arguments, "get-test-struct");
    }

    // #[test]
    #[allow(dead_code, reason = "will make sense once it allows for string interpolation")]
    fn should_transform_to_shouty_kebab_case() {
        let arguments = vec![
            (quote!{ @[(get_ TestStruct | shoutykebabcase) ] }, "GET-TEST-STRUCT"),
            (quote!{ @[(get_ TestStruct | shoutykebab) ] }, "GET-TEST-STRUCT"),
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
            // TODO: uncomment when it allows for string interpolation
            // (quote!{ @[(get_ TestStruct | traincase) ] }, "Get-Test-Struct"),
            // (quote!{ @[(get_ TestStruct | train) ] }, "Get-Test-Struct"),
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
            
            // TODO: uncomment when it allows for string interpolation
            // this is throwing an error: need to 
            // (quote!{ @[(get_ TestStruct | traincase) ] }, "Get-Test-Struct"),
            // (quote!{ @[(get_ TestStruct | train) ] }, "Get-Test-Struct"),
        ];

        for (input, expected) in arguments {
            
            let result = scan_tokens(TokenStream::from(input));
            let result = result.to_string(); 
            assert_eq!(result, expected, "Welded tokens didnt match: {{ res: {result}, exp: {expected} }}",);
        } 
    }
}