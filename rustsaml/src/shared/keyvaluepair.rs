#[derive(Debug)]
pub struct KeyValuePair {
    key: String,
    value: String,
}

impl KeyValuePair {
    #[warn(dead_code)]
    pub fn new(key: &str, value: &str) -> KeyValuePair {
        return KeyValuePair {
            key: String::from(key),
            value: String::from(value),
        };
    }

    #[warn(dead_code)]
    pub fn deconstruct(&self) -> (&str, &str) {
        return (&self.key, &self.value);
    }
}
