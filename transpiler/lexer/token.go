package lexer

import "fmt"

type TokenType int

const (
	EOF TokenType = iota
	ILLEGAL
	NEWLINE
	INDENT
	DEDENT

	IDENT
	STRING
	INT

	KW_FN
	KW_AS
	KW_RETURN

	LPAREN
	RPAREN
	COMMA
)

var tokenNames = map[TokenType]string{
	EOF:      "EOF",
	ILLEGAL:  "ILLEGAL",
	NEWLINE:  "NEWLINE",
	INDENT:   "INDENT",
	DEDENT:   "DEDENT",
	IDENT:    "IDENT",
	STRING:   "STRING",
	INT:      "INT",
	KW_FN:    "fn",
	KW_AS:    "as",
	KW_RETURN: "return",
	LPAREN:   "(",
	RPAREN:   ")",
	COMMA:    ",",
}

func (t TokenType) String() string {
	if name, ok := tokenNames[t]; ok {
		return name
	}
	return fmt.Sprintf("TokenType(%d)", int(t))
}

type Token struct {
	Type    TokenType
	Literal string
	Line    int
}

func (t Token) String() string {
	return fmt.Sprintf("Token(%s, %q, line %d)", t.Type, t.Literal, t.Line)
}
