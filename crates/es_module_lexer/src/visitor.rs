use oxc_ast::{
  ast::{
    BindingIdentifier, BindingPatternKind, Declaration, ExportDefaultDeclarationKind, Expression,
    ModuleDeclaration, Statement,
  },
  AstKind, Visit,
};
use oxc_span::GetSpan;

use crate::constants::{
  BRACKET_LEFT, BRACKET_RIGHT, CURLY_BRACE_LEFT, CURLY_BRACE_RIGHT, QUOTE, SINGLE_QUOTE,
};

#[derive(Debug)]
pub struct Import {
  /// name
  ///
  /// For non-string dynamic import expressions
  pub n: Option<String>,
  /// name start
  pub s: u32,
  /// name end
  pub e: u32,
  /// statement start
  pub ss: u32,
  /// statement end
  pub se: u32,
  /// dynamic index
  /// - `-2`: import.meta
  /// - `-1`: no dynamic import
  /// - `> -1`: dynamic import
  pub d: i32,
  /// attributes index
  /// - `-1`: no assertion
  pub a: i32,
}

impl Default for Import {
  fn default() -> Self {
    Self {
      n: None,
      s: 0,
      e: 0,
      ss: 0,
      se: 0,
      a: -1,
      d: -1,
    }
  }
}

#[derive(Debug)]
pub struct Export {
  /// name start
  pub s: u32,
  /// name end
  pub e: u32,
  /// local start
  ///
  /// for `export { x as 'external name' } from 'external'`,
  /// it will be -1
  pub ls: i32,
  /// local end
  ///
  /// for `export { x as 'external name' } from 'external'`,
  /// it will be -1
  pub le: i32,
  /// name
  pub n: Option<String>,
  /// local name
  pub ln: Option<String>,
}

impl Default for Export {
  fn default() -> Self {
    Self {
      s: 0,
      e: 0,
      ls: -1,
      le: -1,
      n: None,
      ln: None,
    }
  }
}

#[derive(Debug)]
pub struct ParseResult {
  pub imports: Vec<Import>,
  pub exports: Vec<Export>,
  pub facade: bool,
  pub has_module_syntax: bool,
}

pub struct Visitor<'a> {
  pub source_text: &'a str,
  pub source_text_chars: Vec<char>,
  pub byte_to_char: Vec<usize>,
  pub result: ParseResult,
}

trait AddToExports {
  fn add_to_exports(&self, visitor: &mut Visitor);
}

impl AddToExports for BindingIdentifier {
  fn add_to_exports(&self, visitor: &mut Visitor) {
    visitor.add_export(Export {
      n: Some(self.name.to_string()),
      s: self.span.start,
      e: self.span.end,
      ln: Some(self.name.to_string()),
      ls: self.span.start as i32,
      le: self.span.end as i32,
    });
  }
}

enum FindIndexByCharType {
  Last,
  Next,
}

impl<'a> Visitor<'a> {
  pub fn new(source_text: &'a str) -> Self {
    let mut byte_to_char = vec![0; source_text.len()];

    for (char_index, (byte_index, ch)) in source_text.char_indices().enumerate() {
      byte_to_char[byte_index..byte_index + ch.len_utf8()]
        .iter_mut()
        .for_each(|x| *x = char_index);
    }

    Self {
      source_text,
      source_text_chars: source_text.chars().collect(),
      byte_to_char,
      result: ParseResult {
        imports: vec![],
        exports: vec![],
        facade: false,
        has_module_syntax: false,
      },
    }
  }

  fn add_import(&mut self, import: Import) {
    self.result.imports.push(Import {
      n: import.n,
      s: self.byte_to_char[import.s as usize] as u32,
      e: self.get_char_index(import.e as i32) as u32,
      ss: self.byte_to_char[import.ss as usize] as u32,
      se: self.get_char_index(import.se as i32) as u32,
      a: self.get_char_index(import.a) as i32,
      d: self.get_char_index(import.d) as i32,
    })
  }

  fn add_export(&mut self, export: Export) {
    self.result.exports.push(Export {
      n: export.n,
      ln: export.ln,
      s: self.byte_to_char[export.s as usize] as u32,
      e: self.byte_to_char[export.e as usize] as u32,
      ls: self.get_char_index(export.ls) as i32,
      le: self.get_char_index(export.le) as i32,
    })
  }

  fn get_char_index(&self, byte_index: i32) -> usize {
    if byte_index < 0 || byte_index >= self.source_text.len() as i32 {
      byte_index as usize
    } else {
      self.byte_to_char[byte_index as usize]
    }
  }

  fn adjust_statement_end(&mut self, se: u32) -> u32 {
    let last_ch = self.source_text_chars[self.byte_to_char[se as usize - 1]];

    if last_ch == SINGLE_QUOTE
      || last_ch == QUOTE
      || last_ch == CURLY_BRACE_RIGHT
      || last_ch == BRACKET_RIGHT
    {
      se
    } else {
      se - 1
    }
  }

  fn find_index_by_char(
    &mut self,
    mut current: usize,
    ch: char,
    find_type: FindIndexByCharType,
  ) -> usize {
    while self.source_text_chars[self.byte_to_char[current]] != ch {
      match find_type {
        FindIndexByCharType::Last => current -= 1,
        FindIndexByCharType::Next => current += 1,
      }
    }

    current
  }
}

impl<'a> Visit<'a> for Visitor<'a> {
  fn enter_node(&mut self, kind: AstKind<'a>) {
    match kind {
      AstKind::Program(program) => {
        self.result.facade = program.body.iter().all(|stmt| {
          if matches!(stmt, Statement::ModuleDeclaration(_)) {
            true
          } else if let Statement::ExpressionStatement(expr) = stmt {
            matches!(expr.expression, Expression::ImportExpression(_))
          } else {
            false
          }
        });
      }
      AstKind::ModuleDeclaration(decl) => {
        self.result.has_module_syntax = true;

        match decl {
          // import xx from 'mod' assert { type: 'json' }
          ModuleDeclaration::ImportDeclaration(decl) => {
            let mut import = Import::default();

            // import xx from 'mod' assert { type: 'json' }
            //                 ^^^
            import.n = Some(decl.source.value.to_string());
            import.s = decl.source.span.start + 1;
            import.e = decl.source.span.end - 1;
            import.ss = decl.span.start;
            import.se = self.adjust_statement_end(decl.span.end);

            // import xx from 'mod' assert { type: 'json' }
            //                             ^
            if let Some(attr) = &decl.with_clause {
              import.a = self.find_index_by_char(
                attr.attributes_keyword.span.end as usize,
                CURLY_BRACE_LEFT,
                FindIndexByCharType::Next,
              ) as i32;
            }

            self.add_import(import);
          }
          // export default xxx
          ModuleDeclaration::ExportDefaultDeclaration(decl) => {
            self.result.facade = false;

            let mut export = Export::default();

            // export default function foo() {}
            //        ^^^^^^^
            export.n = Some(decl.exported.name().to_string());
            export.s = decl.exported.span().start;
            export.e = decl.exported.span().end;

            // export default function foo() {}
            //                         ^^^
            match &decl.declaration {
              ExportDefaultDeclarationKind::FunctionDeclaration(fn_decl) => {
                if let Some(id) = &fn_decl.id {
                  export.ln = Some(id.name.to_string());
                  export.ls = id.span.start as i32;
                  export.le = id.span.end as i32;
                }
              }
              ExportDefaultDeclarationKind::ClassDeclaration(class_decl) => {
                if let Some(id) = &class_decl.id {
                  export.ln = Some(id.name.to_string());
                  export.ls = id.span.start as i32;
                  export.le = id.span.end as i32;
                }
              }
              _ => (),
            }

            self.add_export(export);
          }
          // export * as all from 'xxx'
          ModuleDeclaration::ExportAllDeclaration(decl) => {
            let mut import = Import::default();

            import.n = Some(decl.source.value.to_string());
            import.ss = decl.span.start;
            import.se = self.adjust_statement_end(decl.span.end);
            import.s = decl.source.span.start + 1;
            import.e = decl.source.span.end - 1;

            self.add_import(import);

            if let Some(exported) = &decl.exported {
              let mut export = Export::default();

              export.n = Some(exported.name().to_string());
              export.s = exported.span().start;
              export.e = exported.span().end;

              self.add_export(export);
            }
          }
          // export { xxx } from 'mod';
          // export const a = 1;
          // export { b as c };
          ModuleDeclaration::ExportNamedDeclaration(decl) => {
            let mut has_import = false;

            // export { xxx } from 'mod';
            //                      ^^^
            if let Some(source) = &decl.source {
              has_import = true;

              let mut import = Import::default();

              import.n = Some(source.value.to_string());
              import.ss = decl.span.start;
              import.se = self.adjust_statement_end(decl.span.end);
              import.s = source.span.start + 1;
              import.e = source.span.end - 1;

              self.add_import(import);
            }

            // export const a = 1;
            //              ^
            // ----------------------------
            // export function foo() {}
            //                 ^^^
            // ----------------------------
            // export class Bar {}
            //              ^^^
            if let Some(inner_decl) = &decl.declaration {
              self.result.facade = false;

              match inner_decl {
                // export const a = 1;
                //              ^
                Declaration::VariableDeclaration(var_decl) => {
                  if var_decl.declarations.len() != 0 {
                    if let BindingPatternKind::BindingIdentifier(id) =
                      &var_decl.declarations[0].id.kind
                    {
                      id.add_to_exports(self);
                    }
                  }
                }
                // export function foo() {}
                //                 ^^^
                Declaration::FunctionDeclaration(fn_decl) => {
                  if let Some(id) = &fn_decl.id {
                    id.add_to_exports(self);
                  }
                }
                // export class Bar {}
                //              ^^^
                Declaration::ClassDeclaration(class_decl) => {
                  if let Some(id) = &class_decl.id {
                    id.add_to_exports(self);
                  }
                }
                Declaration::TSEnumDeclaration(enum_decl) => {
                  enum_decl.id.add_to_exports(self);
                }
                Declaration::TSInterfaceDeclaration(interface_decl) => {
                  interface_decl.id.add_to_exports(self);
                }
                Declaration::TSTypeAliasDeclaration(type_alias_decl) => {
                  type_alias_decl.id.add_to_exports(self);
                }
                _ => (),
              }
            }

            // export { c as d }
            //          ^    ^
            decl.specifiers.iter().for_each(|specifier| {
              let mut export = Export::default();

              export.n = Some(specifier.exported.name().to_string());
              export.s = specifier.exported.span().start;
              export.e = specifier.exported.span().end;

              if !has_import {
                export.ln = Some(specifier.local.name().to_string());
                export.ls = specifier.local.span().start as i32;
                export.le = specifier.local.span().end as i32;
              }

              self.add_export(export);
            })
          }
          _ => (),
        };
      }
      // import('xx', { assert: { type: 'json' } })
      AstKind::ImportExpression(expr) => {
        let mut import = Import::default();

        import.ss = expr.span.start;
        import.se = self.adjust_statement_end(expr.span.end);
        import.s = expr.source.span().start;
        import.e = expr.source.span().end;

        // import('xx', { assert: { type: 'json' } })
        //       ^
        import.d =
          self.find_index_by_char(import.s as usize, BRACKET_LEFT, FindIndexByCharType::Last)
            as i32;

        // import('xx', { assert: { type: 'json' } })
        //         ^^
        if let Expression::StringLiteral(s) = &expr.source {
          import.n = Some(s.value.to_string());
        } else {
          self.result.facade = false;
        }

        if expr.arguments.len() != 0 {
          self.result.facade = false;

          if let Expression::ObjectExpression(obj_expr) = &expr.arguments[0] {
            // import('xx', { assert: { type: 'json' } })
            //              ^
            import.a = obj_expr.span.start as i32;
          }
        }

        self.add_import(import);
      }
      // import.meta
      AstKind::MetaProperty(meta) => {
        self.result.has_module_syntax = true;

        let mut import = Import::default();

        import.d = -2;
        import.ss = meta.span.start;
        import.se = meta.span.end;
        import.s = meta.span.start;
        import.e = meta.span.end;

        self.add_import(import);
      }
      _ => (),
    };
  }
}
