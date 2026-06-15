import { Token, TokenType } from './token';

export class Lexer {
  private tokens: Token[] = [];
  private pos = 0;

  constructor(input: string) {
    this.tokenize(input);
  }

  nextToken(): Token {
    if (this.pos >= this.tokens.length) {
      return { type: TokenType.EOF, literal: '', line: 0 };
    }
    return this.tokens[this.pos++];
  }

  peek(): Token {
    if (this.pos >= this.tokens.length) {
      return { type: TokenType.EOF, literal: '', line: 0 };
    }
    return this.tokens[this.pos];
  }

  private tokenize(input: string): void {
    const indentStack = [0];
    const lines = input.split('\n');

    for (let i = 0; i < lines.length; i++) {
      const lineNum = i + 1;
      const line = lines[i].trimEnd();
      if (!line) continue;

      const [indentLevel, charPos] = measureIndent(line);

      if (charPos < line.length && line[charPos] === '#') continue;

      const top = indentStack[indentStack.length - 1];
      if (indentLevel > top) {
        indentStack.push(indentLevel);
        this.emit({ type: TokenType.INDENT, literal: 'INDENT', line: lineNum });
      } else if (indentLevel < top) {
        while (indentStack.length > 1 && indentStack[indentStack.length - 1] > indentLevel) {
          indentStack.pop();
          this.emit({ type: TokenType.DEDENT, literal: 'DEDENT', line: lineNum });
        }
      }

      this.lexLine(line.slice(charPos), lineNum);
      this.emit({ type: TokenType.NEWLINE, literal: '\n', line: lineNum });
    }

    while (indentStack.length > 1) {
      indentStack.pop();
      this.emit({ type: TokenType.DEDENT, literal: 'DEDENT', line: 0 });
    }

    this.emit({ type: TokenType.EOF, literal: '', line: 0 });
  }

  private lexLine(line: string, lineNum: number): void {
    let pos = 0;
    while (pos < line.length) {
      const ch = line[pos];

      if (ch === ' ' || ch === '\t') { pos++; continue; }
      if (ch === '#') break;

      if (ch === '"') {
        pos++;
        const start = pos;
        while (pos < line.length && line[pos] !== '"') {
          if (line[pos] === '\\') pos++;
          pos++;
        }
        const s = line.slice(start, pos);
        if (pos < line.length) pos++;
        this.emit({ type: TokenType.STRING, literal: s, line: lineNum });
        continue;
      }

      if (/\d/.test(ch)) {
        const start = pos;
        while (pos < line.length && /[\d_]/.test(line[pos])) pos++;
        this.emit({ type: TokenType.INT, literal: line.slice(start, pos), line: lineNum });
        continue;
      }

      if (/[a-zA-Z_]/.test(ch)) {
        const start = pos;
        while (pos < line.length && /[a-zA-Z0-9_]/.test(line[pos])) pos++;
        const word = line.slice(start, pos);
        this.emit(toKeywordToken(word, lineNum));
        continue;
      }

      switch (ch) {
        case '(': this.emit({ type: TokenType.LPAREN,  literal: '(', line: lineNum }); break;
        case ')': this.emit({ type: TokenType.RPAREN,  literal: ')', line: lineNum }); break;
        case ',': this.emit({ type: TokenType.COMMA,   literal: ',', line: lineNum }); break;
        default:  this.emit({ type: TokenType.ILLEGAL, literal: ch,  line: lineNum }); break;
      }
      pos++;
    }
  }

  private emit(token: Token): void {
    this.tokens.push(token);
  }
}

function measureIndent(line: string): [number, number] {
  let level = 0;
  let pos = 0;
  while (pos < line.length) {
    if (line[pos] === ' ')      { level++;                            pos++; }
    else if (line[pos] === '\t') { level = Math.floor(level / 4 + 1) * 4; pos++; }
    else break;
  }
  return [level, pos];
}

function toKeywordToken(word: string, line: number): Token {
  switch (word) {
    case 'fn':     return { type: TokenType.KW_FN,     literal: word, line };
    case 'as':     return { type: TokenType.KW_AS,     literal: word, line };
    case 'return': return { type: TokenType.KW_RETURN, literal: word, line };
    default:       return { type: TokenType.IDENT,     literal: word, line };
  }
}
