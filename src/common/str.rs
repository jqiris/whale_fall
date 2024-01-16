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

pub fn to_snake_case(s: &str) -> String {
    let match_first_cap = Regex::new(r"(.)([A-Z][a-z]+)").unwrap();
    let match_all_cap = Regex::new(r"([a-z0-9])([A-Z])").unwrap();

    let snake = match_first_cap.replace_all(s, |caps: &regex::Captures| {
        format!("{}_{}", &caps[1], &caps[2])
    });

    let snake = match_all_cap.replace_all(&snake, |caps: &regex::Captures| {
        format!("{}_{}", &caps[1], &caps[2])
    });
    snake.to_lowercase()
}

pub fn in_slice(slices: &[&str], name: &str) -> bool {
    slices.iter().any(|&x| x == name)
}

pub fn to_lower_first(str: &str) -> String {
    let mut result = String::new();
    let mut chars = str.chars();
    if let Some(first_char) = chars.next() {
        let lower_first_char = first_char.to_lowercase().to_string();
        let rest_of_str = chars.collect::<String>();
        result = lower_first_char + &rest_of_str;
    }
    result
}

pub fn to_upper_first(str: &str) -> String {
    let mut result = String::new();
    let mut chars = str.chars();
    if let Some(first_char) = chars.next() {
        let lower_first_char = first_char.to_uppercase().to_string();
        let rest_of_str = chars.collect::<String>();
        result = lower_first_char + &rest_of_str;
    }
    result
}

pub fn strip_breaks(str: &str) -> String {
    let mut result = String::new();
    for c in str.chars() {
        if c != '\r' && c != '\n' {
            result.push(c);
        }
    }
    result
}
