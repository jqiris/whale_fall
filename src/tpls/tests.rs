use anyhow::Result;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct HandlebarTest {
    name: String,
}

#[test]
fn handlebars_use() -> Result<()> {
    let mut reg = Handlebars::new();
    // render without register
    let test_data = HandlebarTest {
        name: "jason".to_string(),
    };
    println!(
        "{}",
        // reg.render_template("Hello {{name}}", &json!({"name": "foo"}))?
        reg.render_template("Hello {{name}}", &test_data)?
    );

    // register template using given name
    reg.register_template_string("tpl_1", "Good afternoon, {{name}}")?;
    println!("{}", reg.render("tpl_1", &json!({"name": "foo"}))?);
    Ok(())
}


