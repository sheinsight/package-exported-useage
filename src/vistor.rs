use swc_ecmascript::{
  ast::{ImportDecl, ImportSpecifier},
  visit::Visit,
};

pub struct ImportVisitor {
  pub imports: Vec<String>,
  pub package_name: String,
}

static ES_DEFAULT: &str = "ES:DEFAULT";

static ES_NAMESPACE: &str = "ES:NAMESPACE";

impl Visit for ImportVisitor {
  fn visit_import_decl(&mut self, n: &ImportDecl) {
    if n.src.value != self.package_name {
      return;
    }

    for specifier in &n.specifiers {
      match specifier {
        ImportSpecifier::Named(named) => {
          if let Some(swc_ecmascript::ast::ModuleExportName::Ident(dent)) = named.imported.as_ref()
          {
            self.imports.push(dent.sym.to_string());
          } else if let Some(swc_ecmascript::ast::ModuleExportName::Str(dent)) =
            named.imported.as_ref()
          {
            self.imports.push(dent.value.to_string());
          } else {
            self.imports.push(named.local.sym.to_string());
          }
        }
        ImportSpecifier::Default(_) => {
          self.imports.push(ES_DEFAULT.to_string());
        }
        ImportSpecifier::Namespace(_) => {
          self.imports.push(ES_NAMESPACE.to_string());
        }
      }
    }
  }
}
