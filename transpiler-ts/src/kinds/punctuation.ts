import { Token, TokenType } from '../token';

type ScanResult = { token: Token; newPos: number };

export function scanPunctuation(line: string, pos: number, lineNum: number): ScanResult {
  const ch = line[pos];
  const type = punctuationType(ch);
  return { token: { type, literal: ch, line: lineNum }, newPos: pos + 1 };
}

function punctuationType(ch: string): TokenType {
  switch (ch) {
    case '(': return TokenType.LPAREN;
    case ')': return TokenType.RPAREN;
    case ',': return TokenType.COMMA;
    default:  return TokenType.ILLEGAL;
  }
}
