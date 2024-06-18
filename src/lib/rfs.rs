// Import necessary libraries and modules
use crate::structs::toml_conf::Conf;
use core::fmt;
use std::{io::Read, str::from_utf8};

// Function to read a file from a directory and return its content as a string
#[allow(dead_code)]
pub fn fs_to_str(directory: &str) -> String {
    let file = std::fs::read(directory)
        .unwrap_or_else(|_| panic!("File could not be found!\n{directory}"));

    let file_str = from_utf8(&file).expect("Failed to deserialize! Is this possible?");

    String::from(file_str)
}

// Function to write a string content to a file in a directory
#[allow(dead_code)]
pub fn str_to_fs(directory: &str, content: &str) {
    std::fs::write(directory, content).expect("Failed to write to file.");
}

// Function to recreate a directory (delete if exists and then create)
#[allow(dead_code)]
pub fn dir_remake(directory: &str) {
    match std::fs::read_dir(directory) {
        Err(_) => std::fs::create_dir(directory).expect("Failed to create directory."),
        Ok(_) => {
            std::fs::remove_dir_all(directory).expect("Failed to delete directory.");
            std::fs::create_dir(directory).expect("Failed to create directory.")
        }
    };
}

// Function to copy all files from one directory to another
#[allow(dead_code)]
pub fn copy_dir(input: &str, generated: &str) {
    // If the output directory already exists, it is removed and recreated.
    // If it doesn't exist, it is created.
    dir_remake(generated);

    // Iterate over each file in the input directory
    for x in std::fs::read_dir(input).expect("Failed to read input.") {
        // Initialize a new String to hold the content of the current file
        let mut file_str = String::new();

        // Open the current file and read its content into the String
        std::fs::File::open(x.as_ref().unwrap().path())
            .expect("Failed to open files.")
            .read_to_string(&mut file_str)
            .expect("Failed to read file content.");

        // Write the content of the current file to a new file in the output directory.
        // The new file has the same name as the current file.
        std::fs::write(
            format!("{}/{}", generated, x.unwrap().file_name().to_str().unwrap()),
            file_str,
        )
        .expect("Failed to write to new file.");
    }
}

// Custom error type for importing configs
#[derive(Debug, Clone)]
pub struct ConfError(CError);

// Enum to represent different types of errors
#[derive(Debug, Clone)]
pub enum CError {
    // Error related to file operations
    File(String, String),
    // Error related to TOML parsing
    Toml(String, String),
}

// Implement Display trait for ConfError to provide custom error messages
impl fmt::Display for ConfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self.0.clone() {
            CError::File(error, page) => {
                format!("Invalid file! failed to convert page {page}\n{error}")
            }
            CError::Toml(error, page) => {
                format!("Invalid TOML! failed to convert page {page}\n{error}")
            }
        };

        write!(f, "{str}")
    }
}

// Define a Result type alias for operations that return a Conf or a ConfError
type Result<Conf> = std::result::Result<Conf, ConfError>;

// Function to import a configuration from a file
pub fn import_conf(directory: &str) -> Result<Conf> {
    // Initialize a new String to hold the file content
    let mut content = String::new();

    // Attempt to open the file at the given directory
    let mut file = match std::fs::File::open(directory) {
        Ok(a) => a,
        Err(e) => {
            return Err(ConfError(CError::File(
                e.to_string(),
                String::from(directory),
            )))
        }
    };

    // Attempt to read the file content into the String
    if let Err(e) = file.read_to_string(&mut content) {
        return Err(ConfError(CError::File(
            e.to_string(),
            directory.to_string(),
        )));
    }

    // Attempt to parse the file content as TOML into a Conf struct
    match toml::from_str::<Conf>(&content) {
        Ok(a) => Ok(a),
        Err(e) => Err(ConfError(CError::Toml(
            e.to_string(),
            directory.to_string(),
        ))),
    }
}

// Function to copy a file from one directory to another
#[allow(dead_code)]
pub fn copy_file(filename: &str, input_dir: &str, output_dir: &str) {
    str_to_fs(
        &format!("{output_dir}/{filename}"),
        &fs_to_str(&format!("{input_dir}/{filename}")),
    );
}
