// Import necessary libraries and modules
use crate::structs::toml_conf::{Conf, Main, Object};
use rss::{ChannelBuilder, ItemBuilder};
use std::fs;
use std::process::Command;

// Define constant strings for help messages
const HELP: &str = "Adduce Feed - create blogs or other simple documents.

Usage: adduce feed [COMMAND] <argument>

Commands:
	new <file_name> 	create new article
	edit <file_name>	modify an existing article
	publish <file_name>	build the file with Adduce";

// Main function to process command line arguments
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
        ([.., "new"], _) => println!("New requires article name."),
        ([.., "edit"], _) => println!("Edit requires article name."),
        ([.., "publish"], _) => println!("{GEN_HELP}"),
        ([.., "search"], _) => println!("Search requires article name."),
        ([.., "rm"], _) => println!("Remove requires article name."),

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

fn generate_rss() {
    let mut items = Vec::new();

    for entry in fs::read_dir("feed/documents/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let content = fs::read_to_string(&path).unwrap_or_default();

        let item = ItemBuilder::default()
            .title(Some(
                path.file_name().unwrap().to_string_lossy().to_string(),
            ))
            .description(Some(content))
            .build();

        items.push(item);
    }

    let channel = ChannelBuilder::default()
        .title("My RSS Feed".to_string())
        .link("http://example.com".to_string())
        .description("An RSS feed of my documents".to_string())
        .items(items)
        .build();

    fs::write("feed.rss", channel.to_string()).unwrap();
}

// Function to remove a document
fn cli_remove(a: &str) {
    if let Err(error) = std::fs::remove_file(format!("feed/documents/{a}.md")) {
        println!("Error removing document\n{error}");
    };
}

// Function to search for a document
fn cli_search(a: &str) {
    let mut list = Vec::new();

    for x in std::fs::read_dir("feed/documents/").unwrap() {
        list.push(x.unwrap().file_name().into_string().unwrap_or_default());
    }

    for x in list {
        if x.contains(a) {
            println!("{x}");
        };
    }
}

const GEN_HELP: &str = "Adduce Feed Generate
generate <name> <platform>
run generate help for more info";

// Function to publish a document
fn cli_pub(document: &str) {
    // adduce feed generate article-name platform

    let conf = match std::fs::read("feed/conf.toml") {
        Ok(a) => String::from_utf8(a).unwrap(),
        Err(e) => {
            println!("{e}\nUse `feed conf generate` to make a conf file.");
            return;
        }
    };

    let mut toml = toml::from_str::<Conf>(&conf).unwrap();

    // adding content to toml

    let text = Object {
        from_str: Some(format!("feed/documents/{document}.md")),
        style: Some(String::from("md")),
        ..Default::default()
    };

    if toml.main.is_none() {
        toml.main = Some(Main { block: vec![text] });
    } else {
        toml.main.as_mut().unwrap().block.push(text);
    };

    std::fs::write(format!("feed/export/{document}.html"), toml.to_html()).unwrap();

    generate_rss();
}

// Function to create a configuration file
fn conf_make() {
    if std::fs::read("feed/conf.toml").is_ok() {
        println!("This file already exists. Press enter if you wish to continue.");

        let mut response = String::new();
        std::io::stdin().read_line(&mut response).unwrap();
        let response = response.trim();

        if !response.is_empty() {
            return;
        };
    };

    let generate = conf_wizard();

    let toml = toml::to_string_pretty(&generate).unwrap();

    std::fs::write("feed/conf.toml", toml).unwrap();
}

// Function to interactively create a configuration file
fn conf_wizard() -> Conf {
    let mut conf = Conf::new();

    println!("Author's name? Press enter for none");
    let mut author = String::new();
    std::io::stdin().read_line(&mut author).unwrap();
    let author = author.trim();

    if !author.is_empty() {
        conf.author = Some(String::from(author));
    };

    // STYLESHEETS

    let mut style = Vec::new();

    let mut iter = 0;

    println!("Stylesheet? Pick one\n'enter' to continue");
    for x in std::fs::read_dir("feed/styles").unwrap() {
        println!("{}", x.unwrap().file_name().to_string_lossy());
        iter += 1;
    }

    if iter == 0 {
        let yeslist = ["y", "Y", "yes", "Yes", "YES"];

        println!("Failed to find stylesheet. Would you like to download one from us?");
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).unwrap();
        let temp = temp.trim();

        if yeslist.contains(&temp) {
            std::process::Command::new("wget")
                .args(vec![
                    "https://raw.githubusercontent.com/toastxc/Adduce/main/config/style.css",
                    "-S",
                    "-P",
                    "feed/styles/",
                ])
                .spawn()
                .unwrap()
                .wait_with_output()
                .unwrap();

            println!("Stylesheet? Pick one\n'next' to continue");

            for x in std::fs::read_dir("feed/styles/").unwrap() {
                println!("{}", x.unwrap().file_name().to_string_lossy());
                iter += 1;
            }
        }
    };

    loop {
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).unwrap();
        temp = temp.trim().to_string();

        let dir = format!("feed/styles/{temp}");

        if dir == *"feed/styles/" {
            break;
        };

        match std::fs::File::open(&dir) {
            Ok(_) => style.push(dir),
            Err(e) => println!("{e}"),
        };
    }

    conf.style = match style.is_empty() {
        true => None,
        false => Some(style),
    };

    println!("pre-page content?");
    for x in std::fs::read_dir("feed/content").unwrap() {
        println!("{}", x.unwrap().file_name().to_string_lossy());
    }

    let mut before_page: Vec<Object> = Vec::new();

    loop {
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).unwrap();
        temp = temp.trim().to_string();

        let dir = format!("feed/content/{temp}");

        if dir == *"feed/content/" {
            break;
        };

        let file_type: Vec<&str> = dir.split('.').collect();

        let temp_object = Object {
            style: Some(String::from(file_type[1])),
            from_str: Some(String::from(&dir)),
            ..Default::default()
        };

        match std::fs::File::open(&dir) {
            Ok(_) => before_page.push(temp_object),
            Err(e) => println!("{e}"),
        };
    }

    if !before_page.is_empty() {
        conf.main = Some(Main { block: before_page })
    }

    conf
}

// Function to edit a document
fn cli_edit(a: &str) {
    let path = format!("feed/documents//{a}.md");

    if std::fs::read(&path).is_err() {
        println!("No article with this name.");
        return;
    }

    Command::new("nvim")
        .arg(path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

// Function to create a new document
fn cli_new(a: &str) {
    let path = format!("feed/documents/{a}.md");

    if std::fs::read(&path).is_ok() {
        println!("An article with this name already exists.");
        return;
    }

    Command::new("nvim")
        .arg(path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

// Function to ensure necessary directories exist
fn feed_dir() {
    for x in &[
        "feed",
        "feed/documents",
        "feed/export",
        "feed/styles",
        "feed/content",
    ] {
        if std::fs::read_dir(x).is_err() {
            println!("Could not find directory {x}, creating...");

            if let Err(error) = std::fs::create_dir(x) {
                println!("Could not create directory\n{error}");
            }
        };
    }
}
