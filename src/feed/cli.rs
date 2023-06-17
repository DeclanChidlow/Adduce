use crate::{
    common::{fs::File, result::Error},
    data::toml::{Conf, Main, Object},
};

use super::apps::Com;

const HELP: &str = "Adduce Feed - create blogs or other simple documents.

Usage: adduce feed [COMMAND] <argument>

Commands:
    new <file_name> 	create new article
    edit <file_name>	modify an existing article
    publish <file_name>	build the file with Adduce";

pub fn process(mut original_args: Vec<String>) -> Result<(), Error> {
    original_args.remove(0);
    original_args.remove(0);

    let original_args = original_args.to_owned();
    let args: Vec<&str> = original_args.iter().map(|s| s.as_str()).collect();

    match args.as_slice() {
        // no commands given
        a if a.is_empty() => {
            println!("{HELP}");
        }
        // nvim editor
        ["new", docname, ..] | ["edit", docname, ..] => {
            Com::spawn(
                "nvim",
                Some(vec![&format!("./feed/documents/{docname}.md")]),
            )?;
        }

        ["publish", docname, ..] => {
            let mut config = File::from_path("./feed/conf.toml")?.toml_from_str::<Conf>()?;

            // adding content to toml
            let text = Object {
                from_str: Some(format!("./feed/documents/{docname}.md")),
                style: Some(String::from("md")),
                ..Default::default()
            };

            if let Some(mut content) = config.main.clone() {
                content.block.push(text)
            } else {
                config.main = Some(Main { block: vec![text] });
            };

            File::new()
                .set_path(&format!("./feed/export/{docname}.html"))
                .set_content(&config.to_html()?)
                .write()?;
        }

        _ => println!("{HELP}"),
    }
    Ok(())
}
