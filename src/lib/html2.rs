use crate::structs::toml_conf::{Conf, Object};

use super::rfs::fs_to_str;

impl Conf {
    pub fn to_html(&self) -> String {
        let mut divs = String::new();
        for x in self.main.clone().unwrap_or_default().block.iter() {
            divs += &compile_html(x);
        }

        // CSS stylesheets - not to be confused with 'style'
        let mut style_conf = String::new();

        if let Some(styles) = self.clone().style {
            for x in styles {
                style_conf += &fs_to_str(&x);
            }
        }

        let mut head_conf = String::new();
        if let Some(head) = self.clone().head {
            for x in head {
                head_conf += &fs_to_str(&x);
            }
        }

        format!("<!DOCTYPE html>\n<head>\n<style>\n{style_conf}\n</style>\n{head_conf}\n</head>\n<body>\n<div class=\"page\">\n{divs}\n</div>\n</body>")
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
        Some(a) => format!(" id=\"{a}\""),
    };

    // mutually exclusive conditionals
    let mut html = match &style as &str {
        "br" => String::from("\n<br>"),
        "html" => String::from("\n") + &pt_text,
        "md" => markdown(&text),
        // standard
        _ => format!("\n<{style}{id}>{pt_text}\n</{style}>"),
    };

    // optional restructing

    // anchor
    if let Some(link) = conf.link {
        html = format!("\n<a href=\"{link}\">\n{html}\n</a>");
    };

    html
}

fn markdown(text: &str) -> String {
    let mut fin = String::new();
    let mut inside_codeblock = false;
    for x in text.split('\n') {
        let mut x = x.to_string();

        while x.starts_with(' ') {
            x.remove(0);
        }

        let mut c = x.chars();

        let (style, popper, at_end) =
            match (c.next(), c.next(), c.next(), c.next(), c.next(), c.next()) {
                // h6 -> h1
                (Some('#'), Some('#'), Some('#'), Some('#'), Some('#'), Some('#')) => {
                    ("h6", 6, false)
                }
                (Some('#'), Some('#'), Some('#'), Some('#'), Some('#'), _) => ("h5", 5, false),
                (Some('#'), Some('#'), Some('#'), Some('#'), _, _) => ("h4", 4, false),
                (Some('#'), Some('#'), Some('#'), _, _, _) => ("h3", 3, false),
                (Some('#'), Some('#'), _, _, _, _) => ("h2", 2, false),
                (Some('#'), _, _, _, _, _) => ("h1", 1, false),

                // bold italics
                (Some('*'), Some('*'), _, _, _, _) => ("bold", 2, true),
                (Some('*'), _, _, _, _, _) => ("italic", 1, true),

                // quotes
                (Some('>'), Some('>'), _, _, _, _) => ("nestquote", 2, false),
                (Some('>'), _, _, _, _, _) => ("quote", 1, false),

                (Some('<'), Some('/'), _, _, _, _) => ("html_end", 0, false),
                (Some('<'), Some(_), _, _, _, _) => ("html_start", 0, false),

                // images
                (Some('!'), Some('['), Some(_), Some(_), Some(_), Some(_)) => ("img_emb", 0, false),

                // links
                (Some('['), Some(_), Some(_), Some(_), Some(_), Some(_)) => ("link_emb", 0, false),

                (Some('-'), _, _, _, _, _) => ("li", 1, false),



                (Some(' '), _, _, _, _, _) => ("no", 0, false),
                (Some('`'), Some('`'), Some('`'), Some(_), _, _) => ("code_block", 3, false),
                (Some('`'), Some('`'), Some('`'), None, _, _) => ("code_block_end", 3, false),

                (Some('`'), ..) => ("code", 1, true),

                (Some('\n'), _, _, _, _, _) => ("no", 0, false),

                (Some(_), _, _, _, _, _) => ("p", 0, false),

                _ => ("no", 0, false),
            };

        // text minus the md formating
        let mut text_min: String = x.to_owned();

        for _ in 0..popper {
            text_min.remove(0);
            if at_end {
                text_min.pop();
            };
        }

        // Set inside_codeblock if the target style
        // is the start or end of a codeblock.
        // This is after the inside_codeblock so there
        // isn't any extra line endings on codeblocks.
        inside_codeblock = match style {
            "code_block" => true,
            "code_block_end" => false,
            _ => inside_codeblock,
        };

        // add support for proper use of pre/code elements
        // for actual support for monospace/codeblocks.
        fin += match (style, inside_codeblock) {
            // breakline in and out of codeblock
            ("br", true) => String::from("\n"),
            ("br", false) => String::from("\n<br>"),

            //  nulltext
            ("no", _) => String::new(),

            ("html_start", _) | ("html_end", _) => format!("\n{text_min}"),

            ("img_emb", _) => embed(&text_min, "img_emb"),
            ("link_emb", _) => embed(&text_min, "link_emb"),

            // codeblock start and end
            ("code_block", _) => String::from("\n<pre class=\"codeblock\"><code>"),
            ("code_block_end", _) => String::from("\n</code></pre>"),

            // html in and out of codeblock
            (_, false) => format!("\n<{style}>\n    {text_min}\n</{style}>"),
            (_, true) => format!("\n{text_min}"),
        }
        .as_ref();
    }

    fin
}

fn embed(text: &str, style: &str) -> String {
    let text = text.replace(['!', '[', ']', ')'], "");

    let text_split: Vec<String> = text.split('(').map(|s| s.to_string()).collect();

    match style {
        "img_emb" => format!("<img src=\"{}\" alt=\"{}\">", text_split[1], text_split[0]),
        "link_emb" => format!("<a href=\"{}\">{}</a> ", text_split[1], text_split[0]),
        _ => String::new(),
    }
}

fn pretty_text(text: &str) -> String {
    let mut fin = String::new();
    for x in text.split('\n') {
        fin += &format!("\n    {x}");
    }
    fin
}
