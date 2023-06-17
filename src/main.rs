use adduce::{
    common::{
        fs::args,
        result::{CLIErrors, Error, ErrorType},
    },
    data::html::Generate,
    feed,
};

fn main() {
    let args = args();

    const HELP: &str = "Adduce - static site generator, blog creator & markdown html tool

Usage: adduce [OPTIONS]

Options:
    -c, --config <path_to_configuration>
    -n, --name <file_name.html>
    -o, --ouput <html_destination>

See `adduce feed` for Adduce Feed (blogger) usage.";

    // if the length is 2 then there are no CLI args
    if args.len() < 2 {
        println!("{HELP}");
        return;
    };

    // adduce feed is a seperate service to the main site builder
    if args.contains(&String::from("feed")) {
        if let Err(error) = feed::cli::process(args) {
            println!("{:#?}", error)
        };
        return;
    }

    // main adduce engine
    if let Err(error) = adduce_standard(args) {
        println!("{:#?}", error);
    }
}

fn adduce_standard(args: Vec<String>) -> Result<(), Error> {
    // the CLI is strucuted so that - if used properly there are an odd number of args
    if args.len() % 2 == 0 {
        return Err(ErrorType::CLI(CLIErrors::TooFewArguments).into());
    }

    let mut genconf = Generate::new();

    for x in 0..args.len() {
        genconf = match args[x].as_str() {
            "--config" | "-c" => genconf.conf_str(&format!("{}/conf.toml", &args[x + 1]))?,
            "--output" | "-o" => genconf.output_dir(&args[x + 1]),
            "--name" | "-n" => genconf.filename(&args[x + 1]),
            _ => Generate::void(genconf),
        };
    }

    Generate::from_conf(genconf)
}
