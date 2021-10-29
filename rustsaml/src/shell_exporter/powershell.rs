use crate::shared::KeyValuePair;
use crate::shell_exporter::export_error::ExportError;

pub fn export_to_powershell(pair: KeyValuePair) -> Result<String, ExportError> {
    let (key, value) = pair.deconstruct();

    if key.is_empty() {
        return Err(ExportError::InvalidKeyValuePair);
    }
    return Ok(format!("$env:{}='{}'", key, value));
}
