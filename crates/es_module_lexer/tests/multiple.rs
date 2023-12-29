use es_module_lexer::{parse_multiple, ParseMultipleInput};

#[test]
fn test_parse_multiple() {
  let source_text = r#"
import { name } from 'mod'
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

  let result = parse_multiple(&[ParseMultipleInput {
    source_text: source_text,
    file_path,
  }]);

  assert_eq!(result.len(), 1);
  assert!(result.contains_key(file_path));
}
