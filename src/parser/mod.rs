mod convert;

use std::io::Error;

enum State {
    UnordList,
    OrdList,
    CodeBlock(String),
    Paragraph,
}

impl State {
    fn start_tag(&self) -> String {
        match self {
            State::UnordList       => "<ul>".to_string(),
            State::OrdList         => "<ol>".to_string(),
            State::CodeBlock(lang) => format!("<pre><code class=\"language-{}\">", lang),
            State::Paragraph       => "<p>".to_string(),
        }
    }

    fn end_tag(&self) -> String {
        match self {
            State::UnordList    => "</ul>".to_string(),
            State::OrdList      => "</ol>".to_string(),
            State::CodeBlock(_) => "</code></pre>".to_string(),
            State::Paragraph    => "</p>".to_string(),
        }
    }
}

pub struct Parser<I: Iterator<Item=Result<String, Error>>> {
    lines: I,
    buf: String,
    state: Option<State>,
}

impl<I> Parser<I> where I: Iterator<Item=Result<String, Error>> {
    pub fn new(lines: I) -> Parser<I> {
        Parser {
            lines,
            buf: String::with_capacity(80),
            state: None,
        }
    }

    fn change_state(mut self, new_state: Option<State>) -> Option<String> {
        let flush: Option<String>;

        if let Some(state) = self.state {
            self.buf.push_str(&state.end_tag());
        }

        if self.buf.is_empty() {
            flush = None;
        } else {
            flush = Some(self.buf);
        }

        match new_state {
            Some(state) => {
                self.buf = state.start_tag();
                self.state = Some(state);
            },
            None => {
                self.buf = String::with_capacity(80);
                self.state = None;
            },
        }

        return flush;
    }
}
