import { Token, TokenType } from '../token';

type ScanResult = { token: Token; newPos: number };

export function scanString(line: string, pos: number, lineNum: number): ScanResult | null {
  if (!isStringStart(line[pos])) return null;
  pos++;
  let literal = '';
  while (pos < line.length && !isStringEnd(line[pos])) {
    if (isEscapeChar(line[pos])) {
      pos++;
      if (pos < line.length) {
        switch (line[pos]) {
          case 'n':  literal += '\n'; break;
          case 't':  literal += '\t'; break;
          case '\\': literal += '\\'; break;
          case '"':  literal += '"';  break;
          default:   literal += '\\' + line[pos];
        }
        pos++;
      }
    } else {
      literal += line[pos];
      pos++;
    }
  }
  if (pos < line.length) pos++;
  return { token: { type: TokenType.STRING, literal, line: lineNum }, newPos: pos };
}

function isStringStart(ch: string): boolean { return ch === '"'; }
function isStringEnd(ch: string):   boolean { return ch === '"'; }
function isEscapeChar(ch: string):  boolean { return ch === '\\'; }
