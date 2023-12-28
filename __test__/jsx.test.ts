import fs from 'fs';
import { expect, test } from "vitest";
import { parseAsync } from "..";
import { beforeEach } from 'node:test';

test('parse jsx', async () => {
  const source = fs.readFileSync('__test__/fixtures/Helmet.tsx', 'utf-8');
  const output = await parseAsync(source, 'Helmet.tsx');

  expect(output).toEqual({
    "exports": [
      {
        "e": 694,
        "le": -1,
        "ls": -1,
        "n": "HelmetData",
        "s": 684,
      },
      {
        "e": 752,
        "le": -1,
        "ls": -1,
        "n": "HelmetProvider",
        "s": 738,
      },
      {
        "e": 832,
        "le": 832,
        "ln": "Helmet",
        "ls": 826,
        "n": "Helmet",
        "s": 826,
      },
    ],
    "facade": false,
    "hasModuleSyntax": true,
    "imports": [
      {
        "a": -1,
        "d": -1,
        "e": 132,
        "n": "react",
        "s": 127,
        "se": 133,
        "ss": 62,
      },
      {
        "a": -1,
        "d": -1,
        "e": 174,
        "n": "react",
        "s": 169,
        "se": 175,
        "ss": 135,
      },
      {
        "a": -1,
        "d": -1,
        "e": 220,
        "n": "react-fast-compare",
        "s": 202,
        "se": 221,
        "ss": 177,
      },
      {
        "a": -1,
        "d": -1,
        "e": 255,
        "n": "invariant",
        "s": 246,
        "se": 256,
        "ss": 223,
      },
      {
        "a": -1,
        "d": -1,
        "e": 294,
        "n": "./Provider",
        "s": 284,
        "se": 295,
        "ss": 259,
      },
      {
        "a": -1,
        "d": -1,
        "e": 346,
        "n": "./HelmetData",
        "s": 334,
        "se": 347,
        "ss": 297,
      },
      {
        "a": -1,
        "d": -1,
        "e": 385,
        "n": "./HelmetData",
        "s": 373,
        "se": 386,
        "ss": 349,
      },
      {
        "a": -1,
        "d": -1,
        "e": 444,
        "n": "./Dispatcher",
        "s": 432,
        "se": 445,
        "ss": 388,
      },
      {
        "a": -1,
        "d": -1,
        "e": 483,
        "n": "./Dispatcher",
        "s": 471,
        "se": 484,
        "ss": 447,
      },
      {
        "a": -1,
        "d": -1,
        "e": 518,
        "n": "./utils",
        "s": 511,
        "se": 519,
        "ss": 486,
      },
      {
        "a": -1,
        "d": -1,
        "e": 590,
        "n": "./constants",
        "s": 579,
        "se": 591,
        "ss": 521,
      },
      {
        "a": -1,
        "d": -1,
        "e": 634,
        "n": "./types",
        "s": 627,
        "se": 635,
        "ss": 593,
      },
      {
        "a": -1,
        "d": -1,
        "e": 660,
        "n": "./types",
        "s": 653,
        "se": 661,
        "ss": 638,
      },
      {
        "a": -1,
        "d": -1,
        "e": 715,
        "n": "./HelmetData",
        "s": 703,
        "se": 716,
        "ss": 664,
      },
      {
        "a": -1,
        "d": -1,
        "e": 771,
        "n": "./Provider",
        "s": 761,
        "se": 772,
        "ss": 718,
      },
    ],
  });
});