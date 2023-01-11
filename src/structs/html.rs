use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone)]
pub struct Div {
    pub element: Vec<Values>
}
#[derive(Default, Debug, Clone)]
pub struct Values {
    pub style: Option<Style>,
    pub text: Option<String>,
    pub id: Option<String>,
}

impl Div {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn element(&mut self, element: Values) -> Self {
        self.element.push(element);
        self.to_owned()

    }


    #[allow(dead_code)]
    pub fn compile(&self) -> String {
        div_compiler(self)
    }


}
impl  Values {

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
}

#[allow(dead_code)]
fn div_compiler(input: &Div) -> String {
    let mut master = String::from("<div>");

    for x in input.element.iter() {

        let text = x.text.clone().unwrap_or(String::from("PLACEHOLDER"));
        let style = style_from_enum(&x.clone().style.unwrap_or(Style::None));
        let id = x.clone().id;

        let mut id = match id {
            None => String::new(),
            Some(a) => format!(" id=\"{a}\""),
        };


        println!("{}", id);
        if id == String::from("id=\"\"") {
            id = String::new();
        };

        // exceptions

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
