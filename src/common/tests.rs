use crate::common::file::rel_path;

#[test]

fn rel_path_use() {
    let root = "D:\\workspace\\studys\\gen\\examples";
    let path = "D:\\workspace\\studys\\gen\\examples\\src\\common\\file.rs";
    let rel_path = rel_path(root, path);
    println!("{}", rel_path);
}

#[test]
fn path_parent_use() {
    let path = "D:\\workspace\\studys\\gen\\examples\\src\\common";
    let parent = crate::common::file::path_parent(path);
    println!("{}", parent);
}
