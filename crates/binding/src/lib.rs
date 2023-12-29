#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::collections::HashMap;

use es_module_lexer::{
  parse as parse_es_module, parse_multiple as parse_multiple_es_module, ParseResult,
};
use napi::{bindgen_prelude::AsyncTask, Error, Task};

#[napi(object)]
pub struct Import {
  pub n: Option<String>,
  pub s: u32,
  pub e: u32,
  pub ss: u32,
  pub se: u32,
  pub d: i32,
  pub a: i32,
}

impl From<es_module_lexer::Import> for Import {
  fn from(value: es_module_lexer::Import) -> Self {
    Self {
      n: value.n,
      s: value.s,
      e: value.e,
      ss: value.ss,
      se: value.se,
      d: value.d,
      a: value.a,
    }
  }
}

#[napi(object)]
pub struct Export {
  pub s: u32,
  pub e: u32,
  pub ls: i32,
  pub le: i32,
  pub n: Option<String>,
  pub ln: Option<String>,
}

impl From<es_module_lexer::Export> for Export {
  fn from(value: es_module_lexer::Export) -> Self {
    Self {
      s: value.s,
      e: value.e,
      ls: value.ls,
      le: value.le,
      n: value.n,
      ln: value.ln,
    }
  }
}

#[napi(object)]
pub struct Output {
  pub imports: Vec<Import>,
  pub exports: Vec<Export>,
  pub facade: bool,
  pub has_module_syntax: bool,
}

impl From<ParseResult> for Output {
  fn from(value: ParseResult) -> Self {
    Self {
      imports: value.imports.into_iter().map(|x| x.into()).collect(),
      exports: value.exports.into_iter().map(|x| x.into()).collect(),
      facade: value.facade,
      has_module_syntax: value.has_module_syntax,
    }
  }
}

#[napi]
pub fn parse(source_text: String, file_path: String) -> Result<Output, Error> {
  match parse_es_module(&source_text, &file_path) {
    Ok(value) => Ok(value.into()),
    Err(errors) => Err(Error::from_reason(format!("\n{}", errors.join("\n")))),
  }
}

pub struct ParseTask {
  source_text: String,
  file_path: String,
}

impl Task for ParseTask {
  type Output = Output;
  type JsValue = Output;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    parse(self.source_text.clone(), self.file_path.clone())
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }

  fn reject(&mut self, _env: napi::Env, err: Error) -> napi::Result<Self::JsValue> {
    Err(err)
  }
}

#[napi(ts_return_type = "Promise<Output>")]
pub fn parse_async(source_text: String, file_path: String) -> AsyncTask<ParseTask> {
  AsyncTask::new(ParseTask {
    source_text,
    file_path,
  })
}

#[derive(Clone)]
#[napi(object)]
pub struct ParseMultipleInput {
  pub source_text: String,
  pub file_path: String,
}

#[napi]
pub fn parse_multiple(inputs: Vec<ParseMultipleInput>) -> Result<HashMap<String, Output>, Error> {
  let results = parse_multiple_es_module(
    &inputs
      .iter()
      .map(move |input| es_module_lexer::ParseMultipleInput {
        source_text: &input.source_text,
        file_path: &input.file_path,
      })
      .collect::<Vec<es_module_lexer::ParseMultipleInput>>(),
  );

  let mut outputs: HashMap<String, Output> = HashMap::new();
  let mut errors: Vec<String> = Vec::new();

  for (file_path, result) in results {
    match result {
      Ok(value) => {
        outputs.insert(file_path, value.into());
      }
      Err(errs) => {
        errs
          .into_iter()
          .map(|error| format!("\n{file_path}:\n{error}"))
          .for_each(|err| errors.push(err));
      }
    }
  }

  if errors.len() > 0 {
    return Err(Error::from_reason(errors.join("")));
  }

  Ok(outputs)
}

pub struct ParseMultipleTask {
  inputs: Vec<ParseMultipleInput>,
}

impl Task for ParseMultipleTask {
  type Output = HashMap<String, Output>;
  type JsValue = HashMap<String, Output>;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    parse_multiple(self.inputs.clone())
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }

  fn reject(&mut self, _env: napi::Env, err: Error) -> napi::Result<Self::JsValue> {
    Err(err)
  }
}

#[napi(ts_return_type = "Promise<Record<string, Output>>")]
pub fn parse_multiple_async(inputs: Vec<ParseMultipleInput>) -> AsyncTask<ParseMultipleTask> {
  AsyncTask::new(ParseMultipleTask { inputs })
}
