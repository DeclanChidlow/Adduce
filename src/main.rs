mod lib {
    pub mod html2;
    pub mod rfs;
}
mod structs {
    pub mod html_conf;
    pub mod toml_conf;
}

pub use serde::{Deserialize, Serialize};
use structs::html_conf::Generate;

fn main() {
    let genconf = Generate::new()
        .conf_str("config/conf.toml")
        .output_dir("output");

    Generate::from_conf(genconf)
}
