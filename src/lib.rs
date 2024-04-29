#![deny(clippy::all)]

use crate::vistor::ImportVisitor;
use napi::Result as NapiResult;
use rayon::prelude::*;
use serde_json::json;
use std::{
  collections::HashMap,
  fs::read_to_string,
  sync::{Arc, Mutex},
};
use swc_core::{
  common::{FileName, SourceMap},
  ecma::parser::{lexer::Lexer, StringInput, Syntax},
};
use swc_ecmascript::{
  ast::EsVersion,
  parser::{Parser, TsConfig},
  visit::VisitWith,
};
use usage::PackageExportedUsage;
use wax::Glob;

mod usage;
mod vistor;

#[macro_use]
extern crate napi_derive;

static PATTERN: &str = "**/*.{js,ts,jsx,tsx}";

#[napi]
pub fn inspect_package_usage(
  package_name: String,
  workspace: String,
) -> NapiResult<serde_json::Value> {
  let glob = Glob::new(PATTERN).unwrap();

  // let map = Arc::new(Mutex::new(HashMap::<String, usize>::new()));

  let vec = Arc::new(Mutex::new(Vec::<PackageExportedUsage>::new()));

  let entries = glob
    .walk(workspace)
    .not(["**/node_modules/**", "**/*.d.ts"])
    .unwrap();

  entries.par_bridge().for_each(|entry| {
    let e = entry.unwrap();
    let path = e.path();

    let cm = Arc::<SourceMap>::default();

    let content = read_to_string(path).unwrap();

    let fm = cm.new_source_file(FileName::Anon, content);

    let lexer = Lexer::new(
      Syntax::Typescript(TsConfig {
        tsx: true,
        decorators: true,
        dts: false,
        no_early_errors: false,
        ..Default::default()
      }),
      EsVersion::EsNext,
      StringInput::from(&*fm),
      None,
    );

    let mut parser = Parser::new_from(lexer);

    let list_error = parser.take_errors();
    if list_error.iter().len() > 0 {
      let err_msg = list_error
        .iter()
        .map(|err| err.kind().msg())
        .collect::<Vec<_>>()
        .join("");
      println!("Error: {:?}", err_msg);
    }

    match parser.parse_program() {
      Ok(module_result) => {
        let mut import_controller = ImportVisitor {
          package_name: package_name.clone(),
          imports: vec![],
        };
        module_result.visit_with(&mut import_controller);
        // let mut map = map.lock().unwrap();
        let mut vec = vec.lock().unwrap();
        for import in import_controller.imports {
          vec.push(PackageExportedUsage {
            file_path: path.display().to_string(),
            exported_name: import,
          })
          // *map.entry(import).or_insert(0) += 1;
        }
      }
      Err(error) => {
        println!("Error: {:?} {:?}", error, path);
      }
    }
  });

  let vec = vec.lock().unwrap().clone();

  Ok(json!(vec))
}
