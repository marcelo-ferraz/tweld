use crate::{
    models::{Modifier, Output},
    parser::modifiers::parse_modifiers,
};
use syn::parse::Parser;

#[macro_export]
macro_rules! define_tests_for_syntax {
        ($name: ident, $modi:pat, $($input:expr),+ ) => {
            #[test]
            fn $name() {
                vec![$($input),+]
                    .iter()
                    .for_each(|input| {
                        let result = Parser::parse_str(parse_modifiers, &input).unwrap();
                        assert!(matches!(&result[0], $modi));
                    });
            }
        };

        ($(($name:ident, $modi:pat, $($input:expr),+)),+ $(,)?) => {
            $(define_tests_for_syntax!($name, $modi, $($input),+);)+
        };
    }

define_tests_for_syntax![
    (test_syntax_for_singular, Modifier::Singular, "|singular"),
    (test_syntax_for_plural, Modifier::Plural, "|plural"),
    (
        test_syntax_for_lower,
        Modifier::Lowercase,
        "|lower",
        "|lowercase"
    ),
    (
        test_syntax_for_upper,
        Modifier::Uppercase,
        "|upper",
        "|uppercase"
    ),
    (
        test_syntax_for_pascal,
        Modifier::PascalCase,
        "|pascal",
        "|pascalcase",
        "|uppercamelcase"
    ),
    (
        test_syntax_for_camel,
        Modifier::LowerCamelCase,
        "|lowercamelcase",
        "|camelcase",
        "|camel"
    ),
    (
        test_syntax_for_snek,
        Modifier::SnakeCase,
        "|snakecase",
        "|snake",
        "|snekcase",
        "|snek"
    ),
    (
        test_syntax_for_kebab,
        Modifier::KebabCase,
        "|kebabcase",
        "|kebab"
    ),
    (
        test_syntax_for_shouty_nek,
        Modifier::ShoutySnakeCase,
        "|shoutysnakecase",
        "|shoutysnake",
        "|shoutysnekcase",
        "|shoutysnek"
    ),
    (
        test_syntax_for_title,
        Modifier::TitleCase,
        "|titlecase",
        "|title"
    ),
    (
        test_syntax_for_shouty_kebab,
        Modifier::ShoutyKebabCase,
        "|shoutykebabcase",
        "|shoutykebab"
    ),
    (
        test_syntax_for_train,
        Modifier::TrainCase,
        "|traincase",
        "|train"
    ),
    (
        test_syntax_for_replace,
        Modifier::Replace(_, _),
        "|replace{'a','b'}",
        "|replace{\"a\",'b'}",
        "|replace{'a',\"b\"}",
        "|replace{\"a\",\"b\"}"
    ),
    (
        test_syntax_for_substring,
        Modifier::Substr(_, _),
        "|substr{,}",
        "|substr{1,}",
        "|substr{,2}",
        "|substr{1,2}",
        "|substring{,}",
        "|substring{1,}",
        "|substring{,2}",
        "|substring{1,2}"
    ),
    (
        test_syntax_for_reverse,
        Modifier::Reverse,
        "|reverse",
        "|rev"
    ),
    (
        test_syntax_for_repeat,
        Modifier::Repeat(_),
        "|repeat{1}",
        "|rep{1}",
        "|times{1}"
    ),
    (
        test_syntax_for_split_at,
        Modifier::SplitAt(_),
        "|splitat{1}"
    ),
    (
        test_syntax_for_split,
        Modifier::Split(_),
        "|split{\"a\"}",
        "|split{'a'}"
    ),
    (
        test_syntax_for_join,
        Modifier::Join(_),
        "|join",
        "|join{\"a\"}",
        "|join{'a'}"
    ),
    (
        test_syntax_for_pad_start,
        Modifier::PadStart(_, _),
        "|padstart{1, 'a'}",
        "|padstart{1, \"a\"}",
        "|padleft{1, 'a'}",
        "|padleft{1, \"a\"}",
        "|padl{1, 'a'}",
        "|padl{1, \"a\"}"
    ),
    (
        test_syntax_for_pad_end,
        Modifier::PadEnd(_, _),
        "|padend{1, 'a'}",
        "|padend{1, \"a\"}",
        "|padright{1, 'a'}",
        "|padright{1, \"a\"}",
        "|padr{1, 'a'}",
        "|padr{1, \"a\"}"
    ),
    (
        test_syntax_for_slice,
        Modifier::Slice(_, _),
        "|slice{}",
        "|slice{,}",
        "|slice{1,}",
        "|slice{,2}",
        "|slice{1,2}"
    ),
    (
        test_syntax_for_splice_out,
        Modifier::Splice(_, _, _, _),
        "|spliceout{,,}",
        "|spliceout{1,,}",
        "|spliceout{,1,}",
        "|spliceout{1,1,}",
        "|spliceout{,,'a'}",
        "|spliceout{,,\"a\"}",
        "|spliceout{1,,'a'}",
        "|spliceout{1,,\"a\"}",
        "|spliceout{,1,'a'}",
        "|spliceout{,1,\"a\"}",
        "|spliceout{1,1,'a'}",
        "|spliceout{1,1,\"a\"}",
        "|splice_out{,,}",
        "|splice_out{1,,}",
        "|splice_out{,1,}",
        "|splice_out{1,1,}",
        "|splice_out{,,'a'}",
        "|splice_out{,,\"a\"}",
        "|splice_out{1,,'a'}",
        "|splice_out{1,,\"a\"}",
        "|splice_out{,1,'a'}",
        "|splice_out{,1,\"a\"}",
        "|splice_out{1,1,'a'}",
        "|splice_out{1,1,\"a\"}"
    ),
    (
        test_syntax_for_splice_into,
        Modifier::Splice(_, _, _, _),
        "|spliceinto{,,}",
        "|spliceinto{1,,}",
        "|spliceinto{,1,}",
        "|spliceinto{1,1,}",
        "|spliceinto{,,'a'}",
        "|spliceinto{,,\"a\"}",
        "|spliceinto{1,,'a'}",
        "|spliceinto{1,,\"a\"}",
        "|spliceinto{,1,'a'}",
        "|spliceinto{,1,\"a\"}",
        "|spliceinto{1,1,'a'}",
        "|spliceinto{1,1,\"a\"}",
        "|splice_into{,,}",
        "|splice_into{1,,}",
        "|splice_into{,1,}",
        "|splice_into{1,1,}",
        "|splice_into{,,'a'}",
        "|splice_into{,,\"a\"}",
        "|splice_into{1,,'a'}",
        "|splice_into{1,,\"a\"}",
        "|splice_into{,1,'a'}",
        "|splice_into{,1,\"a\"}",
        "|splice_into{1,1,'a'}",
        "|splice_into{1,1,\"a\"}"
    ),
    (
        test_syntax_for_splice,
        Modifier::Splice(_, _, _, _),
        "|splice{into,,,}",
        "|splice{into,1,,}",
        "|splice{into,,1,}",
        "|splice{into,1,1,}",
        "|splice{into,,,'a'}",
        "|splice{into,,,\"a\"}",
        "|splice{into,1,,'a'}",
        "|splice{into,1,,\"a\"}",
        "|splice{into,,1,'a'}",
        "|splice{into,,1,\"a\"}",
        "|splice{into,1,1,'a'}",
        "|splice{into,1,1,\"a\"}",
        "|splice{value,,,}",
        "|splice{value,1,,}",
        "|splice{value,,1,}",
        "|splice{value,1,1,}",
        "|splice{value,,,'a'}",
        "|splice{value,,,\"a\"}",
        "|splice{value,1,,'a'}",
        "|splice{value,1,,\"a\"}",
        "|splice{value,,1,'a'}",
        "|splice{value,,1,\"a\"}",
        "|splice{value,1,1,'a'}",
        "|splice{value,1,1,\"a\"}",
        "|splice{val,,,}",
        "|splice{val,1,,}",
        "|splice{val,,1,}",
        "|splice{val,1,1,}",
        "|splice{val,,,'a'}",
        "|splice{val,,,\"a\"}",
        "|splice{val,1,,'a'}",
        "|splice{val,1,,\"a\"}",
        "|splice{val,,1,'a'}",
        "|splice{val,,1,\"a\"}",
        "|splice{val,1,1,'a'}",
        "|splice{val,1,1,\"a\"}",
        "|splice{,,,}",
        "|splice{,1,,}",
        "|splice{,,1,}",
        "|splice{,1,1,}",
        "|splice{,,,'a'}",
        "|splice{,,,\"a\"}",
        "|splice{,1,,'a'}",
        "|splice{,1,,\"a\"}",
        "|splice{,,1,'a'}",
        "|splice{,,1,\"a\"}",
        "|splice{,1,1,'a'}",
        "|splice{,1,1,\"a\"}",
        "|splice{out,,,}",
        "|splice{out,1,,}",
        "|splice{out,,1,}",
        "|splice{out,1,1,}",
        "|splice{out,,,'a'}",
        "|splice{out,,,\"a\"}",
        "|splice{out,1,,'a'}",
        "|splice{out,1,,\"a\"}",
        "|splice{out,,1,'a'}",
        "|splice{out,,1,\"a\"}",
        "|splice{out,1,1,'a'}",
        "|splice{out,1,1,\"a\"}",
        "|splice{value,,,}",
        "|splice{value,1,,}",
        "|splice{value,,1,}",
        "|splice{value,1,1,}",
        "|splice{value,,,'a'}",
        "|splice{value,,,\"a\"}",
        "|splice{value,1,,'a'}",
        "|splice{value,1,,\"a\"}",
        "|splice{value,,1,'a'}",
        "|splice{value,,1,\"a\"}",
        "|splice{value,1,1,'a'}",
        "|splice{value,1,1,\"a\"}",
        "|splice{rm,,,}",
        "|splice{rm,1,,}",
        "|splice{rm,,1,}",
        "|splice{rm,1,1,}",
        "|splice{rm,,,'a'}",
        "|splice{rm,,,\"a\"}",
        "|splice{rm,1,,'a'}",
        "|splice{rm,1,,\"a\"}",
        "|splice{rm,,1,'a'}",
        "|splice{rm,,1,\"a\"}",
        "|splice{rm,1,1,'a'}",
        "|splice{rm,1,1,\"a\"}"
    ),
];

#[test]
fn test_substr_values() {
    let arguments = vec![
        ("|substr{,}", (None, None)),
        ("|substr{1,}", (Some(1), None)),
        ("|substr{,2}", (None, Some(2))),
        ("|substr{1,2}", (Some(1), Some(2))),
        ("|substring{,}", (None, None)),
        ("|substring{1,}", (Some(1), None)),
        ("|substring{,2}", (None, Some(2))),
        ("|substring{1,2}", (Some(1), Some(2))),
    ];

    arguments.iter().for_each(|(input, argument)| {
        let parsed = Parser::parse_str(parse_modifiers, input).unwrap();
        let result = parsed.get(0).unwrap();

        let Modifier::Substr(start, end) = result else {
            panic!("Invalid result");
        };

        let (exp_start, exp_end) = argument;

        assert_eq!(exp_start, start);
        assert_eq!(exp_end, end);
    });
}

#[test]
fn test_slice_values() {
    let arguments = vec![
        ("|slice{}", (None, None)),
        ("|slice{,}", (None, None)),
        ("|slice{1,}", (Some(1), None)),
        ("|slice{,2}", (None, Some(2))),
        ("|slice{1,2}", (Some(1), Some(2))),
    ];

    arguments.iter().for_each(|(input, argument)| {
        let parsed = Parser::parse_str(parse_modifiers, input).unwrap();
        let result = parsed.get(0).unwrap();

        let Modifier::Slice(start, end) = result else {
            panic!("Invalid result");
        };

        let (exp_start, exp_end) = argument;

        assert_eq!(exp_start, start);
        assert_eq!(exp_end, end);
    });
}

#[test]
fn test_splice_values() {
    let arguments = vec![
        ("|splice{,,,}", (Output::Value, None, None, None)),
        ("|splice{val,,,}", (Output::Value, None, None, None)),
        ("|splice{val,1,,}", (Output::Value, Some(1), None, None)),
        ("|splice{val,,1,}", (Output::Value, None, Some(1), None)),
        ("|splice{val,1,1,}", (Output::Value, Some(1), Some(1), None)),
        ("|splice{val,,,'a'}", (Output::Value, None, None, Some("a"))),
        (
            "|splice{val,,,\"a\"}",
            (Output::Value, None, None, Some("a")),
        ),
        (
            "|splice{val,1,,'a'}",
            (Output::Value, Some(1), None, Some("a")),
        ),
        (
            "|splice{val,1,,\"a\"}",
            (Output::Value, Some(1), None, Some("a")),
        ),
        (
            "|splice{val,,1,'a'}",
            (Output::Value, None, Some(1), Some("a")),
        ),
        (
            "|splice{val,,1,\"a\"}",
            (Output::Value, None, Some(1), Some("a")),
        ),
        (
            "|splice{val,1,1,'a'}",
            (Output::Value, Some(1), Some(1), Some("a")),
        ),
        (
            "|splice{val,1,1,\"a\"}",
            (Output::Value, Some(1), Some(1), Some("a")),
        ),
        ("|splice{out,,,}", (Output::Removed, None, None, None)),
        ("|splice{out,1,,}", (Output::Removed, Some(1), None, None)),
        ("|splice{out,,1,}", (Output::Removed, None, Some(1), None)),
        (
            "|splice{out,1,1,}",
            (Output::Removed, Some(1), Some(1), None),
        ),
        (
            "|splice{out,,,'a'}",
            (Output::Removed, None, None, Some("a")),
        ),
        (
            "|splice{out,,,\"a\"}",
            (Output::Removed, None, None, Some("a")),
        ),
        (
            "|splice{out,1,,'a'}",
            (Output::Removed, Some(1), None, Some("a")),
        ),
        (
            "|splice{out,1,,\"a\"}",
            (Output::Removed, Some(1), None, Some("a")),
        ),
        (
            "|splice{out,,1,'a'}",
            (Output::Removed, None, Some(1), Some("a")),
        ),
        (
            "|splice{out,,1,\"a\"}",
            (Output::Removed, None, Some(1), Some("a")),
        ),
        (
            "|splice{out,1,1,'a'}",
            (Output::Removed, Some(1), Some(1), Some("a")),
        ),
        (
            "|splice{out,1,1,\"a\"}",
            (Output::Removed, Some(1), Some(1), Some("a")),
        ),
    ];

    arguments.iter().for_each(|(input, argument)| {
        let parsed = Parser::parse_str(parse_modifiers, input).unwrap();
        let result = parsed.get(0).unwrap();

        let Modifier::Splice(output, start, end, replace) = result else {
            panic!("Invalid result");
        };

        let (exp_out, exp_start, exp_end, exp_repl) = argument;
        let exp_repl = exp_repl.and_then(|v| Some(v.to_string()));

        assert_eq!(exp_out, output);
        assert_eq!(exp_start, start);
        assert_eq!(exp_end, end);
        assert_eq!(exp_repl, replace.clone());
    });
}

// Slice(Option<i32>, Option<i32>),
// Splice(Output, Option<i32>, Option<i32>, Option<String>),
