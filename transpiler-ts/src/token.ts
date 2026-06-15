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
  KW_FOR,
  KW_IN,
  KW_BREAK,
  KW_CONTINUE,
  KW_AND,
  KW_OR,
  KW_NOT,
  KW_IS,
  KW_TRUE,
  KW_FALSE,
  KW_NONE,
  KW_STRUCT,
  KW_SHAPE,
  KW_LIST,
  KW_OF,
  KW_INSERT,
  KW_RUST,
  KW_ENUM,

  RUST_BLOCK,

  LPAREN,
  RPAREN,
  LBRACKET,
  RBRACKET,
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
