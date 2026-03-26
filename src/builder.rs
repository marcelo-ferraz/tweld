use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase, ToTrainCase}; 

use crate::models::{Modifier, TokenPart};

pub fn build_string(parts: Vec<TokenPart>) -> String {
    println!("parts: {parts:?}");
    let mut result = String::new();
    for part in parts {
        let partial = build_from_part(part);
        result.push_str(&partial);
    }

    result.replace("r#", "")
}

fn build_from_part(part: TokenPart) -> String {
    match part {
        TokenPart::Plain(value) => return value.clone(),
        TokenPart::Grouped(grouped_parts) => {
            let word = grouped_parts
                .iter()
                .map(|sub_part| build_from_part(sub_part.clone()))
                .collect::<String>();
            word
        },
        TokenPart::Modified(target, modifiers) => {            
            match *target {
                TokenPart::Plain(value) => modify(value, &modifiers),
                TokenPart::Grouped(grouped_parts) => {
                let value = grouped_parts
                    .iter()
                    .map(|sub_part| build_from_part(sub_part.clone()))
                    .collect::<String>();
                println!("grouped result: {value}");
                
                modify(value, &modifiers)
            },
                TokenPart::Modified(token_part, nested_modifiers) => {
                    println!("token_part: {token_part:?}");
                    let nested_result = modify(
                        build_from_part(*token_part), 
                        &nested_modifiers
                    );
                    println!("nested_result: `{nested_result}`");

                    let rr= modify(nested_result, &modifiers);
                    println!("result: {rr}");

                    rr                    
                },
            }
        }
    }
}

fn modify(value: String, modifiers: &Vec<Modifier>) -> String {
    let mut values = vec![value.to_string()];
                    
    println!("modified value `{values:?}`");
                    
    for modified in modifiers {
        for i in 0..values.len() {                    
            match modified {
                Modifier::Singular => { if values[i].ends_with('s') { values[i].pop(); } },
                Modifier::Plural => { if !values[i].ends_with('s') { values[i].push_str("s"); } },
                Modifier::Lowercase => {
                    println!("before lower {}", values[i]);
                    values[i] = values[i].to_lowercase();
                    println!("after lower {}", values[i]);
                },
                Modifier::Uppercase => values[i] = values[i].to_uppercase(),
                Modifier::PascalCase => values[i] = values[i].to_pascal_case(),
                Modifier::LowerCamelCase => values[i] = values[i].to_lower_camel_case(),
                Modifier::SnakeCase => values[i] = values[i].to_snake_case(),
                Modifier::KebabCase => values[i] = values[i].to_kebab_case(),
                Modifier::ShoutySnakeCase => values[i] = values[i].to_shouty_snake_case(),
                Modifier::TitleCase => {
                    println!("before {:?}", values[i]);
                    values[i] = values[i].to_title_case();
                    println!("after {:?}", values[i]);
                },
                Modifier::ShoutyKebabCase => values[i] = values[i].to_shouty_kebab_case(),
                Modifier::TrainCase => values[i] = values[i].to_train_case(),
                Modifier::Replace(from, to) => values[i] = values[i].replace(from, to),
                Modifier::Substr(start, end) => {
                    let start = start.unwrap_or(0);
                    let end =  end.unwrap_or(values[i].len());
                    values[i] = values[i][start..end].to_string()                            
                },
                Modifier::Reverse => values[i] = values[i].chars().rev().collect::<String>(),
                Modifier::Repeat(times) => {
                    println!("before {:?}", values[i]);
                    values[i] = values[i].repeat(*times);
                    println!("after {:?}", values[i]); 
                },
                Modifier::Split(pat) => {
                    let value = values[i]
                        .split(pat)
                        .map(|v|v.to_string())
                        .collect::<Vec<String>>();
                    values.remove(i);
                    println!("before {values:?}");                                
                    values.splice(i..i, value);
                    println!("after {values:?}");                                

                },
                Modifier::SplitAt(mid) => {
                    let (left, right) = values[i].split_at(*mid);                                    
                    values.splice(i..i, vec![ left.to_string(), right.to_string(),]);
                },
                Modifier::Join(sep) => {
                    let result = values.join(&sep);
                    values.clear();
                    values.push(result);
                },
                Modifier::PadStart(width, pat) => {
                    let mut value = pat.repeat(*width);
                    value.push_str(&values[i]);
                    println!("pads :`{value}`");
                    values[i] = value;
                },
                Modifier::PadEnd(width, pat) => {
                    values[i].push_str(&pat.repeat(*width));                                
                },
            }                    
        }                    
    }
    values.join("")
}
