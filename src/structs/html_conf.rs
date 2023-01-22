use crate::lib::rfs::{dir_remake, import_conf, str_to_fs};

use super::toml_conf::Conf;

#[derive(Default, Debug, Clone)]
pub struct Generate {
    pub config: Conf,
    pub input: Option<String>,
    pub ouput: Option<String>,
    pub method: GenMethod,
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
        self.config = import_conf(config);
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

    #[allow(dead_code)]
    pub fn method(mut self, method: GenMethod) -> Self {
        self.method = method;
        self
    }

    pub fn from_conf(genconf: Generate) {
        generate_html(genconf);
    }
}

pub fn generate_html(conf: Generate) {
    // defining output directory
    let output = conf.ouput.clone().unwrap_or_else(|| String::from("output"));
    //let input = conf.input.clone().unwrap_or_else(|| String::from("config"));

    // recreate output directory
    dir_remake(&output);

    // create and move html file
    let html_dir = format!("{output}/index.html");

    std::fs::File::create(&html_dir).unwrap();
    str_to_fs(&html_dir, &conf.config.to_html());
}
