# @rust-it/es-module-lexer

A Rust version of [es-module-lexer](https://github.com/guybedford/es-module-lexer), 
and it supports parsing `jsx` / `ts` / `tsx` out of the box. Powered by [oxc](https://github.com/oxc-project/oxc).

## Install

```sh
npm install @rust-it/es-module-lexer
```

## Usage

sync:

```ts
import { parse, parseAsync } from '@rust-it/es-module-lexer';

const source = 'export { foo } from "./foo.ts";';

// sync
const { imports, exports } = parse(source, 'index.ts');
// async
const { imports, exports } = await parseAsync(source, 'index.ts');
```

multiple files:

```ts
import { parseMultiple, parseMultipleAsync } from '@rust-it/es-module-lexer';

const inputs = [
  {
    sourceText: 'export { foo } from "./foo.ts";',
    filePath: 'index.ts',
  },
  {
    sourceText: 'import bar from "./bar.ts";',
    filePath: 'other.ts',
  },
];

// sync
const { imports, exports } = parseMultiple(inputs);
// async
const { imports, exports } = await parseMultipleAsync(inputs);
```

> Check out [es-module-lexer](https://github.com/guybedford/es-module-lexer) for details of the parse results.

## Benchmark

Parse files in [__test__/samples](https://github.com/codpoe/es_module_lexer/tree/master/__test__/samples).

```sh
 ✓ __test__/index.bench.ts (9) 10778ms
     name                                     hz      min      max     mean
   · es-module-lexer                     36.7348  26.5142  27.7018  27.2221   fastest
   · @rust-it/es-module-lexer                  8.4034   114.91   124.62   119.00  
   · @rust-it/es-module-lexer async           24.6149  38.4170  44.5475  40.6257  
   · @rust-it/es-module-lexer multiple        26.4290  35.4089  41.0913  37.8372  
   · @rust-it/es-module-lexer multiple async  26.5319  35.8229  39.6239  37.6905  
   · rs-module-lexer                      5.7273   172.67   177.16   174.60   slowest
   · rs-module-lexer async               15.8369  59.5024  69.4358  63.1438  
   · rs-module-lexer multiple            18.3675  51.5449  56.9917  54.4440  
   · rs-module-lexer multiple async      17.8458  51.1092  64.5350  56.0355  


 BENCH  Summary

  es-module-lexer - __test__/index.bench.ts > 
    1.38x faster than @rust-it/es-module-lexer multiple async
    1.39x faster than @rust-it/es-module-lexer multiple
    1.49x faster than @rust-it/es-module-lexer async
    2.00x faster than rs-module-lexer multiple
    2.06x faster than rs-module-lexer multiple async
    2.32x faster than rs-module-lexer async
    4.37x faster than @rust-it/es-module-lexer
    6.41x faster than rs-module-lexer
```
