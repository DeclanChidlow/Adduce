// Import necessary traits from the serde crate
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

// Implement methods for the Conf struct
impl Conf {
    pub fn new() -> Self {
        Default::default()
    }
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
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_str: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

// Define a div struct
#[derive(Default, Debug, Clone)]
pub struct Div {
    // Vector of Objects for the div
    pub element: Vec<Object>,
}
