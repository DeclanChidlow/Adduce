use crate::config::toml::Conf;
use rss::{ChannelBuilder, ItemBuilder};
use std::{fs, process::Command};
use crate::config::toml::Object;
use crate::config::toml::Main;

const HELP: &str = r#"
Adduce Feed - create blogs or other simple documents.

Usage: adduce feed [COMMAND] <argument>

Commands:
    new <file_name>     create new article
    edit <file_name>    modify an existing article
    publish <file_name> build the file with Adduce"

See `adduce` for Adduce Standard usage.
"#;

// Main function to process command line arguments
pub fn process(args: Vec<String>) {
    feed_dir();

    if args.len() < 3 {
        println!("{HELP}");
        return;
    }

    let command = args[1].as_str();
    let argument = args[2].as_str();

    match command {
        "new" => cli_new(argument),
        "edit" => cli_edit(argument),
        "publish" => cli_pub(argument),
        "search" => cli_search(argument),
        "rm" => cli_remove(argument),
        "conf" if argument == "generate" => conf_make(),
        _ => println!("{HELP}"),
    }
}

// TODO: Set item title to og:title in the header of the document
// TODO: Set item description to contents of <article> tag in the document

// Generate RSS feed based on contents of feed/documents/
fn generate_rss() {
    let mut items = Vec::new();

    for entry in fs::read_dir("feed/documents/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let content = fs::read_to_string(&path).unwrap_or_default();

        let item = ItemBuilder::default()
            .title(Some(path.file_name().unwrap().to_string_lossy().to_string()))
            .description(Some(content))
            .build();

        items.push(item);
    }

    let conf = match fs::read_to_string("feed/conf.toml") {
        Ok(content) => toml::from_str::<Conf>(&content).unwrap(),
        Err(e) => {
            println!("{e}\nUse `feed conf generate` to make a conf file.");
            return;
        }
    };

    // Use the title, link, and description from the conf.toml file
    let channel_title = conf.title.unwrap_or_else(|| "My RSS Feed".to_string());
    let channel_link = conf.link.unwrap_or_else(|| "https://example.com".to_string());
    let channel_description = conf.description.unwrap_or_else(|| "An RSS feed of my documents".to_string());

    let channel = ChannelBuilder::default()
        .title(channel_title)
        .link(channel_link)
        .description(channel_description)
        .generator(Some("Adduce".to_string()))
        .items(items)
        .build();

    fs::write("feed/export/feed.rss", channel.to_string()).expect("Failed to write RSS feed.");
}

// Function to remove a document
fn cli_remove(filename: &str) {
    if let Err(error) = fs::remove_file(format!("feed/documents/{filename}.md")) {
        println!("Error removing document: {error}");
    }
}

// Function to search for a document
fn cli_search(keyword: &str) {
    let entries = fs::read_dir("feed/documents/")
        .expect("Failed to read documents directory.")
        .filter_map(|entry| entry.ok().map(|e| e.file_name().into_string().unwrap_or_default()));

    for entry in entries {
        if entry.contains(keyword) {
            println!("{entry}");
        }
    }
}

fn cli_pub(document: &str) {
    let conf = match fs::read_to_string("feed/conf.toml") {
        Ok(content) => toml::from_str::<Conf>(&content).unwrap(),
        Err(e) => {
            println!("{e}\nUse `feed conf generate` to make a conf file.");
            return;
        }
    };

    let text = Object {
        from_str: Some(format!("feed/documents/{document}.md")),
        style: Some(String::from("md")),
        ..Default::default()
    };

    let mut toml = conf;
    if toml.main.is_none() {
        toml.main = Some(Main { block: vec![text] });
    } else {
        toml.main.as_mut().unwrap().block.push(text);
    }

    fs::write(format!("feed/export/{document}.html"), toml.to_html()).expect("Failed to write HTML.");
    generate_rss();
}

// Function to create a configuration file
fn conf_make() {
    if fs::read("feed/conf.toml").is_ok() {
        println!("This file already exists. Press enter if you wish to continue.");

        let mut response = String::new();
        std::io::stdin().read_line(&mut response).unwrap();
        let response = response.trim();

        if !response.is_empty() {
            return;
        }
    }

    let generate = conf_wizard();
    let toml = toml::to_string_pretty(&generate).expect("Failed to serialize configuration.");

    fs::write("feed/conf.toml", toml).expect("Failed to write configuration file.");
}

fn conf_wizard() -> Conf {
    let mut conf = Conf::new();

    println!("Author's name? Press enter for none:");
    let mut author = String::new();
    std::io::stdin().read_line(&mut author).unwrap();
    let author = author.trim();

    if !author.is_empty() {
        conf.author = Some(author.to_string());
    }

    println!("Pre-page content? Enter filenames from 'feed/content' (press enter to finish):");
    let mut before_page = Vec::new();

    for entry in fs::read_dir("feed/content").unwrap() {
        println!("{}", entry.unwrap().file_name().to_string_lossy());
    }

    loop {
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).unwrap();
        let temp = temp.trim();

        if temp.is_empty() {
            break;
        }

        let dir = format!("feed/content/{}", temp);
        if let Ok(_) = fs::File::open(&dir) {
            let file_type = dir.split('.').last().unwrap_or("");
            let temp_object = Object {
                style: Some(file_type.to_string()),
                from_str: Some(dir.clone()),
                ..Default::default()
            };
            before_page.push(temp_object);
        } else {
            println!("File not found: {}", dir);
        }
    }

    if !before_page.is_empty() {
        conf.main = Some(Main { block: before_page });
    }

    conf
}

fn cli_edit(filename: &str) {
    let path = format!("feed/documents/{filename}.md");

    if fs::read(&path).is_err() {
        println!("No article with this name.");
        return;
    }

    Command::new("nvim")
        .arg(path)
        .spawn()
        .expect("Failed to launch editor")
        .wait()
        .expect("Editor exited with error");
}

fn cli_new(filename: &str) {
    let path = format!("feed/documents/{filename}.md");

    if fs::read(&path).is_ok() {
        println!("An article with this name already exists.");
        return;
    }

    Command::new("nvim")
        .arg(path)
        .spawn()
        .expect("Failed to launch editor")
        .wait()
        .expect("Editor exited with error");
}

fn feed_dir() {
    for dir in &[
        "feed",
        "feed/documents",
        "feed/export",
        "feed/styles",
        "feed/content",
    ] {
        if fs::read_dir(dir).is_err() {
            println!("Directory not found: {dir}, creating...");
            fs::create_dir(dir).expect("Failed to create directory.");
        }
    }
}
