use crate::config::toml::{Conf, Main, Object};
use crate::lib::filesystem::fs_to_str;
use atom_syndication::{ContentBuilder, EntryBuilder, FeedBuilder, GeneratorBuilder, Text};
use chrono::Utc;
use std::{env, fs, process::Command};
use toml::de::Error as TomlError;

const HELP: &str = r#"
Adduce Feed - create pages with shared configuration.

Usage: adduce feed [COMMAND] <argument>

Commands:
    establish                   create directory structure
    create <document_name>      create new document
    remove <document_name>      delete a document
    edit <document_name>        modify an existing document
    export <document_name>      generate HTML from document
    search <query>              search your documents
    atom                        generate Atom feed

See `adduce` for creating individual pages.
"#;

pub fn process(args: Vec<String>) {
    if args.len() < 2 {
        println!("{HELP}");
        return;
    }

    let command = args[1].as_str();

    match command {
        "establish" => cli_establish(),
        "atom" => cli_atom(),
        "create" | "remove" | "edit" | "export" | "search" => {
            if args.len() < 3 {
                println!("{HELP}");
                return;
            }
            let argument = args[2].as_str();
            match command {
                "create" => cli_create(argument),
                "remove" => cli_remove(argument),
                "edit" => cli_edit(argument),
                "export" => cli_export(argument),
                "search" => cli_search(argument),
                _ => println!("{HELP}"),
            }
        }
        _ => println!("{HELP}"),
    }
}

// Create the required directory structure
fn cli_establish() {
    for dir in &["documents", "export"] {
        if fs::read_dir(dir).is_err() {
            println!("Creating {dir}...");
            fs::create_dir(dir).expect("Failed to create {dir}.");
        }
    }
}

// Create a new document
fn cli_create(filename: &str) {
    let folder_path = "documents";
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
    let md_file_path = format!("documents/{filename}.md");
    if let Err(error) = fs::remove_file(&md_file_path) {
        println!("Error removing source document {filename}: {error}.");
    } else {
        println!("Deleted source document '{filename}'.");
    }

    let html_file_path = format!("export/{filename}.html");
    if let Err(error) = fs::remove_file(&html_file_path) {
        println!("Error removing exported document {filename}: {error}.");
    } else {
        println!("Deleted exported document '{filename}'.");
    }
}

// Edit a requested document
fn cli_edit(filename: &str) {
    let file_path = format!("documents/{filename}.md");

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
fn cli_export(document: &str) {
    let md_file_path = format!("documents/{document}.md");
    if fs::metadata(&md_file_path).is_err() {
        println!("Input file '{document}' does not exist. Please create it first.");
        return;
    }

    let conf = match fs::read_to_string("conf.toml") {
        Ok(content) => toml::from_str::<Conf>(&content).unwrap(),
        Err(e) => {
            println!("{e}\nYou must manually create a conf.toml file for your feed.");
            return;
        }
    };

    let content = fs_to_str(&md_file_path);
    let md_object = Object {
        format: Some("md".to_string()),
        content: Some(content),
        ..Default::default()
    };

    let mut toml = conf;
    if let Some(main) = toml.main.as_mut() {
        let position = main
            .block
            .iter()
            .position(|obj| obj.format.as_deref() == Some("document"));

        if let Some(pos) = position {
            main.block[pos] = md_object;
        } else {
            main.block.push(md_object);
        }
    } else {
        toml.main = Some(Main {
            block: vec![md_object],
        });
    }

    if let Err(err) = fs::write(format!("export/{document}.html"), toml.to_html()) {
        eprintln!("Failed to export {document}: {err}.");
        return;
    }

    println!("Successfully exported {document}.");
}

// Search documents
fn cli_search(keyword: &str) {
    let entries = fs::read_dir("documents/")
        .expect("Failed to read documents directory.")
        .filter_map(|entry| {
            entry
                .ok()
                .map(|e| e.file_name().into_string().unwrap_or_default())
        });

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

// TODO: Allow front matter in documents so it can be referenced here:

// Generate an Atom feed
fn cli_atom() {
    let mut entries = Vec::new();

    for entry in fs::read_dir("export/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let content = fs::read_to_string(&path).unwrap_or_default();

        if path.file_name().unwrap() == "feed.xml" {
            continue;
        }

        let entry = EntryBuilder::default()
            .title(Text::plain(
                path.file_name().unwrap().to_string_lossy().to_string(),
            ))
            .content(ContentBuilder::default().value(content).build())
            .build();

        entries.push(entry);
    }

    let conf_content = match fs::read_to_string("conf.toml") {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading configuration file: {e}\nNo configuration file found.");
            return;
        }
    };

    let conf: Result<Conf, TomlError> = toml::from_str(&conf_content);
    let conf = match conf {
        Ok(conf) => conf,
        Err(e) => {
            println!("Error parsing configuration file: {e}");
            return;
        }
    };

    if conf.title.is_none() || conf.id.is_none() {
        let mut missing_fields = Vec::new();

        if conf.title.is_none() {
            missing_fields.push("title");
        }
        if conf.id.is_none() {
            missing_fields.push("id");
        }

        println!(
            "Atom feed not generated. Missing required fields: {}.",
            missing_fields.join(", ")
        );
        return;
    }

    let generator = GeneratorBuilder::default()
        .value("Adduce".to_string())
        .uri("http://adduce.vale.rocks".to_string())
        .version(env!("CARGO_PKG_VERSION").to_string())
        .build();

    let feed = FeedBuilder::default()
        .title(Text::plain(conf.title.unwrap()))
        .id(conf.id.as_ref().unwrap())
        .updated(Utc::now())
        // TODO: Authors
        // TODO: Categories
        .generator(generator)
        .icon(conf.icon)
        // TODO: Links
        .logo(conf.logo)
        .rights(conf.rights.map(Text::plain))
        .entries(entries)
        .subtitle(conf.subtitle.map(Text::plain))
        .base(conf.base)
        .lang(conf.lang)
        .build();

    if let Err(e) = fs::write("export/feed.xml", feed.to_string()) {
        eprintln!("Failed to write Atom feed: {e}");
    } else {
        println!("Atom feed generated successfully.");
    }
}
