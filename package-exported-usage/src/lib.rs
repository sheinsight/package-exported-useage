#![deny(clippy::all)]
// use std::path::Path;
use rayon::prelude::*;

use std::{
  collections::HashMap,
  fs::read_to_string,
  sync::{Arc, Mutex},
};

use swc_core::{
  common::{input, FileName, SourceFile, SourceMap, Spanned},
  ecma::parser::{lexer::Lexer, EsConfig, StringInput, Syntax},
};

use swc_ecmascript::{
  ast::{ImportDecl, ImportSpecifier},
  visit::Visit,
};

use swc_ecmascript::{
  ast::EsVersion,
  parser::{Parser, TsConfig},
  visit::{VisitAllWith, VisitWith},
};
use wax::Glob;

#[macro_use]
extern crate napi_derive;

struct ImportVisitor {
  imports: Vec<String>,
  package_name: String,
}

impl Visit for ImportVisitor {
  fn visit_import_decl(&mut self, n: &ImportDecl) {
    if n.src.value == self.package_name {
      for specifier in &n.specifiers {
        match specifier {
          ImportSpecifier::Named(named) => {
            self.imports.push(named.local.sym.to_string());
          }
          ImportSpecifier::Default(default) => {
            self.imports.push(default.local.sym.to_string());
          }
          ImportSpecifier::Namespace(namespace) => {
            self.imports.push(namespace.local.sym.to_string());
          }
        }

        // self.imports.push(specifier.clone());
      }
    }
  }
}

#[napi]
pub fn inspect_package_usage(package_name: String, workspace: String) {
  let pattern = "**/*.{js,ts,jsx,tsx}";

  println!("{:?}", pattern);

  let glob = Glob::new(&pattern).unwrap();

  let map = Arc::new(Mutex::new(HashMap::<String, usize>::new()));

  let entries = glob
    .walk(workspace)
    .not(["**/node_modules/**", "**/*.d.ts"])
    .unwrap();

  let num_cpus = num_cpus::get();

  println!("num_cpus: {:?}", num_cpus);

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
        let mut map = map.lock().unwrap();
        for import in import_controller.imports {
          *map.entry(import).or_insert(0) += 1;
        }
      }
      Err(error) => {
        println!("Error: {:?}", error);
      }
    }
  });

  println!("---> {:?}", map);

  println!("package-exported-usage");
}
