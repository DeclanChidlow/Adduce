use serde::{Deserialize, Serialize};

// Define a configuration struct with optional fields
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Conf {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social: Option<Social>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<Main>,
}

// Define a social struct with optional fields
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Social {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
}

// Define a main content struct
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Main {
    pub block: Vec<Object>,
}

// Define an object struct with optional fields
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
