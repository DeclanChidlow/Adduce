use crate::config::toml::{Conf, Main, Object};
use crate::lib::document::{attribute_value, element_inner_html, for_each_element};
use crate::lib::filesystem::fs_to_str;
use atom_syndication::{
    Category, CategoryBuilder, ContentBuilder, Entry, EntryBuilder, FeedBuilder, GeneratorBuilder,
    LinkBuilder, Person, PersonBuilder, Text,
};
use chrono::{DateTime, FixedOffset, Utc};
use html_escape::decode_html_entities;
use iri_string::types::{IriAbsoluteStr, IriReferenceStr};
use std::collections::{HashMap, HashSet};
use std::{env, fs, path::Path, process::Command};

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

pub fn process(args: Vec<String>) -> Result<(), String> {
    if args.len() < 2 || matches!(args[1].as_str(), "--help" | "-h") {
        println!("{HELP}");
        return Ok(());
    }

    let command = args[1].as_str();

    match command {
        "establish" => cli_establish(),
        "atom" => cli_atom(),
        "create" | "remove" | "edit" | "export" | "search" => {
            let Some(argument) = args.get(2) else {
                return Err(format!(
                    "Missing argument for `adduce feed {command}`.\n\n{HELP}"
                ));
            };
            match command {
                "create" => cli_create(argument),
                "remove" => cli_remove(argument),
                "edit" => cli_edit(argument),
                "export" => cli_export(argument),
                "search" => cli_search(argument),
                _ => unreachable!(),
            }
        }
        _ => Err(format!("Unknown feed command: {command}.\n\n{HELP}")),
    }
}

// Create the required directory structure
fn cli_establish() -> Result<(), String> {
    for dir in &["documents", "export"] {
        if !Path::new(dir).exists() {
            println!("Creating {dir}...");
            fs::create_dir(dir).map_err(|error| format!("Failed to create {dir}: {error}"))?;
        }
    }
    Ok(())
}

// Create a new document
fn cli_create(filename: &str) -> Result<(), String> {
    let folder_path = "documents";
    let file_path = format!("{folder_path}/{filename}.md");

    if !Path::new(folder_path).exists() {
        return Err(
            "The documents folder does not exist. Please run `adduce feed establish`.".to_string(),
        );
    }

    if Path::new(&file_path).exists() {
        return Err(format!("Document already exists: {file_path}."));
    }

    let initial_content = format!("# {filename}\n");
    fs::write(&file_path, initial_content)
        .map_err(|error| format!("Failed to create file {file_path}: {error}"))?;

    println!("Created new file: {file_path}.");
    Ok(())
}

// Remove a requested document
fn cli_remove(filename: &str) -> Result<(), String> {
    let md_file_path = format!("documents/{filename}.md");
    if !Path::new(&md_file_path).exists() {
        return Err(format!("Document '{filename}' does not exist."));
    }

    fs::remove_file(&md_file_path)
        .map_err(|error| format!("Error removing source document {filename}: {error}"))?;

    let html_file_path = format!("export/{filename}.html");
    if Path::new(&html_file_path).exists() {
        fs::remove_file(&html_file_path)
            .map_err(|error| format!("Error removing exported document {filename}: {error}"))?;
    }

    println!("Deleted '{filename}'.");
    Ok(())
}

// Edit a requested document
fn cli_edit(filename: &str) -> Result<(), String> {
    let file_path = format!("documents/{filename}.md");

    if !Path::new(&file_path).exists() {
        return Err(format!("No document named '{filename}'."));
    }

    let editor_command = env::var("EDITOR").unwrap_or_else(|_| "notepad".to_string());

    let mut child = Command::new(editor_command)
        .arg(file_path)
        .spawn()
        .map_err(|error| format!("Failed to launch editor: {error}"))?;

    child
        .wait()
        .map_err(|error| format!("Editor exited with an error: {error}"))?;

    Ok(())
}

// Generate an HTML version of the input document
fn cli_export(document: &str) -> Result<(), String> {
    let md_file_path = format!("documents/{document}.md");
    if !Path::new(&md_file_path).exists() {
        return Err(format!(
            "Input file '{document}' does not exist. Please create it first."
        ));
    }

    if !Path::new("export").exists() {
        return Err("Export directory not found. Please run `adduce feed establish`.".to_string());
    }

    let conf_content = fs::read_to_string("conf.toml").map_err(|error| {
        format!("{error}\nYou must manually create a conf.toml file for your feed.")
    })?;
    let conf = toml::from_str::<Conf>(&conf_content)
        .map_err(|error| format!("Error parsing configuration file: {error}"))?;

    let content = fs_to_str(&md_file_path)?;
    let md_object = Object {
        format: Some("md".to_string()),
        content: Some(content),
        ..Default::default()
    };

    let mut toml = conf;
    if let Some(main) = toml.main.as_mut() {
        if let Some(pos) = main
            .block
            .iter()
            .position(|obj| obj.format.as_deref() == Some("document"))
        {
            main.block[pos] = md_object;
        } else {
            main.block.push(md_object);
        }
    } else {
        toml.main = Some(Main {
            block: vec![md_object],
        });
    }

    let html = toml.to_html()?;
    fs::write(format!("export/{document}.html"), html)
        .map_err(|error| format!("Failed to export {document}: {error}"))?;

    println!("Successfully exported {document}.");
    Ok(())
}

// Search documents
fn cli_search(keyword: &str) -> Result<(), String> {
    let entries = fs::read_dir("documents/")
        .map_err(|error| format!("Failed to read documents directory: {error}"))?;

    let entries = entries.filter_map(|entry| {
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

    Ok(())
}

// Generate an Atom feed
fn cli_atom() -> Result<(), String> {
    let conf_content = fs::read_to_string("conf.toml").map_err(|error| {
        format!("Error reading configuration file: {error}\nNo configuration file found.")
    })?;

    let mut conf: Conf = toml::from_str(&conf_content)
        .map_err(|error| format!("Error parsing configuration file: {error}"))?;

    let mut missing_fields = Vec::new();
    if conf.title.is_none() {
        missing_fields.push("title");
    }
    if conf.id.is_none() {
        missing_fields.push("id");
    }
    if !missing_fields.is_empty() {
        return Err(format!(
            "Atom feed not generated. Missing required fields: {}.",
            missing_fields.join(", ")
        ));
    }

    let base = conf
        .base
        .as_deref()
        .filter(|base| IriAbsoluteStr::new(base).is_ok())
        .or(conf.id.as_deref());

    let export_dir = fs::read_dir("export/").map_err(|error| {
        format!("Export directory not found: {error}. Please establish and export documents first.")
    })?;

    let mut entries = Vec::new();

    for entry in export_dir.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let is_html = path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| {
                extension.eq_ignore_ascii_case("html") || extension.eq_ignore_ascii_case("htm")
            });
        if !is_html {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            eprintln!("Skipping non-UTF-8 file name: {}", path.display());
            continue;
        };

        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(error) => {
                eprintln!("Skipping {}: {error}", path.display());
                continue;
            }
        };

        let mut metadata = parse_page_metadata(&content, base, file_name);
        metadata.updated = metadata
            .updated
            .or(metadata.published)
            .or_else(|| file_modified(&path));
        entries.push(build_entry(file_name, metadata, base));
    }

    // Atom feeds are expected to list the most recent entries first.
    entries.sort_by(|a, b| {
        b.updated()
            .cmp(a.updated())
            .then_with(|| a.title().as_str().cmp(b.title().as_str()))
    });

    // The feed's updated timestamp should reflect the newest entry, not the time the feed happened to be generated.
    let updated = entries
        .iter()
        .map(|entry| *entry.updated())
        .max()
        .unwrap_or_else(|| Utc::now().fixed_offset());

    let generator = GeneratorBuilder::default()
        .value("Adduce".to_string())
        .uri("http://adduce.vale.rocks".to_string())
        .version(env!("CARGO_PKG_VERSION").to_string())
        .build();

    let title = conf.title.take().expect("title checked above");
    let id = conf.id.take().expect("id checked above");

    let feed = FeedBuilder::default()
        .title(Text::plain(title))
        .id(id)
        .updated(updated)
        .generator(generator)
        .icon(conf.icon)
        .logo(conf.logo)
        .rights(conf.rights.map(Text::plain))
        .entries(entries)
        .subtitle(conf.subtitle.map(Text::plain))
        .base(conf.base)
        .lang(conf.lang)
        .build();

    fs::write("export/feed.xml", feed.to_string())
        .map_err(|error| format!("Failed to write Atom feed: {error}"))?;

    println!("Atom feed generated successfully.");
    Ok(())
}

#[derive(Debug, Default)]
struct PageMetadata {
    title: Option<String>,
    summary: Option<String>,
    url: Option<String>,
    published: Option<DateTime<FixedOffset>>,
    updated: Option<DateTime<FixedOffset>>,
    authors: Vec<PageAuthor>,
    categories: Vec<String>,
    content: Option<String>,
}

#[derive(Debug)]
struct PageAuthor {
    name: String,
    uri: Option<String>,
}

fn parse_page_metadata(html: &str, base: Option<&str>, file_name: &str) -> PageMetadata {
    let mut properties: HashMap<String, Vec<String>> = HashMap::new();
    let mut names: HashMap<String, Vec<String>> = HashMap::new();
    let mut canonical = None;

    for_each_element(html, |element, attrs| {
        if element.eq_ignore_ascii_case("meta") {
            let Some(content) = decoded_attribute(attrs, "content") else {
                return;
            };
            let content = content.trim();
            if content.is_empty() {
                return;
            }

            if let Some(property) = decoded_attribute(attrs, "property") {
                properties
                    .entry(property.trim().to_ascii_lowercase())
                    .or_default()
                    .push(content.to_string());
            }

            if let Some(name) = decoded_attribute(attrs, "name") {
                names
                    .entry(name.trim().to_ascii_lowercase())
                    .or_default()
                    .push(content.to_string());
            }
        } else if element.eq_ignore_ascii_case("link") && canonical.is_none() {
            let Some(rel) = decoded_attribute(attrs, "rel") else {
                return;
            };
            if !rel
                .split_ascii_whitespace()
                .any(|token| token.eq_ignore_ascii_case("canonical"))
            {
                return;
            }

            if let Some(href) = decoded_attribute(attrs, "href") {
                let href = href.trim();
                if !href.is_empty() {
                    canonical = Some(href.to_string());
                }
            }
        }
    });

    let title = first_meta(&properties, "og:title")
        .or_else(|| document_title(html))
        .map(|title| clean_text(&title))
        .filter(|title| !title.is_empty());

    let summary = first_meta(&properties, "og:description")
        .or_else(|| first_meta(&names, "description"))
        .map(|summary| clean_text(&summary))
        .filter(|summary| !summary.is_empty());

    // A canonical URL is authoritative for both the entry ID and its link.
    // Only fall back to og:url when no canonical link is present.
    let url = canonical
        .or_else(|| first_meta(&properties, "og:url"))
        .and_then(|url| resolve_url(&url, base));

    let published = meta_date(&properties, "article:published_time", file_name);
    let updated = meta_date(&properties, "article:modified_time", file_name);

    let authors = page_authors(&properties, &names);
    let categories = page_categories(&properties);
    let content = page_content(html);

    PageMetadata {
        title,
        summary,
        url,
        published,
        updated,
        authors,
        categories,
        content,
    }
}

fn first_meta(metadata: &HashMap<String, Vec<String>>, key: &str) -> Option<String> {
    metadata.get(key)?.first().cloned()
}

fn decoded_attribute(attrs: &str, name: &str) -> Option<String> {
    attribute_value(attrs, name).map(|value| decode_html_entities(value).into_owned())
}

fn document_title(html: &str) -> Option<String> {
    element_inner_html(html, "title").map(|title| decode_html_entities(&title).into_owned())
}

fn resolve_url(value: &str, base: Option<&str>) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }

    if let Some(base) = base {
        if let Ok(base) = IriAbsoluteStr::new(base.trim()) {
            if let Ok(reference) = IriReferenceStr::new(value) {
                return Some(reference.resolve_against(base).to_string());
            }
        }
    }

    IriAbsoluteStr::new(value).ok().map(|url| url.to_string())
}

fn meta_date(
    metadata: &HashMap<String, Vec<String>>,
    key: &str,
    file_name: &str,
) -> Option<DateTime<FixedOffset>> {
    for value in metadata.get(key).into_iter().flatten() {
        match DateTime::parse_from_rfc3339(value.trim()) {
            Ok(date) => return Some(date),
            Err(_) => eprintln!("Ignoring invalid {key} in {file_name}: {value}"),
        }
    }
    None
}

fn page_authors(
    properties: &HashMap<String, Vec<String>>,
    names: &HashMap<String, Vec<String>>,
) -> Vec<PageAuthor> {
    let mut authors = Vec::new();
    let mut seen = HashSet::new();

    for name in names.get("author").into_iter().flatten() {
        let name = clean_text(name);
        if !name.is_empty() && seen.insert(name.to_ascii_lowercase()) {
            authors.push(PageAuthor { name, uri: None });
        }
    }

    if !authors.is_empty() {
        return authors;
    }

    for value in properties.get("article:author").into_iter().flatten() {
        let value = value.trim();
        if value.is_empty() {
            continue;
        }

        let author = if let Ok(url) = IriAbsoluteStr::new(value) {
            let path_name = url
                .path_str()
                .trim_end_matches('/')
                .rsplit('/')
                .next()
                .filter(|segment| !segment.is_empty());
            let host = url
                .authority_str()
                .and_then(|authority| authority.rsplit('@').next())
                .map(|host| host.split(':').next().unwrap_or(host))
                .filter(|host| !host.is_empty());
            let name = path_name.or(host).unwrap_or(value).to_string();
            PageAuthor {
                name,
                uri: Some(url.to_string()),
            }
        } else {
            PageAuthor {
                name: clean_text(value),
                uri: None,
            }
        };

        if !author.name.is_empty() && seen.insert(author.name.to_ascii_lowercase()) {
            authors.push(author);
        }
    }

    authors
}

fn page_categories(properties: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut categories = Vec::new();
    let mut seen = HashSet::new();

    for value in properties
        .get("article:section")
        .into_iter()
        .chain(properties.get("article:tag"))
        .flatten()
    {
        let category = clean_text(value);
        if !category.is_empty() && seen.insert(category.to_ascii_lowercase()) {
            categories.push(category);
        }
    }

    categories
}

fn page_content(html: &str) -> Option<String> {
    for element in ["article", "main", "body"] {
        if let Some(content) = element_inner_html(html, element) {
            if !content.trim().is_empty() {
                return Some(content);
            }
        }
    }

    None
}

fn clean_text(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn file_modified(path: &Path) -> Option<DateTime<FixedOffset>> {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .map(|modified| DateTime::<Utc>::from(modified).fixed_offset())
}

fn build_entry(file_name: &str, metadata: PageMetadata, base: Option<&str>) -> Entry {
    let PageMetadata {
        title,
        summary,
        url,
        published,
        updated,
        authors,
        categories,
        content,
    } = metadata;

    let fallback_url = resolve_url(file_name, base);
    let entry_url = url.or(fallback_url);
    let id = entry_url
        .clone()
        .unwrap_or_else(|| format!("urn:adduce:{file_name}"));
    let title = title.unwrap_or_else(|| file_name.to_string());
    let updated = updated
        .or(published)
        .unwrap_or_else(|| Utc::now().fixed_offset());

    let mut builder = EntryBuilder::default();
    builder
        .title(Text::plain(title.clone()))
        .id(id)
        .updated(updated);

    if let Some(published) = published {
        builder.published(published);
    }

    if let Some(summary) = summary {
        builder.summary(Text::plain(summary));
    }

    if let Some(url) = entry_url {
        builder.links(vec![
            LinkBuilder::default()
                .href(url)
                .rel("alternate")
                .mime_type(Some("text/html".to_string()))
                .title(title)
                .build(),
        ]);
    }

    if !authors.is_empty() {
        let authors: Vec<Person> = authors
            .into_iter()
            .map(|author| {
                let mut builder = PersonBuilder::default();
                builder.name(author.name);
                if let Some(uri) = author.uri {
                    builder.uri(uri);
                }
                builder.build()
            })
            .collect();
        builder.authors(authors);
    }

    if !categories.is_empty() {
        let categories: Vec<Category> = categories
            .into_iter()
            .map(|category| CategoryBuilder::default().term(category).build())
            .collect();
        builder.categories(categories);
    }

    if let Some(content) = content {
        builder.content(
            ContentBuilder::default()
                .content_type(Some("html".to_string()))
                .value(content)
                .build(),
        );
    }

    builder.build()
}
