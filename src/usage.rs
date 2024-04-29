use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PackageExportedUsage {
  pub exported_name: String,
  pub file_path: String,
}
