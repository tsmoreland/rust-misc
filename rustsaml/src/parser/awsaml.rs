use crate::parser::parse_error::ParseError;
use crate::shared::KeyValuePair;

pub fn parse_key_value(source: String) -> Result<KeyValuePair, ParseError> {

    if source.len() < 5 {
        return Err(ParseError::SourceTooShort);
    }

    if !source[0..4].eq_ignore_ascii_case("SET ") {
        return Err(ParseError::SourceDoesNotStartWithSet);
    }

    let parts :Vec<&str> = source[4..].split("=").collect();
    if parts.len() != 2 {
        return Err(ParseError::SourceMissingEquals);
    }

    if parts[0].trim().len() == 0 {
        return Err(ParseError::EmptyKey);
    }

    return Ok(KeyValuePair::new(parts[0], parts[1]));
}