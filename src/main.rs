mod utils;
mod types;

use std::collections::HashMap;

use utils::extract;
use utils::replace::replace_variable;
use utils::variables::get_segments;
use crate::types::variable::Segment;

use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bad_url = "https://docs.google.com/document/d/1TZy8iYhii76f7X-7GmUvo0_DQRsFjzUx8a-biGJuKHA/edit?usp=sharing";
    let generate_url = extract::format_url(bad_url, "html");

    let content_html = extract::get_html_content(&generate_url).await?;


    let variables: Vec<Segment> = get_segments(&content_html);

    let mut values: HashMap<String, String> = HashMap::new();
    values.insert("name".to_string(), "mathieu".to_string());
    values.insert("username".to_string(), "john".to_string());
    values.insert("now".to_string(), "2024-04-23".to_string());
    values.insert("deadline".to_string(), "2024-04-28".to_string());
    values.insert("ask".to_string(), "true".to_string());
    values.insert("content".to_string(), "Les modalités sont  ....".to_string());

    println!("{:#?}", variables);
    let new_html = replace_variable(content_html, variables, values);


    save_to_file("test.html", &new_html)?;
    Ok(())
}



fn save_to_file(file_path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}


