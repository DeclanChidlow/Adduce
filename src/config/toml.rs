use serde::{Deserialize, Serialize};

// Define a main content struct
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Main {
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

// RSS specific
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Conf {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managing_editor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webmaster: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_hours: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_days: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<Main>,
}
