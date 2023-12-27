mod constants;
mod visitor;

use std::path::Path;

use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;
use rayon::prelude::*;
use visitor::Visitor;
pub use visitor::{Export, Import, ParseResult};

pub fn parse(source_text: &str, file_path: &str) -> Result<ParseResult, Vec<String>> {
  let allocator = Allocator::default();
  let source_type = SourceType::from_path(Path::new(file_path)).unwrap();
  let res = Parser::new(&allocator, source_text, source_type).parse();

  if res.errors.len() > 0 {
    let source = source_text.to_string();
    let errors: Vec<String> = res
      .errors
      .into_iter()
      .map(|error| error.with_source_code(source.clone()))
      .map(|error| format!("{error:?}"))
      .collect();

    return Err(errors);
  }

  let mut visitor = Visitor::new(source_text);
  visitor.visit_program(&res.program);

  Ok(visitor.result)
}

#[derive(Debug)]
pub struct ParseMultipleInput<'a> {
  pub source_text: &'a str,
  pub file_path: &'a str,
}

pub fn parse_multiple(inputs: &[ParseMultipleInput]) -> Result<Vec<ParseResult>, Vec<String>> {
  inputs
    .par_iter()
    .map(|input| parse(input.source_text, input.file_path))
    .collect()
}
