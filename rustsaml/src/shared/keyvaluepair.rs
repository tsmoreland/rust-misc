
#[derive(Debug)]
pub struct KeyValuePair<'a> {
    key: &'a str,
    value: &'a str,
}

impl KeyValuePair<'_> {
    #[warn(dead_code)]
    pub fn new<'a>(key: &'a str, value: &'a str) -> KeyValuePair<'a> {
        return KeyValuePair { key, value }
    }
}