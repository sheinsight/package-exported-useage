use std::collections::HashMap;

use oxc_ast::{
  ast::{Expression, ImportDeclarationSpecifier, JSXElementName, JSXMemberExpressionObject},
  Visit,
};
use serde::Serialize;

#[napi[object]]
pub struct Options {
  pub ignore: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct ImportedNameAndNpmLibName {
  imported_name: String,
  npm_lib_name: String,
}

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct Location {
  pub lib_name: String,
  pub member_name: String,
  pub start: u32,
  pub end: u32,
  pub file_path: String,
}
pub struct ImportVisitor<'a> {
  pub mapper: HashMap<String, ImportedNameAndNpmLibName>,
  pub npm_libs: Vec<String>,
  pub used: &'a mut Vec<Location>,
  pub file_path: String,
}

static ES_DEFAULT: &str = "ES:DEFAULT";

static ES_NAMESPACE: &str = "ES:NAMESPACE";

impl<'a> Visit<'a> for ImportVisitor<'a> {
  fn visit_import_declaration(&mut self, decl: &oxc_ast::ast::ImportDeclaration) {
    let value = decl.source.value.to_string();
    if self.npm_libs.contains(&value) {
      if let Some(specifiers) = &decl.specifiers {
        specifiers.iter().for_each(|specifier| match specifier {
          ImportDeclarationSpecifier::ImportSpecifier(spec) => {
            println!("{:?}", spec.local.name.to_string());
            self.mapper.insert(
              spec.local.name.to_string(),
              ImportedNameAndNpmLibName {
                imported_name: spec.imported.name().to_string(),
                npm_lib_name: value.to_string(),
              },
            );
          }
          ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
            self.mapper.insert(
              spec.local.name.to_string(),
              ImportedNameAndNpmLibName {
                imported_name: ES_DEFAULT.to_string(),
                npm_lib_name: value.to_string(),
              },
            );
          }
          ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => {
            self.mapper.insert(
              spec.local.name.to_string(),
              ImportedNameAndNpmLibName {
                imported_name: ES_NAMESPACE.to_string(),
                npm_lib_name: value.to_string(),
              },
            );
          }
        });
      }
    }
  }

  fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
    if let Some(v) = self.mapper.get(&ident.name.to_string()) {
      self.used.push(Location {
        lib_name: v.npm_lib_name.to_string(),
        member_name: v.imported_name.to_string(),
        start: ident.span.start,
        end: ident.span.end,
        file_path: self.file_path.to_string(),
      });
    }
  }

  fn visit_member_expression(&mut self, expr: &oxc_ast::ast::MemberExpression) {
    if let Expression::Identifier(reference) = expr.object() {
      let name = reference.name.to_string();
      if let Some(v) = self.mapper.get(&name) {
        if v.imported_name == ES_DEFAULT || v.imported_name == ES_NAMESPACE {
          if let Some(property_name) = expr.static_property_name() {
            self.used.push(Location {
              lib_name: v.npm_lib_name.to_string(),
              member_name: property_name.to_string(),
              start: reference.span.start,
              end: reference.span.end,
              file_path: self.file_path.to_string(),
            })
          }
        } else {
          self.used.push(Location {
            lib_name: v.npm_lib_name.to_string(),
            member_name: name,
            start: reference.span.start,
            end: reference.span.end,
            file_path: self.file_path.to_string(),
          })
        }
      }
    }
  }

  fn visit_jsx_opening_element(&mut self, elem: &oxc_ast::ast::JSXOpeningElement<'a>) {
    match &elem.name {
      JSXElementName::MemberExpression(expr) => {
        if let JSXMemberExpressionObject::Identifier(reference) = &expr.object {
          let name = reference.name.to_string();
          if let Some(v) = self.mapper.get(&name) {
            if v.imported_name == ES_DEFAULT || v.imported_name == ES_NAMESPACE {
              self.used.push(Location {
                lib_name: v.npm_lib_name.to_string(),
                member_name: expr.property.name.to_string(),
                start: reference.span.start,
                end: reference.span.end,
                file_path: self.file_path.to_string(),
              })
            } else {
              self.used.push(Location {
                lib_name: v.npm_lib_name.to_string(),
                member_name: name,
                start: reference.span.start,
                end: reference.span.end,
                file_path: self.file_path.to_string(),
              })
            }
          }
        }
      }
      JSXElementName::Identifier(ident) => {
        let name = ident.name.to_string();
        if let Some(v) = self.mapper.get(&name) {
          self.used.push(Location {
            lib_name: v.npm_lib_name.to_string(),
            member_name: v.imported_name.to_string(),
            start: ident.span.start,
            end: ident.span.end,
            file_path: self.file_path.to_string(),
          })
        }
      }
      JSXElementName::NamespacedName(_) => todo!(),
    }
  }
}
