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
        let html = if self.buf.is_empty() {
            None
        } else {
            match self.state {
                Some(state) => Some(state.parse(self.buf.as_str())),
                None => None,
            }
        };

        self.buf = String::with_capacity(80);
        self.state = new_state;

        return html;
    }
}

macro_rules! flush {
    ($self:ident, $line:ident, $state:ident) => {
        {
            match $self.state {
                Some(State::$state) => $self.buf.push_str($line),
                _ => {
                    let item = $self.flush(Some(State::$state));
                    $self.buf.push_str($line);
                    if item.is_some() {
                        return item;
                    }
                }
            }
        }
    }
}

macro_rules! push {
    ($self:ident, $line:ident, $state:ident, $check_fn:ident) => {
        {
            if convert::$check_fn($line) {
                flush!($self, $line, $state);
            } else {
                flush!($self, $line, Paragraph);
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
                                push!(self, line, UnordList, is_unord_list_item);
                            } else if line.starts_with("1. ") {  // TODO: support nesting
                                push!(self, line, OrdList, is_ord_list_item);
                            } else if line.starts_with("```") {

                            } else if line.starts_with('#') {
                                push!(self, line, Heading, is_heading);
                            } else if line.starts_with("> ") {
                                push!(self, line, Quote, is_quote);
                            } else if line.starts_with('!') {
                                push!(self, line, Image, is_image);
                            } else if line.is_empty() {
                                if let Some(State::Paragraph) = self.state {
                                    return self.flush(Some(State::Paragraph));
                                }
                            } else {
                                match self.state {
                                    Some(State::Paragraph) => self.buf.push_str(line),
                                    _ => {
                                        let item = self.flush(Some(State::Paragraph));
                                        self.buf.push_str(line);
                                        return item;
                                    },
                                }
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
