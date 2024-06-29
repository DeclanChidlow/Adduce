mod lib {
    pub mod feed;
    pub mod filesystem;
    pub mod markdown;
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

pub use serde::{Deserialize, Serialize};
use config::html::Generate;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("{HELP}");
        return;
    }

    // If the command line arguments contain "feed", process them with the feed module and return
    if args.contains(&String::from("feed")) {
        lib::feed::process(args);
        return;
    }

    if args.len() % 2 != 0 {
        println!("Invalid arguments");
        return;
    }

    let mut genconf = Generate::new();
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        genconf = match arg.as_str() {
            "--config" | "-c" => {
                if let Some(path) = iter.next() {
                    genconf.conf_str(&format!("{}/conf.toml", path))
                } else {
                    eprintln!("Error: --config requires a path");
                    return;
                }
            }
            "--output" | "-o" => {
                if let Some(path) = iter.next() {
                    genconf.output_dir(path)
                } else {
                    eprintln!("Error: --output requires a path");
                    return;
                }
            }
            "--name" | "-n" => {
                if let Some(name) = iter.next() {
                    genconf.filename(name)
                } else {
                    eprintln!("Error: --name requires a file name");
                    return;
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                return;
            }
        };
    }

    Generate::from_conf(genconf);
}
