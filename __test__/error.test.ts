import { expect, test } from "vitest";
import { parse, parseAsync, parseMultiple, parseMultipleAsync } from "..";

test('parse error', () => {
  expect(() => parse("var a number = 1", 'index.js')).toThrow('Expected a semicolon or an implicit semicolon after a statement, but found none');
});

test('parse async error', () => {
  expect(() => parseAsync("var a number = 1", 'index.js')).rejects.toThrow('Expected a semicolon or an implicit semicolon after a statement, but found none');
});

test('parse multiple error', async () => {
  expect(() => parseMultiple([
    {
      sourceText: "var a number = 1",
      filePath: 'a.js',
    },
    {
      sourceText: "var b string = 'b'",
      filePath: 'b.js',
    },
  ])).toThrow('Expected a semicolon or an implicit semicolon after a statement, but found none');
});

test('parse multiple async error', () => {
  const rejects = expect(() => parseMultipleAsync([
    {
      sourceText: "var a number = 1",
      filePath: 'a.js',
    },
    {
      sourceText: "var b string = 'b'",
      filePath: 'b.js',
    },
  ])).rejects;

  rejects.toThrow(/\s+a\.js:[\s\S]+?Expected a semicolon or an implicit semicolon after a statement, but found none/);
  rejects.toThrow(/\s+b\.js:[\s\S]+?Expected a semicolon or an implicit semicolon after a statement, but found none/);
});

