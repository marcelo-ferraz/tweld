mod models;
mod parser;
mod builder;
mod scanner;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{TokenTree, Delimiter, Group};
use quote::format_ident;
use syn::parse2;

use crate::scanner::scan_tokens;

#[proc_macro]
pub fn concat(input: TokenStream) -> TokenStream {
    scan_tokens(TokenStream2::from(input)).into()
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use crate::scan_tokens;

    struct TestStruct {}

    #[test]
    fn should_capture_correct_modifiers() {
        
        let input = quote::quote! { @[get_ (TestStruct | snek)] };
        
        let result = scan_tokens(TokenStream::from(input));

        assert_eq!(result.to_string(), "get_test_struct");
    }
}