mod lib {

    pub mod html;
    pub mod rfs;
}
mod structs {
    pub mod config;
    pub mod html;
}

use crate::lib::rfs::{fs_to_str};
use lib::rfs::{import_conf, str_to_fs};
pub use serde::{Deserialize, Serialize};
use structs::{html::{Div, Style}};

fn main() {
    // import config
    let conf = import_conf("config/example.toml");

    // define hyml based on config and
    let html = compile_html(conf);

    str_to_fs("index.html", &html);
}

fn compile_html(conf: structs::config::Conf) -> String {
    // all divs are compiled and added to master
    let mut master = String::from("<html>");

    // head

    master += "\n<head>\n</head>";

    // body

    master += "\n<body>\n";


    // div derived from config conditons

    let mut div = Div::new();

    if let Some(i) = conf.title {
        div.add(Style::h1, &i);
    };
    if let Some(i) = conf.locale {
        div.add(Style::h2, &i);
    };
    if let Some(i) = conf.author {
        div.add(Style::h3, &i);
    };
    if let Some(i) = conf.social {
        if let Some(x) = i.name {
            div.add(Style::h4, &x);
        };
    };

    // div derived from toml block vector (user generated)
    let mut yml_div = Div::new();

    for x in conf.main.unwrap().block.iter() {
        if x.from_str.is_none() {
            yml_div.add(x.style.clone().unwrap_or(Style::None), &x.content.clone().unwrap_or_else(|| String::from("PLACEHOLDER")));
        }else {
            yml_div.add(x.style.clone().unwrap_or(Style::None), &fs_to_str(&x.from_str.clone().unwrap()));
        };

    }
    // compiling divs
    format!(
        "{master}{}{}\n</body>\n</html>",
        div.compile(),
        yml_div.compile(),
    )
}