use crate::config::toml::Conf;
use core::fmt;
use std::fs;

// Function to read a file from a directory and return its content as a string
pub fn fs_to_str(directory: &str) -> String {
    fs::read_to_string(directory).unwrap_or_else(|_| panic!("File could not be found: {directory}"))
}

// Function to write a string content to a file in a directory
pub fn str_to_fs(directory: &str, content: &str) {
    fs::write(directory, content).expect("Failed to write to file.");
}

// Custom error type for importing configs
#[derive(Debug, Clone)]
pub struct ConfError(CError);

// Enum to represent different types of errors
#[derive(Debug, Clone)]
pub enum CError {
    File(String, String),
    Toml(String, String),
}

// Implement Display trait for ConfError to provide custom error messages
impl fmt::Display for ConfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match &self.0 {
            CError::File(error, page) => format!("File error on page {page}: {error}"),
            CError::Toml(error, page) => format!("TOML error on page {page}: {error}"),
        };
        write!(f, "{msg}")
    }
}

// Define a Result type alias for operations that return a Conf or a ConfError
type Result<T> = std::result::Result<T, ConfError>;

// Function to import a configuration from a file
pub fn import_conf(directory: &str) -> Result<Conf> {
    let content = fs::read_to_string(directory)
        .map_err(|e| ConfError(CError::File(e.to_string(), directory.to_string())))?;

    toml::from_str::<Conf>(&content)
        .map_err(|e| ConfError(CError::Toml(e.to_string(), directory.to_string())))
}
