#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "input string was empty")]
    EmptyText,
}
