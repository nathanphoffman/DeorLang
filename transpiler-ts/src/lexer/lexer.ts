import { Token, TokenType } from './token';
import { scanString } from './kinds/string';
import { scanInt } from './kinds/int';
import { scanWord } from './kinds/word';
import { scanPunctuation } from './kinds/punctuation';
import { isWhitespace } from './kinds/whitespace';
import { isLineComment } from './kinds/comment';

export class Lexer {
  private tokens: Token[] = [];
  private pos = 0;

  constructor(input: string) {
    this.tokenize(input);
  }

  private getEOFTokenIfEOF(): Token | undefined {
    if (this.pos >= this.tokens.length) {
      return { type: TokenType.EOF, literal: '', line: 0 };
    }
  }

  nextToken(): Token {
    return this.getEOFTokenIfEOF() ?? this.tokens[this.pos++];
  }

  peek(): Token {
    return this.getEOFTokenIfEOF() ?? this.tokens[this.pos];
  }

  private tokenize(input: string): void {
    const indentStack = [0];
    const lines = input.split('\n');

    for (let i = 0; i < lines.length; i++) {
      const lineNum = i + 1;
      const line = lines[i].trimEnd();
      if (!line) continue;

      const indentLevel = measureIndent(line);

      // Indents are single characters = 1 tab, and indents start lines
      //  so the charPos indents leave off at is literally the number of tab-levels
      const charPos = indentLevel;

      const charPosIsInsideLine = charPos < line.length;
      if (charPosIsInsideLine && isLineComment(line[charPos])) continue;

      const top = indentStack[indentStack.length - 1];
      if (indentLevel > top) {
        indentStack.push(indentLevel);
        this.addToken({ type: TokenType.INDENT, literal: 'INDENT', line: lineNum });
      } else if (indentLevel < top) {
        while (indentStack.length > 1 && indentStack[indentStack.length - 1] > indentLevel) {
          indentStack.pop();
          this.addToken({ type: TokenType.DEDENT, literal: 'DEDENT', line: lineNum });
        }
      }

      this.lexLine(line.slice(charPos), lineNum);
      this.addToken({ type: TokenType.NEWLINE, literal: '\n', line: lineNum });
    }

    // add closing dedents & EOF token after all lines are read
    this.addClosingDedents(indentStack);
    this.addToken({ type: TokenType.EOF, literal: '', line: 0 });
  }

  private addClosingDedents(indentStack: number[]) {
    while (indentStack.length > 1) {
      indentStack.pop();
      this.addToken({ type: TokenType.DEDENT, literal: 'DEDENT', line: 0 });
    }
  }

  private lexLine(line: string, lineNum: number): void {
    let pos = 0;
    while (pos < line.length) {
      const ch = line[pos];

      if (isWhitespace(ch)) { pos++; continue; }
      if (isLineComment(ch)) break;

      const result =
        scanString(line, pos, lineNum) ??
        scanInt(line, pos, lineNum) ??
        scanWord(line, pos, lineNum) ??
        scanPunctuation(line, pos, lineNum);

      this.addToken(result.token);
      pos = result.newPos;
    }
  }

  private addToken(token: Token): void {
    this.tokens.push(token);
  }
}

function measureIndent(line: string): number {
  const TAB = '\t';
  let level = 0;
  while (level < line.length && line[level] === TAB) level++;
  return level;
}

