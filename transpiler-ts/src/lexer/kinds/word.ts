import { Token, TokenType } from '../token';

type ScanResult = { token: Token; newPos: number };

export function scanWord(line: string, pos: number, lineNum: number): ScanResult | null {
  if (!isIdentStart(line[pos])) return null;
  const start = pos;
  while (pos < line.length && isIdentContinue(line[pos])) pos++;
  return { token: toKeywordToken(line.slice(start, pos), lineNum), newPos: pos };
}

function isIdentStart(ch: string):    boolean { return /[a-zA-Z_]/.test(ch); }
function isIdentContinue(ch: string): boolean { return /[a-zA-Z0-9_]/.test(ch); }

function toKeywordToken(word: string, line: number): Token {
  switch (word) {
    case 'fn':     return { type: TokenType.KW_FN,     literal: word, line };
    case 'as':     return { type: TokenType.KW_AS,     literal: word, line };
    case 'return': return { type: TokenType.KW_RETURN, literal: word, line };
    default:       return { type: TokenType.IDENT,     literal: word, line };
  }
}
