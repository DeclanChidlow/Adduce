use crate::config::toml::{Conf, Head, Object};
use crate::lib::document::{Document, Section, is_void_element};
use crate::lib::filesystem::fs_to_str;
use html_escape::encode_double_quoted_attribute;
use pulldown_cmark::{Options, Parser, html};

impl Conf {
    pub fn to_html(&self) -> String {
        let mut document = Document::default();

        if let Some(head) = &self.head {
            let source = match head {
                Head::Files(files) => files
                    .iter()
                    .map(|file| fs_to_str(file))
                    .collect::<Vec<_>>()
                    .join("\n"),
                Head::Blocks(main) => compile_blocks(&main.block),
            };
            document.add(&source, Section::Head);
        }

        if let Some(styles) = &self.style {
            let css = styles
                .iter()
                .map(|file| fs_to_str(file))
                .collect::<Vec<_>>()
                .join("\n");
            document.add_style(&css);
        }

        if let Some(main) = &self.main {
            document.add(&compile_blocks(&main.block), Section::Body);
        }

        document.render(self.lang.as_deref())
    }
}

fn compile_blocks(blocks: &[Object]) -> String {
    blocks.iter().map(compile_html).collect()
}

/// Compile the input as outlined in the config to HTML
fn compile_html(obj: &Object) -> String {
    let format = obj
        .format
        .as_deref()
        .filter(|format| !format.is_empty())
        .unwrap_or("html");
    let content = match &obj.content_file {
        Some(file_path) => fs_to_str(file_path),
        None => obj
            .content
            .clone()
            .unwrap_or_else(|| "PLACEHOLDER".to_string()),
    };

    let id_attribute = obj.id.as_deref().map_or(String::new(), |id| {
        format!(" id=\"{}\"", encode_double_quoted_attribute(id))
    });

    match format {
        "br" => "<br/>\n".to_string(),
        "hr" => "<hr/>\n".to_string(),
        "html" => format!("{content}\n"),
        "md" => format!("{}\n", convert_markdown_to_html(&content)),
        "document" => String::new(),
        format if is_void_element(format) => format!("<{format}{id_attribute}/>\n"),
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
