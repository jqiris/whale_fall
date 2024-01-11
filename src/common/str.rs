use std::collections::HashMap;

use regex::Regex;

pub fn find_string_sub_match(re: &Regex, text: &str) -> Vec<String> {
    let mut list = vec![];
    if let Some(caps) = re.captures(text) {
        for cap in caps.iter() {
            if let Some(s) = cap {
                list.push(s.as_str().to_string());
            }
        }
    }
    list
}

pub fn is_first_uppercase(text: &str) -> bool {
    if let Some(first_char) = text.chars().next() {
        return first_char.is_uppercase();
    }
    false
}

pub fn is_first_lowwercase(text: &str) -> bool {
    if let Some(first_char) = text.chars().next() {
        return first_char.is_lowercase();
    }
    false
}

pub fn parse_field_tag_map(tag: &str) -> HashMap<String, String> {
    let tag_parts: Vec<&str> = tag.split("\" ").collect();
    let mut result = HashMap::new();
    for parts in tag_parts {
        let pairs: Vec<&str> = parts.splitn(2, ':').collect();
        if pairs.len() < 2 {
            continue;
        }
        result.insert(pairs[0].to_string(), pairs[1].trim_matches('"').to_string());
    }
    result
}

pub fn search_index(body: &str, search: &str) -> i64 {
    match body.find(search) {
        Some(idx) => idx as i64,
        None => -1,
    }
}

pub fn first_upper_index(body: &str) -> i64 {
    match body.find(|c: char| c.is_uppercase()) {
        Some(idx) => idx as i64,
        None => -1,
    }
}

pub fn to_snake_case(str: &str) -> String {
    let mut result = String::new();

    for (i, c) in str.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}
