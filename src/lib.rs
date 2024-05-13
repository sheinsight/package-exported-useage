#![deny(clippy::all)]

use crate::vistor::ImportVisitor;
use napi::threadsafe_function::{
  ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
};
use napi::{JsFunction, Result as NapiResult};
use rayon::prelude::*;
use serde_json::json;
use std::{
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
  callback: JsFunction,
) -> NapiResult<serde_json::Value> {
  let fns: ThreadsafeFunction<String, ErrorStrategy::Fatal> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<String>| {
      ctx.env.create_string(&ctx.value).map(|v| vec![v])
    })?;

  let glob = Glob::new(PATTERN).unwrap();

  let vec = Arc::new(Mutex::new(Vec::<PackageExportedUsage>::new()));

  if let Ok(entries) = glob
    .walk(&workspace)
    .not(["**/node_modules/**", "**/*.d.ts"])
  {
    entries.par_bridge().for_each(|entry| {
      let e = entry.unwrap();
      let path = e.path();

      let cm = Arc::<SourceMap>::default();

      if let Ok(content) = read_to_string(path) {
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

        if let Ok(module_result) = parser.parse_program() {
          let mut import_controller = ImportVisitor {
            package_name: package_name.clone(),
            imports: vec![],
          };
          module_result.visit_with(&mut import_controller);
          let mut vec = vec.lock().unwrap();
          for import in import_controller.imports {
            vec.push(PackageExportedUsage {
              file_path: path.display().to_string(),
              exported_name: import,
            })
          }
        } else {
          fns.call(
            format!("parse file {} fail", path.display()),
            ThreadsafeFunctionCallMode::Blocking,
          );
        }
      } else {
        fns.call(
          format!("read file {} fail", path.display()),
          ThreadsafeFunctionCallMode::Blocking,
        );
      }
    });
  } else {
    fns.call(
      format!("glob workspace {} fail", &workspace),
      ThreadsafeFunctionCallMode::Blocking,
    );
  }

  let vec = vec.lock().unwrap().clone();

  Ok(json!(vec))
}
