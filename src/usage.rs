use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PackageExportedUsage {
  #[serde(rename = "exportedName")]
  pub exported_name: String,
  #[serde(rename = "filePath")]
  pub file_path: String,
}
