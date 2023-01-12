mod lib {

    pub mod html;
    pub mod rfs;
}
mod structs {
    pub mod config;
    pub mod html;
}

use crate::structs::config::Object;
use lib::rfs::{import_conf, str_to_fs};
pub use serde::{Deserialize, Serialize};
use structs::html::{Div, Style};

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
        let temp = Object::new().style(Style::h1).text(&i);
        div.element(temp);
    };
    if let Some(i) = conf.locale {
        let temp = Object::new().style(Style::h3).text(&i);
        div.element(temp);
    };
    if let Some(i) = conf.author {
        let temp = Object::new().style(Style::h4).text(&i);
        div.element(temp);
    };
    if let Some(i) = conf.social {
        if let Some(x) = i.name {
            let temp = Object::new().style(Style::h3).text(&x);
            div.element(temp);
        };
    };

    // div derived from toml block vector (user generated)
    let mut yml_div = Div::new();

    // for every block, generate the style and text (mandatory fields)
    for x in conf.main.unwrap().block.iter() {
        let mut element = Object::new()
            .style(x.style.clone().unwrap_or_default())
            .text(x.text.as_ref().unwrap_or(&String::from("TEMPLATE")));

        // if applicable, generate optional fields
        if x.id.is_some() {
            element.id(x.id.as_ref().unwrap());
        };
        if x.from_str.is_some() {
            element.from_str(x.from_str.as_ref().unwrap());
        }
        yml_div.element(element);
    }

    // compiling divs
    format!(
        "{master}{}{}\n</body>\n</html>",
        div.compile(),
        yml_div.compile(),
    )
}
