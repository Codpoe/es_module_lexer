use es_module_lexer::parse;

#[test]
fn test_parse() {
  let source_text = r#"
export enum Fruit {
  Apple = 'apple',
  Banana = 'banana',
  Orange = 'orange',
  Pear = 'pear',
}

export interface Opts {
  name: string;
  color: string;
  count: number;
  enabled: boolean;
}

export type Key = 'a' | 'b' | 'c';
"#;

  let file_path = "index.ts";
  let result = parse(source_text, file_path);

  assert!(result.is_ok());

  let result = result.unwrap();

  assert_eq!(result.imports.len(), 0);
  assert_eq!(result.exports.len(), 3);
  assert_eq!(result.exports[0].n.clone().unwrap(), "Fruit");
  assert_eq!(result.exports[0].s, 13);
  assert_eq!(result.exports[0].e, 18);
  assert_eq!(result.exports[1].n.clone().unwrap(), "Opts");
  assert_eq!(result.exports[2].n.clone().unwrap(), "Key");

  assert!(!result.facade);
  assert!(result.has_module_syntax);
}
