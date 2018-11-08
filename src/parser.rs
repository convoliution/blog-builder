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
}
