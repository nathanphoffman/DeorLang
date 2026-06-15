package parser

// Node is the base interface for all AST nodes.
type Node interface {
	nodeMarker()
}

type Program struct {
	Decls []Node
}

func (p *Program) nodeMarker() {}

type FunctionDecl struct {
	Name       string
	ReturnType string // empty = void
	Params     []Param
	Body       []Node
}

func (f *FunctionDecl) nodeMarker() {}

type Param struct {
	Type string
	Name string
}

// AsBinding represents `name as expr` — type inferred from literal shape.
type AsBinding struct {
	Name  string
	Value Node
}

func (a *AsBinding) nodeMarker() {}

// CallStmt represents a function call used as a statement.
type CallStmt struct {
	Func string
	Args []Node
}

func (c *CallStmt) nodeMarker() {}

type StringLiteral struct {
	Value string
}

func (s *StringLiteral) nodeMarker() {}

type IntLiteral struct {
	Value string
}

func (i *IntLiteral) nodeMarker() {}

type Identifier struct {
	Name string
}

func (id *Identifier) nodeMarker() {}
