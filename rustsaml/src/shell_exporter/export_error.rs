use std::fmt;

#[derive(Debug)]
pub enum ExportError {
    ExporterNotFound,
    InvalidKeyValuePair
}

impl std::error::Error for ExportError {}

impl fmt::Display for ExportError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ExportError::ExporterNotFound => write!(f, "exporter not found"),
      ExportError::InvalidKeyValuePair => write!(f, "invalid key/value pair")
    }
  }
}