use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase, ToTrainCase}; 

use crate::{models::{Modifier, TokenPart}, parser::TweldDsl};

pub fn build_string(parts: Vec<TokenPart>) -> String {
    println!("parts: {parts:?}");
    let mut full = String::new();
    for part in parts {
        match part {
            TokenPart::Literal(value) => full.push_str(&value),
            TokenPart::Plain(value) => full.push_str(&value.replace(" ", "")),
            TokenPart::Modified(mut value, modifiers) => {
                println!("modified value `{value}`");
                for modified in modifiers {
                    match modified {
                        Modifier::Singular => { if value.ends_with('s') { value.pop(); } },
                        Modifier::Plural => { if !value.ends_with('s') { value.push_str("s"); } },
                        Modifier::Lowercase => value = value.to_lowercase(),
                        Modifier::Uppercase => value = value.to_uppercase(),
                        Modifier::PascalCase => value = value.to_pascal_case(),
                        Modifier::LowerCamelCase => value = value.to_lower_camel_case(),
                        Modifier::SnakeCase => value = value.to_snake_case(),
                        Modifier::KebabCase => value = value.to_kebab_case(),
                        Modifier::ShoutySnakeCase => value = value.to_shouty_snake_case(),
                        Modifier::TitleCase => value = value.to_title_case(),
                        Modifier::ShoutyKebabCase => value = value.to_shouty_kebab_case(),
                        Modifier::TrainCase => value = value.to_train_case(),
                        Modifier::Replace(from, to) => value = value.replace(&from, &to),
                        Modifier::Substr(start, end) => {
                            let start = start.unwrap_or(0);
                            let end =  end.unwrap_or(value.len());
                            value = value[start..end].to_string()                            
                        }
                    }                    
                }
                full.push_str(&value);
            }
        }
    }

    return full;
}
