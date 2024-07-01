use super::toml::Conf;
use crate::lib::filesystem::{import_conf, str_to_fs};
use std::fs;

#[derive(Default, Debug, Clone)]
pub struct Generate {
    pub config: Conf,
    pub output: Option<String>,
    pub filename: Option<String>,
}

impl Generate {
    pub fn new() -> Self {
        Default::default()
    }

    // Method to set the configuration from a string
    pub fn conf_str(mut self, config: &str) -> Self {
        match import_conf(config) {
            Ok(conf) => self.config = conf,
            Err(e) => eprintln!("Error importing configuration: {e}"),
        }
        self
    }

    // Method to set the output directory
    pub fn output_dir(mut self, directory: &str) -> Self {
        self.output = Some(directory.to_string());
        self
    }

    // Method to set the filename
    pub fn filename(mut self, name: &str) -> Self {
        self.filename = Some(name.to_string());
        self
    }

    // Method to generate HTML from a Generate object
    pub fn from_conf(genconf: Generate) {
        generate_html(genconf);
    }
}

pub fn generate_html(conf: Generate) {
    let output = conf.output.clone().unwrap_or_else(|| "output".to_string());
    let html_filename = conf.filename.clone().unwrap_or_else(|| "index.html".to_string());
    let html_dir = format!("{output}/{html_filename}");

    // If the HTML file already exists, remove it
    if fs::File::open(&html_dir).is_ok() {
        if let Err(e) = fs::remove_file(&html_dir) {
            eprintln!("Error removing existing file {html_dir}: {e}");
            return;
        }
    }

    // If the output directory does not exist, create it
    if fs::read_dir(&output).is_err() {
        if let Err(e) = fs::create_dir(&output) {
            eprintln!("Error creating directory {output}: {e}");
            return;
        }
    }

    // Write the HTML to the file
    if let Err(e) = fs::File::create(&html_dir) {
        eprintln!("Error creating file {html_dir}: {e}");
        return;
    }

    str_to_fs(&html_dir, &conf.config.to_html());
}
