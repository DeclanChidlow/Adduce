mod lib {

    pub mod rfs;
}
mod structs {
    pub mod config;
}

use crate::lib::rfs::{copy_dir, fs_to_str};
use lib::rfs::import_conf;
pub use serde::{Deserialize, Serialize};

fn main() {
    // clone a directory
    copy_dir("origin", "generated");

    // return the content of a file
    println!("{:?}", fs_to_str("bar/foobo"));

    // import config
    let _ = import_conf("config/example.toml");
}
