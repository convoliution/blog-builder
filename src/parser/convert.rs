use std::str::Chars;

pub fn is_unord_list_item(md: &str) -> bool {
    md.starts_with("- ")
}

pub fn is_ord_list_item(md: &str) -> bool {
    md.starts_with("1. ")
}

pub fn is_heading(md: &str) -> bool {
    let mut chars = md.chars();

    let mut num_hashes = 0;
    while let Some(c) = chars.next() {
        match c {
            '#' => {
                num_hashes += 1;
                if num_hashes > 6 {
                    return false;
                }
            },
            ' ' => return true,
             _  => return false,
        }
    }

    false
}

pub fn is_quote(md: &str) -> bool {
    md.starts_with("> ")
}

pub fn is_image(md: &str) -> bool {
    let mut chars = md.chars();

    if let Some('[') = chars.next() {
        let mut chars = chars.skip_while(|c| *c != ']').skip(1);

        if let Some('(') = chars.next() {
            let mut chars = chars.skip_while(|c| *c != ')');

            if let Some(')') = chars.next() {
                if chars.collect::<String>().trim().is_empty() {
                    return true
                }
            }
        }
    }

    false
}

pub fn unord_list(md: &str) -> Result<String, &str> {
    if !md.lines().all(|line| line.starts_with("- ")) {
        Err(md)
    } else {
        let items_html: String = md.lines()
            .map(|line| line.chars().skip(2))
            .map(|chars| text(chars.collect::<String>().as_str()).trim())
            .map(|item_html| format!("<li>{}</li>", item_html))
            .collect();

        Ok(format!("<ul>{}</ul>", items_html))
    }
}

pub fn ord_list(md: &str) -> Result<String, &str> {
    if !md.lines().all(|line| line.starts_with("1. ")) {
        Err(md)
    } else {
        let items_html: String = md.lines()
            .map(|line| line.chars().skip_while(|c| c.is_digit(10)).skip(2))
            .map(|chars| text(chars.collect::<String>().as_str()).trim())
            .map(|item_html| format!("<li>{}</li>", item_html))
            .collect();

        Ok(format!("<ul>{}</ul>", items_html))
    }
}

pub fn code_block(md: &str) -> Result<String, &str> {
    if !md.starts_with("```") || !md.trim_end().ends_with("```") {
        return Err(md)
    } else {
        let (opening_line, code_text) = md.trim_end()
            .trim_end_matches('`')
            .split_at(md.find('\n').unwrap());

        let lang = opening_line.trim_start_matches("```").trim();

        if lang.is_empty() {
            Err(md)
        } else {
            Ok(format!("<pre><code class=\"language-{}\">{}</code></pre>", lang, code_text))
        }
    }
}

pub fn heading(md: &str) -> Result<String, &str> {
    if !md.starts_with("#") {
        Err(md)
    } else {
        let mut level = 0;
        let mut chars = md.chars();

        while let Some(c) = chars.next() {
            match c {
                '#' => {
                    level += 1;
                    if level > 6 {
                        return Err(md);
                    }
                },
                ' ' => return Ok(format!("<h{level}>{}</h{level}>", text(chars.collect::<String>().as_str()).trim(), level=level)),
                 _  => return Err(md),
            }
        }

        Err(md)
    }
}

pub fn quote(md: &str) -> Result<String, &str> {
    if !md.starts_with("> ") {
        Err(md)
    } else {
        let mut chars = md.chars().skip(2);

        Ok(format!("<blockquote>{}</blockquote>", text(chars.collect::<String>().as_str()).trim()))
    }
}

pub fn image(md: &str) -> Result<String, &str> {
    if !md.starts_with("[") {
        Err(md)
    } else {
        let mut chars = md.chars().skip(1);

        let mut alt_text = String::new();

        while let Some(c) = chars.next() {
            match c {
                ']' => match chars.next() {
                    Some('(') => {
                        let mut src = String::new();

                        while let Some(c) = chars.next() {
                            match c {
                                ')' => if chars.collect::<String>().trim().is_empty() {
                                    return Ok(format!("<img src=\"{}\" alt=\"{}\"/>", src, alt_text))
                                } else {
                                    return Err(md)
                                },
                                 _  => src.push(c),
                            }
                        }

                        return Err(md)
                    },
                    _ => return Err(md),
                },
                 _  => alt_text.push(c),
            };
        }

        Err(md)
    }
}

pub fn paragraph(md: &str) -> Result<String, &str> {
    let md = md.lines()
        .map(|line| line.trim_end_matches('\n'))
        .map(|line| line.trim_end_matches('\r'))
        .collect::<String>().as_str();

    Ok(format!("<p>{}</p>", text(md).trim()))
}

pub fn text(md: &str) -> String {
    let mut html = String::with_capacity(md.len());

    let mut chars = md.replace("<", "&lt;").replace(">", "&gt;").chars();

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
