package main

import (
	"fmt"
	"os"

	"deorlang/codegen"
	"deorlang/lexer"
	"deorlang/parser"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Fprintf(os.Stderr, "usage: deor <input.deor> <output.rs>\n")
		os.Exit(1)
	}

	inputPath := os.Args[1]
	outputPath := os.Args[2]

	src, err := os.ReadFile(inputPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error reading %s: %v\n", inputPath, err)
		os.Exit(1)
	}

	l := lexer.New(string(src))
	p := parser.New(l)

	prog, err := p.ParseProgram()
	if err != nil {
		fmt.Fprintf(os.Stderr, "parse error: %v\n", err)
		os.Exit(1)
	}

	gen := codegen.New()
	rustCode, err := gen.Generate(prog)
	if err != nil {
		fmt.Fprintf(os.Stderr, "codegen error: %v\n", err)
		os.Exit(1)
	}

	if err := os.WriteFile(outputPath, []byte(rustCode), 0644); err != nil {
		fmt.Fprintf(os.Stderr, "error writing %s: %v\n", outputPath, err)
		os.Exit(1)
	}

	fmt.Printf("transpiled %s -> %s\n", inputPath, outputPath)
}
