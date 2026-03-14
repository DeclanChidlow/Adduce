use crate::config::toml::{Conf, Object};
use crate::lib::filesystem::fs_to_str;
use pulldown_cmark::{html, Options, Parser};

impl Conf {
    pub fn to_html(&self) -> String {
        let blocks = self
            .main
            .as_ref()
            .map(|m| m.block.as_slice())
            .unwrap_or_default();

        blocks
            .iter()
            .map(compile_html)
            .collect::<Vec<String>>()
            .join("")
            .trim_end()
            .to_string()
    }
}

/// Compile the input as outlined in the config to HTML
fn compile_html(obj: &Object) -> String {
    let format = obj.format.as_deref().unwrap_or_default();
    let content = match &obj.content_file {
        Some(file_path) => fs_to_str(file_path),
        None => obj
            .content
            .clone()
            .unwrap_or_else(|| "PLACEHOLDER".to_string()),
    };

    let id_attribute = obj
        .id
        .as_deref()
        .map_or(String::new(), |id| format!(" id=\"{id}\""));

    match format {
        "br" => "<br/>\n".to_string(),
        "hr" => "<hr/>\n".to_string(),
        "html" => format!("{content}\n"),
        "md" => format!("{}\n", convert_markdown_to_html(&content)),
        _ => format!("<{format}{id_attribute}>{content}</{format}>\n"),
    }
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
