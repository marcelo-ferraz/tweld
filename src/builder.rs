use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToTitleCase, ToTrainCase,
};

use crate::models::{Modifier, Output, TokenPart};

pub fn build_string(parts: Vec<TokenPart>) -> String {
    let mut result = String::new();
    for part in parts {
        let partial = build_from_part(part);
        result.push_str(&partial);
    }

    result.replace("r#", "")
}

fn build_from_part(part: TokenPart) -> String {
    match part {
        TokenPart::Plain(value) => value.clone(),
        TokenPart::ConcatGroup(grouped_parts) => grouped_parts
            .iter()
            .map(|sub_part| build_from_part(sub_part.clone()))
            .collect::<String>(),
        TokenPart::ListGroup(grouped_parts) => grouped_parts
            .iter()
            .map(|sub_part| build_from_part(sub_part.clone()))
            .collect::<String>(),
        TokenPart::Modified(target, modifiers) => match *target {
            TokenPart::Plain(value) => modify_single(value, &modifiers),
            TokenPart::ListGroup(items) => {
                let value = items
                    .iter()
                    .map(|sub_part| build_from_part(sub_part.clone()))
                    .collect::<Vec<String>>();

                modify_list(value, &modifiers)
            }
            TokenPart::ConcatGroup(grouped_parts) => {
                let value = grouped_parts
                    .iter()
                    .map(|sub_part| build_from_part(sub_part.clone()))
                    .collect::<String>();

                modify_single(value, &modifiers)
            }
            TokenPart::Modified(token_part, nested_modifiers) => {
                let nested_result = modify_single(build_from_part(*token_part), &nested_modifiers);

                modify_single(nested_result, &modifiers)
            }
        },
    }
}

fn modify_single(value: String, modifiers: &Vec<Modifier>) -> String {
    let values = vec![value.to_string()];

    modify_list(values, modifiers)
}
fn modify_list(mut values: Vec<String>, modifiers: &Vec<Modifier>) -> String {
    for modified in modifiers {
        let list_mode = values.len() > 1;
        for i in 0..values.len() {
            match modified {
                Modifier::Singular => {
                    if values[i].ends_with('s') {
                        values[i].pop();
                    }
                }
                Modifier::Plural => {
                    if !values[i].ends_with('s') {
                        values[i].push('s');
                    }
                }
                Modifier::Lowercase => {
                    values[i] = values[i].to_lowercase();
                }
                Modifier::Uppercase => values[i] = values[i].to_uppercase(),
                Modifier::PascalCase => values[i] = values[i].to_pascal_case(),
                Modifier::LowerCamelCase => values[i] = values[i].to_lower_camel_case(),
                Modifier::SnakeCase => values[i] = values[i].to_snake_case(),
                Modifier::KebabCase => values[i] = values[i].to_kebab_case(),
                Modifier::ShoutySnakeCase => values[i] = values[i].to_shouty_snake_case(),
                Modifier::TitleCase => {
                    values[i] = values[i].to_title_case();
                }
                Modifier::ShoutyKebabCase => values[i] = values[i].to_shouty_kebab_case(),
                Modifier::TrainCase => values[i] = values[i].to_train_case(),
                Modifier::Replace(from, to) => values[i] = values[i].replace(from, to),
                Modifier::Substr(start, end) => {
                    let start = start.unwrap_or(0);
                    let end = end.unwrap_or(values[i].len());
                    values[i] = values[i][start..end].to_string()
                }
                Modifier::Reverse => {
                    if list_mode {
                        values.reverse();
                        break;
                    }

                    values[i] = values[i].chars().rev().collect::<String>();
                }
                Modifier::Repeat(times) => {
                    if list_mode {
                        values = values
                            .iter()
                            .cloned()
                            .cycle()
                            .take(values.len() * *times)
                            .collect();
                        break;
                    }
                    values[i] = values[i].repeat(*times);
                }
                Modifier::Split(pat) => {
                    if list_mode {
                        values = values
                            .iter()
                            .flat_map(|val| val.split(pat))
                            .filter(|val| !val.is_empty())
                            .map(|val| val.to_owned())
                            .collect::<Vec<String>>();

                        break;
                    }

                    let value = values[i].clone();
                    let value = value
                        .split(pat)
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>();
                    values.remove(i);
                    if !value.is_empty() {
                        values.splice(i..i, value);
                    }
                }
                Modifier::SplitAt(mid) => {
                    if list_mode {
                        values = values
                            .iter()
                            .flat_map(|val| {
                                let len = val.len();
                                // to avoid out of bounds
                                let mid = mid.min(&len);
                                let (a, b) = val.split_at(*mid);
                                [a, b]
                            })
                            .map(|val| val.to_owned())
                            .filter(|val| !val.is_empty())
                            .collect::<Vec<String>>();

                        break;
                    }

                    let value = values.remove(i);
                    let (left, right) = value.split_at(*mid);
                    values.splice(i..i, vec![left.to_string(), right.to_string()]);
                }
                Modifier::Join(sep) => {
                    let result = values.join(sep);
                    values.clear();
                    values.push(result);
                    break;
                }
                Modifier::PadStart(width, pat) => {
                    let width: i32 = (*width as i32) - (values[i].len() as i32);
                    if width > 0 {
                        // TODO: use (width / pat.len) or (width / pat.len) for the repeat, to avoid large truncates
                        // this can happen when the user adds a pattern that is long, like `|_|` per instance and max remaning width for padding is 2
                        // per ex:
                        //------------------------------------------------01234567890
                        // oneStr | padstart { '|_|', 8 } should render  "|_oneStr"    -> remaining padding is 2, as  8 - 6 = 2
                        // oneStr | padstart { '|_|', 11 } should render "|_||_oneStr" -> remaining padding is 5, as 11 - 6 = 5
                        let mut val = pat.repeat(width as usize)[0..(width as usize)].to_string();
                        val.push_str(&values[i]);
                        values[i] = val;
                    }
                }
                Modifier::PadEnd(width, pat) => {
                    let width: i32 = (*width as i32) - (values[i].len() as i32);
                    if width > 0 {
                        // TODO: use (width / pat.len) or (width / pat.len) for the repeat, to avoid large truncates
                        // this can happen when the user adds a pattern that is long, like `|_|` per instance and max remaning width for padding is 2
                        // per ex:
                        //----------------------------------------------01234567890
                        // oneStr | padend { '|_|', 8 } should render  "oneStr|_"    -> remaining padding is 2, as  8 - 6 = 2
                        // oneStr | padend { '|_|', 11 } should render "oneStr|_||_" -> remaining padding is 5, as 11 - 6 = 5
                        values[i].push_str(&pat.repeat(width as usize)[0..(width as usize)]);
                    }
                }
                Modifier::Slice(start, end) => {
                    
                    if list_mode {
                        let len = values.len() as i32;
                        let start = parse_pos(len, start.unwrap_or(0));
                        let end = parse_pos(len, end.unwrap_or(len));
                        
                        if start >= end {
                            values = vec![];
                        } else {
                            values = values.get(start..end).unwrap_or_default().to_vec();
                        }
                        break;
                    }
                    
                    let len = values[i].len() as i32;
                    let start = parse_pos(len, start.unwrap_or(0));
                    let end = parse_pos(len, end.unwrap_or(len));

                    if start >= end {
                        values[i] = String::new();
                        continue;
                    }

                    values[i] = values[i].get(start..end).unwrap_or_default().to_string();
                }
                Modifier::Splice(output, start, delete_end, replace_with) => {
                    let len = values[i].len() as i32;

                    let start = parse_pos(len, start.unwrap_or(0));
                    let end = parse_pos(len, delete_end.unwrap_or(len));

                    if list_mode {
                        if start > end {
                            values = vec![];
                        } else {
                            let removed = values.splice(start..end, replace_with.clone()).collect();

                            if let Output::Removed = output {
                                values = removed;
                            }
                        }
                        break;
                    }

                    let replace_with = replace_with.clone().unwrap_or(String::new());

                    if start > end {
                        values[i] = String::new();
                        continue;
                    }

                    let removed = values[i].get(start..end).unwrap_or_default().to_string();

                    values[i].replace_range(start..end, &replace_with);

                    if let Output::Removed = output {
                        values[i] = removed;
                    }
                }
            }
        }
    }
    values.join("")
}

// couldnt think of a name...
fn parse_pos(len: i32, val: i32) -> usize {
    if val < 0 {
        (len + val) as usize
    } else {
        val.min(len) as usize
    }
}
