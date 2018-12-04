mod convert;

use std::str::Lines;

enum State {
    UnordList,
    OrdList,
    CodeBlock,
    Heading,
    Quote,
    Image,
    Paragraph,
}

impl State {
    fn parse<'a>(&self, md: &'a str) -> Result<String, &'a str> {
        match self {
            State::UnordList => convert::unord_list(md),
            State::OrdList   => convert::ord_list(md),
            State::CodeBlock => convert::code_block(md),
            State::Heading   => convert::heading(md),
            State::Quote     => convert::quote(md),
            State::Image     => convert::image(md),
            State::Paragraph => convert::paragraph(md),
        }
    }
}

pub struct Parser<'a> {
    lines: Lines<'a>,
    buf: String,  // buffer stores markdown
    state: Option<State>,
}

impl<'a> Parser<'a> {
    pub fn new(md_lines: Lines<'a>) -> Parser<'a> {
        Parser {
            lines: md_lines,
            buf: String::with_capacity(80),
            state: None,
        }
    }

    fn flush(mut self, new_state: Option<State>) -> Option<String> {
        let html = if !self.buf.is_empty() && self.state.is_some() {
            Some(
                self.state
                .unwrap()
                .parse(self.buf.as_str())
                .expect("Markdown should have been validated on push to buf")
            )
        } else {
            None
        };

        self.buf = String::with_capacity(80);
        self.state = new_state;

        return html;
    }
}

macro_rules! handle_item {
    ($parser:ident, $line:ident, $state:ident) => {
        {
            let item = $parser.flush(Some(State::$state));
            $parser.buf.push_str($line);
            if item.is_some() {
                return item;
            }
        }
    }
}

macro_rules! handle_state {
    ($parser:ident, $line:ident, $state:ident) => {
        {
            match $parser.state {
                Some(State::$state) => $parser.buf.push_str($line),
                _ => {
                    handle_item!($parser, $line, $state);
                }
            }
        }
    }
}

macro_rules! handle_line {
    ($parser:ident, $line:ident, $state:ident, $check_fn:ident) => {
        {
            if convert::$check_fn($line) {
                handle_state!($parser, $line, $state);
            } else {
                handle_state!($parser, $line, Paragraph);
            }
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.lines.next() {
                Some(line) => {
                    match self.state {
                        Some(State::CodeBlock) => {
                            if line.starts_with("```") {
                                return self.flush(None);
                            } else {
                                self.buf.push_str(line);
                            }
                        },
                        _ => {
                            if line.starts_with("- ") {  // TODO: support nesting
                                handle_line!(self, line, UnordList, is_unord_list_item);
                            } else if line.starts_with("1. ") {  // TODO: support nesting
                                handle_line!(self, line, OrdList, is_ord_list_item);
                            } else if line.starts_with("```") {
                                handle_item!(self, line, CodeBlock);
                            } else if line.starts_with('#') {
                                handle_line!(self, line, Heading, is_heading);
                            } else if line.starts_with("> ") {
                                handle_line!(self, line, Quote, is_quote);
                            } else if line.starts_with('!') {
                                handle_line!(self, line, Image, is_image);
                            } else if line.is_empty() {
                                if let Some(State::Paragraph) = self.state {
                                    return self.flush(Some(State::Paragraph));
                                }
                            } else {
                                handle_state!(self, line, Paragraph);
                            }
                        }
                    }
                },
                None => {
                    if let Some(State::CodeBlock) = self.state {
                        // we hit EOF while looking for end of code block, so we have to rewind
                        self.lines = self.buf.lines();
                    } else {
                        return self.flush(None);
                    }
                },
            }
        }
    }
}
