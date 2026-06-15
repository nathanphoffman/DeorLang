package codegen

import (
	"fmt"
	"strings"

	"deorlang/parser"
)

type Generator struct {
	buf strings.Builder
}

func New() *Generator {
	return &Generator{}
}

func (g *Generator) Generate(prog *parser.Program) (string, error) {
	for _, decl := range prog.Decls {
		if err := g.genDecl(decl); err != nil {
			return "", err
		}
	}
	return g.buf.String(), nil
}

func (g *Generator) genDecl(node parser.Node) error {
	switch n := node.(type) {
	case *parser.FunctionDecl:
		return g.genFunctionDecl(n)
	default:
		return fmt.Errorf("unknown top-level node: %T", node)
	}
}

func (g *Generator) genFunctionDecl(fn *parser.FunctionDecl) error {
	var paramStrs []string
	for _, p := range fn.Params {
		paramStrs = append(paramStrs, fmt.Sprintf("%s: %s", p.Name, rustType(p.Type)))
	}

	returnSig := ""
	if fn.ReturnType != "" {
		returnSig = " -> " + rustType(fn.ReturnType)
	}

	g.buf.WriteString(fmt.Sprintf("fn %s(%s)%s {\n", fn.Name, strings.Join(paramStrs, ", "), returnSig))

	for _, stmt := range fn.Body {
		if err := g.genStmt(stmt, 1); err != nil {
			return err
		}
	}

	g.buf.WriteString("}\n")
	return nil
}

func (g *Generator) genStmt(node parser.Node, depth int) error {
	pad := strings.Repeat("    ", depth)

	switch n := node.(type) {
	case *parser.AsBinding:
		val, err := g.genExpr(n.Value)
		if err != nil {
			return err
		}
		if _, ok := n.Value.(*parser.StringLiteral); ok {
			g.buf.WriteString(fmt.Sprintf("%slet %s = %s.to_string();\n", pad, n.Name, val))
		} else {
			g.buf.WriteString(fmt.Sprintf("%slet %s = %s;\n", pad, n.Name, val))
		}

	case *parser.CallStmt:
		args, err := g.genExprList(n.Args)
		if err != nil {
			return err
		}
		if n.Func == "print" {
			if len(args) == 1 {
				g.buf.WriteString(fmt.Sprintf("%sprintln!(\"{}\", %s);\n", pad, args[0]))
			} else {
				g.buf.WriteString(fmt.Sprintf("%sprintln!(%s);\n", pad, strings.Join(args, ", ")))
			}
		} else {
			g.buf.WriteString(fmt.Sprintf("%s%s(%s);\n", pad, n.Func, strings.Join(args, ", ")))
		}

	default:
		return fmt.Errorf("unknown statement node: %T", node)
	}

	return nil
}

func (g *Generator) genExpr(node parser.Node) (string, error) {
	switch n := node.(type) {
	case *parser.StringLiteral:
		return fmt.Sprintf("%q", n.Value), nil
	case *parser.IntLiteral:
		return n.Value, nil
	case *parser.Identifier:
		return n.Name, nil
	default:
		return "", fmt.Errorf("unknown expression node: %T", node)
	}
}

func (g *Generator) genExprList(nodes []parser.Node) ([]string, error) {
	out := make([]string, 0, len(nodes))
	for _, n := range nodes {
		s, err := g.genExpr(n)
		if err != nil {
			return nil, err
		}
		out = append(out, s)
	}
	return out, nil
}

func rustType(t string) string {
	switch t {
	case "int":
		return "i32"
	case "float":
		return "f64"
	case "bool":
		return "bool"
	case "string":
		return "String"
	default:
		return t
	}
}
