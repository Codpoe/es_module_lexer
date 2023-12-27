use es_module_lexer::parse;

#[test]
fn test_parse_errors() {
  let source_text = r#"error import { name } from 'mod'
import json from './json.json' assert { type: 'json' }
export var p = 5;
export function q () {

};
export { x as 'external name' } from 'external';

// Comments provided to demonstrate edge cases
import /*comment!*/ (  'asdf', { assert: { type: 'json' }});
import /*comment!*/.meta.asdf;
"#;

  let file_path = "index.ts";
  let result = parse(source_text, file_path);

  assert!(result.is_err());
}
