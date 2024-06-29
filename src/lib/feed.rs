use rss::{ChannelBuilder, ItemBuilder};
use std::{fs, env, process::Command};
use crate::config::toml::{Conf, Object, Main};

const HELP: &str = r#"
Adduce Feed - create blogs or other simple documents.

Usage: adduce feed [COMMAND] <argument>

Commands:
    establish               create feed structure
    create <file_name>      create new document
    remove <file_name>      delete a document
    edit <file_name>        modify an existing document
    generate <file_name>    generate HTML from document
    search <query>          search your documents

See `adduce` for Adduce Standard usage.
"#;

pub fn process(args: Vec<String>) {

    if args.len() < 2 {
        println!("{HELP}");
        return;
    }

    let command = args[1].as_str();

    match command {
        "establish" => cli_establish(),
        "create" | "remove" | "edit" | "generate" | "search" => {
            if args.len() < 3 {
                println!("{HELP}");
                return;
            }
            let argument = args[2].as_str();
            match command {
                "create" => cli_create(argument),
                "remove" => cli_remove(argument),
                "edit" => cli_edit(argument),
                "generate" => cli_generate(argument),
                "search" => cli_search(argument),
                _ => println!("{HELP}"),
            }
        }
        _ => println!("{HELP}"),
    }
}

// Create the required directory structure
fn cli_establish() {
    for dir in &[
        "feed",
        "feed/documents",
        "feed/export",
    ] {
        if fs::read_dir(dir).is_err() {
            println!("Creating {dir}...");
            fs::create_dir(dir).expect("Failed to create {dir}.");
        }
    }
}

// Create a new document
fn cli_create(filename: &str) {
    let folder_path = "feed/documents";
    let file_path = format!("{folder_path}/{filename}.md");

    if !fs::metadata(folder_path).is_ok() {
        eprintln!("The documents folder does not exist. Please run `adduce feed establish` to create the necessary file structure.");
        return;
    }

    if fs::metadata(&file_path).is_ok() {
        eprintln!("Document already exists: {file_path}.");
        return;
    }

    let initial_content = format!("# {filename}\n");
    if let Err(err) = fs::write(&file_path, initial_content) {
        eprintln!("Failed to create file {file_path}: {err}.");
        return;
    }

    println!("Created new file: {file_path}.");
}

// Remove a requested document
fn cli_remove(filename: &str) {
    let md_file_path = format!("feed/documents/{filename}.md");
    if let Err(error) = fs::remove_file(&md_file_path) {
        println!("Error removing source document {filename}: {error}.");
    } else {
        println!("Deleted source document '{filename}'.");
    }

    let html_file_path = format!("feed/export/{filename}.html");
    if let Err(error) = fs::remove_file(&html_file_path) {
        println!("Error removing exported document {filename}: {error}.");
    } else {
        println!("Deleted exported document '{filename}'.");
    }
    
    generate_rss();
}

// Edit a requested document
fn cli_edit(filename: &str) {
    let file_path = format!("feed/documents/{filename}.md");

    if fs::read(&file_path).is_err() {
        println!("No documents with that name.");
        return;
    }

        let editor_command = env::var("EDITOR").unwrap_or_else(|_| "notepad".to_string());

    Command::new(editor_command)
        .arg(file_path)
        .spawn()
        .expect("Failed to launch editor.")
        .wait()
        .expect("Editor exited with error.");
}

// Generate a HTML version of the input document
fn cli_generate(document: &str) {
    let md_file_path = format!("feed/documents/{document}.md");
    if fs::metadata(&md_file_path).is_err() {
        println!("Input file '{document}' does not exist. Please create it first.");
        return;
    }

    let conf = match fs::read_to_string("feed/conf.toml") {
        Ok(content) => toml::from_str::<Conf>(&content).unwrap(),
        Err(e) => {
            println!("{e}\nYou must manually create a conf.toml file for your feed.");
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

    if let Err(err) = fs::write(format!("feed/export/{document}.html"), toml.to_html()) {
        eprintln!("Failed to generate {document}: {err}.");
        return;
    }

    generate_rss();

    println!("Successfully generated {document}.");
}

// Search documents
fn cli_search(keyword: &str) {
    let entries = fs::read_dir("feed/documents/")
        .expect("Failed to read documents directory.")
        .filter_map(|entry| entry.ok().map(|e| e.file_name().into_string().unwrap_or_default()));

        let mut found_results = false;

    for entry in entries {
        if entry.contains(keyword) {
            println!("{entry}");
            found_results = true;
        }
    }

    if !found_results {
        println!("No results found for '{keyword}'.");
    }
}

// TODO: Set item title to og:title in the header of the document
// TODO: Set item description to contents of <article> tag in the document

// Generate an RSS feed
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
            println!("{e}\nNo configuration file found.");
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

