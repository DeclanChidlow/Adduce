mod lib {

    pub mod html;
    pub mod rfs;
}
mod structs {
    pub mod config;
    pub mod html;
}

use crate::{lib::rfs::fs_to_str, structs::html::Values};
use lib::rfs::{import_conf, str_to_fs};
pub use serde::{Deserialize, Serialize};
use structs::html::{Div, Style};
use toml::Value;

fn main() {
    // import config
    let conf = import_conf("config/example.toml");

    // define hyml based on config and
    let html = compile_html(conf);

    str_to_fs("index.html", &html);
    test();
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
        let temp = Values::new().style(Style::h1).text(&i);
        div.element(temp);
    };
    if let Some(i) = conf.locale {
        let temp = Values::new().style(Style::h3).text(&i);
        div.element(temp);
    };
    if let Some(i) = conf.author {
        let temp = Values::new().style(Style::h4).text(&i);
        div.element(temp);
    };
    if let Some(i) = conf.social {
        if let Some(x) = i.name {
            let temp = Values::new().style(Style::h3).text(&x);
            div.element(temp);
        };
    };

    // div derived from toml block vector (user generated)

    let mut yml_div = Div::new();

    for x in conf.main.unwrap().block.iter() {
        let element = Values::new()
            .style(x.clone().style.unwrap_or(Style::None))
            .text(&x.clone().content.unwrap_or(String::from("TEMPLATE")))
            .id(&x.id.clone().unwrap_or(String::new()));

        yml_div.element(element);
    }

    // compiling divs
    format!(
        "{master}{}{}\n</body>\n</html>",
        div.compile(),
        yml_div.compile(),
    )
}

fn test() {
    let mut div = Div::new();

    let element_1 = Values::new()
        .style(Style::None)
        .text("hewo!")
        .id("44434343");

    div.element(element_1);

    println!("{:?}", div);
}
