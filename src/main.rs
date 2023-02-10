mod lib {
    pub mod html2;
    pub mod rfs;
}
mod structs {
    pub mod html_conf;
    pub mod toml_conf;
}

const HELP: &str = "--generate\n";

pub use serde::{Deserialize, Serialize};
use structs::html_conf::Generate;

fn main() {
    let args = args();

    if args.len() < 2 {
        println!("{}", HELP);
        return;
    };

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
