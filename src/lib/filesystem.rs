use crate::config::toml::Conf;
use core::fmt;
use std::{fs, io::Read, str::from_utf8};

// Function to read a file from a directory and return its content as a string
#[allow(dead_code)]
pub fn fs_to_str(directory: &str) -> String {
    let file = fs::read(directory).unwrap_or_else(|_| panic!("File could not be found: {directory}"));
    from_utf8(&file).expect("Failed to deserialize file content").to_string()
}

// Function to write a string content to a file in a directory
#[allow(dead_code)]
pub fn str_to_fs(directory: &str, content: &str) {
    fs::write(directory, content).expect("Failed to write to file.");
}

// Function to recreate a directory (delete if exists and then create)
#[allow(dead_code)]
pub fn dir_remake(directory: &str) {
    if fs::read_dir(directory).is_ok() {
        fs::remove_dir_all(directory).expect("Failed to delete directory.");
    }
    fs::create_dir(directory).expect("Failed to create directory.");
}

// Function to copy all files from one directory to another
#[allow(dead_code)]
pub fn copy_dir(input: &str, generated: &str) {
    dir_remake(generated);

    for entry in fs::read_dir(input).expect("Failed to read input directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.is_file() {
            let mut file_str = String::new();
            fs::File::open(&path)
                .expect("Failed to open file")
                .read_to_string(&mut file_str)
                .expect("Failed to read file content");

            let output_path = format!("{}/{}", generated, entry.file_name().to_str().unwrap());
            fs::write(&output_path, file_str).expect("Failed to write to new file");
        }
    }
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
    let mut content = String::new();
    let mut file = fs::File::open(directory).map_err(|e| ConfError(CError::File(e.to_string(), directory.to_string())))?;
    file.read_to_string(&mut content).map_err(|e| ConfError(CError::File(e.to_string(), directory.to_string())))?;
    toml::from_str::<Conf>(&content).map_err(|e| ConfError(CError::Toml(e.to_string(), directory.to_string())))
}

// Function to copy a file from one directory to another
#[allow(dead_code)]
pub fn copy_file(filename: &str, input_dir: &str, output_dir: &str) {
    let content = fs_to_str(&format!("{input_dir}/{filename}"));
    str_to_fs(&format!("{output_dir}/{filename}"), &content);
}
