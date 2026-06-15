import * as AST from './ast';
import { renderFunction, renderParam, renderRustType } from './emitters/function';
import { renderAsBinding, renderTypedBinding } from './emitters/binding';
import { renderCallStmt } from './emitters/builtins';
import { renderIf, mapOperator } from './emitters/if';
import { renderFor } from './emitters/loops';
import { renderDestructure } from './emitters/destructure';

export class Generator {
  private out = '';
  // names assigned to after declaration within the current function — declared as let mut
  private mutableNames: Set<string> = new Set();

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
    // pre-scan the function body to find all reassigned names so we can emit let mut
    this.mutableNames = collectMutableNames(fn.body);
    const params = fn.params.map(p => renderParam(p.name, p.type)).join(', ');
    const body = fn.body.map(stmt => this.genStmt(stmt, 1)).join('');
    this.out += renderFunction(fn.name, params, fn.returnType, body);
  }

  private genStmt(node: AST.Node, depth: number): string {
    const pad = '    '.repeat(depth);

    if (node.kind === 'AsBinding') {
      const val = this.genExpr(node.value);
      const isString = node.value.kind === 'StringLiteral';
      const isMut = this.mutableNames.has(node.name);
      return `${pad}${renderAsBinding(node.name, val, isString, isMut)}\n`;
    }

    if (node.kind === 'TypedBinding') {
      const rustType = renderRustType(node.varType);
      const val = this.genExpr(node.value);
      // none initializer signals a mutable Option<T> — will be reassigned later
      if (node.value.kind === 'NoneLiteral') {
        return `${pad}let mut ${node.name}: Option<${rustType}> = None;\n`;
      }
      const isMut = this.mutableNames.has(node.name);
      return `${pad}${renderTypedBinding(node.name, rustType, val, isMut)}\n`;
    }

    if (node.kind === 'AssignStmt') {
      const val = this.genExpr(node.value);
      return `${pad}${node.name} = ${val};\n`;
    }

    if (node.kind === 'ReturnStmt') {
      const val = this.genExpr(node.value);
      return `${pad}return ${val};\n`;
    }

    if (node.kind === 'CallStmt') {
      const args = node.args.map(a => this.genExpr(a));
      return `${pad}${renderCallStmt(node.func, args)}\n`;
    }

    if (node.kind === 'IfStmt') {
      const condition = this.genExpr(node.condition);
      const thenBlock = node.thenBlock.map(s => this.genStmt(s, depth + 1)).join('');
      const elseIfClauses = node.elseIfClauses.map(c => ({
        condition: this.genExpr(c.condition),
        block: c.block.map(s => this.genStmt(s, depth + 1)).join(''),
      }));
      const elseBlock = node.elseBlock
        ? node.elseBlock.map(s => this.genStmt(s, depth + 1)).join('')
        : null;
      return renderIf(condition, thenBlock, elseIfClauses, elseBlock, pad);
    }

    if (node.kind === 'ForStmt') {
      return renderFor(
        node,
        stmt => this.genStmt(stmt, depth + 1),
        expr => this.genExpr(expr),
        pad,
      );
    }

    if (node.kind === 'DestructureStmt') {
      const source = this.genExpr(node.source);
      return renderDestructure(node.fields, source, pad);
    }

    if (node.kind === 'BreakStmt')    return `${pad}break;\n`;
    if (node.kind === 'ContinueStmt') return `${pad}continue;\n`;

    throw new Error(`unknown statement node: ${(node as AST.Node).kind}`);
  }

  private genExpr(node: AST.Node): string {
    switch (node.kind) {
      case 'StringLiteral': return JSON.stringify(node.value);
      case 'IntLiteral':    return node.value;
      case 'BoolLiteral':   return node.value ? 'true' : 'false';
      case 'NoneLiteral':   return 'None';
      case 'Identifier':    return node.name;
      case 'UnaryExpr':     return `!${this.genExpr(node.operand)}`;
      case 'BinaryExpr':    return `${this.genExpr(node.left)} ${mapOperator(node.op)} ${this.genExpr(node.right)}`;
      case 'CallExpr':      return `${node.func}(${node.args.map(a => this.genExpr(a)).join(', ')})`;
      default:
        throw new Error(`unknown expression node: ${(node as AST.Node).kind}`);
    }
  }
}

// deep-scan a block for AssignStmt targets — names that must be declared let mut
function collectMutableNames(stmts: AST.Node[]): Set<string> {
  const names = new Set<string>();
  for (const stmt of stmts) {
    if (stmt.kind === 'AssignStmt') {
      names.add(stmt.name);
    } else if (stmt.kind === 'IfStmt') {
      for (const n of collectMutableNames(stmt.thenBlock)) names.add(n);
      for (const c of stmt.elseIfClauses) {
        for (const n of collectMutableNames(c.block)) names.add(n);
      }
      if (stmt.elseBlock) {
        for (const n of collectMutableNames(stmt.elseBlock)) names.add(n);
      }
    } else if (stmt.kind === 'ForStmt') {
      for (const n of collectMutableNames(stmt.body)) names.add(n);
    }
  }
  return names;
}
