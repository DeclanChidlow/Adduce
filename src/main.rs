mod lib {
    pub mod feed;
    pub mod filesystem;
    pub mod parse;
}

mod config {
    pub mod html;
    pub mod toml;
}

const HELP: &str = r#"
Adduce - Versatile static site generator written in Rust

Usage: adduce [OPTIONS]

Options:
    -c, --config <path_to_configuration>
    -n, --name <file_name.html>
    -o, --output <html_destination>

See `adduce feed` for Adduce Feed usage.
"#;

use config::html::Generate;
pub use serde::{Deserialize, Serialize};
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("{HELP}");
        return;
    }

    // Process feed module if it's the primary command
    if args[0] == "feed" {
        lib::feed::process(args);
        return;
    }

    if args.len() % 2 != 0 {
        println!("Invalid arguments. Ensure options are provided in pairs.");
        return;
    }

    let mut genconf = Generate::new();
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        let Some(val) = iter.next() else { break };

        genconf = match arg.as_str() {
            "--config" | "-c" => genconf.conf_str(&format!("{}/conf.toml", val)),
            "--output" | "-o" => genconf.output_dir(val),
            "--name" | "-n" => genconf.filename(val),
            _ => {
                eprintln!("Unknown argument: {arg}");
                return;
            }
        };
    }

    genconf.generate();
}
