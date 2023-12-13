// Import necessary libraries and modules
use super::rfs::fs_to_str;
use crate::structs::toml_conf::{Conf, Object};
use pulldown_cmark::{html, Options, Parser};

// Implement methods for the Conf struct
impl Conf {
    // Method to convert a Conf object to HTML
    pub fn to_html(&self) -> String {
        let mut divs = String::new();
        for x in self.main.clone().unwrap_or_default().block.iter() {
            divs += &compile_html(x);
        }

        // Initialize a new String to hold the CSS stylesheets
        let mut style_conf = String::new();
        // If the Conf object has a style field, read each file
        // in the field and add its content to the stylesheets
        if let Some(styles) = self.clone().style {
            for x in styles {
                style_conf += &fs_to_str(&x);
            }
        }

        // Initialize a new String to hold the head configuration
        let mut head_conf = String::new();
        // If the Conf object has a head field, read each file
        // in the field and add its content to the head configuration
        if let Some(head) = self.clone().head {
            for x in head {
                head_conf += &fs_to_str(&x);
            }
        }

        // Return the final HTML document
        format!("<!DOCTYPE html>\n<head>\n<style>\n{style_conf}\n</style>\n{head_conf}\n</head>\n<body>\n<div class=\"page\">\n{divs}\n</div>\n</body>")
    }
}

fn compile_html(conf: &Object) -> String {
    // Clone the Object to own it
    let conf: Object = conf.to_owned();

    // Get the style of the Object, or use a default style
    let style = conf.style.unwrap_or_default();

    // Get the text of the Object, or read it from a file,
    // or use a placeholder text
    let text = match conf.from_str {
        None => conf.text.unwrap_or_else(|| String::from("PLACEHOLDER")),
        Some(a) => fs_to_str(&a),
    };

    // Process the text to make it pretty
    let pt_text = pretty_text(&text);

    // Get the id of the Object, or use an empty string
    let id = match conf.id {
        None => String::new(),
        Some(a) => format!(" id=\"{a}\""),
    };

    // Compile the Object to HTML based on its style
    let mut html = match &style as &str {
        "br" => String::from("\n<br>"),
        "html" => String::from("\n") + &pt_text,
        "md" => md_two(&text),
        // standard
        _ => format!("\n<{style}{id}>{pt_text}\n</{style}>"),
    };

    // optional restructing

    // If the Object has a link, wrap the HTML in an anchor tag
    if let Some(link) = conf.link {
        html = format!("\n<a href=\"{link}\">\n{html}\n</a>");
    };

    html
}

// Function to convert Markdown to HTML
fn md_two(text: &str) -> String {
    let mut html_output = String::new();
    html::push_html(
        &mut html_output,
        Parser::new_ext(
            text,
            Options::ENABLE_HEADING_ATTRIBUTES
                | Options::ENABLE_STRIKETHROUGH
                | Options::ENABLE_TABLES
                | Options::ENABLE_TASKLISTS,
        ),
    );
    html_output
}

// Function to make text pretty by adding indentation
fn pretty_text(text: &str) -> String {
    let mut fin = String::new();
    for x in text.split('\n') {
        fin += &format!("\n    {x}");
    }
    fin
}