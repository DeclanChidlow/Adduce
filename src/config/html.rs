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

    // Method to generate HTML from this builder
    pub fn generate(&self) {
        let output = self.output.as_deref().unwrap_or("output");
        let html_filename = self.filename.as_deref().unwrap_or("index.html");
        let html_dir = format!("{output}/{html_filename}");

        // Create the directory path safely (creates parent folders natively)
        if let Err(e) = fs::create_dir_all(output) {
            eprintln!("Error creating directory {output}: {e}");
            return;
        }

        // Writes the file natively (handles creation & over-writing seamlessly)
        str_to_fs(&html_dir, &self.config.to_html());
    }
}
