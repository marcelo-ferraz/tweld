#[cfg(test)]
mod casing;
#[cfg(test)]
mod single;
#[cfg(test)]
mod list;
#[cfg(test)]
mod helpers;

use crate::tests::helpers::{assert_transformations, assert_transformations_same_output};
use quote::quote;

#[allow(dead_code)]
struct TestStruct {}

#[allow(dead_code)]
struct TestItems(Vec<i64>);

#[test]
fn should_concatenate_words_in_brackets_str() {
    let arguments = vec![
        quote! { @["get" Test Struct] },
        quote! { @["get" Te stSt ruct] },
        quote! { @["g" et TestStruct] },
        quote! { @["get" TestStruct] },
        quote! { @["getTest" Struct] },
        quote! { @["getT" estSt ruct] },
    ];

    assert_transformations_same_output(arguments, "\"getTestStruct\"");
}

#[test]
fn should_concatenate_words_in_brackets() {
    let arguments = vec![
        quote! { @[get Test Struct] },
        quote! { @[get Te stSt ruct] },
        quote! { @[g et TestStruct] },
        quote! { @[get TestStruct] },
        quote! { @[getTest Struct] },
        quote! { @[getT estSt ruct] },
    ];

    assert_transformations_same_output(arguments, "getTestStruct");
}

#[test]
fn should_concatenate_words_in_groups_str() {
    let arguments = vec![
        quote! { @[(get "Test" Struct)] },
        quote! { @[(get "Te" stSt ruct)] },
        quote! { @[(g "et" TestStruct)] },
        quote! { @[(get "TestStruct")] },
        quote! { @[(getTest "Struct")] },
        quote! { @[("get" TestStruct)] },
    ];

    assert_transformations_same_output(arguments, "\"getTestStruct\"");
}

#[test]
fn should_concatenate_words_in_groups() {
    let arguments = vec![
        quote! { @[(get Test Struct)] },
        quote! { @[(get Te stSt ruct)] },
        quote! { @[(g et TestStruct)] },
        quote! { @[(get TestStruct)] },
        quote! { @[(getTest Struct)] },
        quote! { @[(getT estSt ruct)] },
    ];

    assert_transformations_same_output(arguments, "getTestStruct");
}

#[test]
// rethink on this
fn should_preserve_outside_str() {
    let arguments = vec![
        // it will keep the 2 spaces
        (
            quote! { @["Something  " ("get" Test Struct) " here!"]  },
            "\"Something  getTestStruct here!\"",
        ),
        (
            quote! { @["Something " (get Te stSt ruct) " here!"] },
            "\"Something getTestStruct here!\"",
        ),
        (
            quote! { @["Something  " (g et TestStruct) " \"here\"!"] },
            "\"Something  getTestStruct \\\"here\\\"!\"",
        ),
        (
            quote! { @["Something  (\"" get TestStruct "\") here!"] },
            "\"Something  (\\\"getTestStruct\\\") here!\"",
        ),
        (
            quote! { @["Something  \"" (get TestStruct) "\" here!"] },
            "\"Something  \\\"getTestStruct\\\" here!\"",
        ),
        (
            quote! { @["Something - " (getTestStruct) " Here!"] },
            "\"Something - getTestStruct Here!\"",
        ),
        (
            quote! { @["Some thing  " (getT estSt ruct) "here!"] },
            "\"Some thing  getTestStructhere!\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_transform_to_singular() {
    let arguments = vec![
        quote! { @[(get_ TestItems | singular) ] },
        quote! { @[(get_ TestItem | singular) ] },
    ];

    assert_transformations_same_output(arguments, "get_TestItem");
}

#[test]
fn should_transform_to_plural() {
    let arguments = vec![
        quote! { @[(get_ TestItems | plural) ] },
        quote! { @[(get_ TestItem | plural) ] },
    ];

    assert_transformations_same_output(arguments, "get_TestItems");
}

#[test]
fn should_apply_replace() {
    let arguments = vec![
        (
            quote! { @[(get_ TestStruct | replace{"Struct", "_Info"} )] },
            "get_Test_Info",
        ),
        (
            quote! { @[(get_ TestStruct | replace{"Struct", "_Info"} ) ById] },
            "get_Test_InfoById",
        ),
        (
            quote! { @[(("get_" TestStruct) | replace{"Struct", "_Info"} ) ById] },
            "\"get_Test_InfoById\"",
        ),
        (
            quote! { @[(("get_" TestStruct) | replace{"Struct", "_Info"} ) ById] },
            "\"get_Test_InfoById\"",
        ),
        (
            quote! { @[("get_ TestStruct" | replace{"Struct", "_Info"}) - by -id] },
            "\"get_ Test_Info-by-id\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_split() {
    let arguments = vec![
        (
            quote! { @[(get_ Test_Struct) | split{"_"} | title] },
            "GetTestStruct",
        ),
        (
            quote! { @[(get - Test-Struct) | split{"-"} | lower | join{"_"} ] },
            "get_test_struct",
        ),
        (
            quote! { @[(("get-" Test - Struct) | split{"-"} | lower | join{'_'}) -by-id] },
            "\"get_test_struct-by-id\"",
        ),
        (
            quote! { @[(("get-one" two - "3-4" Struct) | split{"-"} | lower | join{'_'}) -by-id] },
            "\"get_onetwo_3_4struct-by-id\"",
        ),
        (
            quote! { @[(["get-one" two - "3-4" Struct] | split{'-'} | lower | join{", "})] },
            "\"get, one, two, 3, 4, struct\"",
        ),
        (
            quote! { @[(("get-" Test - Struct) | split{"-"} | lower | join{"_"}) -by-id] },
            "\"get_test_struct-by-id\"",
        ),
        (
            quote! { @[(("get-" Test - Struct) | split{6} | lower | join{"_"})] },
            "\"get-te_st-struct\"",
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
fn should_ignore_invalid_ident_chars() {
    let arguments = vec![
        (
            quote! { @[(get "-" Test-Struct ^)| repeat{2} | lower ] },
            "\"get-test-structget-test-struct\"",
        ),
        (
            // in this case Struct will be modified, as ^ is ignored
            quote! { @[get "-" Test-Struct ^| repeat{2} | lower ] },
            "\"get-Test-structstruct\"",
        ),
        (quote! { @[(get > Test-Struct) | camel] }, "getTestStruct"),
        (
            quote! { @[("get" -> Test-Struct ^)| repeat{2} | lower ] },
            "\"get-test-structget-test-struct\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_chain_piped_modifiers() {
    let arguments = vec![
        (
            quote! { @[(get__ TestStruct) | upper | snek ] },
            "get_teststruct",
        ),
        (
            quote! { @[((get__ TestStruct) | snek | upper) ] },
            "GET_TEST_STRUCT",
        ),
        (
            quote! { @[((get_ TestStruct) | replace{"Struct", "_Info"} | camel )] },
            "getTestInfo",
        ),
        (
            quote! { @[((get_ TestStruct) | replace{"Struct", "_Info"} | camel ) ById] },
            "getTestInfoById",
        ),
        (
            quote! { @[(("get_" TestStruct | replace{"Struct", "_Info"}) | camel ) ById] },
            "\"getTestInfoById\"",
        ),
        (
            quote! { @[(("get_" TestStruct) | snek | camel ) ById] },
            "\"getTestStructById\"",
        ),
        (
            quote! { @[((get_ "TestStruct") | replace{"Struct", "_Info"} | camel ) ById] },
            "\"getTestInfoById\"",
        ),
        (
            quote! { @[(("get_" TestStruct) | replace{ "Struct", "_Info"} | kebab) - by -id] },
            "\"get-test-info-by-id\"",
        ),
        (
            quote! { @[g "e" t _ testStruct | replace{"Struct", "_Info"} | pascal] },
            "\"get_TestInfo\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_transform_multiple_groups_in_a_str() {
    let arguments = vec![
        (
            quote! {
                @["The functions are diferent " get TestStruct " != " ((get TestStruct )| replace {"Struct", "Info"}| replace{" ", ""}) " !"]
            },
            "\"The functions are diferent getTestStruct != getTestInfo !\"",
        ),
        (
            quote! {
                @["The functions are diferent " (get TestStruct) " != " ((get TestStruct) | replace {"Struct", "Info"}) " !"]
            },
            "\"The functions are diferent getTestStruct != getTestInfo !\"",
        ),
        (
            quote! {
                @[(("get_" TestStruct) | replace{"Struct", "_Info"} | camel ) ById " != " ((_get_ TestStruct) | plural | substr{1,}) ById]
            },
            "\"getTestInfoById != get_TestStructsById\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_handle_reserved_input_and_output() {
    let arguments = vec![
        (quote! { @[r#as]}, "r#as"),
        (quote! { @[r#break]}, "r#break"),
        (quote! { @[r#const]}, "r#const"),
        (quote! { @[r#continue]}, "r#continue"),
        (quote! { @[r#else]}, "r#else"),
        (quote! { @[r#enum]}, "r#enum"),
        (quote! { @[r#extern]}, "r#extern"),
        (quote! { @[r#false]}, "r#false"),
        (quote! { @[r#fn]}, "r#fn"),
        (quote! { @[r#for]}, "r#for"),
        (quote! { @[r#if]}, "r#if"),
        (quote! { @[r#impl]}, "r#impl"),
        (quote! { @[r#in]}, "r#in"),
        (quote! { @[r#let]}, "r#let"),
        (quote! { @[r#loop]}, "r#loop"),
        (quote! { @[r#match]}, "r#match"),
        (quote! { @[r#mod]}, "r#mod"),
        (quote! { @[r#move]}, "r#move"),
        (quote! { @[r#mut]}, "r#mut"),
        (quote! { @[r#pub]}, "r#pub"),
        (quote! { @[r#ref]}, "r#ref"),
        (quote! { @[r#return]}, "r#return"),
        (quote! { @[r#static]}, "r#static"),
        (quote! { @[r#struct]}, "r#struct"),
        (quote! { @[r#trait]}, "r#trait"),
        (quote! { @[r#true]}, "r#true"),
        (quote! { @[r#type]}, "r#type"),
        (quote! { @[r#unsafe]}, "r#unsafe"),
        (quote! { @[r#use]}, "r#use"),
        (quote! { @[r#where]}, "r#where"),
        (quote! { @[r#while]}, "r#while"),
        (quote! { @[r#async]}, "r#async"),
        (quote! { @[r#await]}, "r#await"),
        (quote! { @[r#dyn]}, "r#dyn"),
        (quote! { @[r#abstract]}, "r#abstract"),
        (quote! { @[r#become]}, "r#become"),
        (quote! { @[r#box]}, "r#box"),
        (quote! { @[r#do]}, "r#do"),
        (quote! { @[r#final]}, "r#final"),
        (quote! { @[r#macro]}, "r#macro"),
        (quote! { @[r#override]}, "r#override"),
        (quote! { @[r#priv]}, "r#priv"),
        (quote! { @[r#typeof]}, "r#typeof"),
        (quote! { @[r#unsized]}, "r#unsized"),
        (quote! { @[r#virtual]}, "r#virtual"),
        (quote! { @[r#yield]}, "r#yield"),
        (quote! { @[r#try]}, "r#try"),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_handle_reserved_input_not_output() {
    let arguments = vec![
        (quote! { @[r#as Not Reserved]}, "asNotReserved"),
        (quote! { @[r#break Not Reserved]}, "breakNotReserved"),
        (quote! { @[r#const Not Reserved]}, "constNotReserved"),
        (quote! { @[r#continue Not Reserved]}, "continueNotReserved"),
        (quote! { @[r#else Not Reserved]}, "elseNotReserved"),
        (quote! { @[r#enum Not Reserved]}, "enumNotReserved"),
        (quote! { @[r#extern Not Reserved]}, "externNotReserved"),
        (quote! { @[r#false Not Reserved]}, "falseNotReserved"),
        (quote! { @[r#fn Not Reserved]}, "fnNotReserved"),
        (quote! { @[r#for Not Reserved]}, "forNotReserved"),
        (quote! { @[r#if Not Reserved]}, "ifNotReserved"),
        (quote! { @[r#impl Not Reserved]}, "implNotReserved"),
        (quote! { @[r#in Not Reserved]}, "inNotReserved"),
        (quote! { @[r#let Not Reserved]}, "letNotReserved"),
        (quote! { @[r#loop Not Reserved]}, "loopNotReserved"),
        (quote! { @[r#match Not Reserved]}, "matchNotReserved"),
        (quote! { @[r#mod Not Reserved]}, "modNotReserved"),
        (quote! { @[r#move Not Reserved]}, "moveNotReserved"),
        (quote! { @[r#mut Not Reserved]}, "mutNotReserved"),
        (quote! { @[r#pub Not Reserved]}, "pubNotReserved"),
        (quote! { @[r#ref Not Reserved]}, "refNotReserved"),
        (quote! { @[r#return Not Reserved]}, "returnNotReserved"),
        (quote! { @[r#static Not Reserved]}, "staticNotReserved"),
        (quote! { @[r#struct Not Reserved]}, "structNotReserved"),
        (quote! { @[r#trait Not Reserved]}, "traitNotReserved"),
        (quote! { @[r#true Not Reserved]}, "trueNotReserved"),
        (quote! { @[r#type Not Reserved]}, "typeNotReserved"),
        (quote! { @[r#unsafe Not Reserved]}, "unsafeNotReserved"),
        (quote! { @[r#use Not Reserved]}, "useNotReserved"),
        (quote! { @[r#where Not Reserved]}, "whereNotReserved"),
        (quote! { @[r#while Not Reserved]}, "whileNotReserved"),
        (quote! { @[r#async Not Reserved]}, "asyncNotReserved"),
        (quote! { @[r#async Not Reserved]}, "asyncNotReserved"),
        (quote! { @[r#await Not Reserved]}, "awaitNotReserved"),
        (quote! { @[r#dyn Not Reserved]}, "dynNotReserved"),
        (quote! { @[r#abstract Not Reserved]}, "abstractNotReserved"),
        (quote! { @[r#become Not Reserved]}, "becomeNotReserved"),
        (quote! { @[r#box Not Reserved]}, "boxNotReserved"),
        (quote! { @[r#do Not Reserved]}, "doNotReserved"),
        (quote! { @[r#final Not Reserved]}, "finalNotReserved"),
        (quote! { @[r#macro Not Reserved]}, "macroNotReserved"),
        (quote! { @[r#override Not Reserved]}, "overrideNotReserved"),
        (quote! { @[r#priv Not Reserved]}, "privNotReserved"),
        (quote! { @[r#typeof Not Reserved]}, "typeofNotReserved"),
        (quote! { @[r#unsized Not Reserved]}, "unsizedNotReserved"),
        (quote! { @[r#virtual Not Reserved]}, "virtualNotReserved"),
        (quote! { @[r#yield Not Reserved]}, "yieldNotReserved"),
        (quote! { @[r#try Not Reserved]}, "tryNotReserved"),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_handle_normal_input_reserved_output() {
    let arguments = vec![
        (quote! { @[a s]}, "r#as"),
        (quote! { @[b reak]}, "r#break"),
        (quote! { @[c onst]}, "r#const"),
        (quote! { @[c ontinue]}, "r#continue"),
        (quote! { @[e lse]}, "r#else"),
        (quote! { @[e num]}, "r#enum"),
        (quote! { @[e xtern]}, "r#extern"),
        (quote! { @[f alse]}, "r#false"),
        (quote! { @[f n]}, "r#fn"),
        (quote! { @[f or]}, "r#for"),
        (quote! { @[i f]}, "r#if"),
        (quote! { @[i mpl]}, "r#impl"),
        (quote! { @[i n]}, "r#in"),
        (quote! { @[l et]}, "r#let"),
        (quote! { @[l oop]}, "r#loop"),
        (quote! { @[m atch]}, "r#match"),
        (quote! { @[m od]}, "r#mod"),
        (quote! { @[m ove]}, "r#move"),
        (quote! { @[m ut]}, "r#mut"),
        (quote! { @[p ub]}, "r#pub"),
        (quote! { @[r ef]}, "r#ref"),
        (quote! { @[r eturn]}, "r#return"),
        (quote! { @[s tatic]}, "r#static"),
        (quote! { @[s truct]}, "r#struct"),
        (quote! { @[t rait]}, "r#trait"),
        (quote! { @[t rue]}, "r#true"),
        (quote! { @[t ype]}, "r#type"),
        (quote! { @[u nsafe]}, "r#unsafe"),
        (quote! { @[u se]}, "r#use"),
        (quote! { @[w here]}, "r#where"),
        (quote! { @[w hile]}, "r#while"),
        (quote! { @[a sync]}, "r#async"),
        (quote! { @[a wait]}, "r#await"),
        (quote! { @[d yn]}, "r#dyn"),
        (quote! { @[a bstract]}, "r#abstract"),
        (quote! { @[b ecome]}, "r#become"),
        (quote! { @[b ox]}, "r#box"),
        (quote! { @[d o]}, "r#do"),
        (quote! { @[f inal]}, "r#final"),
        (quote! { @[m acro]}, "r#macro"),
        (quote! { @[o verride]}, "r#override"),
        (quote! { @[p riv]}, "r#priv"),
        (quote! { @[t ypeof]}, "r#typeof"),
        (quote! { @[u nsized]}, "r#unsized"),
        (quote! { @[v irtual]}, "r#virtual"),
        (quote! { @[y ield]}, "r#yield"),
        (quote! { @[t ry]}, "r#try"),
    ];

    assert_transformations(arguments);
}

#[test]
fn handling_forbidden_keywords() {
    let arguments = vec![
        // `crate` cannot be a raw identifier
        (quote! { @[not cra te]}, "notcrate"),
        // self` cannot be a raw identifier
        (quote! { @[__ selfie | replace{"ie",""}]}, "__self"),
        // `super` cannot be a raw identifier
        (quote! { @[(sup er-duper)|camel] }, "superDuper"),
    ];
    assert_transformations(arguments);
}

#[test]
fn handling_nested_modifiers() {
    let arguments = vec![(
        quote! { @[([[[er sup] |reverse ]-duper]|pascal)| camel ] },
        "superDuper",
    )];
    assert_transformations(arguments);
}

#[test]
fn should_apply_substr_end_only() {
    let arguments = vec![
        (
            quote! { @[( a _ long identifier) | substr{, 9}] },
            "a_longide",
        ),
        (
            quote! { @[( a _ long identifier) | substring{, 9}] },
            "a_longide",
        ),
        (
            quote! { @[("a long identifier") | substr{, 9}] },
            "\"a long id\"",
        ),
        //            01234567890123456789
        //            0         1
    ];
    assert_transformations(arguments);
}

#[test]
fn should_apply_substr_start_only() {
    let arguments = vec![
        (
            quote! { @[( a long identifier) | substr{3}] },
            "ngidentifier",
        ),
        (
            quote! { @[( a long identifier) | substring{3}] },
            "ngidentifier",
        ),
        (
            quote! { @[("a long identifier") | substr{4}] },
            "\"ng identifier\"",
        ),
        //            01234567890123456789
        //            0         1
    ];
    assert_transformations(arguments);
}

#[test]
fn should_apply_substr_start_and_end() {
    let arguments = vec![
        (quote! { @[( a long identifier) | substr{1, 7}] }, "longid"),
        (
            quote! { @[( a long identifier) | substring{1, 7}] },
            "longid",
        ),
        (
            quote! { @[("a long identifier") | substr{2, 9}] },
            "\"long id\"",
        ),
        //            01234567890123456789
        //            0         1
    ];
    assert_transformations(arguments);
}

#[test]
fn should_apply_substr_no_args() {
    // no args returns the full value unchanged
    let arguments = vec![
        quote! { @[(a _ long identifier) | substr{}] },
        quote! { @[(a _ long identifier) | substring{}] },
    ];
    assert_transformations_same_output(arguments, "a_longidentifier");
}

#[test]
fn should_apply_substr_chained() {
    let arguments = vec![
        (
            quote! { @[(a Long identifier) | substr{, 9} | snek] },
            "a_longiden",
        ),
        (
            quote! { @[(a long identifier) | substr{2, 8} | upper] },
            "ONGIDE",
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
fn should_apply_splice_default_mode() {
    // omitting the mode keyword defaults to into behaviour
    let arguments = vec![
        (quote! { @[(get_ Test_Struct)| splice{, 1}] }, "g"),
        (
            quote! { @[(get_ Test_Struct)| splice{, 1, 4}] },
            "gTest_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{, 1, 4, "ot_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{,, 4, "got_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{, 1,, "ot_"}] },
            "got_",
        ),
        (quote! { @[(get_ Test_Struct)| splice{,,, "new"}] }, "new"),
        // negative indexes
        (quote! { @[(get_ Test_Struct)| splice{, -5}] }, "get_Test_S"),
        (
            quote! { @[(get_ Test_Struct)| splice{, -5, -1}] },
            "get_Test_St",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{, -5, -1, "ot_"}] },
            "get_Test_Sot_t",
        ),
    ];
    assert_transformations(arguments);
}
