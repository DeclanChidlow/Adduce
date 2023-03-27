use crate::lib::rfs::{import_conf, str_to_fs};

use super::toml_conf::Conf;

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

    pub fn conf_str(mut self, config: &str) -> Self {
        match import_conf(config) {
            Ok(a) => self.config = a,
            Err(e) => {
                println!("{e}")
            }
        }
        self
    }
    #[allow(dead_code)]
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

    pub fn from_conf(genconf: Generate) {
        generate_html(genconf);
    }

    pub fn void(self) -> Self {
        self
    }
}

pub fn generate_html(conf: Generate) {
    // defining output directory
    let output = conf.ouput.clone().unwrap_or_else(|| String::from("output"));

    // create and move html file
    let html_dir = format!(
        "{output}/{}",
        conf.filename.unwrap_or_else(|| String::from("index.html"))
    );

    if std::fs::File::open(&html_dir).is_ok() {
        std::fs::remove_file(&html_dir).unwrap();
    }

    if std::fs::read_dir(output.clone()).is_err() {
        std::fs::create_dir(output).unwrap()
    };

    std::fs::File::create(&html_dir).unwrap();
    str_to_fs(&html_dir, &conf.config.to_html());
}
