package lexer

import (
	"strings"
	"unicode"
)

type Lexer struct {
	tokens []Token
	pos    int
}

func New(input string) *Lexer {
	l := &Lexer{}
	l.tokenize(input)
	return l
}

func (l *Lexer) NextToken() Token {
	if l.pos >= len(l.tokens) {
		return Token{Type: EOF}
	}
	t := l.tokens[l.pos]
	l.pos++
	return t
}

func (l *Lexer) Peek() Token {
	if l.pos >= len(l.tokens) {
		return Token{Type: EOF}
	}
	return l.tokens[l.pos]
}

func (l *Lexer) tokenize(input string) {
	indentStack := []int{0}

	lines := strings.Split(input, "\n")
	for lineNum, line := range lines {
		lineNum++ // 1-based

		// Strip trailing whitespace/CR
		line = strings.TrimRight(line, " \t\r")
		if line == "" {
			continue
		}

		// Count indentation (character position and space-equivalent level)
		indentLevel, charPos := measureIndent(line)

		// Skip comment-only lines
		if charPos < len(line) && line[charPos] == '#' {
			continue
		}

		// Emit INDENT or DEDENT tokens based on indent change
		top := indentStack[len(indentStack)-1]
		if indentLevel > top {
			indentStack = append(indentStack, indentLevel)
			l.emit(Token{Type: INDENT, Literal: "INDENT", Line: lineNum})
		} else if indentLevel < top {
			for len(indentStack) > 1 && indentStack[len(indentStack)-1] > indentLevel {
				indentStack = indentStack[:len(indentStack)-1]
				l.emit(Token{Type: DEDENT, Literal: "DEDENT", Line: lineNum})
			}
		}

		l.lexLine(line[charPos:], lineNum)
		l.emit(Token{Type: NEWLINE, Literal: "\n", Line: lineNum})
	}

	// Close any remaining open indent levels
	for len(indentStack) > 1 {
		indentStack = indentStack[:len(indentStack)-1]
		l.emit(Token{Type: DEDENT, Literal: "DEDENT"})
	}

	l.emit(Token{Type: EOF})
}

// measureIndent returns (space-equivalent indent level, byte position of first non-whitespace char).
func measureIndent(line string) (int, int) {
	level := 0
	pos := 0
	for pos < len(line) {
		switch line[pos] {
		case ' ':
			level++
			pos++
		case '\t':
			// Round up to next multiple of 4
			level = (level/4+1) * 4
			pos++
		default:
			return level, pos
		}
	}
	return level, pos
}

func (l *Lexer) lexLine(line string, lineNum int) {
	pos := 0
	for pos < len(line) {
		ch := line[pos]

		if ch == ' ' || ch == '\t' {
			pos++
			continue
		}

		if ch == '#' {
			break
		}

		if ch == '"' {
			pos++ // consume opening quote
			start := pos
			for pos < len(line) && line[pos] != '"' {
				if line[pos] == '\\' {
					pos++ // skip escape char
				}
				pos++
			}
			s := line[start:pos]
			if pos < len(line) {
				pos++ // consume closing quote
			}
			l.emit(Token{Type: STRING, Literal: s, Line: lineNum})
			continue
		}

		if unicode.IsDigit(rune(ch)) {
			start := pos
			for pos < len(line) && (unicode.IsDigit(rune(line[pos])) || line[pos] == '_') {
				pos++
			}
			l.emit(Token{Type: INT, Literal: line[start:pos], Line: lineNum})
			continue
		}

		if unicode.IsLetter(rune(ch)) || ch == '_' {
			start := pos
			for pos < len(line) && (unicode.IsLetter(rune(line[pos])) || unicode.IsDigit(rune(line[pos])) || line[pos] == '_') {
				pos++
			}
			word := line[start:pos]
			l.emit(keyword(word, lineNum))
			continue
		}

		switch ch {
		case '(':
			l.emit(Token{Type: LPAREN, Literal: "(", Line: lineNum})
		case ')':
			l.emit(Token{Type: RPAREN, Literal: ")", Line: lineNum})
		case ',':
			l.emit(Token{Type: COMMA, Literal: ",", Line: lineNum})
		default:
			l.emit(Token{Type: ILLEGAL, Literal: string(ch), Line: lineNum})
		}
		pos++
	}
}

func keyword(word string, line int) Token {
	switch word {
	case "fn":
		return Token{Type: KW_FN, Literal: word, Line: line}
	case "as":
		return Token{Type: KW_AS, Literal: word, Line: line}
	case "return":
		return Token{Type: KW_RETURN, Literal: word, Line: line}
	default:
		return Token{Type: IDENT, Literal: word, Line: line}
	}
}

func (l *Lexer) emit(t Token) {
	l.tokens = append(l.tokens, t)
}
