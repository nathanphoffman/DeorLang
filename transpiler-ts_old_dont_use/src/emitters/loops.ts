import * as AST from '../ast';

export function renderFor(
  node: AST.ForStmt,
  genStmt: (stmt: AST.Node) => string,
  genExpr: (node: AST.Node) => string,
  pad: string,
): string {
  const body = node.body.map(genStmt).join('');

  if (node.iterable.kind === 'ForRange') {
    const end = genExpr(node.iterable.end);
    return `${pad}for ${node.varName} in 0..${end} {\n${body}${pad}}\n`;
  }

  if (node.iterable.kind === 'ForExplicitRange') {
    const start = genExpr(node.iterable.start);
    const end = genExpr(node.iterable.end);
    return `${pad}for ${node.varName} in ${start}..${end} {\n${body}${pad}}\n`;
  }

  // ForCollection — move into loop (list is consumed); revisit when ownership tracking lands
  const source = genExpr(node.iterable.source);
  return `${pad}for ${node.varName} in ${source} {\n${body}${pad}}\n`;
}
