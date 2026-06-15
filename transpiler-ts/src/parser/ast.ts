export type Node =
  | Program
  | FunctionDecl
  | AsBinding
  | CallStmt
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

export interface CallStmt {
  kind: 'CallStmt';
  func: string;
  args: Node[];
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
