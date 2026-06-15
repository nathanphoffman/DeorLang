export type Node =
  | Program
  | FunctionDecl
  | AsBinding
  | TypedBinding
  | AssignStmt
  | ReturnStmt
  | CallStmt
  | CallExpr
  | IfStmt
  | ForStmt
  | DestructureStmt
  | BreakStmt
  | ContinueStmt
  | BinaryExpr
  | UnaryExpr
  | StringLiteral
  | IntLiteral
  | BoolLiteral
  | NoneLiteral
  | Identifier;

export interface Program {
  kind: 'Program';
  decls: Node[];
}

export interface FunctionDecl {
  kind: 'FunctionDecl';
  name: string;
  returnType: string;
  params: Param[];
  body: Node[];
}

export interface Param {
  type: string;
  name: string;
}

export interface AsBinding {
  kind: 'AsBinding';
  name: string;
  value: Node;
}

export interface TypedBinding {
  kind: 'TypedBinding';
  varType: string;
  name: string;
  value: Node;
}

export interface AssignStmt {
  kind: 'AssignStmt';
  name: string;
  value: Node;
}

export interface ReturnStmt {
  kind: 'ReturnStmt';
  value: Node;
}

export interface CallStmt {
  kind: 'CallStmt';
  func: string;
  args: Node[];
}

// function call used as a value inside an expression
export interface CallExpr {
  kind: 'CallExpr';
  func: string;
  args: Node[];
}

export interface ElseIfClause {
  condition: Node;
  block: Node[];
}

export interface IfStmt {
  kind: 'IfStmt';
  condition: Node;
  thenBlock: Node[];
  elseIfClauses: ElseIfClause[];
  elseBlock: Node[] | null;
}

// three forms of for-loop iterable:
//   for x in list_expr       → ForCollection
//   for i in range(n)        → ForRange (sugar for 0..n)
//   for i in (start, end)    → ForExplicitRange
export type ForIterable =
  | { kind: 'ForCollection'; source: Node }
  | { kind: 'ForRange'; end: Node }
  | { kind: 'ForExplicitRange'; start: Node; end: Node };

export interface ForStmt {
  kind: 'ForStmt';
  varName: string;
  iterable: ForIterable;
  body: Node[];
}

export interface DestructureStmt {
  kind: 'DestructureStmt';
  fields: string[];
  source: Node;
}

export interface BreakStmt {
  kind: 'BreakStmt';
}

export interface ContinueStmt {
  kind: 'ContinueStmt';
}

export interface BinaryExpr {
  kind: 'BinaryExpr';
  left: Node;
  op: string;
  right: Node;
}

export interface UnaryExpr {
  kind: 'UnaryExpr';
  op: string;
  operand: Node;
}

export interface StringLiteral {
  kind: 'StringLiteral';
  value: string;
}

export interface IntLiteral {
  kind: 'IntLiteral';
  value: string;
}

export interface BoolLiteral {
  kind: 'BoolLiteral';
  value: boolean;
}

export interface NoneLiteral {
  kind: 'NoneLiteral';
}

export interface Identifier {
  kind: 'Identifier';
  name: string;
}
