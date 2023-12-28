import fs from 'fs';
import { expect, test } from "vitest";
import { parseAsync } from "..";
import { beforeEach } from 'node:test';
import { init, parse } from 'es-module-lexer';

beforeEach(async () => {
  await init;
})

test('parse jsx', async () => {
  const source = fs.readFileSync('__test__/fixtures/Helmet.tsx', 'utf-8');
  const output = await parseAsync(source, 'Helmet.tsx');

  expect(output).toMatchSnapshot();
});