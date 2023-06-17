use std::fs;

use super::result::{Error, ErrorConvert};

#[derive(Debug, Default, Clone)]
pub struct File {
    path: String,
    content: String,
}

// todo crud
impl File {
    pub fn from_path(path: &str) -> Result<Self, Error> {
        let path = String::from(path);
        let content = String::from_utf8(fs::read(&path).res_msg(Some(&path))?).res()?;

        Ok(Self { path, content })
    }

    pub fn write_directory(&self) -> Result<(), Error> {
        fs::create_dir(&self.path).res_msg(Some(&self.path))
    }
    pub fn delete_directory(&self) -> Result<(), Error> {
        fs::remove_dir(&self.path).res_msg(Some(&self.path))
    }

    pub fn set_path(&mut self, path: &str) -> Self {
        self.path = String::from(path);
        self.to_owned()
    }
    pub fn set_content(&mut self, content: &str) -> Self {
        self.content = String::from(content);
        self.to_owned()
    }

    pub fn read(&self) -> Result<Self, Error> {
        Self::from_path(&self.path)
    }

    pub fn to_path(&mut self, new_path: Option<&str>) -> Result<(), Error> {
        if let Some(new_path) = new_path {
            self.path = String::from(new_path);
        };
        self.write()
    }

    pub fn write(&self) -> Result<(), Error> {
        fs::write(&self.path, &self.content).res_msg(Some(&self.path))
    }
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_content(&self) -> String {
        self.content.to_owned()
    }
    pub fn get_path(&self) -> String {
        self.path.to_owned()
    }
    pub fn delete(self) -> Result<(), Error> {
        std::fs::remove_file(&self.path).res_msg(Some(&self.path))
    }

    pub fn toml_from_str<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        &self,
    ) -> Result<T, Error> {
        toml::from_str(&self.content).res_msg(Some(&self.content))
    }
    pub fn toml_from_path<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        &self,
    ) -> Result<T, Error> {
        self.read()?;
        self.toml_from_str()
    }
}

pub fn args() -> Vec<String> {
    std::env::args().collect()
}
