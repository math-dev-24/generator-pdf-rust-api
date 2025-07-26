use std::fmt::Display;

#[derive(Debug)]
pub enum FormatFile {
    HTML
}

impl Display for FormatFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            FormatFile::HTML => "html".to_string()
        })
    }
}