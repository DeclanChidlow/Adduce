use std::process::Command;

use crate::structs::toml_conf::{Conf, Main, Object};

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

fn cli_remove(a: &str) {
    if let Err(error) = std::fs::remove_file(format!("feed/documents/{a}.md")) {
        println!("error removing document\n{error}");
    };
}

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

fn cli_pub(document: &str) {
    // adduce feed generate article-name platform

    let conf = match std::fs::read("feed/conf.toml") {
        Ok(a) => String::from_utf8(a).unwrap(),
        Err(e) => {
            println!("{e}\nuse feed conf generate to make a conf file");
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
}

fn conf_make() {
    if std::fs::read("feed/conf.toml").is_ok() {
        println!("this file already exists press enter if you wish to continue");

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

fn conf_wizard() -> Conf {
    let mut conf = Conf::new();

    println!("Author's name? enter for None");
    let mut author = String::new();
    std::io::stdin().read_line(&mut author).unwrap();
    let author = author.trim();

    if !author.is_empty() {
        conf.author = Some(String::from(author));
    };

    // STYLESHEETS

    let mut style = Vec::new();

    let mut iter = 0;

    println!("stylesheet? pick one\n'enter' to continue");
    for x in std::fs::read_dir("feed/styles").unwrap() {
        println!("{}", x.unwrap().file_name().to_string_lossy());
        iter += 1;
    }

    if iter == 0 {
        let yeslist = vec!["y", "Y", "yes", "Yes", "YES"];

        println!("no stylesheets found! would you like to download one from us?");
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

            println!("stylesheet? pick one\n'next' to continue");

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

fn cli_edit(a: &str) {
    let path = format!("feed/documents//{a}.md");

    if std::fs::read(&path).is_err() {
        println!("No article with this name");
        return;
    }

    Command::new("nvim")
        .arg(path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn cli_new(a: &str) {
    let path = format!("feed/documents/{a}.md");

    if std::fs::read(&path).is_ok() {
        println!("Article already exists with this name");
        return;
    }

    Command::new("nvim")
        .arg(path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn feed_dir() {
    for x in &[
        "feed",
        "feed/documents",
        "feed/export",
        "feed/styles",
        "feed/content",
    ] {
        if std::fs::read_dir(x).is_err() {
            println!("could not find directory {x}, creating...");

            if let Err(error) = std::fs::create_dir(x) {
                println!("could not create directory\n{error}");
            }
        };
    }
}
