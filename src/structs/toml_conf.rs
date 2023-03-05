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
    pub style:  Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<Main>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Vec<String>>,

}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct Social {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Main {
    pub block: Vec<Object>,
}
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

#[derive(Default, Debug, Clone)]
pub struct Div {
    pub element: Vec<Object>,
}
