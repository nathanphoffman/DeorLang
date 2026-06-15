export enum TokenType {
  EOF,
  ILLEGAL,
  NEWLINE,
  INDENT,
  DEDENT,

  IDENT,
  STRING,
  INT,

  KW_FN,
  KW_AS,
  KW_RETURN,
  KW_IF,
  KW_ELSE,
  KW_AND,
  KW_OR,
  KW_NOT,
  KW_IS,

  LPAREN,
  RPAREN,
  COMMA,

  PLUS,
  MINUS,
  STAR,
  SLASH,
  PERCENT,
  EQUALS,
  GT,
  LT,
  GTE,
  LTE,
}

export interface Token {
  type: TokenType;
  literal: string;
  line: number;
}
