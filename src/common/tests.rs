use regex::Regex;

use crate::common::{file::rel_path, str::*};

#[test]

fn rel_path_test() {
    let root = "D:\\workspace\\studys\\gen\\examples";
    let path = "D:\\workspace\\studys\\gen\\examples\\src\\common\\file.rs";
    let rel_path = rel_path(root, path);
    println!("{}", rel_path);
}

#[test]
fn path_parent_test() {
    let path = "D:\\workspace\\studys\\gen\\examples\\src\\common";
    let parent = crate::common::file::path_parent(path);
    println!("{}", parent);
}

#[test]
fn snake_case_test() {
    let input_str = "matchFirstCap";
    let snake_case_str = to_snake_case(input_str);
    println!("Snake case string: {}", snake_case_str);
}

#[test]
fn to_lower_first_test() {
    let input_str = "M";
    let to_lower_first_str = to_lower_first(input_str);
    println!("to_lower_first_str: {}", to_lower_first_str);
}

#[test]
fn match_gi_test() {
    let re_di = Regex::new(r"@DI\[([\w|.]+)]").unwrap();
    let input_str = "// KittenfishBaitRecordRepo . @GI";
    let rs = find_string_sub_match(&re_di, &input_str);
    println!("rs: {:?}", rs);
}
