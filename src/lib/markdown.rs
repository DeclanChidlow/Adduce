use crate::lib::filesystem::fs_to_str;
use crate::config::toml::{Conf, Object};
use pulldown_cmark::{html, Options, Parser};

// Implement methods for the Conf struct
impl Conf {
    /// Convert a Conf object to an HTML string
    pub fn to_html(&self) -> String {
        let mut divs = String::new();
        for obj in self.main.clone().unwrap_or_default().block.iter() {
            divs += &compile_html(obj);
        }
        divs
    }
}

/// Compile an Object to an HTML string based on its style
fn compile_html(obj: &Object) -> String {
    let obj = obj.to_owned();

    let style = obj.style.unwrap_or_default();
    let text = match obj.from_str {
        None => obj.text.unwrap_or_else(|| "PLACEHOLDER".to_string()),
        Some(file_path) => fs_to_str(&file_path),
    };

    let formatted_text = format_text(&text);
    let id_attribute = obj.id.map_or(String::new(), |id| format!(" id=\"{id}\""));

    let mut html = match style.as_str() {
        "br" => "\n<br>".to_string(),
        "html" => format!("\n{}", formatted_text),
        "md" => convert_markdown_to_html(&text),
        _ => format!("\n<{style}{id_attribute}>{formatted_text}\n</{style}>"),
    };

    if let Some(link) = obj.link {
        html = format!("\n<a href=\"{link}\">\n{html}\n</a>");
    }

    html
}

/// Convert a Markdown string to an HTML string
fn convert_markdown_to_html(text: &str) -> String {
    let mut html_output = String::new();
    let parser = Parser::new_ext(
        text,
        Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_SMART_PUNCTUATION
            | Options::ENABLE_HEADING_ATTRIBUTES
            | Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
            | Options::ENABLE_MATH
            | Options::ENABLE_GFM,
    );
    html::push_html(&mut html_output, parser);
    html_output
}

/// Format text by adding indentation
fn format_text(text: &str) -> String {
    text.lines()
        .map(|line| format!("\n    {}", line))
        .collect::<String>()
}
