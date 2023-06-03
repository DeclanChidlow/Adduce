use pulldown_cmark::{html, Parser};

use crate::{
    common::{fs::File, result::Error},
    data::toml::{Conf, Main, Object},
};

impl Conf {
    pub fn to_html(&self) -> Result<String, Error> {
        // main html page
        let mut blocks = String::new();
        if let Some(Main { block }) = self.main {
            for item in block {
                blocks += &compile_html(&item)?;
            }
        }

        // CSS stylesheets - not to be confused with 'style'
        let mut style_conf = String::new();

        if let Some(styles) = self.style {
            for path in styles {
                style_conf += &File::from_path(&path)?.get_content();
            }
        }

        // HTML head values
        let mut head_conf = String::new();
        if let Some(head) = self.head {
            for path in head {
                head_conf += &File::from_path(&path)?.get_content();
            }
        }
        let html =  format!("<!DOCTYPE html>\n<head>\n<style>\n{style_conf}\n</style>\n{head_conf}\n</head>\n<body>\n<div class=\"page\">\n{divs}\n</div>\n</body>");
        Ok(html)
    }
}

pub fn compile_html(conf: &Object) -> Result<String, Error> {
    /*
    // import configuration and own it
    let conf: Object = conf.to_owned();

    // define basic style
    let style = conf.style.unwrap_or_default();

    // define text
    let text = match conf.from_str {
        None => conf.text.unwrap_or_else(|| String::from("PLACEHOLDER")),
        Some(a) => fs_to_str(&a),
    };

    let pt_text = pretty_text(&text);

    // defines id
    let id = match conf.id {
        None => String::new(),
        Some(a) => format!(" id=\"{a}\""),
    };

    // mutually exclusive conditionals
    let mut html = match &style as &str {
        "br" => String::from("\n<br>"),
        "html" => String::from("\n") + &pt_text,
        "md" => md_two(&text),
        // standard
        _ => format!("\n<{style}{id}>{pt_text}\n</{style}>"),
    };

    // optional restructing

    // anchor
    if let Some(link) = conf.link {
        html = format!("\n<a href=\"{link}\">\n{html}\n</a>");
    };

    html
    */

    // import configuration and own it
    let conf: Object = conf.to_owned();

    // define basic style
    let style = conf.style.unwrap_or_default();

    // define text
    let text = match conf.from_str {
        None => conf.text.unwrap_or_else(|| String::from("PLACEHOLDER")),
        Some(path) => File::from_path(&path)?.get_content(),
    };

    let pt_text = pretty_text(&text);

    // defines id
    let id = match conf.id {
        None => String::new(),
        Some(a) => format!(" id=\"{a}\""),
    };

    // mutually exclusive conditionals
    let mut html = match &style as &str {
        "br" => String::from("\n<br>"),
        "html" => String::from("\n") + &pt_text,
        "md" => md_two(&text),
        // standard
        _ => format!("\n<{style}{id}>{pt_text}\n</{style}>"),
    };

    // anchor
    if let Some(link) = conf.link {
        html = format!("\n<a href=\"{link}\">\n{html}\n</a>");
    };

    Ok(html)
}

fn md_two(text: &str) -> String {
    let mut html_output = String::new();
    html::push_html(&mut html_output, Parser::new(text));
    html_output
}

fn pretty_text(text: &str) -> String {
    /*
    let mut fin = String::new();
    for x in text.split('\n') {
        fin += &format!("\n    {x}");
    }
    fin
    */
    text.split('\n')
        .map(|item| format!("\n    {item}"))
        .collect()
}
