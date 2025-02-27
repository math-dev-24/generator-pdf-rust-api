use std::error::Error;
use std::fs::File;
use std::io::Write;
use reqwest::{Client, Response};

pub fn save_to_file(file_path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn format_url(doc_url: &str, format: &str) -> String {
    let url = doc_url.split("/").collect::<Vec<&str>>();
    let new_url = url[0..url.len() - 1].join("/");
    format!("{}/export?format={}", new_url, format)
}

pub async fn get_html_content(doc_url: &str) -> Result<String, Box<dyn Error>> {
    let client: Client = Client::new();
    let response: Response = client.get(doc_url).send().await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(format!("Failed to fetch document. Status: {}", response.status()).into())
    }
}