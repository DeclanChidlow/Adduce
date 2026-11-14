use serde::{Deserialize, Serialize};

// Define a main content struct
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Main {
    #[serde(default)]
    pub block: Vec<Object>,
}

// General TOML fields
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Object {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// Head content can be either a list of HTML files or a block list
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Head {
    Files(Vec<String>),
    Blocks(Main),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Conf {
    // Atom specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    // Content for the document head. Accepts either a list of HTML files or a [head] table containing [[head.block]] entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Head>,
    // CSS files to inline into a style element in the head
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Vec<String>>,

    // Body content. Document-level tags are normalised into the generated document, so full-document configurations remain supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<Main>,
}
