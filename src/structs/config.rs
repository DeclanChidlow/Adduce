use serde::{Deserialize, Serialize};

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
    pub defaults: Option<Defaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct Social {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct Defaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<Scope>>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct Scope {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Values>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct Values {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanlink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
}
