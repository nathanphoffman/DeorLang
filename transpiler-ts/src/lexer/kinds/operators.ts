import { Token, TokenType } from '../token';

type ScanResult = { token: Token; newPos: number };

export function scanOperator(line: string, pos: number, lineNum: number): ScanResult | null {
  const ch = line[pos];
  const type = operatorType(ch);
  if (type === null) return null;
  return { token: { type, literal: ch, line: lineNum }, newPos: pos + 1 };
}

function operatorType(ch: string): TokenType | null {
  switch (ch) {
    case '+': return TokenType.PLUS;
    case '-': return TokenType.MINUS;
    case '*': return TokenType.STAR;
    case '/': return TokenType.SLASH;
    case '=': return TokenType.EQUALS;
    default:  return null;
  }
}
