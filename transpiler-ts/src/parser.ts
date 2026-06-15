import { Lexer } from './lexer';
import { Token, TokenType } from './token';
import * as AST from './ast';

export class Parser {
  private current: Token;
  private peekToken: Token;

  constructor(private lexer: Lexer) {

    // purely to avoid nulls, just default EOF -- as no processing = EOF = none
    const defaultEOF = { type: TokenType.EOF, literal: '', line: 0 };
    this.current = defaultEOF;
    this.peekToken = defaultEOF;

    // 2 advances required to load the 2nd token as current, and 1st as peekToken
    this.advance();
    this.advance();
  }

  private advance(): Token {
    const prev = this.current;
    this.current = this.peekToken;
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

  // entry point — collects all top-level declarations until EOF
  parseProgram(): AST.Program {
    const decls: AST.Node[] = [];

    while (this.current.type !== TokenType.EOF) {
      if (this.current.type === TokenType.NEWLINE) { this.advance(); continue; }
      decls.push(this.parseTopLevel());
    }

    return { kind: 'Program', decls };
  }

  // routes to the correct top-level declaration parser
  private parseTopLevel(): AST.Node {
    if (this.current.type === TokenType.KW_FN)     return this.parseFunctionDecl();
    if (this.current.type === TokenType.KW_STRUCT)  return this.parseStructDecl();
    if (this.current.type === TokenType.KW_SHAPE)   return this.parseShapeDecl();
    if (this.current.type === TokenType.KW_ENUM)    return this.parseEnumDecl();
    throw new Error(`line ${this.current.line}: unexpected token ${JSON.stringify(this.current.literal)} at top level`);
  }

  // enum name\n INDENT variants DEDENT
  private parseEnumDecl(): AST.EnumDecl {
    this.advance(); // consume 'enum'
    const name = this.expect(TokenType.IDENT).literal;
    this.expect(TokenType.NEWLINE);
    this.expect(TokenType.INDENT);

    const variants: string[] = [];
    while (this.current.type !== TokenType.DEDENT && this.current.type !== TokenType.EOF) {
      if (this.current.type === TokenType.NEWLINE) { this.advance(); continue; }
      variants.push(this.expect(TokenType.IDENT).literal);
      this.skipNewline();
    }
    if (this.current.type === TokenType.DEDENT) this.advance();

    return { kind: 'EnumDecl', name, variants };
  }

  // struct Name\n INDENT fields DEDENT
  private parseStructDecl(): AST.StructDecl {
    this.advance(); // consume 'struct'
    const name = this.expect(TokenType.IDENT).literal;
    this.expect(TokenType.NEWLINE);
    this.expect(TokenType.INDENT);

    const fields: AST.FieldDecl[] = [];
    while (this.current.type !== TokenType.DEDENT && this.current.type !== TokenType.EOF) {
      if (this.current.type === TokenType.NEWLINE) { this.advance(); continue; }
      const type = this.expect(TokenType.IDENT).literal;
      const fieldName = this.expect(TokenType.IDENT).literal;
      this.skipNewline();
      fields.push({ type, name: fieldName });
    }
    if (this.current.type === TokenType.DEDENT) this.advance();

    return { kind: 'StructDecl', name, fields };
  }

  // shape name = list of ElemType
  private parseShapeDecl(): AST.ShapeDecl {
    this.advance(); // consume 'shape'
    const name = this.expect(TokenType.IDENT).literal;
    this.expect(TokenType.EQUALS);
    this.expect(TokenType.KW_LIST);
    this.expect(TokenType.KW_OF);
    const elemType = this.expect(TokenType.IDENT).literal;
    this.skipNewline();
    return { kind: 'ShapeDecl', name, elemType };
  }

  // fn [ReturnType] name(params) + indented body block
  // if two IDENTs appear before '(', the first is the return type
  private parseFunctionDecl(): AST.FunctionDecl {
    this.advance(); // consume 'fn'

    let returnType = '';
    if (this.current.type === TokenType.IDENT && this.peekToken.type === TokenType.IDENT) {
      returnType = this.advance().literal;
    }

    const name = this.expect(TokenType.IDENT).literal;
    this.expect(TokenType.LPAREN);

    const params = this.buildParams();

    this.expect(TokenType.RPAREN);
    this.expect(TokenType.NEWLINE);
    this.expect(TokenType.INDENT);

    const body = this.parseBlock();

    return { kind: 'FunctionDecl', name, returnType, params, body };
  }

  // comma-separated list of `Type name` pairs inside ( )
  private buildParams(): AST.Param[] {
    const params: AST.Param[] = [];
    while (this.current.type !== TokenType.RPAREN && this.current.type !== TokenType.EOF) {

      // right now user types and primitives are all the same structurally
      //  so there is little reason to treat them as anything but generic IDENT
      //   as mapping happens later in the process
      const type = this.expect(TokenType.IDENT).literal;
      const paramName = this.expect(TokenType.IDENT).literal;

      params.push({ type, name: paramName });
      if (this.current.type === TokenType.COMMA) this.advance();
    }
    return params;
  }

  // collects statements until DEDENT — represents one indented block
  private parseBlock(): AST.Node[] {
    const stmts: AST.Node[] = [];

    while (this.current.type !== TokenType.DEDENT && this.current.type !== TokenType.EOF) {
      if (this.current.type === TokenType.NEWLINE) { this.advance(); continue; }
      stmts.push(this.parseStatement());
    }

    if (this.current.type === TokenType.DEDENT) this.advance();
    return stmts;
  }

  // routes to the correct statement parser based on the leading token
  private parseStatement(): AST.Node {
    // rust block — verbatim Rust code; RUST_BLOCK token holds raw content
    if (this.current.type === TokenType.KW_RUST) {
      this.advance(); // consume 'rust'
      this.skipNewline();
      const content = this.expect(TokenType.RUST_BLOCK).literal;
      return { kind: 'RustBlock', content };
    }

    // fn inside a block is always an error in Deor — functions must be top-level
    if (this.current.type === TokenType.KW_FN) {
      throw new Error(
        `line ${this.current.line}: functions must be declared at the top level, not inside a block`
      );
    }

    // return expr
    if (this.current.type === TokenType.KW_RETURN) {
      this.advance();
      const value = this.parseExpr();
      this.skipNewline();
      return { kind: 'ReturnStmt', value };
    }

    // for loop
    if (this.current.type === TokenType.KW_FOR) {
      return this.parseFor();
    }

    // break
    if (this.current.type === TokenType.KW_BREAK) {
      this.advance();
      this.skipNewline();
      return { kind: 'BreakStmt' };
    }

    // continue
    if (this.current.type === TokenType.KW_CONTINUE) {
      this.advance();
      this.skipNewline();
      return { kind: 'ContinueStmt' };
    }

    // if / else if / else conditional
    if (this.current.type === TokenType.KW_IF) {
      return this.parseIf();
    }

    // multi-field destructuring: (a, b) in source
    if (this.current.type === TokenType.LPAREN) {
      return this.parseMultiDestructure();
    }

    if (this.current.type !== TokenType.IDENT) {
      throw new Error(
        `line ${this.current.line}: expected identifier to start statement, got ${TokenType[this.current.type]}`
      );
    }

    const ident = this.advance();
    const currentType = this.current.type as TokenType;

    // inferred binding: name as expr  OR  name as (f1, f2) struct construction
    if (currentType === TokenType.KW_AS) {
      this.advance(); // consume 'as'
      // as (f1, f2, ...) — struct construction: all fields must be identifiers
      if (this.current.type === TokenType.LPAREN) {
        this.advance(); // consume '('
        const fields: string[] = [];
        while (this.current.type !== TokenType.RPAREN && this.current.type !== TokenType.EOF) {
          fields.push(this.expect(TokenType.IDENT).literal);
          if (this.current.type === TokenType.COMMA) this.advance();
        }
        this.expect(TokenType.RPAREN);
        this.skipNewline();
        return { kind: 'StructConstruct', name: ident.literal, fields };
      }
      // regular as-binding from a literal or expression
      const value = this.parseExpr();
      this.skipNewline();
      return { kind: 'AsBinding', name: ident.literal, value };
    }

    // function call statement: name(args)
    if (currentType === TokenType.LPAREN) {
      this.advance();
      const args = this.parseArgList();
      this.expect(TokenType.RPAREN);
      this.skipNewline();
      return { kind: 'CallStmt', func: ident.literal, args };
    }

    // single-field destructuring: name in source
    if (currentType === TokenType.KW_IN) {
      this.advance(); // consume 'in'
      const source = this.parseExpr();
      this.skipNewline();
      return { kind: 'DestructureStmt', fields: [ident.literal], source };
    }

    // list mutation: name insert expr
    if (currentType === TokenType.KW_INSERT) {
      this.advance(); // consume 'insert'
      const value = this.parseExpr();
      this.skipNewline();
      return { kind: 'InsertStmt', list: ident.literal, value };
    }

    // assignment: name = expr  (must come before typed binding check)
    if (currentType === TokenType.EQUALS) {
      this.advance(); // consume '='
      const value = this.parseExpr();
      this.skipNewline();
      return { kind: 'AssignStmt', name: ident.literal, value };
    }

    // typed declaration: Type name = expr
    if (currentType === TokenType.IDENT && this.peekToken.type === TokenType.EQUALS) {
      const varType = ident.literal;
      const name = this.advance().literal; // consume variable name
      this.advance();                       // consume =
      const value = this.parseExpr();
      this.skipNewline();
      return { kind: 'TypedBinding', varType, name, value };
    }

    throw new Error(
      `line ${this.current.line}: unexpected token after identifier ${JSON.stringify(ident.literal)}: ${TokenType[this.current.type]}`
    );
  }

  // for varName in iterable\n INDENT body DEDENT
  private parseFor(): AST.ForStmt {
    this.advance(); // consume 'for'
    const varName = this.expect(TokenType.IDENT).literal;
    this.expect(TokenType.KW_IN);

    let iterable: AST.ForIterable;

    // for i in range(n) — sugar for 0..n
    if (this.current.type === TokenType.IDENT && this.current.literal === 'range'
        && this.peekToken.type === TokenType.LPAREN) {
      this.advance(); // consume 'range'
      this.advance(); // consume '('
      const end = this.parseExpr();
      this.expect(TokenType.RPAREN);
      iterable = { kind: 'ForRange', end };
    }
    // for i in (start, end) — explicit range
    else if (this.current.type === TokenType.LPAREN) {
      this.advance(); // consume '('
      const start = this.parseExpr();
      this.expect(TokenType.COMMA);
      const end = this.parseExpr();
      this.expect(TokenType.RPAREN);
      iterable = { kind: 'ForExplicitRange', start, end };
    }
    // for x in collection — borrow iteration
    else {
      const source = this.parseExpr();
      iterable = { kind: 'ForCollection', source };
    }

    this.expect(TokenType.NEWLINE);
    this.expect(TokenType.INDENT);
    const body = this.parseBlock();

    return { kind: 'ForStmt', varName, iterable, body };
  }

  // (field1, field2) in source — extracts fields from a struct
  private parseMultiDestructure(): AST.DestructureStmt {
    this.advance(); // consume '('
    const fields: string[] = [];
    while (this.current.type !== TokenType.RPAREN && this.current.type !== TokenType.EOF) {
      fields.push(this.expect(TokenType.IDENT).literal);
      if (this.current.type === TokenType.COMMA) this.advance();
    }
    this.expect(TokenType.RPAREN);
    this.expect(TokenType.KW_IN);
    const source = this.parseExpr();
    this.skipNewline();
    return { kind: 'DestructureStmt', fields, source };
  }

  // if condition\n INDENT body DEDENT, with optional else if / else branches
  private parseIf(): AST.IfStmt {
    this.advance(); // consume 'if'
    const condition = this.parseExpr();
    this.expect(TokenType.NEWLINE);
    this.expect(TokenType.INDENT);
    const thenBlock = this.parseBlock();

    const elseIfClauses: AST.ElseIfClause[] = [];
    let elseBlock: AST.Node[] | null = null;

    while (this.current.type === TokenType.KW_ELSE) {
      this.advance(); // consume 'else'
      const afterElse = this.current.type as TokenType;

      if (afterElse === TokenType.KW_IF) {
        // else if branch
        this.advance(); // consume 'if'
        const elseIfCondition = this.parseExpr();
        this.expect(TokenType.NEWLINE);
        this.expect(TokenType.INDENT);
        elseIfClauses.push({ condition: elseIfCondition, block: this.parseBlock() });
      } else {
        // plain else — must be last branch
        this.expect(TokenType.NEWLINE);
        this.expect(TokenType.INDENT);
        elseBlock = this.parseBlock();
        break;
      }
    }

    return { kind: 'IfStmt', condition, thenBlock, elseIfClauses, elseBlock };
  }

  // comma-separated expressions inside ( ) — used for function call arguments
  private parseArgList(): AST.Node[] {
    const args: AST.Node[] = [];
    while (this.current.type !== TokenType.RPAREN && this.current.type !== TokenType.EOF) {
      args.push(this.parseExpr());
      if (this.current.type === TokenType.COMMA) this.advance();
    }
    return args;
  }

  // parses an expression — handles binary ops by checking for an operator after the left side
  // precedence: arithmetic/comparison first, then and/or chains
  private parseExpr(): AST.Node {
    let left = this.parseBinaryOp();

    // and/or have lower precedence than comparisons — loop for chaining
    while (this.current.type === TokenType.KW_AND || this.current.type === TokenType.KW_OR) {
      const op = this.advance().literal; // 'and' or 'or'
      const right = this.parseBinaryOp();
      left = { kind: 'BinaryExpr', left, op, right };
    }

    return left;
  }

  // parses arithmetic and comparison operations: left op right
  private parseBinaryOp(): AST.Node {
    const left = this.parsePrimary();

    // is not — two-word operator, must be checked before plain 'is'
    if (this.current.type === TokenType.KW_IS && this.peekToken.type === TokenType.KW_NOT) {
      this.advance(); // consume 'is'
      this.advance(); // consume 'not'
      const right = this.parsePrimary();
      return { kind: 'BinaryExpr', left, op: 'is not', right };
    }

    if (isOperator(this.current.type)) {
      const op = this.advance().literal;
      const right = this.parsePrimary();
      return { kind: 'BinaryExpr', left, op, right };
    }

    return left;
  }

  // parses a single value — a literal, identifier, prefix unary, or list literal
  private parsePrimary(): AST.Node {
    // not expr — prefix unary, binds to the immediately following primary
    if (this.current.type === TokenType.KW_NOT) {
      this.advance();
      const operand = this.parsePrimary();
      return { kind: 'UnaryExpr', op: 'not', operand };
    }

    // [ ] or [ items ] — list literal
    if (this.current.type === TokenType.LBRACKET) {
      this.advance(); // consume '['
      if (this.current.type === TokenType.RBRACKET) {
        this.advance(); // consume ']'
        return { kind: 'EmptyList' };
      }
      const items: AST.Node[] = [];
      while (this.current.type !== TokenType.RBRACKET && this.current.type !== TokenType.EOF) {
        items.push(this.parseExpr());
        if (this.current.type === TokenType.COMMA) this.advance();
      }
      this.expect(TokenType.RBRACKET);
      return { kind: 'ListLiteral', items };
    }

    switch (this.current.type) {
      case TokenType.STRING:   return { kind: 'StringLiteral', value: this.advance().literal };
      case TokenType.INT:      return { kind: 'IntLiteral',    value: this.advance().literal };
      case TokenType.IDENT: {
        const name = this.advance().literal;
        // name(args) in expression position — function call as a value
        if (this.current.type === TokenType.LPAREN) {
          this.advance(); // consume '('
          const args = this.parseArgList();
          this.expect(TokenType.RPAREN);
          return { kind: 'CallExpr', func: name, args };
        }
        return { kind: 'Identifier', name };
      }
      case TokenType.KW_TRUE:  this.advance(); return { kind: 'BoolLiteral', value: true };
      case TokenType.KW_FALSE: this.advance(); return { kind: 'BoolLiteral', value: false };
      case TokenType.KW_NONE:  this.advance(); return { kind: 'NoneLiteral' };
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

function isOperator(type: TokenType): boolean {
  return type === TokenType.PLUS    ||
    type === TokenType.MINUS        ||
    type === TokenType.STAR         ||
    type === TokenType.SLASH        ||
    type === TokenType.PERCENT      ||
    type === TokenType.GT           ||
    type === TokenType.LT           ||
    type === TokenType.GTE          ||
    type === TokenType.LTE          ||
    type === TokenType.KW_IS;
}
