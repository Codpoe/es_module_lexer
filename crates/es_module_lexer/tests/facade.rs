use es_module_lexer::parse;

#[test]
fn test_facade() {
  let source_text = r#"
import { name } from 'mod'
import json from './json.json' assert { type: 'json' }
import { a } from './a';
        
export { a as b }
export { x as 'external name' } from 'external';
"#;

  let file_path = "index.ts";
  let result = parse(source_text, file_path);

  assert!(result.unwrap().facade);
}
