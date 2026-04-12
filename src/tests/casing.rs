use quote::quote;

use crate::tests::helpers::{assert_transformations, assert_transformations_same_output};

#[test]
fn should_transform_to_lower_case() {
    let arguments = vec![
        quote! { @[((get_ Test) | lower) (Struct | lower)] },
        quote! { @[((get_ TestStruct) | lowercase) ] },
    ];

    assert_transformations_same_output(arguments, "get_teststruct");
}

#[test]
fn should_transform_to_upper_case() {
    let arguments = vec![
        quote! { @[(get_ TestStruct )| uppercase ] },
        quote! { @[(get_ TestStruct) | upper ] },
    ];

    assert_transformations_same_output(arguments, "GET_TESTSTRUCT");
}

#[test]
fn should_transform_to_pascal_case() {
    let arguments = vec![
        quote! { @[(get_ TestStruct) | pascal ] },
        quote! { @[(get_ TestStruct) | pascalcase ] },
        quote! { @[(get_ TestStruct) | uppercamelcase ] },
    ];

    assert_transformations_same_output(arguments, "GetTestStruct");
}

#[test]
fn should_transform_to_camel_case() {
    let arguments = vec![
        quote! { @[(get_ TestStruct) | lowercamelcase ] },
        quote! { @[(get_ TestStruct) | camelcase ] },
        quote! { @[(get_ TestStruct) | camel ] },
    ];

    assert_transformations_same_output(arguments, "getTestStruct");
}

#[test]
fn should_transform_to_snake_case() {
    let arguments = vec![
        quote! { @[get_ (TestStruct | snakecase) ] },
        quote! { @[get_ (TestStruct | snake) ] },
        quote! { @[get_ (TestStruct | snekcase) ] },
        quote! { @[get_ (TestStruct | snek) ] },
    ];

    assert_transformations_same_output(arguments, "get_test_struct");
}

#[test]
fn should_transform_to_shouty_snake_case() {
    let arguments = vec![
        quote! { @[(get_ TestStruct) | shoutysnakecase ] },
        quote! { @[(get_ TestStruct) | shoutysnake ] },
        quote! { @[(get_ TestStruct) | shoutysnekcase ] },
        quote! { @[(get_ TestStruct) | shoutysnek ] },
    ];

    assert_transformations_same_output(arguments, "GET_TEST_STRUCT");
}

#[test]
fn should_transform_to_kebab_case() {
    let arguments = vec![
        quote! { @[(("get_" Test) | kebab) - (Struct | kebabcase) ] },
        quote! { @[(("get_" TestStruct) | kebab) ] },
    ];

    assert_transformations_same_output(arguments, "\"get-test-struct\"");
}

#[test]
fn should_transform_to_shouty_kebab_case() {
    let arguments = vec![
        (
            quote! { @["ge" t _ (TestStruct) | shoutykebabcase ] },
            "\"get_TEST-STRUCT\"",
        ),
        (
            quote! { @[("get_" TestStruct) | shoutykebabcase ] },
            "\"GET-TEST-STRUCT\"",
        ),
        (
            quote! { @[("get_" TestStruct) | shoutykebab ] },
            "\"GET-TEST-STRUCT\"",
        ),
    ];

    assert_transformations(arguments);
}

#[test]
fn should_transform_to_title_case() {
    let arguments = vec![
        quote! { @[("get__" TestStruct)| titlecase ] },
        quote! { @[("get_" TestStruct) | title ] },
        quote! { @[("get"- TestStruct) | title ] },
        quote! { @[("get"-- TestStruct) | title ] },
        quote! { @[("get"--_ TestStruct) | title ] },
        quote! { @[("get" _TestStruct) | title ] },
        quote! { @[("get" -TestStruct) | title ] },
    ];

    assert_transformations_same_output(arguments, "\"Get Test Struct\"");
}

#[test]
fn should_transform_to_train_case() {
    let arguments = vec![
        quote! { @[("get__" TestStruct) | traincase ] },
        quote! { @[("get_" TestStruct) | train ] },
        quote! { @[("get-" TestStruct) | train ] },
        quote! { @[("get--" TestStruct) | train ] },
        quote! { @[("get--" _ TestStruct) | train ] },
        quote! { @[("get" _TestStruct) | train ] },
        quote! { @[("get" -TestStruct) | train ] },
    ];

    assert_transformations_same_output(arguments, "\"Get-Test-Struct\"");
}
