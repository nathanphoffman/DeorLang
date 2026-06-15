import { Lexer } from '../lexer/lexer';
import { Token, TokenType } from '../lexer/token';
import * as AST from './ast';

export class Parser {
  private current: Token;
  private peekToken: Token;

  constructor(private lexer: Lexer) {
    this.current  = { type: TokenType.EOF, literal: '', line: 0 };
    this.peekToken = { type: TokenType.EOF, literal: '', line: 0 };
    this.advance();
    this.advance();
  }

  private advance(): Token {
    const prev    = this.current;
    this.current  = this.peekToken;
    this.peekToken = this.lexer.nextToken();
    return prev;
  }

  private expect(type: TokenType): Token {
    if (this.current.type !== type) {
      throw new Error(
        `line ${this.current.line}: expected ${TokenType[type]}, got ${TokenType[this.current.type]} (${JSON.stringify(this.current.literal)})`
      );
    }
    return this.advance();
  }

  parseProgram(): AST.Program {
    const decls: AST.Node[] = [];

    while (this.current.type !== TokenType.EOF) {
      if (this.current.type === TokenType.NEWLINE) { this.advance(); continue; }
      decls.push(this.parseTopLevel());
    }

    return { kind: 'Program', decls };
  }

  private parseTopLevel(): AST.Node {
    if (this.current.type === TokenType.KW_FN) {
      return this.parseFunctionDecl();
    }
    throw new Error(`line ${this.current.line}: unexpected token ${JSON.stringify(this.current.literal)} at top level`);
  }

  private parseFunctionDecl(): AST.FunctionDecl {
    this.advance(); // consume 'fn'

    const name = this.expect(TokenType.IDENT).literal;
    this.expect(TokenType.LPAREN);

    const params: AST.Param[] = [];
    while (this.current.type !== TokenType.RPAREN && this.current.type !== TokenType.EOF) {
      const type     = this.expect(TokenType.IDENT).literal;
      const paramName = this.expect(TokenType.IDENT).literal;
      params.push({ type, name: paramName });
      if (this.current.type === TokenType.COMMA) this.advance();
    }

    this.expect(TokenType.RPAREN);
    this.expect(TokenType.NEWLINE);
    this.expect(TokenType.INDENT);

    const body = this.parseBlock();

    return { kind: 'FunctionDecl', name, returnType: '', params, body };
  }

  private parseBlock(): AST.Node[] {
    const stmts: AST.Node[] = [];

    while (this.current.type !== TokenType.DEDENT && this.current.type !== TokenType.EOF) {
      if (this.current.type === TokenType.NEWLINE) { this.advance(); continue; }
      stmts.push(this.parseStatement());
    }

    if (this.current.type === TokenType.DEDENT) this.advance();
    return stmts;
  }

  private parseStatement(): AST.Node {
    if (this.current.type !== TokenType.IDENT) {
      throw new Error(
        `line ${this.current.line}: expected identifier to start statement, got ${TokenType[this.current.type]}`
      );
    }

    const ident = this.advance();

    if (this.current.type === TokenType.KW_AS) {
      this.advance();
      const value = this.parseExpr();
      this.skipNewline();
      return { kind: 'AsBinding', name: ident.literal, value };
    }

    if (this.current.type === TokenType.LPAREN) {
      this.advance();
      const args = this.parseArgList();
      this.expect(TokenType.RPAREN);
      this.skipNewline();
      return { kind: 'CallStmt', func: ident.literal, args };
    }

    throw new Error(
      `line ${this.current.line}: unexpected token after identifier ${JSON.stringify(ident.literal)}: ${TokenType[this.current.type]}`
    );
  }

  private parseArgList(): AST.Node[] {
    const args: AST.Node[] = [];
    while (this.current.type !== TokenType.RPAREN && this.current.type !== TokenType.EOF) {
      args.push(this.parseExpr());
      if (this.current.type === TokenType.COMMA) this.advance();
    }
    return args;
  }

  private parseExpr(): AST.Node {
    switch (this.current.type) {
      case TokenType.STRING: return { kind: 'StringLiteral', value: this.advance().literal };
      case TokenType.INT:    return { kind: 'IntLiteral',    value: this.advance().literal };
      case TokenType.IDENT:  return { kind: 'Identifier',    name:  this.advance().literal };
      default:
        throw new Error(
          `line ${this.current.line}: unexpected token in expression: ${TokenType[this.current.type]} (${JSON.stringify(this.current.literal)})`
        );
    }
  }

  private skipNewline(): void {
    if (this.current.type === TokenType.NEWLINE) this.advance();
  }
}
