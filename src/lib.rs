use napi::{Error, Result};
use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;
use std::{collections::HashMap, fs::read};
use vistor::{ImportVisitor, Location, Options};

use wax::Glob;
mod vistor;

#[macro_use]
extern crate napi_derive;

static PATTERN: &str = "**/*.{js,ts,jsx,tsx}";

#[napi]
pub async fn inspect_package_usage(
  workspace: String,
  npm_name_vec: Vec<String>,
  options: Option<Options>,
) -> Result<Vec<Location>> {
  let glob = Glob::new(PATTERN).unwrap();

  let mut used: Vec<Location> = Vec::new();

  let ignore_patterns = options
    .as_ref()
    .and_then(|opts| opts.ignore.clone())
    .unwrap_or_else(|| vec!["**/node_modules/**".to_string(), "**/*.d.ts".to_string()]);

  let ignore: Vec<&str> = ignore_patterns.iter().map(String::as_str).collect();

  let entries = glob
    .walk(&workspace)
    .not(ignore)
    .map_err(|e| Error::new(napi::Status::GenericFailure, e.to_string()))?;

  for entry in entries {
    let entry = entry.map_err(|e| Error::new(napi::Status::GenericFailure, e.to_string()))?;
    let path = entry.path();

    if path.is_file() {
      let source_text = read(path).map_err(|err| {
        Error::new(
          napi::Status::GenericFailure,
          format!("Failed to read file: {}: {}", path.display(), err),
        )
      })?;

      let source_text = String::from_utf8_lossy(&source_text);

      let allocator = Allocator::default();
      let source_type = SourceType::from_path(&path)
        .map_err(|e| Error::new(napi::Status::GenericFailure, e.0.to_string()))?;
      let ret = Parser::new(&allocator, &source_text, source_type).parse();
      let mut visitor = ImportVisitor {
        mapper: HashMap::new(),
        npm_libs: npm_name_vec.clone(),
        used: &mut used,
        file_path: path.to_str().unwrap().to_string(),
      };
      visitor.visit_program(&ret.program);
    }
  }

  Ok(used)
}
