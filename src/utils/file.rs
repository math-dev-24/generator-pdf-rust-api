use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn save_to_file(file_path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
