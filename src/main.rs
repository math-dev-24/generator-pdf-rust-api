mod utils;
mod types;

use serde_json::{Value, from_str};

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


    let json = r#"{
        "name": "mathieu",
        "username": "Doriane",
        "now": "2024-04-23",
        "ask": "true",
        "deadline": "2024-04-28",
        "content": "Les modalités sont  ....",
        "list" : [
            { "name": "John" }, {"name": "Alicia"} , {"name": "Bernard"}
        ]
    }"#;


    let parsed_json: Value = from_str(json).expect("Erreur lors de la désérialisation");

    println!("{:#?}", variables);
    println!("{:#?}", parsed_json);
    let new_html = replace_variable(content_html, variables, &parsed_json);


    save_to_file("test.html", &new_html)?;
    Ok(())
}



fn save_to_file(file_path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}


