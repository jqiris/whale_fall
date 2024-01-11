use crate::common::{file::rel_path, str::to_snake_case};

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
