import * as AST from './ast';
import { renderFunction, renderParam, renderRustType } from './emitters/function';
import { renderAsBinding, renderTypedBinding } from './emitters/binding';
import { renderCallStmt } from './emitters/builtins';
import { renderIf, mapOperator } from './emitters/if';
import { renderFor } from './emitters/loops';
import { renderDestructure } from './emitters/destructure';
import { renderStructDecl, renderStructConstruct } from './emitters/struct';

export class Generator {
  private out = '';
  // names assigned to after declaration within the current function — declared as let mut
  private mutableNames: Set<string> = new Set();
  // struct name → ordered field names, built from StructDecl nodes before codegen
  private structRegistry: Map<string, string[]> = new Map();
  // shape name (camelCase) → element type, built from ShapeDecl nodes before codegen
  private shapeRegistry: Map<string, string> = new Map();
  // enum name (camelCase) → Rust PascalCase name, e.g. colorTag → ColorTag
  private enumRegistry: Map<string, string> = new Map();
  // variant name → Rust enum name, e.g. Red → ColorTag
  private variantRegistry: Map<string, string> = new Map();

  generate(prog: AST.Program): string {
    // pre-scan: register all structs, shapes, and enums before generating any code
    for (const decl of prog.decls) {
      if (decl.kind === 'StructDecl') {
        this.structRegistry.set(decl.name, decl.fields.map(f => f.name));
      }
      if (decl.kind === 'ShapeDecl') {
        this.shapeRegistry.set(decl.name, decl.elemType);
      }
      if (decl.kind === 'EnumDecl') {
        const rustName = pascalCase(decl.name);
        this.enumRegistry.set(decl.name, rustName);
        for (const variant of decl.variants) {
          this.variantRegistry.set(variant, rustName);
        }
      }
    }

    for (const decl of prog.decls) {
      this.genDecl(decl);
    }
    return this.out;
  }

  private genDecl(node: AST.Node): void {
    if (node.kind === 'ShapeDecl') {
      const elemRust = renderRustType(node.elemType);
      const rustAlias = pascalCase(node.name);
      this.out += `type ${rustAlias} = Vec<${elemRust}>;\n\n`;
      return;
    }

    if (node.kind === 'EnumDecl') {
      const rustName = pascalCase(node.name);
      const variants = node.variants.map(v => `    ${v},`).join('\n');
      this.out += `#[derive(Clone, Copy, PartialEq, Debug)]\nenum ${rustName} {\n${variants}\n}\n\n`;
      return;
    }

    if (node.kind === 'StructDecl') {
      this.out += renderStructDecl(node.name, node.fields, t => this.resolveType(t)) + '\n';
      return;
    }

    if (node.kind === 'FunctionDecl') {
      this.genFunctionDecl(node);
      return;
    }

    throw new Error(`unknown top-level node: ${(node as AST.Node).kind}`);
  }

  private genFunctionDecl(fn: AST.FunctionDecl): void {
    // pre-scan the function body to find all reassigned names so we can emit let mut
    this.mutableNames = collectMutableNames(fn.body);
    const params = fn.params.map(p => renderParam(p.name, p.type, this.shapeRegistry, this.enumRegistry)).join(', ');
    const retType = fn.returnType ? this.resolveType(fn.returnType) : '';
    const body = fn.body.map(stmt => this.genStmt(stmt, 1)).join('');
    this.out += renderFunction(fn.name, params, retType, body);
  }

  private genStmt(node: AST.Node, depth: number): string {
    const pad = '    '.repeat(depth);

    if (node.kind === 'AsBinding') {
      const val = this.genExpr(node.value);
      const isString = node.value.kind === 'StringLiteral';
      const isMut = this.mutableNames.has(node.name);
      return `${pad}${renderAsBinding(node.name, val, isString, isMut)}\n`;
    }

    if (node.kind === 'StructConstruct') {
      const structName = this.resolveStructByFields(node.fields);
      const isMut = this.mutableNames.has(node.name);
      return `${pad}${renderStructConstruct(node.name, structName, node.fields, isMut)}\n`;
    }

    if (node.kind === 'TypedBinding') {
      const rustType = this.resolveType(node.varType);
      // none initializer → mutable Option<T>
      if (node.value.kind === 'NoneLiteral') {
        return `${pad}let mut ${node.name}: Option<${rustType}> = None;\n`;
      }
      // list initializer → always mutable
      if (node.value.kind === 'EmptyList') {
        return `${pad}let mut ${node.name}: ${rustType} = Vec::new();\n`;
      }
      let val = this.genExpr(node.value);
      if (node.value.kind === 'StringLiteral') val += '.to_string()';
      const isMut = this.mutableNames.has(node.name) || node.value.kind === 'ListLiteral';
      return `${pad}${renderTypedBinding(node.name, rustType, val, isMut)}\n`;
    }

    if (node.kind === 'AssignStmt') {
      const val = this.genExpr(node.value);
      return `${pad}${node.name} = ${val};\n`;
    }

    if (node.kind === 'InsertStmt') {
      const val = this.genExpr(node.value);
      return `${pad}${node.list}.push(${val});\n`;
    }

    if (node.kind === 'ReturnStmt') {
      const val = this.genExpr(node.value);
      return `${pad}return ${val};\n`;
    }

    if (node.kind === 'CallStmt') {
      const args = node.args.map(a => this.genCallArg(a));
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

    if (node.kind === 'RustBlock') {
      return node.content.split('\n')
        .map(line => line ? `${pad}${line}\n` : '\n')
        .join('');
    }

    throw new Error(`unknown statement node: ${(node as AST.Node).kind}`);
  }

  private genExpr(node: AST.Node): string {
    switch (node.kind) {
      case 'StringLiteral': return JSON.stringify(node.value);
      case 'IntLiteral':    return node.value;
      case 'BoolLiteral':   return node.value ? 'true' : 'false';
      case 'NoneLiteral':   return 'None';
      case 'EmptyList':     return 'Vec::new()';
      case 'ListLiteral':   return `vec![${node.items.map(i => this.genExpr(i)).join(', ')}]`;
      case 'Identifier': {
        const enumName = this.variantRegistry.get(node.name);
        if (enumName) return `${enumName}::${node.name}`;
        return node.name;
      }
      case 'UnaryExpr':     return `!${this.genExpr(node.operand)}`;
      case 'BinaryExpr':    return `${this.genExpr(node.left)} ${mapOperator(node.op)} ${this.genExpr(node.right)}`;
      case 'CallExpr':      return `${node.func}(${node.args.map(a => this.genCallArg(a)).join(', ')})`;
      default:
        throw new Error(`unknown expression node: ${(node as AST.Node).kind}`);
    }
  }

  // identifier args at call sites are auto-cloned to prevent use-after-move errors;
  // primitives (i32, bool, etc.) are Copy so the clone is a no-op at runtime
  private genCallArg(node: AST.Node): string {
    const val = this.genExpr(node);
    return node.kind === 'Identifier' ? `${val}.clone()` : val;
  }

  // resolve a Deor type name to its Rust equivalent
  private resolveType(deorType: string): string {
    const enumRustName = this.enumRegistry.get(deorType);
    if (enumRustName) return enumRustName;
    const elemType = this.shapeRegistry.get(deorType);
    if (elemType) return `Vec<${renderRustType(elemType)}>`;
    return renderRustType(deorType);
  }

  // find which struct in the registry has exactly these fields in this order
  private resolveStructByFields(fields: string[]): string {
    for (const [name, structFields] of this.structRegistry) {
      if (structFields.length === fields.length &&
          structFields.every((f, i) => f === fields[i])) {
        return name;
      }
    }
    throw new Error(`no struct found with fields: ${fields.join(', ')}`);
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

// camelCase → PascalCase for Rust type aliases (rollList → RollList)
function pascalCase(name: string): string {
  return name.charAt(0).toUpperCase() + name.slice(1);
}
