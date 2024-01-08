use regex::Regex;

#[test]
fn re_impl_match() {
    let text = "@IMPL[core]";
    let re_impl = Regex::new(r"@IMPL\[([\w|.]+)]").unwrap();
    let caps = re_impl.captures(text).unwrap();
    for cap in caps.iter() {
        println!("{}", cap.unwrap().as_str());
    }
}
