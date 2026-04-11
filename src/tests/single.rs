
use quote::quote;

use crate::tests::helpers::{assert_transformations, assert_transformations_same_output};

#[test]
fn should_apply_pad_on_left() {
    let arguments = vec![
        (
            quote! { @[(get_ Test_Struct) | padstart{20, "_"}] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "_____get_Test_Struct",
        ),
        (
            quote! { @[("get-" Test-Struct) | padleft{20, "-"} ] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "\"-----get-Test-Struct\"",
        ),
        (
            quote! { @[(get "-" Test-Struct) | padstart{20, "-"} | lower ] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "\"-----get-test-struct\"",
        ),
        (
            quote! { @[(get_ Test_Struct) | padstart{5, "_"}] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "get_Test_Struct",
        ),
        (
            quote! { @[("get-" Test-Struct) | padleft{5, "-"} ] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "\"get-Test-Struct\"",
        ),
        (
            quote! { @[(get "-" Test-Struct) | padstart{16, "-"} | lower ] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "\"-get-test-struct\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_pad_on_right() {
    let arguments = vec![
        (
            quote! { @[(get_ Test_Struct) | padend{20, "_"}] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "get_Test_Struct_____",
        ),
        (
            quote! { @[("get-" Test-Struct) | padright{20, "-"} ] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "\"get-Test-Struct-----\"",
        ),
        (
            quote! { @[(get "-" Test-Struct) | padend{20, "-"} | lower ] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "\"get-test-struct-----\"",
        ),
        (
            quote! { @[(get_ Test_Struct) | padr{5, "_"}] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "get_Test_Struct",
        ),
        (
            quote! { @[("get-" Test-Struct) | padend{5, "-"} ] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "\"get-Test-Struct\"",
        ),
        (
            quote! { @[(get "-" Test-Struct) | padRight{16, "-"} | lower ] },
            /*
            0         1         2         3
            0123456789012345678901234567890 */
            "\"get-test-struct-\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_repeat_single() {
    let arguments = vec![
        (
            quote! { @[(get_ Test_Struct) | repeat{2}] },
            "get_Test_Structget_Test_Struct",
        ),
        (
            quote! { @["get-" (Test-Struct) | rep{2}] },
            "\"get-Test-StructTest-Struct\"",
        ),
        (
            quote! { @[("get-" Test-Struct) | times{2}] },
            "\"get-Test-Structget-Test-Struct\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_reverse_single() {
    let arguments = vec![
        (quote! { @[get_ Test_Struct | reverse] }, "get_tcurtS_tseT"),
        (
            quote! { @[("get-" Test-Struct) | reverse ] },
            "\"tcurtS-tseT-teg\"",
        ),
        (
            quote! { @[("get-" TestStruct) | reverse | pascal | reverse ] },
            "\"geTTestStrucT\"",
        ),
        (
            quote! { @[("get-" TestStruct | reverse | pascal | reverse )] },
            "\"get-TestStrucT\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_slice_single() {
    let arguments = vec![
        (quote! { @[get_ Test_Struct | slice{-4}] }, "get_ruct"),
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
fn should_apply_splice_into() {
    let arguments = vec![
        (quote! { @[(get_ Test_Struct)| splice{into, 1}] }, "g"),
        (quote! { @[(get_ Test_Struct)| splice{value, 1}] }, "g"),
        (quote! { @[(get_ Test_Struct)| splice{val, 1}] }, "g"),
        (
            quote! { @[(get_ Test_Struct)| splice{into, 1, 4}] },
            "gTest_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{value, 1, 4}] },
            "gTest_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{val, 1, 4}] },
            "gTest_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{into, 1, 4, "ot_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{value, 1, 4, "ot_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{val,, 4, "got_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{into,, 4, "got_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{value,, 4, "got_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{val,, 4, "got_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{into, 1,, "ot_"}] },
            "got_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{value, 1,, "ot_"}] },
            "got_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{val, 1,, "ot_"}] },
            "got_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{into,,, "new"}] },
            "new",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{value,,, "new"}] },
            "new",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{val,,, "new"}] },
            "new",
        ),
        // alias spliceinto
        (quote! { @[(get_ Test_Struct)| spliceinto{ 1}] }, "g"),
        (
            quote! { @[(get_ Test_Struct)| spliceinto{ 1, 4}] },
            "gTest_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| spliceinto{ 1, 4, "ot_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| spliceinto{, 4, "got_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| spliceinto{ 1,, "ot_"}] },
            "got_",
        ),
        (
            quote! { @[(get_ Test_Struct)| spliceinto{,, "new"}] },
            "new",
        ),
        // alias splice_into
        (quote! { @[(get_ Test_Struct)| splice_into{ 1}] }, "g"),
        (
            quote! { @[(get_ Test_Struct)| splice_into{ 1, 4}] },
            "gTest_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice_into{ 1, 4, "ot_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice_into{, 4, "got_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice_into{ 1,, "ot_"}] },
            "got_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice_into{,, "new"}] },
            "new",
        ),
        // negative indexes
        (
            quote! { @[(get_ Test_Struct)| splice{into, -5}] },
            "get_Test_S",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{into, -5, -1}] },
            "get_Test_St",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{into, -5, -1, "ot_"}] },
            "get_Test_Sot_t",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_splice_out() {
    let arguments = vec![
        (
            quote! { @[(get_ Test_Struct)| splice{out, 1}] },
            "et_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{removed, 1}] },
            "et_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{rm, 1}] },
            "et_Test_Struct",
        ),
        (quote! { @[(get_ Test_Struct)| splice{out, 1, 4}] }, "et_"),
        (
            quote! { @[(get_ Test_Struct)| splice{removed, 1, 4}] },
            "et_",
        ),
        (quote! { @[(get_ Test_Struct)| splice{rm, 1, 4}] }, "et_"),
        (
            quote! { @[(get_ Test_Struct)| splice{out, 1, 4, "ot_"}] },
            "et_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{removed, 1, 4, "ot_"}] },
            "et_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{out,, 4, "got_"}] },
            "get_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{removed,, 4, "got_"}] },
            "get_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{rm,, 4, "got_"}] },
            "get_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{out, 1,, "ot_"}] },
            "et_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{removed, 1,, "ot_"}] },
            "et_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{rm, 1,, "ot_"}] },
            "et_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{out,,, "new"}] },
            "get_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{removed,,, "new"}] },
            "get_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{rm,,, "new"}] },
            "get_Test_Struct",
        ),
        // alias spliceout
        (
            quote! { @[(get_ Test_Struct)| spliceout {1, } ] },
            "et_Test_Struct",
        ),
        (quote! { @[(get_ Test_Struct)| spliceout{ 1, 4}] }, "et_"),
        (
            quote! { @[(get_ Test_Struct)| spliceout{ 1, 4, "ot_"}] },
            "et_",
        ),
        (
            quote! { @[(get_ Test_Struct)| spliceout{, 4, "got_"}] },
            "get_",
        ),
        (
            quote! { @[(get_ Test_Struct)| spliceout{ 1,, "ot_"}] },
            "et_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| spliceout{,, "new"}] },
            "get_Test_Struct",
        ),
        // alias splice_out
        (
            quote! { @[(get_ Test_Struct)| splice_out {1, } ] },
            "et_Test_Struct",
        ),
        (quote! { @[(get_ Test_Struct)| splice_out{ 1, 4}] }, "et_"),
        (
            quote! { @[(get_ Test_Struct)| splice_out{ 1, 4, "ot_"}] },
            "et_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice_out{, 4, "got_"}] },
            "get_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice_out{ 1,, "ot_"}] },
            "et_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice_out{,, "new"}] },
            "get_Test_Struct",
        ),
        // negative indexes
        (quote! { @[(get_ Test_Struct)| splice{out, -5}] }, "truct"),
        (
            quote! { @[(get_ Test_Struct)| splice{out, -5, -1}] },
            "truc",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{out, -5, -1, "ot_"}] },
            "truc",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_splice_no_output() {
    let arguments = vec![
        (quote! { @[(get_ Test_Struct)| splice{,,, "new"}] }, "new"),
        (
            quote! { @[(get_ Test_Struct)| splice{, 1,, "ot_"}] },
            "got_",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{,, 4, "got_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{, 1, 4, "ot_"}] },
            "got_Test_Struct",
        ),
        (
            quote! { @[(get_ Test_Struct)| splice{, 1, 4}] },
            "gTest_Struct",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_apply_split_single() {
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
            quote! { @[(("get-" Test - Struct) | split{"-"} | lower | join{"_"}) -by-id] },
            "\"get_test_struct-by-id\"",
        ),
        (
            quote! { @[(("get-" Test - Struct) | split{6} | lower | join{"_"})] },
            "\"get-te_st-struct\"",
        ),        
    ];

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
        //              01234567890123456789
        //              0         1
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
        //               01234567890123456789
        //               0         1
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
        //               01234567890123456789
        //               0         1
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
fn should_apply_splice_default_mode_single() {
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