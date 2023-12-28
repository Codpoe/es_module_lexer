import fs from 'fs';
import { test, beforeAll, expect } from 'vitest';
import { init, parse as parseByLexer } from 'es-module-lexer';
import { parse as parseByOxc, parseMultiple as parseMultipleByOxc } from '..';

beforeAll(async () => {
  await init;
});

test('parse simple', () => {
  const sourceText = `
import { name } from 'mod\\u1011';
import json from './json.json' assert { type: 'json' }
export var p = 5;
export function q () {

};
export { x as 'external name' } from 'external';

// Comments provided to demonstrate edge cases
import /*comment!*/ (  'asdf', { assert: { type: 'json' }});
import /*comment!*/.meta.asdf;

export default function foo() {}
`;

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
  expect(outputFromOxc.facade).toBe(false);
  expect(outputFromOxc.hasModuleSyntax).toBe(true);
});

test('parse multiple', () => {
  const sourceText = `
import { name } from 'mod\\u1011';
import json from './json.json' assert { type: 'json' }
export var p = 5;
export function q () {

};
export { x as 'external name' } from 'external';

// Comments provided to demonstrate edge cases
import /*comment!*/ (  'asdf', { assert: { type: 'json' }});
import /*comment!*/.meta.asdf;
`;

  const outputFromOxc = parseMultipleByOxc([{ sourceText, filePath: 'index.js' }, { sourceText, filePath: 'index.js' }]);
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc[0]).toEqual({ imports, exports, facade, hasModuleSyntax });
});

test('facade', () => {
  const sourceText = `
import { name } from 'mod\\u1011';
import json from './json.json' assert { type: 'json' }
export { x as 'external name' } from 'external';
`;

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
  expect(outputFromOxc.facade).toBe(true);
});

test('has module syntax', () => {
  const sourceText = `import.meta`;

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
  expect(outputFromOxc.hasModuleSyntax).toBe(true);
});

test('no module syntax: dynamic import', () => {
  const sourceText = `import('./foo', { assert: { type: 'json' } })`;

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
  expect(outputFromOxc.hasModuleSyntax).toBe(false);
});

test('parse angular.js', () => {
  const sourceText = fs.readFileSync('__test__/samples/angular.js', 'utf-8');

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
});

test('parse angular.min.js', () => {
  const sourceText = fs.readFileSync('__test__/samples/angular.min.js', 'utf-8');

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
});

test('parse d3.js', () => {
  const sourceText = fs.readFileSync('__test__/samples/d3.js', 'utf-8');

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
});

test('parse d3.min.js', () => {
  const sourceText = fs.readFileSync('__test__/samples/d3.min.js', 'utf-8');

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
});

test('parse magic-string.js', () => {
  const sourceText = fs.readFileSync('__test__/samples/magic-string.js', 'utf-8');

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
});

test('parse magic-string.min.js', () => {
  const sourceText = fs.readFileSync('__test__/samples/magic-string.min.js', 'utf-8');

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
});

test('parse rollup.js', () => {
  const sourceText = fs.readFileSync('__test__/samples/rollup.js', 'utf-8');

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
});

test('parse rollup.min.js', () => {
  const sourceText = fs.readFileSync('__test__/samples/rollup.min.js', 'utf-8');

  const outputFromOxc = parseByOxc(sourceText, 'index.js');
  const [imports, exports, facade, hasModuleSyntax] = parseByLexer(sourceText, 'index.js');

  expect(outputFromOxc).toEqual({ imports, exports, facade, hasModuleSyntax });
});