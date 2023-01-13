use serde::{Deserialize, Serialize};

use crate::lib::rfs::fs_to_str;

use super::config::Object;
#[derive(Default, Debug, Clone)]
pub struct Div {
    pub element: Vec<Object>,
}

impl Div {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn element(&mut self, element: Object) -> Self {
        self.element.push(element);
        self.to_owned()
    }

    #[allow(dead_code)]
    pub fn compile(&self) -> String {
        div_compiler(self)
    }
}
impl Object {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style(&mut self, style: Style) -> Self {
        self.style = Some(style);
        self.to_owned()
    }
    pub fn text(&mut self, text: &str) -> Self {
        self.text = Some(String::from(text));
        self.to_owned()
    }
    pub fn id(&mut self, id: &str) -> Self {
        self.id = Some(String::from(id));
        self.to_owned()
    }
    pub fn from_str(&mut self, directory: &str) -> Self {
        self.from_str = Some(String::from(directory));
        self.to_owned()
    }
}

#[allow(dead_code)]
fn div_compiler(input: &Div) -> String {
    let mut master = String::from("<div>");

    for x in input.element.iter() {
        let text = x.text.as_ref().unwrap();

        let style = style_from_enum(x.style.as_ref().unwrap_or(&Style::None));

        let mut id = String::new();

        if x.id.is_some() {
            id = format!(" id=\"{}\"", x.id.as_ref().unwrap());
        };

        // exceptions
        let text = match &x.from_str {
            Some(a) => fs_to_str(&a),
            None => text.to_owned(),
        };

        master = match &style as &str {
            "br" => format!("{master}\n</br>\n"),
            "html" => format!("{master}\n{text}"),
            _ => format!("{master}\n<{style}{id}> {text} </{style}>"),
        };
    }

    master + "\n</div>\n"
}

fn style_from_enum(style_in: &Style) -> String {
    match style_in {
        Style::None => "p",
        Style::p => "p",
        Style::h1 => "h1",
        Style::h2 => "h2",
        Style::h3 => "h3",
        Style::h4 => "h4",
        Style::h5 => "h5",
        Style::h6 => "h6",
        Style::center => "center",
        Style::br => "br",
        Style::img => "img",
        Style::video => "video",
        Style::audio => "audio",
        Style::li => "li",
        Style::ui => "ui",
        Style::a => "a",
        Style::hr => "hr",
        Style::html => "html",
    }
    .to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Style {
    // None is a substitute for p
    #[default]
    None,
    // standard fields
    h1,
    h2,
    h3,
    h4,
    h5,
    h6,
    center,
    p,
    img,
    video,
    audio,
    li,
    ui,
    a,
    hr,
    // custom methods
    // html imports raw html
    html,
    // br ignores content (just does a breakline)
    br,
}
