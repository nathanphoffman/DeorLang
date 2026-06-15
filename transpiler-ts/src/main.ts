import { readFileSync, writeFileSync } from 'fs';
import { Lexer } from './lexer/lexer';
import { Parser } from './parser/parser';
import { Generator } from './codegen/codegen';

const [,, inputPath, outputPath] = process.argv;

if (!inputPath || !outputPath) {
  console.error('usage: deor-ts <input.deor> <output.rs>');
  process.exit(1);
}

const fileAsText = readFileSync(inputPath, 'utf8');

const lexer  = new Lexer(fileAsText);
const parser = new Parser(lexer);
const prog   = parser.parseProgram();

const gen      = new Generator();
const rustCode = gen.generate(prog);

writeFileSync(outputPath, rustCode);
console.log(`transpiled ${inputPath} -> ${outputPath}`);
