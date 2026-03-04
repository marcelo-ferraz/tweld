use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase, ToTrainCase}; 

use crate::{models::{Modifier, TokenPart}, parser::NamingDsl};

pub fn build_string(dsl: NamingDsl) -> String {
    let mut full = String::new();
    for part in dsl.parts {
        match part {
            TokenPart::Plain(s) => full.push_str(&s),
            TokenPart::Modified(mut s, mods) => {
                for m in mods {
                    match m {
                        Modifier::Singular => { if s.ends_with('s') { s.pop(); } },
                        Modifier::Plural => { s.push_str("s");  },
                        Modifier::Lowercase => s = s.to_lowercase(),
                        Modifier::Uppercase => s = s.to_uppercase(),
                        Modifier::PascalCase => s = s.to_pascal_case(),
                        Modifier::LowerCamelCase => s = s.to_lower_camel_case(),
                        Modifier::SnakeCase => s = s.to_snake_case(),
                        Modifier::KebabCase => s = s.to_kebab_case(),
                        Modifier::ShoutySnakeCase => s = s.to_shouty_snake_case(),
                        Modifier::TitleCase => s = s.to_title_case(),
                        Modifier::ShoutyKebabCase => s = s.to_shouty_kebab_case(),
                        Modifier::TrainCase => s = s.to_train_case(),
                        Modifier::Replace(from, to) => s = s.replace(&from, &to),
                        Modifier::Substr(begin, end) => {
                            match (begin, end) {
                                (None, None) => s = s[..].to_string(),
                                (None, Some(end)) => s = s[0..end].to_string(),
                                (Some(begin), None) => s = s[begin..s.len()].to_string(),
                                (Some(begin), Some(end)) => s = s[begin..end].to_string(),
                            }
                        }
                    }
                }
                full.push_str(&s);
            }
        }
    }
    full.replace(' ', "")
}
