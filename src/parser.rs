use failure::Error;

use std::io::{BufRead, Lines};

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "input string was empty")]
    EmptyText,
}

mod convert {
    use std::str::Chars;

    use parser::ParseError;

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

    // TODO: support nesting
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

    // TODO: support nesting
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
}
