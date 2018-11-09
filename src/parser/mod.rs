mod convert;

use failure::Error;

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "input string was empty")]
    EmptyText,
}

pub struct Parser<I: Iterator<Item=Result<String, Error>>> {
    lines: I,
    hold: Option<String>,
}

impl<I> Parser<I> where I: Iterator<Item=Result<String, Error>> {
    pub fn new(lines: I) -> Parser<I> {
        Parser {
            lines,
            hold: None,
        }
    }
}
