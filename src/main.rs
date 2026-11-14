mod lib {
    pub mod document;
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
    -h, --help
    -V, --version

See `adduce feed` for Adduce Feed usage.
"#;

use config::html::Generate;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || matches!(args[0].as_str(), "--help" | "-h") {
        println!("{HELP}");
        return ExitCode::SUCCESS;
    }

    if matches!(args[0].as_str(), "--version" | "-V") {
        println!("adduce {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    // Process feed module if it's the primary command
    if args[0] == "feed" {
        lib::feed::process(args);
        return ExitCode::SUCCESS;
    }

    if args.len() % 2 != 0 {
        eprintln!("Invalid arguments. Ensure options are provided in pairs.");
        return ExitCode::FAILURE;
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
                return ExitCode::FAILURE;
            }
        };
    }

    match genconf.generate() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
