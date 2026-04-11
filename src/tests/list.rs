use proc_macro2::TokenStream;
use quote::quote;

use crate::{scan_tokens, scanner::IDENT_EMPTY_MSG, tests::helpers::{assert_failure, assert_transformations, assert_transformations_same_output}};

#[test]
fn should_apply_repeat_list() {
    let arguments = vec![
        (
            quote! { @[get_ [Test_Struct] | repeat{2}] },
            "get_Test_StructTest_Struct",
        ),
        (
            quote! { @["get-" [Test-Struct] | rep{2}] },
            "\"get-Test-StructTest-Struct\"",
        ),
        (
            quote! { @[["get-" Test-Struct] | times{2}] },
            "\"get-Test-Structget-Test-Struct\"",
        ),
    ];

    assert_transformations(arguments);
}


#[test]
fn should_apply_reverse_list() {
    let arguments = vec![
        (quote! { @[get_ Test_Struct | reverse] }, "get_tcurtS_tseT"),
        (
            quote! { @[["get-" Test-Struct] | reverse | join{','} ] },
            "\"Struct,-,Test,get-\"",
        ),
        (
            quote! { @[["get-" TestStruct] | reverse | pascal | reverse ] },
            "\"GetTestStruct\"",
        ),
        (
            quote! { @[["get-" TestStruct | reverse | pascal | reverse ]] },
            "\"get-TestStrucT\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_slice_list() {
    let arguments = vec![
        (quote! { @[[get_ Test_Struct] | slice{-4}] }, "get_ruct"),
        (quote! { @[(get_ Test_Struct) | slice{-4}] }, "ruct"),
        (quote! { @[get_ Test_Struct | slice{5}] }, "get_Struct"),
        (quote! { @[(get_ Test_Struct)| slice{,8}] }, "get_Test"),
        (
            quote! { @[(_get_ Test_Struct) | slice{1, -4}] },
            "get_Test_St",
        ),
        (quote! { @[(_get_ Test_Struct) | slice{-6, -4}] }, "St"),
        (quote! { @[get_ Test_Struct| slice{-4,-6}] }, "get_"),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_slice_list_fail() {
    let arguments = vec![        
        (quote! { @[[get_ Test_Struct] | slice{2}] }, IDENT_EMPTY_MSG),               
        (quote! { @[[get_ Test_Struct] | slice{-4}] }, IDENT_EMPTY_MSG),               
    ];

    assert_failure(arguments);
}


