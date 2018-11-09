use std::str::Chars;

pub fn heading(buf: String) -> Result<String, String> {
    if !buf.starts_with("#") {
        Err(buf)
    } else {
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
                ' ' => return Ok(format!("<h{level}>{}</h{level}>", text(chars.collect::<String>()).trim(), level=level)),
                 _  => return Err(buf),
            }
        }

        Err(buf)
    }
}

pub fn quote(buf: String) -> Result<String, String> {
    if !buf.starts_with("> ") {
        Err(buf)
    } else {
        let mut chars = buf.chars().skip(2);

        Ok(format!("<blockquote>{}</blockquote>", text(chars.collect::<String>()).trim()))
    }
}

pub fn unord_list(buf: String) -> Result<String, String> {
    if !buf.lines().all(|line| line.starts_with("- ")) {
        Err(buf)
    } else {
        let items_html: String = buf.lines()
            .map(|line| line.chars().skip(2))
            .map(|chars| text(chars.collect::<String>()).trim())
            .map(|item_html| format!("<li>{}</li>", item_html))
            .collect();

        Ok(format!("<ul>{}</ul>", items_html))
    }
}

pub fn ord_list(buf: String) -> Result<String, String> {
    if !buf.lines().all(|line| line.starts_with("1. ")) {
        Err(buf)
    } else {
        let items_html: String = buf.lines()
            .map(|line| line.chars().skip_while(|c| c.is_digit(10)).skip(2))
            .map(|chars| text(chars.collect::<String>()).trim())
            .map(|item_html| format!("<li>{}</li>", item_html))
            .collect();

        Ok(format!("<ul>{}</ul>", items_html))
    }
}

pub fn image(buf: String) -> Result<String, String> {
    if !buf.starts_with("[") {
        Err(buf)
    } else {
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
}

pub fn code_block(buf: String) -> Result<String, String> {
    if !buf.starts_with("```") || !buf.trim_end().ends_with("```") {
        return Err(buf)
    } else {
        let (opening_line, code_text) = buf.trim_end()
            .trim_end_matches('`')
            .split_at(buf.find('\n').unwrap());

        let lang = opening_line.trim_start_matches("```").trim();

        if lang.is_empty() {
            Err(buf)
        } else {
            Ok(format!("<pre><code class=\"language-{}\">{}</code></pre>", lang, code_text))
        }
    }
}

pub fn paragraph(buf: String) -> Result<String, String> {
    let md: String = buf.lines()
        .map(|line| line.trim_end_matches('\n'))
        .map(|line| line.trim_end_matches('\r'))
        .collect();

    Ok(format!("<p>{}</p>", text(md).trim()))
}

pub fn text(buf: String) -> String {
    let mut html = String::with_capacity(buf.len());

    let mut chars = buf.replace("<", "&lt;").replace(">", "&gt;").chars();

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
            '`' => return Ok(format!("<code>{}</code>", code_text.replace("&lt;", "<").replace("&gt;", ">"))),
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
