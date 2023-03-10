use core::fmt;
use std::{io::Read, str::from_utf8};

use crate::structs::toml_conf::Conf;

// given a directory return the content
#[allow(dead_code)]
pub fn fs_to_str(directory: &str) -> String {
    let file = std::fs::read(directory)
        .unwrap_or_else(|_| panic!("file could not be found!\n{directory}"));

    let file_str = from_utf8(&file).expect("failed to deserilise! is this possible?");

    String::from(file_str)
}
#[allow(dead_code)]
pub fn str_to_fs(directory: &str, content: &str) {
    std::fs::write(directory, content).expect("failed to write to file");
}

#[allow(dead_code)]
pub fn dir_remake(directory: &str) {
    match std::fs::read_dir(directory) {
        Err(_) => std::fs::create_dir(directory).expect("failed to create directory"),
        Ok(_) => {
            std::fs::remove_dir_all(directory).expect("failed to delete directory");
            std::fs::create_dir(directory).expect("failed to create directory")
        }
    };
}

#[allow(dead_code)]
pub fn copy_dir(input: &str, generated: &str) {
    // if directory exists, remove and remake it, otherwise just make the dir
    dir_remake(generated);

    // for every file in the input directory
    for x in std::fs::read_dir(input).expect("failed to read input") {
        // create a new string, and let the content = the current file's content
        let mut file_str = String::new();

        std::fs::File::open(x.as_ref().unwrap().path())
            .expect("failed to open files")
            .read_to_string(&mut file_str)
            .expect("Error while reading file");

        // write to a new file in the generated directoy with the same filename and content as the input file
        std::fs::write(
            format!("{}/{}", generated, x.unwrap().file_name().to_str().unwrap()),
            file_str,
        )
        .expect("failed to write new file");
    }
}

// custom error type for importing configs
#[derive(Debug, Clone)]
pub struct ConfError(CError);

// each error contains the error type, custom error from that type
// as well as the name of the page that failed to convert
#[derive(Debug, Clone)]
pub enum CError {
    // the file could not be found read or other fs issue
    File(String, String),
    // the toml is invalid
    Toml(String, String),
}

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

type Result<Conf> = std::result::Result<Conf, ConfError>;

pub fn import_conf(directory: &str) -> Result<Conf> {
    // buffer io
    let mut content = String::new();

    // import file
    let mut file = match std::fs::File::open(directory) {
        Ok(a) => a,
        Err(e) => {
            return Err(ConfError(CError::File(
                e.to_string(),
                String::from(directory),
            )))
        }
    };

    // import write to string
    if let Err(e) = file.read_to_string(&mut content) {
        return Err(ConfError(CError::File(
            e.to_string(),
            directory.to_string(),
        )));
    }

    // check for toml errors
    match toml::from_str::<Conf>(&content) {
        Ok(a) => Ok(a),
        Err(e) => Err(ConfError(CError::Toml(
            e.to_string(),
            directory.to_string(),
        ))),
    }
}

#[allow(dead_code)]
pub fn copy_file(filename: &str, input_dir: &str, output_dir: &str) {
    str_to_fs(
        &format!("{output_dir}/{filename}"),
        &fs_to_str(&format!("{input_dir}/{filename}")),
    );
}
