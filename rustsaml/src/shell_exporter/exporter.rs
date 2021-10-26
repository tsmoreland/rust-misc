use crate::shell_exporter::export_error::ExportError;
use crate::shared::KeyValuePair;

pub fn get_exporter(name: &str) -> Result<fn() -> Result<String, ExportError>, ExportError> {
    match name.to_uppercase() {
        //"POWERSHELL" => 
        _ => Err(ExporterNotFound)
    }


}