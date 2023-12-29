mod constants;
mod visitor;

use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;
use rayon::prelude::*;
use std::{collections::HashMap, path::Path};
use visitor::Visitor;
pub use visitor::{Export, Import, ParseResult};

/// Parses a source text and returns `ParseResult` or an `Err` with parsing errors.
///
/// The `parse` function takes a source text and a file path as input. It uses the `Parser` from the `Allocator`
/// to parse the source text. The file path is used to determine the `SourceType`.
///
/// Once parsed, if there are any errors during parsing, they are collected and formatted with their source
/// codes and returned as a vector of strings in an `Err`. The errors are related to the syntax and semantics
/// of the source text.
///
/// If no errors exist, a `Visitor` is used to visit the parsed program and return a `ParseResult` wrapped
/// in an `Ok`.
///
/// # Arguments
///
/// * `source_text`: A &str representing the source text to be parsed.
/// * `file_path`: A &str representing the file path of the source text. This assists in determining the SourceType.
///
/// # Returns
///
/// Returns a `Result<ParseResult, Vec<String>>`.
///
/// In the case of successful parsing, it yields `Ok(ParseResult)`.
/// If there are parsing errors, it yields `Err`, with a vector of strings, each indicating a parsing error.
///
/// # Example
///
/// ```
/// use es_module_lexer::*;
///
/// let source_text = "import something from 'mod'";
/// let file_path = "/path/to/the/file.js";
///
/// let result = parse(source_text, file_path);
/// ```
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

/// Processes an array slice of `ParseMultipleInput` instances by parsing each one and collecting the results into a `HashMap`.
///
/// The `parse_multiple` function takes an array slice of `ParseMultipleInput` instances and processes each one in parallel using the `par_iter()` method provided by the rayon crate.
/// Each `ParseMultipleInput` instance is parsed using its `source_text` and `file_path` via the `parse` function.
///
/// The resulting `HashMap` uses the `file_path` as key and the result of the `parse` function as value.
/// If the `parse` function successfully parses the `source_text`, it yields `Ok(ParseResult)`.
/// Otherwise, it yields `Err` with a vector of strings indicating errors encountered during the parsing of `source_text`.
///
/// # Arguments
///
/// * `inputs`: An array slice of `ParseMultipleInput` instances.
///
/// # Returns
///
/// A `HashMap<String, Result<ParseResult, Vec<String>>>` where:
/// - The keys of the `HashMap` are the `file_path` strings.
/// - The values of the `HashMap` are a `Result` that, if parsing was successful, yields a `ParseResult`, otherwise a vector of strings indicating parsing errors.
///
/// # Example
///
/// ```
/// use es_module_lexer::*;
///
/// let inputs = vec![
///     ParseMultipleInput {
///         source_text: "import something from 'mod'",
///         file_path: "a.js",
///     },
///     ParseMultipleInput {
///         source_text: "import other from 'other'",
///         file_path: "b.js",
///     },
/// ];
///
/// let result = parse_multiple(&inputs);
/// ```
pub fn parse_multiple(
  inputs: &[ParseMultipleInput],
) -> HashMap<String, Result<ParseResult, Vec<String>>> {
  inputs
    .par_iter()
    .map(|input| {
      (
        input.file_path.to_string(),
        parse(input.source_text, input.file_path),
      )
    })
    .collect()
}
