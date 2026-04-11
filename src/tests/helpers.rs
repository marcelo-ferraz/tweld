use proc_macro2::TokenStream;

use crate::scan_tokens;

pub fn assert_transformations(arguments: Vec<(TokenStream, &'static str)>) {
    for (input, expected) in arguments {
        let result = scan_tokens(TokenStream::from(input)).unwrap();
        let result = result.to_string();

        assert_eq!(
            result, expected,
            "Welded tokens didnt match: {{ res: {result}, exp: {expected} }}",
        );
    }
}

pub fn assert_transformations_same_output(arguments: Vec<TokenStream>, expected: &str) {
    for input in arguments {
        let result = scan_tokens(TokenStream::from(input)).unwrap();
        let result = result.to_string();
        assert_eq!(
            result, expected,
            "Welded tokens didnt match: {{ res: {result}, exp: {expected} }}",
        );
    }
}

pub fn assert_failure(arguments: Vec<(TokenStream, &'static str)>) {
    for (input, expected) in arguments {
        match scan_tokens(TokenStream::from(input)) {
            Ok(result) => {
                panic!("This should've failed! {{{result}}}");                
            },
            Err(err) => {
                assert_eq!(
                    err.to_string(), expected,
                    "Error didnt match: {{ res: {}, exp: {expected} }}", err.to_string(),
                );
            },
        }
    }
}
