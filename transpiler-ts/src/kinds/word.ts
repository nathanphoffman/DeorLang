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
    case 'fn':       return { type: TokenType.KW_FN,       literal: word, line };
    case 'as':       return { type: TokenType.KW_AS,       literal: word, line };
    case 'return':   return { type: TokenType.KW_RETURN,   literal: word, line };
    case 'if':       return { type: TokenType.KW_IF,       literal: word, line };
    case 'else':     return { type: TokenType.KW_ELSE,     literal: word, line };
    case 'for':      return { type: TokenType.KW_FOR,      literal: word, line };
    case 'in':       return { type: TokenType.KW_IN,       literal: word, line };
    case 'break':    return { type: TokenType.KW_BREAK,    literal: word, line };
    case 'continue': return { type: TokenType.KW_CONTINUE, literal: word, line };
    case 'and':      return { type: TokenType.KW_AND,      literal: word, line };
    case 'or':       return { type: TokenType.KW_OR,       literal: word, line };
    case 'not':      return { type: TokenType.KW_NOT,      literal: word, line };
    case 'is':       return { type: TokenType.KW_IS,       literal: word, line };
    case 'true':     return { type: TokenType.KW_TRUE,     literal: word, line };
    case 'false':    return { type: TokenType.KW_FALSE,    literal: word, line };
    case 'none':     return { type: TokenType.KW_NONE,     literal: word, line };
    case 'struct':   return { type: TokenType.KW_STRUCT,   literal: word, line };
    case 'shape':    return { type: TokenType.KW_SHAPE,    literal: word, line };
    case 'list':     return { type: TokenType.KW_LIST,     literal: word, line };
    case 'of':       return { type: TokenType.KW_OF,       literal: word, line };
    case 'insert':   return { type: TokenType.KW_INSERT,   literal: word, line };
    case 'enum':     return { type: TokenType.KW_ENUM,     literal: word, line };
    default:       return { type: TokenType.IDENT,     literal: word, line };
  }
}
