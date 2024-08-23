use crate::config::toml::{Conf, Object};
use crate::lib::filesystem::fs_to_str;
use pulldown_cmark::{html, Options, Parser};

impl Conf {
    pub fn to_html(&self) -> String {
        let mut divs = String::new();
        let blocks = self.main.clone().unwrap_or_default().block;
        for (i, obj) in blocks.iter().enumerate() {
            let html = compile_html(obj);
            if i == blocks.len() - 1 {
                divs += &html.trim_end_matches('\n'); // Avoid trailing newline for the last element
            } else {
                divs += &html;
            }
        }
        divs
    }
}

/// Compile the input as outlined in the config to HTML
fn compile_html(obj: &Object) -> String {
    let obj = obj.to_owned();

    let format = obj.format.unwrap_or_default();
    let content = match obj.content_file {
        None => obj.content.unwrap_or_else(|| "PLACEHOLDER".to_string()),
        Some(file_path) => fs_to_str(&file_path),
    };

    let formatted_text = format_text(&content);
    let id_attribute = obj.id.map_or(String::new(), |id| format!(" id=\"{id}\""));

    let html = match format.as_str() {
        "br" => "<br/>\n".to_string(),
        "hr" => "<hr/>\n".to_string(),
        "html" => format!("{}\n", formatted_text),
        "md" => format!("{}\n", convert_markdown_to_html(&content)),
        _ => format!("<{format}{id_attribute}>{formatted_text}</{format}>\n"),
    };

    html
}

/// Convert a Markdown string to a HTML string
fn convert_markdown_to_html(content: &str) -> String {
    let mut html_output = String::new();
    let parser = Parser::new_ext(
        content,
        Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_SMART_PUNCTUATION
            | Options::ENABLE_HEADING_ATTRIBUTES
            | Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
            | Options::ENABLE_MATH
            | Options::ENABLE_GFM
            | Options::ENABLE_DEFINITION_LIST,
    );
    html::push_html(&mut html_output, parser);
    html_output
}

/// Format text by adding indentation
fn format_text(content: &str) -> String {
    content
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if i == 0 {
                line.to_string()
            } else {
                format!("\n{line}")
            }
        }) // Avoid leading newline for the first element
        .collect::<String>()
}
