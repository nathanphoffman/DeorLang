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

  LPAREN,
  RPAREN,
  COMMA,
}

export interface Token {
  type: TokenType;
  literal: string;
  line: number;
}
