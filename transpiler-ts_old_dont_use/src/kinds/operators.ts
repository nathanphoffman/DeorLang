import { Token, TokenType } from '../token';

type ScanResult = { token: Token; newPos: number };

export function scanOperator(line: string, pos: number, lineNum: number): ScanResult | null {
  const ch = line[pos];

  // two-character operators must be checked before their single-char prefix
  if (ch === '>' && line[pos + 1] === '=') return { token: { type: TokenType.GTE, literal: '>=', line: lineNum }, newPos: pos + 2 };
  if (ch === '<' && line[pos + 1] === '=') return { token: { type: TokenType.LTE, literal: '<=', line: lineNum }, newPos: pos + 2 };

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
    case '%': return TokenType.PERCENT;
    case '=': return TokenType.EQUALS;
    case '>': return TokenType.GT;
    case '<': return TokenType.LT;
    default:  return null;
  }
}
