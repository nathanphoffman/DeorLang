import * as AST from '../parser/ast';

export class Generator {
  private out = '';

  generate(prog: AST.Program): string {
    for (const decl of prog.decls) {
      this.genDecl(decl);
    }
    return this.out;
  }

  private genDecl(node: AST.Node): void {
    if (node.kind === 'FunctionDecl') {
      this.genFunctionDecl(node);
    } else {
      throw new Error(`unknown top-level node: ${node.kind}`);
    }
  }

  private genFunctionDecl(fn: AST.FunctionDecl): void {
    const params = fn.params.map(p => `${p.name}: ${rustType(p.type)}`).join(', ');
    const ret    = fn.returnType ? ` -> ${rustType(fn.returnType)}` : '';
    this.out += `fn ${fn.name}(${params})${ret} {\n`;
    for (const stmt of fn.body) {
      this.genStmt(stmt, 1);
    }
    this.out += '}\n';
  }

  private genStmt(node: AST.Node, depth: number): void {
    const pad = '    '.repeat(depth);

    if (node.kind === 'AsBinding') {
      const val = this.genExpr(node.value);
      if (node.value.kind === 'StringLiteral') {
        this.out += `${pad}let ${node.name} = ${val}.to_string();\n`;
      } else {
        this.out += `${pad}let ${node.name} = ${val};\n`;
      }
    } else if (node.kind === 'CallStmt') {
      const args = node.args.map(a => this.genExpr(a));
      if (node.func === 'print') {
        this.out += args.length === 1
          ? `${pad}println!("{}", ${args[0]});\n`
          : `${pad}println!(${args.join(', ')});\n`;
      } else {
        this.out += `${pad}${node.func}(${args.join(', ')});\n`;
      }
    } else {
      throw new Error(`unknown statement node: ${(node as AST.Node).kind}`);
    }
  }

  private genExpr(node: AST.Node): string {
    switch (node.kind) {
      case 'StringLiteral': return JSON.stringify(node.value);
      case 'IntLiteral':    return node.value;
      case 'Identifier':    return node.name;
      default:
        throw new Error(`unknown expression node: ${(node as AST.Node).kind}`);
    }
  }
}

function rustType(t: string): string {
  switch (t) {
    case 'int':    return 'i32';
    case 'float':  return 'f64';
    case 'bool':   return 'bool';
    case 'string': return 'String';
    default:       return t;
  }
}
