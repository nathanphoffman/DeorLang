package parser

import (
	"fmt"

	"deorlang/lexer"
)

type Parser struct {
	l       *lexer.Lexer
	current lexer.Token
	peek    lexer.Token
}

func New(l *lexer.Lexer) *Parser {
	p := &Parser{l: l}
	p.advance()
	p.advance()
	return p
}

func (p *Parser) advance() lexer.Token {
	prev := p.current
	p.current = p.peek
	p.peek = p.l.NextToken()
	return prev
}

func (p *Parser) expect(t lexer.TokenType) (lexer.Token, error) {
	if p.current.Type != t {
		return p.current, fmt.Errorf("line %d: expected %s, got %s (%q)", p.current.Line, t, p.current.Type, p.current.Literal)
	}
	return p.advance(), nil
}

func (p *Parser) ParseProgram() (*Program, error) {
	prog := &Program{}

	for p.current.Type != lexer.EOF {
		if p.current.Type == lexer.NEWLINE {
			p.advance()
			continue
		}

		decl, err := p.parseTopLevel()
		if err != nil {
			return nil, err
		}
		prog.Decls = append(prog.Decls, decl)
	}

	return prog, nil
}

func (p *Parser) parseTopLevel() (Node, error) {
	if p.current.Type == lexer.KW_FN {
		return p.parseFunctionDecl()
	}
	return nil, fmt.Errorf("line %d: unexpected token %q at top level", p.current.Line, p.current.Literal)
}

func (p *Parser) parseFunctionDecl() (*FunctionDecl, error) {
	p.advance() // consume 'fn'

	name, err := p.expect(lexer.IDENT)
	if err != nil {
		return nil, err
	}

	if _, err := p.expect(lexer.LPAREN); err != nil {
		return nil, err
	}

	var params []Param
	for p.current.Type != lexer.RPAREN && p.current.Type != lexer.EOF {
		typeTok, err := p.expect(lexer.IDENT)
		if err != nil {
			return nil, err
		}
		nameTok, err := p.expect(lexer.IDENT)
		if err != nil {
			return nil, err
		}
		params = append(params, Param{Type: typeTok.Literal, Name: nameTok.Literal})
		if p.current.Type == lexer.COMMA {
			p.advance()
		}
	}

	if _, err := p.expect(lexer.RPAREN); err != nil {
		return nil, err
	}

	if _, err := p.expect(lexer.NEWLINE); err != nil {
		return nil, err
	}

	if _, err := p.expect(lexer.INDENT); err != nil {
		return nil, err
	}

	body, err := p.parseBlock()
	if err != nil {
		return nil, err
	}

	return &FunctionDecl{
		Name:   name.Literal,
		Params: params,
		Body:   body,
	}, nil
}

func (p *Parser) parseBlock() ([]Node, error) {
	var stmts []Node

	for p.current.Type != lexer.DEDENT && p.current.Type != lexer.EOF {
		if p.current.Type == lexer.NEWLINE {
			p.advance()
			continue
		}

		stmt, err := p.parseStatement()
		if err != nil {
			return nil, err
		}
		stmts = append(stmts, stmt)
	}

	if p.current.Type == lexer.DEDENT {
		p.advance()
	}

	return stmts, nil
}

func (p *Parser) parseStatement() (Node, error) {
	if p.current.Type != lexer.IDENT {
		return nil, fmt.Errorf("line %d: expected identifier to start statement, got %s (%q)", p.current.Line, p.current.Type, p.current.Literal)
	}

	ident := p.advance()

	switch p.current.Type {
	case lexer.KW_AS:
		p.advance()
		val, err := p.parseExpr()
		if err != nil {
			return nil, err
		}
		p.skipNewline()
		return &AsBinding{Name: ident.Literal, Value: val}, nil

	case lexer.LPAREN:
		p.advance()
		args, err := p.parseArgList()
		if err != nil {
			return nil, err
		}
		if _, err := p.expect(lexer.RPAREN); err != nil {
			return nil, err
		}
		p.skipNewline()
		return &CallStmt{Func: ident.Literal, Args: args}, nil
	}

	return nil, fmt.Errorf("line %d: unexpected token after identifier %q: %s", p.current.Line, ident.Literal, p.current.Type)
}

func (p *Parser) parseArgList() ([]Node, error) {
	var args []Node
	for p.current.Type != lexer.RPAREN && p.current.Type != lexer.EOF {
		arg, err := p.parseExpr()
		if err != nil {
			return nil, err
		}
		args = append(args, arg)
		if p.current.Type == lexer.COMMA {
			p.advance()
		}
	}
	return args, nil
}

func (p *Parser) parseExpr() (Node, error) {
	switch p.current.Type {
	case lexer.STRING:
		tok := p.advance()
		return &StringLiteral{Value: tok.Literal}, nil
	case lexer.INT:
		tok := p.advance()
		return &IntLiteral{Value: tok.Literal}, nil
	case lexer.IDENT:
		tok := p.advance()
		return &Identifier{Name: tok.Literal}, nil
	}
	return nil, fmt.Errorf("line %d: unexpected token in expression: %s (%q)", p.current.Line, p.current.Type, p.current.Literal)
}

func (p *Parser) skipNewline() {
	if p.current.Type == lexer.NEWLINE {
		p.advance()
	}
}
