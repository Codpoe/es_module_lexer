/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Import {
  n?: string
  s: number
  e: number
  ss: number
  se: number
  d: number
  a: number
}
export interface Export {
  s: number
  e: number
  ls: number
  le: number
  n?: string
  ln?: string
}
export interface Output {
  imports: Array<Import>
  exports: Array<Export>
  facade: boolean
  hasModuleSyntax: boolean
}
export function parse(sourceText: string, filePath: string): Output
export function parseAsync(sourceText: string, filePath: string): Promise<Output>
export interface ParseMultipleInput {
  sourceText: string
  filePath: string
}
export function parseMultiple(inputs: Array<ParseMultipleInput>): Array<Output>
export function parseMultipleAsync(inputs: Array<ParseMultipleInput>): Promise<Output[]>
