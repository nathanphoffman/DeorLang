export type Node =
  | Program
  | FunctionDecl
  | AsBinding
  | TypedBinding
  | CallStmt
  | IfStmt
  | BinaryExpr
  | StringLiteral
  | IntLiteral
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

export interface BinaryExpr {
  kind: 'BinaryExpr';
  left: Node;
  op: string;
  right: Node;
}

export interface CallStmt {
  kind: 'CallStmt';
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

export interface StringLiteral {
  kind: 'StringLiteral';
  value: string;
}

export interface IntLiteral {
  kind: 'IntLiteral';
  value: string;
}

export interface Identifier {
  kind: 'Identifier';
  name: string;
}
