import * as AST from '../parser/ast';
import { renderFunction, renderParam, renderRustType } from './emitters/function';
import { renderAsBinding, renderTypedBinding } from './emitters/binding';
import { renderCallStmt } from './emitters/builtins';
import { renderIf, mapOperator } from './emitters/if';

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
    const params = fn.params.map(p => renderParam(p.name, p.type)).join(', ');
    const body = fn.body.map(stmt => this.genStmt(stmt, 1)).join('');
    this.out += renderFunction(fn.name, params, fn.returnType, body);
  }

  private genStmt(node: AST.Node, depth: number): string {
    const pad = '    '.repeat(depth);

    if (node.kind === 'AsBinding') {
      const val = this.genExpr(node.value);
      const isString = node.value.kind === 'StringLiteral';
      return `${pad}${renderAsBinding(node.name, val, isString)}\n`;
    }

    if (node.kind === 'TypedBinding') {
      const rustType = renderRustType(node.varType);
      const val = this.genExpr(node.value);
      return `${pad}${renderTypedBinding(node.name, rustType, val)}\n`;
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

    throw new Error(`unknown statement node: ${(node as AST.Node).kind}`);
  }

  private genExpr(node: AST.Node): string {
    switch (node.kind) {
      case 'StringLiteral': return JSON.stringify(node.value);
      case 'IntLiteral': return node.value;
      case 'Identifier': return node.name;
      case 'BinaryExpr': return `${this.genExpr(node.left)} ${mapOperator(node.op)} ${this.genExpr(node.right)}`;
      default:
        throw new Error(`unknown expression node: ${(node as AST.Node).kind}`);
    }
  }
}
