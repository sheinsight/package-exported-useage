use swc_ecmascript::{
  ast::{ImportDecl, ImportSpecifier},
  visit::Visit,
};

pub struct ImportVisitor {
  pub imports: Vec<String>,
  pub package_name: String,
}

impl Visit for ImportVisitor {
  fn visit_import_decl(&mut self, n: &ImportDecl) {
    if n.src.value == self.package_name {
      for specifier in &n.specifiers {
        match specifier {
          ImportSpecifier::Named(named) => {
            self.imports.push(named.local.sym.to_string());
          }
          ImportSpecifier::Default(_) => {
            self.imports.push("ES:DEFAULT".to_string());
          }
          ImportSpecifier::Namespace(_) => {
            self.imports.push("ES:NAMESPACE".to_string());
          }
        }
      }
    }
  }
}
