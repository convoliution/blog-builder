use std::str::Chars;

pub fn heading(buf: String) -> Result<String, String> {
    let mut level = 0;
    let mut chars = buf.chars();

    while let Some(c) = chars.next() {
        match c {
            '#' => {
                level += 1;
                if level > 6 {
                    return Err(buf);
                }
            },
            ' ' => {
                match text(&mut chars) {
                    Ok(html) => return Ok(format!("<h{level}>{}</h{level}>", html, level=level)),
                    Err(_) => return Err(buf),
                }
            },
             _  => return Err(buf),
        }
    }

    Err(buf)
}

pub fn quote(buf: String) -> Result<String, String> {
    let mut chars = buf.chars().skip(2);

    match text(&mut chars) {
        Ok(html) => Ok(format!("<blockquote>{}</blockquote>", html)),
        Err(_) => Err(buf),
    }
}

pub fn unord_list(buf: String) -> Result<String, String> {
    let lines: Result<Vec<String>, ParseError> = buf.lines()
        .map(|line| line.chars().skip(2))
        .map(|chars| text(&mut chars))
        .collect();

    match lines {
        Ok(html) => Ok(format!("<ul>{}</ul>",
            html.iter().fold("".to_string(), |acc, line| format!("{}<li>{}</li>", acc, line)))),
        Err(_) => Err(buf),
    }
}

pub fn ord_list(buf: String) -> Result<String, String> {
    let lines: Result<Vec<String>, ParseError> = buf.lines()
        .map(|line| line.chars().skip_while(|c| c.is_digit(10)).skip(2))
        .map(|chars| text(&mut chars))
        .collect();

    match lines {
        Ok(html) => Ok(format!("<ol>{}</ol>",
            html.iter().fold("".to_string(), |acc, line| format!("{}<li>{}</li>", acc, line)))),
        Err(_) => Err(buf),
    }
}

pub fn image(buf: String) -> Result<String, String> {
    let mut chars = buf.chars().skip(1);

    let mut alt_text = String::new();

    while let Some(c) = chars.next() {
        match c {
            ']' => match chars.next() {
                Some('(') => {
                    let mut src = String::new();

                    while let Some(c) = chars.next() {
                        match c {
                            ')' => return Ok(format!("<img src=\"{}\" alt=\"{}\"/>", src, alt_text)),
                             _  => src.push(c),
                        }
                    }

                    return Err(buf)
                },
                _ => return Err(buf),
            },
             _  => alt_text.push(c),
        };
    }

    Err(buf)
}

pub fn code_block(buf: String) -> Result<String, String> {
    let (opening_line, code_text) = buf.trim_end_matches(&['`', '\r', '\n'] as &[_])
        .split_at(buf.find('\n').unwrap());

    let lang = opening_line.trim_start_matches("```").trim();

    if lang.is_empty() {
        Err(buf)
    } else {
        Ok(format!("<pre><code class=\"language-{}\">{}</code></pre>", lang, code_text))
    }
}

pub fn paragraph(buf: String) -> Result<String, String> {
    let mut chars = buf.chars();

    match text(&mut chars) {
        Ok(html) => Ok(format!("<p>{}</p>", html)),
        Err(_) => Err(buf),
    }
}

fn text(chars: &mut Chars) -> String {
    let mut html = String::with_capacity(chars.as_str().len());

    while let Some(c) = chars.next() {
        match c {
            '`' => html.push_str(match code(&mut chars) {
                Ok(s) => &s,
                Err(s) => &s,
            }),
            '_' => html.push_str(match italic(&mut chars) {
                Ok(s) => &s,
                Err(s) => &s,
            }),
            '*' => html.push_str(match bold(&mut chars) {
                Ok(s) => &s,
                Err(s) => &s,
            }),
            '[' => html.push_str(match link(&mut chars) {
                Ok(s) => &s,
                Err(s) => &s,
            }),
             _  => html.push(c),
        };
    }

    html
}

fn code(chars: &mut Chars) -> Result<String, String> {
    let mut code_text = String::new();

    while let Some(c) = chars.next() {
        match c {
            '`' => return Ok(format!("<code>{}</code>", code_text)),
             _  => code_text.push(c),
        };
    }

    Err(format!("`{}", code_text))
}

fn italic(chars: &mut Chars) -> Result<String, String> {
    let mut html = String::new();

    while let Some(c) = chars.next() {
        match c {
            '*' => html.push_str(match bold(&mut chars) {
                Ok(s) => &s,
                Err(s) => &s,
            }),
            '[' => html.push_str(match link(&mut chars) {
                Ok(s) => &s,
                Err(s) => &s,
            }),
            '_' => return Ok(format!("<i>{}</i>", html)),
             _  => html.push(c),
        };
    }

    Err(format!("_{}", html))
}

fn bold(chars: &mut Chars) -> Result<String, String> {
    let mut html = String::new();

    while let Some(c) = chars.next() {
        match c {
            '_' => html.push_str(match italic(&mut chars) {
                Ok(s) => &s,
                Err(s) => &s,
            }),
            '[' => html.push_str(match link(&mut chars) {
                Ok(s) => &s,
                Err(s) => &s,
            }),
            '*' => return Ok(format!("<b>{}</b>", html)),
             _  => html.push(c),
        };
    }

    Err(format!("*{}", html))
}

fn link(chars: &mut Chars) -> Result<String, String> {
    let mut link_text = String::new();

    while let Some(c) = chars.next() {
        match c {
            '`' => link_text.push_str(match code(&mut chars) {
                Ok(s) => &s,
                Err(s) => &s,
            }),
            '_' => link_text.push_str(match italic(&mut chars) {
                Ok(s) => &s,
                Err(s) => &s,
            }),
            '*' => link_text.push_str(match bold(&mut chars) {
                Ok(s) => &s,
                Err(s) => &s,
            }),
            ']' => match chars.next() {
                Some('(') => {
                    let mut href = String::new();

                    while let Some(c) = chars.next() {
                        match c {
                            ')' => return Ok(format!("<a href=\"{}\">{}</a>", href, link_text)),
                             _  => href.push(c),
                        }
                    }

                    return Err(format!("[{}]({}", link_text, href))
                },
                _ => return Err(format!("[{}]", link_text)),
            },
             _  => link_text.push(c),
        };
    }

    Err(format!("[{}", link_text))
}
