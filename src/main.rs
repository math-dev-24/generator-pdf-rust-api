mod utils;
mod types;

use serde_json::{Value, from_str};
use utils::replace::replace_variable;
use utils::variables::get_segments;
use crate::types::segment::Segment;
use crate::utils::file::{save_to_file, format_url, get_html_content};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bad_url = "https://docs.google.com/document/d/1TZy8iYhii76f7X-7GmUvo0_DQRsFjzUx8a-biGJuKHA/edit?usp=sharing";
    let generate_url = format_url(bad_url, "html");

    let mut content_html = get_html_content(&generate_url).await?;
    let variables: Vec<Segment> = get_segments(&content_html);

    let json = r#"{
        "name": "mathieu",
        "username": "Doriane",
        "facture": "LXP-2024",
        "now": "2024-04-23",
        "ask": "true",
        "deadline": "2024-04-28",
        "content": "Les modalités sont  ....",
        "boolloop": "true",
        "list" : [
            { "name": "John", "prix": "11" }, {"name": "Alicia", "prix": "11"} , {"name": "Bernard", "prix": "11"}
        ],
        "total": "201.02"
    }"#;


    let parsed_json: Value = from_str(json).expect("Erreur lors de la désérialisation");

    // println!("{:#?}", variables);
    // println!("{:#?}", parsed_json);
    // println!("{}", content_html);

    replace_variable(&mut content_html, variables, &parsed_json);

    let path_html = "test.html";

    if let Ok(()) =  save_to_file(path_html, &content_html) {
        println!("Document générer : {}", path_html);
    }
    Ok(())
}



