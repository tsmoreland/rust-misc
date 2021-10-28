use crate::shell_exporter::export_error::ExportError;
use crate::shared::KeyValuePair;
use crate::shell_exporter::powershell::export_to_powershell;

pub fn get_exporter(name: &str) -> Result<fn(KeyValuePair) -> Result<String, ExportError>, ExportError> {
    match name.to_uppercase().as_str() {
         "POWERSHELL" => Ok(export_to_powershell),
        _ => Err(ExportError::ExporterNotFound)
    }


}