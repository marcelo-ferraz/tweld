use quote::quote;

use crate::{
    scanner::IDENT_EMPTY_MSG,
    tests::helpers::{assert_failure, assert_transformations},
};

#[test]
fn should_apply_singular_on_list_group() {
    let arguments = vec![
        (
            quote! { @[[Users "Posts" Comments] | singular | join{"_"}] },
            "\"User_Post_Comment\"",
        ),
        (
            quote! { @[[Users Posts Comments] | singular | snek | join{"_"}] },
            "user_post_comment",
        ),
    ];
    assert_transformations(arguments);
}

#[test]
fn should_apply_plural_on_list_group() {
    let arguments = vec![
        (
            quote! { @[[User Post "Comment"] | plural | join{"_"}] },
            "\"Users_Posts_Comments\"",
        ),
        (
            quote! { @[[User Post Comment] | plural | lower | join{"_"}] },
            "users_posts_comments",
        ),
    ];
    assert_transformations(arguments);
}

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
        (quote! { @[[get_ Test_Struct] | slice{-1}] }, "Test_Struct"),
        (quote! { @[[get_ Test_Struct] | slice{1}] }, "Test_Struct"),
        (quote! { @[[get_ Test_Struct]| slice{,1}] }, "get_"),
        (quote! { @[[_get_ Test _ Struct] | slice{1, -2}] }, "Test"),
        (quote! { @[[_get_ Test _ Struct] | slice{1, 3}] }, "Test_"),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_slice_list_fail() {
    let arguments = vec![
        (quote! { @[[get_ Test_Struct] | slice{2}] }, IDENT_EMPTY_MSG),
        (
            quote! { @[[get_ Test_Struct] | slice{-4}] },
            IDENT_EMPTY_MSG,
        ),
    ];

    assert_failure(arguments);
}

#[test]
fn should_apply_split() {
    let arguments = vec![
        (
            quote! { @[["get_" Test_Struct] | split{"_"} | join{", "}] },
            "\"get, Test, Struct\"",
        ),
        (
            quote! { @[(["get-one" two - "3-4" Struct] | split{'-'} | lower | join{", "})] },
            "\"get, one, two, 3, 4, struct\"",
        ),
        (
            quote! { @[["get-" Test - Struct] | split{4} | lower | join{"_"}] },
            "\"get-_test_-_stru_ct\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn handling_complex_lists() {
    let arguments = vec![
        (
            quote! { @[getTestStruct  | snek | split{"_"} | title] },
            "GetTestStruct",
        ),
        (
            /*
             * 1 - group (get Test Struct) -> getTestStruct
             * 1 - to snake -> get_test_struct
             * 2 - split by "_" ->  [get, Test, Struct]
             * 3 - reverse ->  [Struct, Test, get,]
             * 4 - join with {"_"} ->  Struct_Test_get
             * 5 - to pascal  -> StructTestGet
             */
            quote! { @[(get Test Struct) | snek | split{"_"} | reverse | join{"_"}| pascal ] },
            "StructTestGet",
        ),
        (
            /*
             * 1 - group [get Test Struct] -> [get, Test, Struct]
             * 2 - reverse ->  [Struct, Test, get,]
             * 3 - join with {"_"} ->  Struct_Test_get
             * 4 - to pascal  -> StructTestGet
             */
            quote! { @[[get Test Struct] | reverse | join{"_"}| pascal ] },
            "StructTestGet",
        ),
        (
            /*
             * 1 - group (get - Test - Struct) -> getTestStruct
             * 2 - split by "-" ->  [get, Test, Struct]
             * 3 - slice 1 to 3 ->  [Test, Struct]
             */
            quote! { @[(get - Test - Struct) | split{"-"} | slice{1,3} ] },
            "TestStruct",
        ),
        (
            /*
             * 1 - group (get - Test - Struct) -> getTestStruct
             * 2 - split by "-" ->  [get, Test, Struct]
             * 3 - slice 1 to 3 ->  [Test, Struct]
             */
            quote! { @[(get - Test - Struct) | split{"-"} | splice{,1,2, "New"} ] },
            "getNewStruct",
        ),
        (
            /*
             * 1 - group [get Test Struct] -> [get, Test, Struct]
             * 2 - slice 1 to 3 ->  [Test, Struct]
             */
            quote! { @[[get Test Struct] | splice{,1,2, "New"} ] },
            "getNewStruct",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_replace_on_list_group() {
    // replace is applied to each item individually
    let arguments = vec![
        (
            quote! { @[[get TestStruct TestInfo] | replace{"Test", "My"}] },
            "getMyStructMyInfo",
        ),
        (
            quote! { @[["get" TestStruct TestInfo] | replace{"Test", "My"} | join{", "}] },
            "\"get, MyStruct, MyInfo\"",
        ),
        (
            quote! { @[([get TestStruct "TestInfo"] | replace{"Test", "My"} | snek | join{"_"})] },
            "\"get_my_struct_my_info\"",
        ),
    ];
    assert_transformations(arguments);
}

#[test]
fn should_apply_padstart_on_list_group() {
    // padstart applied to each item individually
    let arguments = vec![
        (
            quote! { @[(["get" Test Struct] | padstart{6, "_"} | join{","})] },
            "\"___get,__Test,Struct\"",
        ),
        (
            quote! { @[([a "bb" ccc] | padl{4, "0"} | join{","})] },
            "\"000a,00bb,0ccc\"",
        ),
    ];
    assert_transformations(arguments);
}

#[test]
fn should_apply_padend_on_list_group() {
    // padend applied to each item individually
    let arguments = vec![
        (
            quote! { @[([get "Test" Struct] | padend{6, "_"} | join{","})] },
            "\"get___,Test__,Struct\"",
        ),
        (
            quote! { @[([a "bb" ccc] | padr{4, "0"} | join{","})] },
            "\"a000,bb00,ccc0\"",
        ),
    ];
    assert_transformations(arguments);
}

#[test]
fn should_apply_repeat_on_list_group() {
    // repeat on a list group repeats the items N times
    let arguments = vec![
        (
            quote! { @[["get" Test] | repeat{2} | join{"_"}] },
            "\"get_Test_get_Test\"",
        ),
        (
            quote! { @[["get" Test] | rep{3} | join{""}] },
            "\"getTestgetTestgetTest\"",
        ),
        (
            quote! { @[["get" Test] | times{2} | lower | join{"_"}] },
            "\"get_test_get_test\"",
        ),
    ];
    assert_transformations(arguments);
}

#[test]
fn should_apply_join_without_separator() {
    let arguments = vec![
        (
            quote! { @[["get-" Test - Struct] | join{}] },
            "\"get-Test-Struct\"",
        ),
        (
            quote! { @[["get-" Test - Struct] | split{"-"} | lower | join] },
            "\"getteststruct\"",
        ),
    ];
    assert_transformations(arguments);
}

#[test]
fn should_apply_splice_default_mode_list() {
    // omitting the mode keyword defaults to into behaviour
    let arguments = vec![
        (quote! { @[[get_ Test_Struct]| splice{, 1}] }, "get_"),
        (quote! { @[[get_ Test_Struct]| splice{, 1, 2}] }, "get_"),
        (
            quote! { @[[get_ Test_Struct]| splice{, 1, 4, "ot_"}] },
            "get_ot_",
        ),
        (
            quote! { @[[get_ Test_Struct]| splice{,, 4, "got_"}] },
            "got_",
        ),
        (
            quote! { @[[get_ Test_Struct]| splice{, 1,, "ot_"}] },
            "get_ot_",
        ),
        (quote! { @[[get_ Test_Struct]| splice{,,, "new"}] }, "new"),
        // // negative indexes
        (quote! { @[[get_ Test_Struct]| splice{, -1}] }, "get_"),
        (
            quote! { @[[get_ Test_Struct]| splice{, -2, -1}] },
            "Test_Struct",
        ),
        (
            quote! { @[[get_ Test_Struct]| splice{, -2, -1, "ot_"}] },
            "ot_Test_Struct",
        ),
    ];
    assert_transformations(arguments);
}
