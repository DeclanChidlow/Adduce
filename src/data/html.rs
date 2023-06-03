use crate::common::{fs::File, result::Error};

use super::toml::Conf;

#[derive(Default, Debug, Clone)]
pub struct Generate {
    pub config: Conf,
    pub input: Option<String>,
    pub ouput: Option<String>,
    pub filename: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub enum GenMethod {
    #[default]
    Default,
}

impl Generate {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn conf_str(&mut self, config: &str) -> Result<Self, Error> {
        let file: Conf = File::from_path(config)?.toml_from_path()?;
        self.config = file;
        Ok(self.to_owned())
    }

    pub fn input_dir(mut self, directory: &str) -> Self {
        self.input = Some(String::from(directory));
        self
    }
    pub fn output_dir(mut self, directory: &str) -> Self {
        self.ouput = Some(String::from(directory));
        self
    }

    pub fn filename(mut self, name: &str) -> Self {
        self.filename = Some(String::from(name));
        self
    }

    pub fn from_conf(genconf: Generate) -> Result<(), Error> {
        generate_html(genconf)
    }

    pub fn void(self) -> Self {
        self
    }
}

pub fn generate_html(conf: Generate) -> Result<(), Error> {
    // defining output directory
    let output = conf.ouput.unwrap_or_else(|| String::from("output"));

    // create and move html file
    let html_file_path = format!(
        "{output}/{}",
        conf.filename.unwrap_or_else(|| String::from("index.html"))
    );

    // if html directory doesnt exist; create
    let _ = File::new().set_path(&output).write_directory();

    // if html file exists, remove it
    if let Ok(file) = File::from_path(&html_file_path) {
        file.delete()?;
    };

    // write content
    File::new()
        .set_path(&html_file_path)
        .set_content(&conf.config.to_html()?)
        .write()?;
    Ok(())
}
