import { Token, TokenType } from '../token';

type ScanResult = { token: Token; newPos: number };

export function scanInt(line: string, pos: number, lineNum: number): ScanResult | null {
  if (!isDigit(line[pos])) return null;
  const start = pos;
  while (pos < line.length && isDigitOrUnderscore(line[pos])) pos++;
  return { token: { type: TokenType.INT, literal: line.slice(start, pos), line: lineNum }, newPos: pos };
}

function isDigit(ch: string): boolean { return /\d/.test(ch); }
function isDigitOrUnderscore(ch: string): boolean { return /[\d_]/.test(ch); }
