use es_module_lexer::parse;

#[test]
fn test_has_module_syntax() {
  let source_text = r#"
import { name } from 'mod'
"#;

  let file_path = "index.ts";
  let result = parse(source_text, file_path);

  assert!(result.unwrap().has_module_syntax);
}

#[test]
fn test_has_module_syntax_meta() {
  let source_text = r#"import.meta"#;

  let file_path = "index.ts";
  let result = parse(source_text, file_path);

  assert!(result.unwrap().has_module_syntax);
}

#[test]
fn test_no_module_syntax() {
  let source_text = r#"
import('./foo')
"#;

  let file_path = "index.ts";
  let result = parse(source_text, file_path);

  assert!(!result.unwrap().has_module_syntax);
}
