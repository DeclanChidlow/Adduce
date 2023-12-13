// Import necessary modules and functions
use super::toml_conf::Conf;
use crate::lib::rfs::{import_conf, str_to_fs};

// Define a struct for generating HTML
#[derive(Default, Debug, Clone)]
pub struct Generate {
    pub config: Conf,
    pub input: Option<String>,
    pub ouput: Option<String>,
    pub filename: Option<String>,
}

// Define an enum for the generation method
#[derive(Default, Debug, Clone)]
pub enum GenMethod {
    #[default]
    Default,
}

// Implement methods for the Generate struct
impl Generate {
    // Method to create a new Generate object with default values
    pub fn new() -> Self {
        Default::default()
    }

    // Method to set the configuration from a string
    pub fn conf_str(mut self, config: &str) -> Self {
        match import_conf(config) {
            Ok(a) => self.config = a,
            Err(e) => {
                println!("{e}")
            }
        }
        self
    }

    // Method to set the input directory
    #[allow(dead_code)]
    pub fn input_dir(mut self, directory: &str) -> Self {
        self.input = Some(String::from(directory));
        self
    }

    // Method to set the output directory
    pub fn output_dir(mut self, directory: &str) -> Self {
        self.ouput = Some(String::from(directory));
        self
    }

    // Method to set the filename
    pub fn filename(mut self, name: &str) -> Self {
        self.filename = Some(String::from(name));
        self
    }

    // Method to generate HTML from a Generate object
    pub fn from_conf(genconf: Generate) {
        generate_html(genconf);
    }

    // Method to return the Generate object without any changes
    pub fn void(self) -> Self {
        self
    }
}

// Function to generate HTML from a Generate object
pub fn generate_html(conf: Generate) {
    // Define the output directory
    let output = conf.ouput.clone().unwrap_or_else(|| String::from("output"));

    // Create the path for the HTML file
    let html_dir = format!(
        "{output}/{}",
        conf.filename.unwrap_or_else(|| String::from("index.html"))
    );

    // If the HTML file already exists, remove it
    if std::fs::File::open(&html_dir).is_ok() {
        std::fs::remove_file(&html_dir).unwrap();
    }

    // If the output directory does not exist, create it
    if std::fs::read_dir(output.clone()).is_err() {
        std::fs::create_dir(output).unwrap()
    };

    // Create the HTML file
    std::fs::File::create(&html_dir).unwrap();
    // Write the HTML to the file
    str_to_fs(&html_dir, &conf.config.to_html());
}
