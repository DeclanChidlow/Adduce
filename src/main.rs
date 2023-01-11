mod lib {

    pub mod rfs;
    pub mod html;
}
mod structs {
    pub mod config;
    pub mod html;
}

use crate::lib::{rfs::{copy_dir, fs_to_str}, html::md_paragraph};
use lib::{rfs::{import_conf, str_to_fs}};
pub use serde::{Deserialize, Serialize};
use structs::html::{Div, Style};

fn main() {

    // clone a directory
    //copy_dir("origin", "generated");

    // return the content of a file
    //println!("{:?}", fs_to_str("bar/foobo"));

    // define a file from string
    //str_to_fs("generated/index.html", &gen);

    // import config
    let conf = import_conf("config/example.toml");


    // define hyml based on config and 
    let html = compile_html(conf);

    str_to_fs("generated/index.html", &html);


}

fn compile_html(conf: structs::config::Conf)  -> String {

    // all divs are compiled and added to master
    let mut master = String::from("<html>");

    // head

    master += "\n<head>\n</head>";

    // body

    master += "\n<body>\n";
    // head div

    let mut div = Div::new();

    if let Some(i) = conf.title {
        div.add(Style::H1, &i);
    };
    if let Some(i) = conf.locale {
        div.add(Style::H2, &i);
    };
    if let Some(i) = conf.author {
        div.add(Style::H3, &i);
    };
    if let Some(i) = conf.social {
        if let Some(x) = i.name {
            div.add(Style::H4, &x);
        };
    };

    master +=  &div.compile();


    master += "\n</body>\n</html>";
    master
}
