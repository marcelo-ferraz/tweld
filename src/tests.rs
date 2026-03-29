
#[cfg(test)]
mod tests {
    use crate::scanner::scan_tokens;
    use proc_macro2::TokenStream;
    use quote::quote;

    #[allow(dead_code)]
    struct TestStruct {}

    #[allow(dead_code)]
    struct TestItems(Vec<i64>);

    fn assert_transformations(arguments: Vec<(TokenStream, &'static str)>) {
        for (input, expected) in arguments {
            let result = scan_tokens(TokenStream::from(input)).unwrap();
            println!("result {result:?}");
            let result = result.to_string();

            assert_eq!(
                result, expected,
                "Welded tokens didnt match: {{ res: {result}, exp: {expected} }}",
            );
        }
    }

    fn assert_transformations_same_output(arguments: Vec<TokenStream>, expected: &str) {
        for input in arguments {
            let result = scan_tokens(TokenStream::from(input)).unwrap();
            let result = result.to_string();
            assert_eq!(
                result, expected,
                "Welded tokens didnt match: {{ res: {result}, exp: {expected} }}",
            );
        }
    }


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
            (quote! { @["Something  " ("get" Test Struct) " here!"]  }, "\"Something  getTestStruct here!\""),
            
            (quote! { @["Something " (get Te stSt ruct) " here!"] }, "\"Something getTestStruct here!\""),
            (quote! { @["Something  " (g et TestStruct) " \"here\"!"] }, "\"Something  getTestStruct \\\"here\\\"!\""),
            (quote! { @["Something  (\"" get TestStruct "\") here!"] }, "\"Something  (\\\"getTestStruct\\\") here!\""),
            (quote! { @["Something  \"" (get TestStruct) "\" here!"] }, "\"Something  \\\"getTestStruct\\\" here!\""),
            (quote! { @["Something - " (getTestStruct) " Here!"] }, "\"Something - getTestStruct Here!\""),
            (quote! { @["Some thing  " (getT estSt ruct) "here!"] }, "\"Some thing  getTestStructhere!\""),
        ];

        assert_transformations(arguments);
    }

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
                quote! { @[(("get-" Test - Struct) | split{"-"} | lower | join{"_"}) -by-id] },
                "\"get_test_struct-by-id\"",
            ),
        ];

        assert_transformations(arguments);
    }


    #[test]
    fn should_split() {
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
    fn should_apply_repeat() {
        let arguments = vec![
            (
                quote! { @[get_ Test_Struct | repeat{2}] },
                "get_Test_StructTest_Struct",
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
    fn should_apply_reverse() {
        let arguments = vec![
            (
                quote! { @[get_ Test_Struct | reverse] },
                "get_tcurtS_tseT",
            ),
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
    fn should_apply_slice() {
        let arguments = vec![
            (
                quote! { @[get_ Test_Struct | slice{-4}] },
                "get_ruct",
            ),  
            (
                quote! { @[(get_ Test_Struct) | slice{-4}] },
                "ruct",
            ),  
            (
                quote! { @[get_ Test_Struct | slice{5}] },
                "get_Struct",
            ), 
            (
                quote! { @[(get_ Test_Struct)| slice{,8}] },                 
                "get_Test",
            ),       
            (
                quote! { @[(_get_ Test_Struct) | slice{1, -4}] },
                "get_Test_St",
            ),            
            (
                quote! { @[(_get_ Test_Struct) | slice{-6, -4}] },
                "St",
            ),      
                        
            (
                quote! { @[get_ Test_Struct| slice{-4,-6}] },
                "get_",
            ),       
        ];

        assert_transformations(arguments);
    }


    #[test]
    fn should_apply_splice_into() {
        let arguments = vec![
            (
                quote! { @[(get_ Test_Struct)| splice{into, 1}] },
                "g",
            ), 
            (
                quote! { @[(get_ Test_Struct)| splice{value, 1}] },
                "g",
            ), 
            (
                quote! { @[(get_ Test_Struct)| splice{val, 1}] },
                "g",
            ), 
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
            (
                quote! { @[(get_ Test_Struct)| spliceinto{ 1}] },
                "g",
            ), 
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
            (
                quote! { @[(get_ Test_Struct)| splice_into{ 1}] },
                "g",
            ), 
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
            (
                quote! { @[(get_ Test_Struct)| splice{out, 1, 4}] },
                "et_",
            ),        
            (
                quote! { @[(get_ Test_Struct)| splice{removed, 1, 4}] },
                "et_",
            ), 
            (
                quote! { @[(get_ Test_Struct)| splice{rm, 1, 4}] },
                "et_",
            ),        
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
            (
                quote! { @[(get_ Test_Struct)| spliceout{ 1, 4}] },
                "et_",
            ),                
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
            (
                quote! { @[(get_ Test_Struct)| splice_out{ 1, 4}] },
                "et_",
            ),                
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
        ];

        assert_transformations(arguments);
    }
     

    #[test]
    fn should_apply_splice_no_output() {            
        let arguments = vec![
            (
                quote! { @[(get_ Test_Struct)| splice{,,, "new"}] },
                "new",
            ),
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
            (
                quote! { @[(get > Test-Struct) | camel] },
                "getTestStruct",
            ),
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
                quote!{
                    @[(("get_" TestStruct) | replace{"Struct", "_Info"} | camel ) ById " != " ((_get_ TestStruct) | plural | substr{1,}) ById]
                },
                "\"getTestInfoById != get_TestStructsById\""
            ),
        ];

        assert_transformations(arguments);
    }

    #[test]
    fn should_handle_reserved_input_and_output() {
        let arguments = vec![            
            ( quote! { @[r#as]}, "r#as"),
            ( quote! { @[r#break]}, "r#break"),
            ( quote! { @[r#const]}, "r#const"),
            ( quote! { @[r#continue]}, "r#continue"),
            ( quote! { @[r#else]}, "r#else"),
            ( quote! { @[r#enum]}, "r#enum"),
            ( quote! { @[r#extern]}, "r#extern"),
            ( quote! { @[r#false]}, "r#false"),
            ( quote! { @[r#fn]}, "r#fn"),
            ( quote! { @[r#for]}, "r#for"),
            ( quote! { @[r#if]}, "r#if"),
            ( quote! { @[r#impl]}, "r#impl"),
            ( quote! { @[r#in]}, "r#in"),
            ( quote! { @[r#let]}, "r#let"),
            ( quote! { @[r#loop]}, "r#loop"),
            ( quote! { @[r#match]}, "r#match"),
            ( quote! { @[r#mod]}, "r#mod"),
            ( quote! { @[r#move]}, "r#move"),
            ( quote! { @[r#mut]}, "r#mut"),
            ( quote! { @[r#pub]}, "r#pub"),
            ( quote! { @[r#ref]}, "r#ref"),
            ( quote! { @[r#return]}, "r#return"),
            ( quote! { @[r#static]}, "r#static"),
            ( quote! { @[r#struct]}, "r#struct"),
            ( quote! { @[r#trait]}, "r#trait"),
            ( quote! { @[r#true]}, "r#true"),
            ( quote! { @[r#type]}, "r#type"),
            ( quote! { @[r#unsafe]}, "r#unsafe"),
            ( quote! { @[r#use]}, "r#use"),
            ( quote! { @[r#where]}, "r#where"),
            ( quote! { @[r#while]}, "r#while"),
            ( quote! { @[r#async]}, "r#async"),
            ( quote! { @[r#await]}, "r#await"),
            ( quote! { @[r#dyn]}, "r#dyn"),
            ( quote! { @[r#abstract]}, "r#abstract"),
            ( quote! { @[r#become]}, "r#become"),
            ( quote! { @[r#box]}, "r#box"),
            ( quote! { @[r#do]}, "r#do"),
            ( quote! { @[r#final]}, "r#final"),
            ( quote! { @[r#macro]}, "r#macro"),
            ( quote! { @[r#override]}, "r#override"),
            ( quote! { @[r#priv]}, "r#priv"),
            ( quote! { @[r#typeof]}, "r#typeof"),
            ( quote! { @[r#unsized]}, "r#unsized"),
            ( quote! { @[r#virtual]}, "r#virtual"),
            ( quote! { @[r#yield]}, "r#yield"),
            ( quote! { @[r#try]}, "r#try"),
        ];

        assert_transformations(arguments);
    }

    #[test]
    fn should_handle_reserved_input_not_output() {
        let arguments = vec![            
            ( quote! { @[r#as Not Reserved]}, "asNotReserved"),
            ( quote! { @[r#break Not Reserved]}, "breakNotReserved"),
            ( quote! { @[r#const Not Reserved]}, "constNotReserved"),
            ( quote! { @[r#continue Not Reserved]}, "continueNotReserved"),
            ( quote! { @[r#else Not Reserved]}, "elseNotReserved"),
            ( quote! { @[r#enum Not Reserved]}, "enumNotReserved"),
            ( quote! { @[r#extern Not Reserved]}, "externNotReserved"),
            ( quote! { @[r#false Not Reserved]}, "falseNotReserved"),
            ( quote! { @[r#fn Not Reserved]}, "fnNotReserved"),
            ( quote! { @[r#for Not Reserved]}, "forNotReserved"),
            ( quote! { @[r#if Not Reserved]}, "ifNotReserved"),
            ( quote! { @[r#impl Not Reserved]}, "implNotReserved"),
            ( quote! { @[r#in Not Reserved]}, "inNotReserved"),
            ( quote! { @[r#let Not Reserved]}, "letNotReserved"),
            ( quote! { @[r#loop Not Reserved]}, "loopNotReserved"),
            ( quote! { @[r#match Not Reserved]}, "matchNotReserved"),
            ( quote! { @[r#mod Not Reserved]}, "modNotReserved"),
            ( quote! { @[r#move Not Reserved]}, "moveNotReserved"),
            ( quote! { @[r#mut Not Reserved]}, "mutNotReserved"),
            ( quote! { @[r#pub Not Reserved]}, "pubNotReserved"),
            ( quote! { @[r#ref Not Reserved]}, "refNotReserved"),
            ( quote! { @[r#return Not Reserved]}, "returnNotReserved"),
            ( quote! { @[r#static Not Reserved]}, "staticNotReserved"),
            ( quote! { @[r#struct Not Reserved]}, "structNotReserved"),
            ( quote! { @[r#trait Not Reserved]}, "traitNotReserved"),
            ( quote! { @[r#true Not Reserved]}, "trueNotReserved"),
            ( quote! { @[r#type Not Reserved]}, "typeNotReserved"),
            ( quote! { @[r#unsafe Not Reserved]}, "unsafeNotReserved"),
            ( quote! { @[r#use Not Reserved]}, "useNotReserved"),
            ( quote! { @[r#where Not Reserved]}, "whereNotReserved"),
            ( quote! { @[r#while Not Reserved]}, "whileNotReserved"),
            ( quote! { @[r#async Not Reserved]}, "asyncNotReserved"),
            ( quote! { @[r#async Not Reserved]}, "asyncNotReserved"),
            ( quote! { @[r#await Not Reserved]}, "awaitNotReserved"),
            ( quote! { @[r#dyn Not Reserved]}, "dynNotReserved"),
            ( quote! { @[r#abstract Not Reserved]}, "abstractNotReserved"),
            ( quote! { @[r#become Not Reserved]}, "becomeNotReserved"),
            ( quote! { @[r#box Not Reserved]}, "boxNotReserved"),
            ( quote! { @[r#do Not Reserved]}, "doNotReserved"),
            ( quote! { @[r#final Not Reserved]}, "finalNotReserved"),
            ( quote! { @[r#macro Not Reserved]}, "macroNotReserved"),
            ( quote! { @[r#override Not Reserved]}, "overrideNotReserved"),
            ( quote! { @[r#priv Not Reserved]}, "privNotReserved"),
            ( quote! { @[r#typeof Not Reserved]}, "typeofNotReserved"),
            ( quote! { @[r#unsized Not Reserved]}, "unsizedNotReserved"),
            ( quote! { @[r#virtual Not Reserved]}, "virtualNotReserved"),
            ( quote! { @[r#yield Not Reserved]}, "yieldNotReserved"),
            ( quote! { @[r#try Not Reserved]}, "tryNotReserved"),
        ];

        assert_transformations(arguments);
    }

    #[test]
    fn should_handle_normal_input_reserved_output() {
        let arguments = vec![            
            ( quote! { @[a s]}, "r#as"),
            ( quote! { @[b reak]}, "r#break"),
            ( quote! { @[c onst]}, "r#const"),
            ( quote! { @[c ontinue]}, "r#continue"),
            ( quote! { @[e lse]}, "r#else"),
            ( quote! { @[e num]}, "r#enum"),
            ( quote! { @[e xtern]}, "r#extern"),
            ( quote! { @[f alse]}, "r#false"),
            ( quote! { @[f n]}, "r#fn"),
            ( quote! { @[f or]}, "r#for"),
            ( quote! { @[i f]}, "r#if"),
            ( quote! { @[i mpl]}, "r#impl"),
            ( quote! { @[i n]}, "r#in"),
            ( quote! { @[l et]}, "r#let"),
            ( quote! { @[l oop]}, "r#loop"),
            ( quote! { @[m atch]}, "r#match"),
            ( quote! { @[m od]}, "r#mod"),
            ( quote! { @[m ove]}, "r#move"),
            ( quote! { @[m ut]}, "r#mut"),
            ( quote! { @[p ub]}, "r#pub"),
            ( quote! { @[r ef]}, "r#ref"),
            ( quote! { @[r eturn]}, "r#return"),
            ( quote! { @[s tatic]}, "r#static"),
            ( quote! { @[s truct]}, "r#struct"),
            ( quote! { @[t rait]}, "r#trait"),
            ( quote! { @[t rue]}, "r#true"),
            ( quote! { @[t ype]}, "r#type"),
            ( quote! { @[u nsafe]}, "r#unsafe"),
            ( quote! { @[u se]}, "r#use"),
            ( quote! { @[w here]}, "r#where"),
            ( quote! { @[w hile]}, "r#while"),
            ( quote! { @[a sync]}, "r#async"),
            ( quote! { @[a wait]}, "r#await"),
            ( quote! { @[d yn]}, "r#dyn"),
            ( quote! { @[a bstract]}, "r#abstract"),
            ( quote! { @[b ecome]}, "r#become"),
            ( quote! { @[b ox]}, "r#box"),
            ( quote! { @[d o]}, "r#do"),
            ( quote! { @[f inal]}, "r#final"),
            ( quote! { @[m acro]}, "r#macro"),
            ( quote! { @[o verride]}, "r#override"),
            ( quote! { @[p riv]}, "r#priv"),
            ( quote! { @[t ypeof]}, "r#typeof"),
            ( quote! { @[u nsized]}, "r#unsized"),
            ( quote! { @[v irtual]}, "r#virtual"),
            ( quote! { @[y ield]}, "r#yield"),
            ( quote! { @[t ry]}, "r#try"),
        ];

        assert_transformations(arguments);
    }

    #[test]
    fn handling_forbidden_keywords() {
        let arguments = vec![            
            // `crate` cannot be a raw identifier
            ( quote! { @[not cra te]}, "notcrate"),            
            // self` cannot be a raw identifier 
            ( quote! { @[__ selfie | replace{"ie",""}]}, "__self"),
            // `super` cannot be a raw identifier 
            ( quote! { @[(sup er-duper)|camel] }, "superDuper"),
        ];
        assert_transformations(arguments);
    }


    #[test]
    fn handling_nested_modifiers() {
        let arguments = vec![            
            ( quote! { @[((sup er-duper)|camel)| pascal ] }, "SuperDuper"),
        ];
        assert_transformations(arguments);
    }
}
