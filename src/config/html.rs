use super::toml::Conf;
use crate::lib::filesystem::import_conf;
use std::{fs, path::Path};

#[derive(Default, Debug, Clone)]
pub struct Generate {
    pub config: Conf,
    pub output: Option<String>,
    pub filename: Option<String>,
    error: Option<String>,
}

impl Generate {
    pub fn new() -> Self {
        Default::default()
    }

    // Method to set the configuration from a string
    pub fn conf_str(mut self, config: &str) -> Self {
        match import_conf(config) {
            Ok(conf) => {
                self.config = conf;
                self.error = None;
            }
            Err(error) => self.error = Some(format!("Error importing configuration: {error}")),
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
    pub fn generate(&self) -> Result<(), String> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }

        let output = self.output.as_deref().unwrap_or("output");
        let html_filename = self.filename.as_deref().unwrap_or("index.html");
        let html_path = Path::new(output).join(html_filename);

        if let Some(parent) = html_path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(|error| {
                format!("Error creating directory {}: {error}", parent.display())
            })?;
        }

        let html = self.config.to_html()?;
        fs::write(&html_path, html)
            .map_err(|error| format!("Error writing {}: {error}", html_path.display()))
    }
}
