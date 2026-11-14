use html_escape::encode_double_quoted_attribute;

/// Assembles complete HTML documents from head and body content.
#[derive(Default)]
pub struct Document {
    head: String,
    body: String,
    html_attrs: Option<String>,
    body_attrs: Option<String>,
}

/// The section a fragment belongs to when it doesn't define its own structure.
#[derive(Clone, Copy)]
pub enum Section {
    Head,
    Body,
}

impl Document {
    /// Add a source to the document, extracting any document structure it contains.
    pub fn add(&mut self, source: &str, section: Section) {
        let mut remaining = source;

        loop {
            let split = split_document(remaining);

            if !split.structured {
                let source = strip_leading_declarations(remaining);
                match section {
                    Section::Head => append(&mut self.head, source),
                    Section::Body => {
                        let (head, body) = split_head_elements(source);
                        append(&mut self.head, &head);
                        append(&mut self.body, &strip_structural_tags(&body));
                    }
                }
                break;
            }

            let prefix = clean_prefix(split.prefix);
            if !prefix.is_empty() {
                match section {
                    Section::Head => append(&mut self.head, &prefix),
                    Section::Body => {
                        let (head, body) = split_head_elements(&prefix);
                        append(&mut self.head, &head);
                        append(&mut self.body, &strip_structural_tags(&body));
                    }
                }
            }

            append(&mut self.head, split.head);
            let (head, body) = split_head_elements(split.body);
            append(&mut self.head, &head);
            append(&mut self.body, &strip_structural_tags(&body));

            if self.html_attrs.is_none() {
                self.html_attrs = split
                    .html_attrs
                    .filter(|attrs| !attrs.is_empty())
                    .map(str::to_string);
            }

            if self.body_attrs.is_none() {
                self.body_attrs = split
                    .body_attrs
                    .filter(|attrs| !attrs.is_empty())
                    .map(str::to_string);
            }

            if split.remainder.is_empty() {
                break;
            }

            remaining = split.remainder;
        }
    }

    /// Add inline CSS to the document head.
    pub fn add_style(&mut self, css: &str) {
        let css = css.trim();
        if !css.is_empty() {
            append(&mut self.head, &format!("<style>\n{css}\n</style>"));
        }
    }

    /// Render the complete document.
    pub fn render(&self, lang: Option<&str>) -> String {
        let mut document = String::new();

        document.push_str("<!doctype html>\n<html");
        document.push_str(&render_attributes(
            self.html_attrs.as_deref().unwrap_or_default(),
            lang,
        ));
        document.push_str(">\n<head>\n");

        if !has_charset(&self.head) {
            document.push_str("<meta charset=\"utf-8\">\n");
        }

        if !self.head.is_empty() {
            document.push_str(&self.head);
            document.push('\n');
        }

        document.push_str("</head>\n<body");
        document.push_str(&render_attributes(
            self.body_attrs.as_deref().unwrap_or_default(),
            None,
        ));
        document.push_str(">\n");

        if !self.body.is_empty() {
            document.push_str(&self.body);
            document.push('\n');
        }

        document.push_str("</body>\n</html>\n");
        document
    }
}

fn append(target: &mut String, text: &str) {
    let text = text.trim();
    if text.is_empty() {
        return;
    }

    if !target.is_empty() {
        target.push('\n');
    }
    target.push_str(text);
}

/// Remove declarations that start a fragment, such as a stray doctype.
fn strip_leading_declarations(source: &str) -> &str {
    let mut remaining = source.trim_start();

    while (remaining.starts_with("<!") || remaining.starts_with("<?"))
        && !remaining.starts_with("<!--")
    {
        let Some(end) = remaining.find('>') else {
            break;
        };
        remaining = remaining[end + 1..].trim_start();
    }

    remaining
}

/// Remove comments and declarations that precede a document's structure.
fn clean_prefix(source: &str) -> String {
    let mut cleaned = String::new();
    let mut position = 0;

    while position < source.len() {
        if let Some(comment) = source[position..].strip_prefix("<!--") {
            if let Some(end) = comment.find("-->") {
                position += 4 + end + 3;
                continue;
            }
            break;
        }

        if source[position..].starts_with("<!") || source[position..].starts_with("<?") {
            if let Some(end) = source[position..].find('>') {
                position += end + 1;
                continue;
            }
            break;
        }

        if let Some(character) = source[position..].chars().next() {
            cleaned.push(character);
            position += character.len_utf8();
        } else {
            break;
        }
    }

    cleaned
}

/// Move head-only metadata elements out of body content.
fn split_head_elements(source: &str) -> (String, String) {
    let mut head = String::new();
    let mut body = String::new();
    let mut copied_until = 0;
    let mut foreign_depth = 0usize;
    let mut position = 0;

    while let Some(tag) = next_tag(source, &mut position) {
        if !tag.closing && foreign_depth == 0 && is_head_only(tag.name) {
            body.push_str(&source[copied_until..tag.start]);

            let end = if is_raw_text(tag.name) {
                position
            } else {
                tag.end
            };
            head.push_str(&source[tag.start..end]);
            copied_until = end;
            continue;
        }

        if tag.closing {
            if is_foreign(tag.name) {
                foreign_depth = foreign_depth.saturating_sub(1);
            }
        } else if is_foreign(tag.name) && !tag.self_closing {
            foreign_depth += 1;
        }
    }

    body.push_str(&source[copied_until..]);
    (head, body)
}

/// Remove stray html/head/body tags left in body content.
fn strip_structural_tags(source: &str) -> String {
    let mut cleaned = String::new();
    let mut copied_until = 0;
    let mut position = 0;

    while let Some(tag) = next_tag(source, &mut position) {
        if is_structural(tag.name) {
            cleaned.push_str(&source[copied_until..tag.start]);
            copied_until = tag.end;
        }
    }

    cleaned.push_str(&source[copied_until..]);
    cleaned
}

fn is_head_only(name: &str) -> bool {
    ["base", "link", "meta", "style", "title"]
        .iter()
        .any(|element| name.eq_ignore_ascii_case(element))
}

fn is_structural(name: &str) -> bool {
    ["html", "head", "body"]
        .iter()
        .any(|element| name.eq_ignore_ascii_case(element))
}

fn is_foreign(name: &str) -> bool {
    name.eq_ignore_ascii_case("svg") || name.eq_ignore_ascii_case("math")
}

pub(crate) fn is_void_element(name: &str) -> bool {
    [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ]
    .iter()
    .any(|element| name.eq_ignore_ascii_case(element))
}

/// Return the inner HTML of the first element with the given name.
pub(crate) fn element_inner_html(source: &str, name: &str) -> Option<String> {
    let mut position = 0;

    while let Some(tag) = next_tag(source, &mut position) {
        if tag.closing || !tag.name.eq_ignore_ascii_case(name) {
            continue;
        }

        if is_raw_text(tag.name) {
            let needle = format!("</{name}");
            let end = find_ignore_ascii_case(source, tag.end, &needle).unwrap_or(source.len());
            return Some(source[tag.end..end].to_string());
        }

        if tag.self_closing || is_void_element(tag.name) {
            return Some(String::new());
        }

        let start = tag.end;
        let mut depth = 1usize;

        while let Some(inner) = next_tag(source, &mut position) {
            if inner.closing {
                if inner.name.eq_ignore_ascii_case(name) {
                    depth -= 1;
                    if depth == 0 {
                        return Some(source[start..inner.start].to_string());
                    }
                }
            } else if inner.name.eq_ignore_ascii_case(name)
                && !inner.self_closing
                && !is_void_element(inner.name)
                && !is_opaque(inner.name)
            {
                depth += 1;
            }
        }

        return Some(source[start..].to_string());
    }

    None
}

/// Call a closure for every opening element, skipping raw text content.
pub(crate) fn for_each_element<'a>(source: &'a str, mut callback: impl FnMut(&'a str, &'a str)) {
    let mut position = 0;

    while let Some(tag) = next_tag(source, &mut position) {
        if !tag.closing {
            callback(tag.name, tag.attrs);
        }
    }
}

/// Return an attribute value with its surrounding quotes removed.
pub(crate) fn attribute_value<'a>(attrs: &'a str, name: &str) -> Option<&'a str> {
    let (_, value) = parse_attributes(attrs)
        .into_iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(name))?;
    let value = value?;

    if value.len() >= 2 {
        let first = value.as_bytes()[0];
        let last = value.as_bytes()[value.len() - 1];

        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return Some(&value[1..value.len() - 1]);
        }
    }

    Some(value)
}

/// Head and body portions extracted from a source.
struct Split<'a> {
    structured: bool,
    prefix: &'a str,
    head: &'a str,
    body: &'a str,
    remainder: &'a str,
    html_attrs: Option<&'a str>,
    body_attrs: Option<&'a str>,
}

/// Split a source into head and body portions when it contains document tags.
fn split_document(source: &str) -> Split<'_> {
    let mut structured = false;
    let mut html_open_start = None;
    let mut html_open_end = None;
    let mut html_close_start = None;
    let mut html_close_end = None;
    let mut head_open_start = None;
    let mut head_open_end = None;
    let mut head_close_start = None;
    let mut head_close_end = None;
    let mut body_open_start = None;
    let mut body_open_end = None;
    let mut body_close_start = None;
    let mut body_close_end = None;
    let mut html_attrs = None;
    let mut body_attrs = None;

    let mut position = 0;
    while let Some(tag) = next_tag(source, &mut position) {
        if tag.closing {
            if tag.name.eq_ignore_ascii_case("html") && html_close_start.is_none() {
                html_close_start = Some(tag.start);
                html_close_end = Some(tag.end);
                structured = true;
            } else if tag.name.eq_ignore_ascii_case("head") && head_close_start.is_none() {
                head_close_start = Some(tag.start);
                head_close_end = Some(tag.end);
                structured = true;
            } else if tag.name.eq_ignore_ascii_case("body") && body_close_start.is_none() {
                body_close_start = Some(tag.start);
                body_close_end = Some(tag.end);
                structured = true;
            }
        } else if tag.name.eq_ignore_ascii_case("html") && html_open_end.is_none() {
            html_open_start = Some(tag.start);
            html_open_end = Some(tag.end);
            html_attrs = Some(tag.attrs.trim());
            structured = true;
        } else if tag.name.eq_ignore_ascii_case("head") && head_open_end.is_none() {
            head_open_start = Some(tag.start);
            head_open_end = Some(tag.end);
            structured = true;
        } else if tag.name.eq_ignore_ascii_case("body") && body_open_end.is_none() {
            body_open_start = Some(tag.start);
            body_open_end = Some(tag.end);
            body_attrs = Some(tag.attrs.trim());
            structured = true;
        }
    }

    if !structured {
        return Split {
            structured: false,
            prefix: "",
            head: "",
            body: "",
            remainder: "",
            html_attrs: None,
            body_attrs: None,
        };
    }

    let length = source.len();
    let split_fragment = (head_close_start.is_some() && head_open_end.is_none())
        || (body_close_start.is_some()
            && body_open_end.is_none()
            && head_close_end.is_none()
            && html_open_end.is_none());
    let prefix = if split_fragment {
        ""
    } else {
        let document_start = [html_open_start, head_open_start, body_open_start]
            .into_iter()
            .flatten()
            .min();
        document_start.map_or("", |start| slice(source, 0, start))
    };

    let head = if let Some(start) = head_open_end {
        let end = head_close_start
            .or(body_open_start)
            .or(html_close_start)
            .unwrap_or(length);
        slice(source, start, end)
    } else if let Some(end) = head_close_start {
        slice(source, html_open_end.unwrap_or(0), end)
    } else {
        ""
    };

    let body = if let Some(start) = body_open_end {
        let end = body_close_start.or(html_close_start).unwrap_or(length);
        slice(source, start, end)
    } else if let Some(end) = body_close_start {
        slice(source, head_close_end.or(html_open_end).unwrap_or(0), end)
    } else if let Some(html_start) = html_open_end {
        let start = match (head_open_end, head_close_end) {
            (Some(_), Some(head_end)) => head_end,
            (Some(_), None) => html_close_start.unwrap_or(length),
            (None, _) => html_start,
        };
        slice(source, start, html_close_start.unwrap_or(length))
    } else if let Some(head_end) = head_close_end {
        slice(source, head_end, length)
    } else {
        ""
    };

    let remainder_start = match (body_close_end, html_close_end) {
        (Some(body_end), Some(html_end)) => Some(body_end.max(html_end)),
        (Some(body_end), None) => Some(body_end),
        (None, Some(html_end)) => Some(html_end),
        (None, None) => None,
    };
    let remainder = remainder_start.map_or("", |start| slice(source, start, length));

    Split {
        structured: true,
        prefix,
        head,
        body,
        remainder,
        html_attrs,
        body_attrs,
    }
}

fn slice(source: &str, start: usize, end: usize) -> &str {
    if start <= end && end <= source.len() {
        &source[start..end]
    } else {
        ""
    }
}

struct Tag<'a> {
    name: &'a str,
    attrs: &'a str,
    start: usize,
    end: usize,
    closing: bool,
    self_closing: bool,
}

/// Find the next tag, skipping comments, declarations, and raw text content.
fn next_tag<'a>(source: &'a str, position: &mut usize) -> Option<Tag<'a>> {
    let bytes = source.as_bytes();

    while *position < bytes.len() {
        let relative = source[*position..].find('<')?;
        let start = *position + relative;
        let rest = &source[start..];

        if let Some(comment) = rest.strip_prefix("<!--") {
            if let Some(end) = comment.find("-->") {
                *position = start + 4 + end + 3;
                continue;
            }
            *position = source.len();
            return None;
        }

        if rest.starts_with("<!") || rest.starts_with("<?") {
            if let Some(end) = rest.find('>') {
                *position = start + end + 1;
                continue;
            }
            *position = source.len();
            return None;
        }

        let closing = rest[1..].starts_with('/');
        let mut name_start = start + if closing { 2 } else { 1 };
        while name_start < bytes.len() && bytes[name_start].is_ascii_whitespace() {
            name_start += 1;
        }

        if name_start >= bytes.len() || !bytes[name_start].is_ascii_alphabetic() {
            *position = start + 1;
            continue;
        }

        let name_end = source[name_start..]
            .find(|character: char| {
                character.is_ascii_whitespace() || character == '>' || character == '/'
            })
            .map(|offset| name_start + offset)
            .unwrap_or(source.len());

        let tag_end = find_tag_end(source, name_end)?;
        let attrs = if closing {
            ""
        } else {
            &source[name_end..tag_end]
        };
        let tag = Tag {
            name: &source[name_start..name_end],
            attrs,
            start,
            end: tag_end + 1,
            closing,
            self_closing: !closing && is_self_closing(attrs),
        };

        if !closing && is_opaque(tag.name) {
            *position = raw_text_end(source, tag.end, tag.name).unwrap_or(source.len());
        } else {
            *position = tag.end;
        }

        return Some(tag);
    }

    None
}

fn find_tag_end(source: &str, from: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut quote = None;
    let mut position = from;

    while position < bytes.len() {
        let byte = bytes[position];
        match quote {
            Some(active) => {
                if byte == active {
                    quote = None;
                }
            }
            None => match byte {
                b'"' | b'\'' => quote = Some(byte),
                b'>' => return Some(position),
                _ => {}
            },
        }
        position += 1;
    }

    None
}

fn is_self_closing(attrs: &str) -> bool {
    let attrs = attrs.trim_end();
    let Some(attrs) = attrs.strip_suffix('/') else {
        return false;
    };

    match attrs.as_bytes().last().copied() {
        None => true,
        Some(byte) => byte.is_ascii_whitespace() || byte == b'"' || byte == b'\'',
    }
}

fn is_raw_text(name: &str) -> bool {
    [
        "script",
        "style",
        "textarea",
        "title",
        "xmp",
        "iframe",
        "noembed",
        "noframes",
        "plaintext",
    ]
    .iter()
    .any(|raw| name.eq_ignore_ascii_case(raw))
}

fn is_opaque(name: &str) -> bool {
    is_raw_text(name) || name.eq_ignore_ascii_case("template")
}

fn raw_text_end(source: &str, from: usize, name: &str) -> Option<usize> {
    if name.eq_ignore_ascii_case("plaintext") {
        return Some(source.len());
    }

    let needle = format!("</{name}");
    let mut search_from = from;

    loop {
        let start = find_ignore_ascii_case(source, search_from, &needle)?;
        let next = source.as_bytes().get(start + needle.len()).copied();

        if matches!(
            next,
            Some(b'>') | Some(b'/') | Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')
        ) {
            return Some(find_tag_end(source, start + needle.len())? + 1);
        }

        search_from = start + 1;
    }
}

fn find_ignore_ascii_case(haystack: &str, from: usize, needle: &str) -> Option<usize> {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();

    if needle.is_empty() || from >= haystack.len() || haystack.len() - from < needle.len() {
        return None;
    }

    (from..=haystack.len() - needle.len())
        .find(|&index| haystack[index..index + needle.len()].eq_ignore_ascii_case(needle))
}

fn has_charset(head: &str) -> bool {
    let mut position = 0;
    while let Some(tag) = next_tag(head, &mut position) {
        if tag.closing || !tag.name.eq_ignore_ascii_case("meta") {
            continue;
        }

        if has_attribute(tag.attrs, "charset") {
            return true;
        }

        if has_attribute(tag.attrs, "http-equiv")
            && tag.attrs.to_ascii_lowercase().contains("content-type")
        {
            return true;
        }
    }

    false
}

fn render_attributes(attrs: &str, lang: Option<&str>) -> String {
    let mut rendered = String::new();
    let mut seen: Vec<String> = Vec::new();
    let mut has_lang = false;

    for (name, value) in parse_attributes(attrs) {
        let key = name.to_ascii_lowercase();
        if seen.contains(&key) {
            continue;
        }
        seen.push(key);

        if name.eq_ignore_ascii_case("lang") {
            has_lang = true;
            if let Some(lang) = lang {
                rendered.push_str(" lang=\"");
                rendered.push_str(&encode_double_quoted_attribute(lang));
                rendered.push('"');
                continue;
            }
        }

        rendered.push(' ');
        rendered.push_str(name);
        if let Some(value) = value {
            rendered.push('=');
            rendered.push_str(value);
        }
    }

    if !has_lang {
        if let Some(lang) = lang {
            rendered.push_str(" lang=\"");
            rendered.push_str(&encode_double_quoted_attribute(lang));
            rendered.push('"');
        }
    }

    rendered
}

fn has_attribute(attrs: &str, name: &str) -> bool {
    parse_attributes(attrs)
        .iter()
        .any(|(key, _)| key.eq_ignore_ascii_case(name))
}

/// Parse attributes while preserving their original quoting.
fn parse_attributes(attrs: &str) -> Vec<(&str, Option<&str>)> {
    let bytes = attrs.as_bytes();
    let mut parsed = Vec::new();
    let mut position = 0;

    while position < bytes.len() {
        while position < bytes.len()
            && (bytes[position].is_ascii_whitespace() || bytes[position] == b'/')
        {
            position += 1;
        }

        if position >= bytes.len() {
            break;
        }

        let name_start = position;
        while position < bytes.len()
            && !bytes[position].is_ascii_whitespace()
            && bytes[position] != b'='
            && bytes[position] != b'/'
            && bytes[position] != b'>'
        {
            position += 1;
        }

        if position == name_start {
            position += 1;
            continue;
        }

        let name = &attrs[name_start..position];

        while position < bytes.len() && bytes[position].is_ascii_whitespace() {
            position += 1;
        }

        if position >= bytes.len() || bytes[position] != b'=' {
            parsed.push((name, None));
            continue;
        }

        position += 1;
        while position < bytes.len() && bytes[position].is_ascii_whitespace() {
            position += 1;
        }

        if position >= bytes.len() {
            parsed.push((name, None));
            break;
        }

        if bytes[position] == b'"' || bytes[position] == b'\'' {
            let quote = bytes[position];
            let value_start = position;
            position += 1;
            while position < bytes.len() && bytes[position] != quote {
                position += 1;
            }
            if position < bytes.len() {
                position += 1;
            }
            parsed.push((name, Some(&attrs[value_start..position])));
        } else {
            let value_start = position;
            while position < bytes.len()
                && !bytes[position].is_ascii_whitespace()
                && bytes[position] != b'>'
            {
                position += 1;
            }
            parsed.push((name, Some(&attrs[value_start..position])));
        }
    }

    parsed
}
