export type Node =
  | Program
  | StructDecl
  | ShapeDecl
  | EnumDecl
  | FunctionDecl
  | AsBinding
  | StructConstruct
  | TypedBinding
  | AssignStmt
  | InsertStmt
  | ReturnStmt
  | CallStmt
  | CallExpr
  | IfStmt
  | ForStmt
  | DestructureStmt
  | BreakStmt
  | ContinueStmt
  | RustBlock
  | IndexWriteStmt
  | IndexAppendStmt
  | RemoveStmt
  | BinaryExpr
  | UnaryExpr
  | StringLiteral
  | IntLiteral
  | BoolLiteral
  | NoneLiteral
  | EmptyList
  | ListLiteral
  | IndexExpr
  | Identifier;

export interface Program {
  kind: 'Program';
  decls: Node[];
}

// --- top-level declarations ---

export interface FieldDecl {
  type: string;
  name: string;
}

export interface StructDecl {
  kind: 'StructDecl';
  name: string;
  fields: FieldDecl[];
}

export interface ShapeDecl {
  kind: 'ShapeDecl';
  name: string;     // camelCase Deor name, e.g. rollList
  elemType: string; // element type, e.g. Roll
}

export interface EnumDecl {
  kind: 'EnumDecl';
  name: string;       // camelCase Deor name, e.g. colorTag
  variants: string[]; // PascalCase variant names, e.g. ['Red', 'Green', 'Blue']
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

// --- statements ---

export interface AsBinding {
  kind: 'AsBinding';
  name: string;
  value: Node;
}

// name as (field1, field2) — struct construction from fields already in scope
export interface StructConstruct {
  kind: 'StructConstruct';
  name: string;
  fields: string[];
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

// list insert item — append to list
export interface InsertStmt {
  kind: 'InsertStmt';
  list: string;
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

// list at idx = val — index write
export interface IndexWriteStmt {
  kind: 'IndexWriteStmt';
  list: string;
  index: Node;
  value: Node;
}

// list at end = val — append (same as push)
export interface IndexAppendStmt {
  kind: 'IndexAppendStmt';
  list: string;
  value: Node;
}

// list remove at idx — remove element at index
export interface RemoveStmt {
  kind: 'RemoveStmt';
  list: string;
  index: Node;
}

// verbatim Rust code block — content has Deor base indent stripped, relative indentation preserved
export interface RustBlock {
  kind: 'RustBlock';
  content: string;
}

// --- expressions ---

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

export interface EmptyList {
  kind: 'EmptyList';
}

export interface ListLiteral {
  kind: 'ListLiteral';
  items: Node[];
}

// list at idx — index read expression
export interface IndexExpr {
  kind: 'IndexExpr';
  list: Node;
  index: Node;
}

export interface Identifier {
  kind: 'Identifier';
  name: string;
}
