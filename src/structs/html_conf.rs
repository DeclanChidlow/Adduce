use crate::lib::rfs::{dir_remake, import_conf, str_to_fs};

use super::toml_conf::Conf;

#[derive(Default, Debug)]
pub struct Generate {
    pub config: Conf,
    pub input: Option<String>,
    pub ouput: Option<String>,
    pub method: GenMethod,
}

#[derive(Default, Debug)]
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
    pub fn input_dir(mut self, directory: &str) -> Self {
        self.input = Some(String::from(directory));
        self
    }
    pub fn output_dir(mut self, directory: &str) -> Self {
        self.ouput = Some(String::from(directory));
        self
    }

    pub fn method(mut self, method: GenMethod) -> Self {
        self.method = method;
        self
    }

    pub fn from_conf(genconf: Generate) {
        generate_html(genconf);
    }


}

pub fn generate_html(conf: Generate) {
    //let input = conf.input.unwrap_or(String::from("config"));
    let output = conf.ouput.unwrap_or(String::from("output"));

    dir_remake(&output);
    let html = conf.config.to_html();

    let html_dir = format!("{output}/index.html");

    std::fs::File::create(&html_dir).unwrap();
    str_to_fs(&html_dir, &html);
}
