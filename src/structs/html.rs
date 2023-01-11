#[derive(Default, Debug, Clone)]
pub struct Div {
    pub style: Vec<Style>,
    pub text: Vec<String>,
}
impl Div {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, style: Style, text: &str) -> Self {
        self.style.push(style);
        self.text.push(String::from(text));
        self.to_owned()
    }

    pub fn compile(&self) -> String {
        div_compiler(self)
    }
}

fn div_compiler(input: &Div) -> String {
    let mut master = String::from("<div>");


    for x in 0..input.style.len() {
        let style_in = &input.style[x];
        let style = style_from_enum(style_in);

        let text = &input.text[x];

        master = format!("{master}\n<{style}> {text} </{style}>");
    };

    master + "\n</div>"
}

fn style_from_enum(style_in: &Style) -> String {
    match style_in {
        Style::None => "p",
        Style::H1 => "h1",
        Style::H2 => "h2",
        Style::H3 => "h3",
        Style::H4 => "h4",
        Style::H5 => "h5",
        Style::H6 => "h6",
    }.to_string()
}

#[derive(Default, Debug, Clone)]
pub enum Style {
    #[default]
    None,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

fn gentag(master: &str, tag: &str, conf: &Option<String>) -> String {
    match conf {
        Some(a) => format!("{master}\n<{tag}{}{tag}>", a),
        None => String::from(master),
    }
}
