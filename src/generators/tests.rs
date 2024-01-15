use crate::generators::miman_generator;

#[test]
fn main_micro_parse_use() {
    let gen = miman_generator::MimanGenerator {};
    let text = "// @MICRO[bcfg]";
    let res = gen.main_micro_parse(text);
    assert_eq!(res, vec!["bcfg".to_string()]);
}
