use crate::structs::toml_conf::{Conf, Object};

use super::rfs::fs_to_str;

impl Conf {
    pub fn to_html(self) -> String {
        let mut divs = String::new();
        for x in self.main.unwrap().block.iter() {
            divs += &compile_html(x);
        }

        println!("{}", self.style.is_some());
        let styling = match self.style {
            None => String::new(),
            Some(a) => fs_to_str(&a),
        };

        format!("<!DOCTYPE html>\n<head>\n<style>\n{styling}\n</style>\n</head>\n<body>\n{divs}\n</body>")
    }
}

fn compile_html(conf: &Object) -> String {
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
        Some(a) => format!(" id=\"{}\"", a),
    };

    // in case there is a special style
    match &style as &str {
        "br" => String::from("<br>"),
        "html" => pt_text,
        "md" => markdown(&text),
        // standard
        _ => format!("\n<{style}{id}>{pt_text}\n</{style}>"),
    }
}

fn markdown(text: &str) -> String {
    let mut fin = String::new();
    for x in text.split('\n') {
        let mut c = x.chars();

        let (style, popper) = match (c.next(), c.next(), c.next(), c.next(), c.next()) {
            // h5 -> h1
            (Some('#'), Some('#'), Some('#'), Some('#'), Some('#')) => ("h5", 5),
            (Some('#'), Some('#'), Some('#'), Some('#'), _) => ("h4", 4),
            (Some('#'), Some('#'), Some('#'), _, _) => ("h3", 3),
            (Some('#'), Some('#'), _, _, _) => ("h2", 2),
            (Some('#'), _, _, _, _) => ("h1", 1),

            _ => ("p", 0),
        };
        // text minus the md formating
        let mut text_min: String = x.to_owned();
        for _ in 0..popper {
            text_min.remove(0);
        }
        let temp = format!("\n<{style}>\n    {text_min}\n</{style}>");

        fin += &temp
    }
    fin
}

fn pretty_text(text: &str) -> String {
    let mut fin = String::new();
    for x in text.split('\n') {
        fin += &format!("\n    {x}");
    }
    fin
}
