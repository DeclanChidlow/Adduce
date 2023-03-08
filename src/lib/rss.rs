use std::process::Command;

use crate::structs::toml_conf::{Conf, Main, Object};

const HELP: &str = "Adduce Feed
--new | created new article
--edit | modify an existing article
--publish | release an article to RSS, Twitter, or other platform";

pub fn process(args: Vec<String>) {
    feed_dir();

    // turned into a vec of strs
    let mut newstr = String::new();
    for x in args {
        newstr += &format!(" {x}");
    }

    let args: Vec<&str> = newstr.split(' ').collect();

    match (args.clone().as_slice(), args.len()) {
        // exceptions
        (_, 2) | (_, 3) => println!("{HELP}"),

        // CLI incomplete
        ([.., "new"], _) => println!("New requires article name"),
        ([.., "edit"], _) => println!("Edit requires article name"),
        ([.., "publish"], len) => cli_pub(args, len),
        //  ([.., "conf", "generate"], _) => println!("conf creator requires a file name"),

        // CLI
        ([.., "new", a], _) => cli_new(a),
        ([.., "edit", a], _) => cli_edit(a),
        ([.., "publish", _, _], len) => cli_pub(args, len),
        ([.., "conf", "generate"], _) => conf_make(),

        // un accounted for
        _ => panic!("no"),
    };
}

const GEN_HELP: &str = "Adduce Feed Generate
generate <name> <platform>
run generate help for more info";

const GEN_REF: &str = "Adduce Feed Reference
NAME
Name refers to the article article name, each article has a unique name
PLATFORM
platform is the required target for the article, these include formats such as MD, HTML, TXT.
As well as media platforms like RSS, Twitter, Mastodon, Github, Instagram";

fn cli_pub(args: Vec<&str>, len: usize) {
    // adduce feed generate article-name platform
    if len < 5 {
        println!("{GEN_HELP}");
        return;
    } else if args.contains(&"refrence") {
        println!("{GEN_REF}");
        return;
    };

    if args.len() < 6 {
        println!("invalid args");
        return;
    };

    // supported schemas
    if !match args[5] {
        // type
        "md" | "MD" | "markdown" => true,
        "html" | "HTML" => true,
        "txt" | "text" | "plaintext" => false,
        // social media

        // blogging
        "rss" | "RSS" => false,

        // social media
        "insta" | "Insta" | "Instagram" | "instagram" => false,

        _ => false,
    } {
        println!("sorry this schema is not yet supported");
        return;
    };

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
        from_str: Some(format!("feed/documents/{}.md", args[4])),
        style: Some(String::from("md")),
        ..Default::default()
    };

    if toml.main.is_none() {
        toml.main = Some(Main { block: vec![text] });
    } else {
        toml.main.as_mut().unwrap().block.push(text);
    };

    let html = toml.to_html();

    std::fs::write(format!("feed/export/{}.html", args[4]), html).unwrap();
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

    Command::new("vim")
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

    Command::new("vim")
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
