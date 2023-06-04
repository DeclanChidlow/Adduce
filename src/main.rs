mod lib {
    pub mod html2;
    pub mod rfs;
    pub mod rss;
}
mod structs {
    pub mod html_conf;
    pub mod toml_conf;
}

const HELP: &str = "Adduce - static site generator, blog creator & markdown html tool

Usage: adduce [OPTIONS]

Options:
	-c, --config <path_to_configuration>
	-n, --name <file_name.html>
	-o, --ouput <html_destination>

See `adduce feed` for Adduce Feed (blogger) usage.";

pub use serde::{Deserialize, Serialize};
use structs::html_conf::Generate;

fn main() {
    let args = args();

    // if the length is 2 then there are no CLI args
    if args.len() < 2 {
        println!("{HELP}");
        return;
    };

    // adduce feed is a seperate service to the main site builder
    if args.contains(&String::from("feed")) {
        lib::rss::process(args);
        return;
    }

    // the CLI is strucuted so that - if used properly there are an odd number of args
    if args.len() % 2 == 0 {
        println!("invalid args");
        return;
    }

    let mut genconf = Generate::new();

    for x in 0..args.len() {
        genconf = match args[x].as_str() {
            "--config" | "-c" => genconf.conf_str(&format!("{}/conf.toml", &args[x + 1])),
            "--output" | "-o" => genconf.output_dir(&args[x + 1]),
            "--name" | "-n" => genconf.filename(&args[x + 1]),
            _ => Generate::void(genconf),
        };
    }

    Generate::from_conf(genconf);
}

fn args() -> Vec<String> {
    let mut vec = Vec::new();
    for x in std::env::args() {
        vec.push(x);
    }

    vec
}
