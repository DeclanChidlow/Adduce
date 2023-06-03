/*
use std::process::Command;

use crate::data::toml::{Conf, Main, Object};

const HELP: &str = "Adduce Feed - create blogs or other simple documents.

Usage: adduce feed [COMMAND] <argument>

Commands:
    new <file_name> 	create new article
    edit <file_name>	modify an existing article
    publish <file_name>	build the file with Adduce";

pub fn process(args: Vec<String>) {
    feed_dir();

    // turned into a vec of strs
    let mut newstr = String::new();
    for x in args {
        newstr += &format!(" {x}");
    }

    let args: Vec<&str> = newstr.split(' ').collect();

    match (args.as_slice(), args.len()) {
        // exceptions
        (_, 2) | (_, 3) => println!("{HELP}"),

        // CLI incomplete
        ([.., "new"], _) => println!("New requires article name"),
        ([.., "edit"], _) => println!("Edit requires article name"),
        ([.., "publish"], _) => println!("{GEN_HELP}"),
        ([.., "search"], _) => println!("Search requires article name"),
        ([.., "rm"], _) => println!("Remove requires article name"),

        // CLI
        ([.., "new", a], _) => cli_new(a),
        ([.., "edit", a], _) => cli_edit(a),
        ([.., "publish", a], _) => cli_pub(a),
        ([.., "search", a], _) => cli_search(a),
        ([.., "rm", a], _) => cli_remove(a),
        ([.., "conf", "generate"], _) => conf_make(),

        // un accounted for
        _ => panic!("no"),
    };
}
*/

use crate::common::result::Error;

const _HELP: &str = "Adduce Feed - create blogs or other simple documents.

Usage: adduce feed [COMMAND] <argument>

Commands:
    new <file_name> 	create new article
    edit <file_name>	modify an existing article
    publish <file_name>	build the file with Adduce";

pub fn process(args: Vec<String>) -> Result<(), Error> {
    println!("{:#?}", args);

    Ok(())
}
