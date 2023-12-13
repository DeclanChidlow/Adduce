// Import necessary modules
mod lib {
    pub mod html2;
    pub mod rfs;
    pub mod rss;
}
mod structs {
    pub mod html_conf;
    pub mod toml_conf;
}

// Define a constant string for the help message
const HELP: &str = "Adduce - static site generator, blog creator & markdown html tool

Usage: adduce [OPTIONS]

Options:
	-c, --config <path_to_configuration>
	-n, --name <file_name.html>
	-o, --ouput <html_destination>

See `adduce feed` for Adduce Feed (blogger) usage.";

// Import necessary traits from the serde crate
pub use serde::{Deserialize, Serialize};
use structs::html_conf::Generate;

// Main function
fn main() {
    // Get the command line arguments
    let args = args();

    // If there are no command line arguments, print the help message and return
    if args.len() < 2 {
        println!("{HELP}");
        return;
    };

    // If the command line arguments contain "feed", process them with the RSS module and return
    if args.contains(&String::from("feed")) {
        lib::rss::process(args);
        return;
    }

    // If the number of command line arguments is even, print an error message and return
    if args.len() % 2 == 0 {
        println!("invalid args");
        return;
    }

    // Initialize a new Generate object
    let mut genconf = Generate::new();

    // Iterate over the command line arguments and update the Generate object based on them
    for x in 0..args.len() {
        genconf = match args[x].as_str() {
            "--config" | "-c" => genconf.conf_str(&format!("{}/conf.toml", &args[x + 1])),
            "--output" | "-o" => genconf.output_dir(&args[x + 1]),
            "--name" | "-n" => genconf.filename(&args[x + 1]),
            _ => Generate::void(genconf),
        };
    }

    // Generate the final output from the Generate object
    Generate::from_conf(genconf);
}

// Function to get the command line arguments
fn args() -> Vec<String> {
    let mut vec = Vec::new();
    for x in std::env::args() {
        vec.push(x);
    }

    vec
}