use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone)]
pub struct Div {
    pub style: Vec<Style>,
    pub text: Vec<String>,
}
impl Div {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn add(&mut self, style: Style, text: &str) -> Self {
        self.style.push(style);
        self.text.push(String::from(text));
        self.to_owned()
    }

    #[allow(dead_code)]
    pub fn br(&mut self) -> Self {
        self.text.push(String::new());
        self.style.push(Style::br);

        self.to_owned()
    }

    #[allow(dead_code)]
    pub fn compile(&self) -> String {
        div_compiler(self)
    }
}

#[allow(dead_code)]
fn div_compiler(input: &Div) -> String {
    let mut master = String::from("<div>");

    for x in 0..input.style.len() {
        let style_in = &input.style[x];
        let style = style_from_enum(style_in);
        let text = &input.text[x];

        // exceptions

        master = match &style as &str {
            "br" => format!("{master}\n</br>\n"),
            "html" => format!("{master}\n{text}"),
            _ => format!("{master}\n<{style}> {text} </{style}>"),
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
