use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    SourceTooShort,
    SourceDoesNotStartWithSet,
    SourceMissingEquals,
    EmptyKey,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::SourceTooShort => write!(f, "string too short"),
            ParseError::SourceDoesNotStartWithSet => write!(f, "does not start with 'set '"),
            ParseError::SourceMissingEquals => {
                write!(f, "unable to split key and value, missing '='")
            }
            ParseError::EmptyKey => write!(f, "key cannot be empty"),
        }
    }
}
