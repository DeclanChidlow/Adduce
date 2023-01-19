mod lib {
    pub mod html2;
    pub mod rfs;
}
mod structs {
    pub mod config;
}
use lib::rfs::{import_conf, str_to_fs};

pub use serde::{Deserialize, Serialize};

fn main() {
    // import config
    let conf = import_conf("config/example.toml");

    let html = conf.to_html();

    str_to_fs("index.html", &html);
}
