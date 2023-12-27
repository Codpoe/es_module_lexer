import fs from 'fs';
import { bench, beforeAll } from 'vitest';
import { init, parse as parseByEsModuleLexer } from 'es-module-lexer';
import { parse as parseByRsModuleLexer, parseAsync as parseAsyncByRsModuleLexer } from 'rs-module-lexer';
import { parse as parseByOxc, parseAsync as parseAsyncByOxc, parseMultiple as parseMultipleByOxc, parseMultipleAsync as parseMultipleAsyncByOxc } from '..';

const files = fs.readdirSync('./__test__/samples')
	.map(f => `__test__/samples/${f}`)
	.filter(x => x.endsWith('.js'))
	.map(file => {
		const source = fs.readFileSync(file, 'utf-8');
		return {
			filename: file,
			code: source,
		};
	});

beforeAll(async () => {
  await init;
})

bench('es-module-lexer', async () => {
  files.forEach(f => {
    parseByEsModuleLexer(f.code, f.filename);
  });
});

bench('@rust-it/es-module-lexer', () => {
  files.forEach(f => {
    parseByOxc(f.code, f.filename);
  });
});

bench('@rust-it/es-module-lexer async', async () => {
  await Promise.all(files.map(async f => {
    await parseAsyncByOxc(f.code, f.filename);
  }));
});

bench('@rust-it/es-module-lexer multiple', () => {
  parseMultipleByOxc(files.map(f => ({ sourceText: f.code, filePath: f.filename })));
});

bench('@rust-it/es-module-lexer multiple async', async () => {
  await parseMultipleAsyncByOxc(files.map(f => ({ sourceText: f.code, filePath: f.filename })));
});

bench('rs-module-lexer', () => {
  files.forEach(f => {
    parseByRsModuleLexer({ input: [f] });
  });
});

bench('rs-module-lexer async', async () => {
  await Promise.all(files.map(async f => {
    await parseAsyncByRsModuleLexer({ input: [f] });
  }));
});

bench('rs-module-lexer multiple', () => {
    parseByRsModuleLexer({ input: files });
});

bench('rs-module-lexer multiple async', async () => {
  await parseAsyncByRsModuleLexer({ input: files });
});
